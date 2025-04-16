use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use essay_graphics::api;
use essay_plot::api::{affine2d, Affine2d};
use essay_tensor::tensor::Tensor;
use mind_ecs::Tick;

use crate::{
    body::Body, 
    util::{Heading, Point}, 
    world::World
};

fn update_lateral_line(
    mut lateral_line: ResMut<LateralLine>,
    body: Res<Body>,
    world: Res<World>,
) {
    lateral_line.update(body.as_ref(), world.as_ref());
}

pub struct LateralLine {
    head_rays: Vec<Ray>,
    tail_rays: Vec<Ray>,

    // rank-3 tensor
    // (rays, point in ray, [x, y])
    head_left_points: Tensor,
    head_right_points: Tensor,

    tail_left_points: Tensor,
    tail_right_points: Tensor,

    head_left_sensors: Vec<f32>,
    head_right_sensors: Vec<f32>,

    tail_left_sensors: Vec<f32>,
    tail_right_sensors: Vec<f32>,
}

impl LateralLine {
    pub fn head_rays(&self) -> &Vec<Ray> {
        &self.head_rays
    }

    pub fn tail_rays(&self) -> &Vec<Ray> {
        &self.tail_rays
    }
    
    fn update(
        &mut self, 
        body: &Body, 
        world: &World, 
    ) {
        let pos = body.pos();
        let heading = body.dir();

        let affine = affine2d::rotate(heading.to_turn().to_radians())
            .translate(pos.x(), pos.y()
        );

        self.head_left_sensors = update_sensors(&self.head_left_points, &affine, world);
        self.head_right_sensors = update_sensors(&self.head_right_points, &affine, world);

        self.tail_left_sensors = update_sensors(&self.tail_left_points, &affine, world);
        self.tail_right_sensors = update_sensors(&self.tail_right_points, &affine, world);
    }

    pub fn sensors(&self, segment: Segment) -> &Vec<f32> {
        match segment {
            Segment::HeadLeft => &self.head_left_sensors,
            Segment::HeadRight => &self.head_right_sensors,
            Segment::TailLeft => &self.tail_left_sensors,
            Segment::TailRight => &self.tail_right_sensors,
        }
    }
}

fn update_sensors(rays: &Tensor, affine: &Affine2d, world: &World) -> Vec<f32> {
    let mut sensors = Vec::new();

    let slice = rays.as_slice();
    let n_rays = rays.rows();
    for ray in 0..rays.dim(0) {
        let mut value: f32 = 0.;

        for sensor in 0..n_rays {
            let x = slice[2 * (ray * n_rays + sensor) + 0];
            let y = slice[2 * (ray * n_rays + sensor) + 1];

            let api::Point(x, y) = affine.transform_point([x, y].into());
            let pos = Point(x, y);

            if world.is_collide(pos) {
                value = value.max(((sensor + 1) as f32) / n_rays as f32);
            }
        }

        sensors.push(value);
    }

    sensors
}

struct SensorBuilder {
    _body_len: f32,
    ray_len: f32,

    points_per_ray: usize,

    rays: Vec<Ray>,

    left: Vec<Vec<[f32; 2]>>,
    right: Vec<Vec<[f32; 2]>>,
}

impl SensorBuilder {
    fn new(
        body_len: f32,
        ray_len: f32,

        points_per_ray: usize
    ) -> SensorBuilder {
        Self {
            _body_len: body_len,
            ray_len,

            points_per_ray,

            rays: Vec::new(),

            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn add_ray(&mut self, body_y: f32, heading: Heading) {
        self.rays.push(Ray::new(body_y, heading, self.points_per_ray));

        let (sin, cos) = heading.sin_cos();

        let f = self.ray_len * (self.points_per_ray as f32).recip();

        let mut left = Vec::<[f32; 2]>::new();
        let mut right = Vec::<[f32; 2]>::new();

        for i in 0..self.points_per_ray {
            // order is outside-in because a value of 0 represents not detected
            let f = f * (self.points_per_ray - i) as f32;
            let (x, y) = (f * cos, f * sin + body_y);

            right.push([x, y]);
            left.push([-x, y]);
        }

        self.left.push(left);
        self.right.push(right);
    }
}

impl From<(SensorBuilder, SensorBuilder)> for LateralLine {
    fn from((head, tail): (SensorBuilder, SensorBuilder)) -> Self {
        let head_left_rays = Tensor::from(&head.left);
        let head_right_rays = Tensor::from(&head.right);

        let tail_left_rays = Tensor::from(&tail.left);
        let tail_right_rays = Tensor::from(&tail.right);

        let mut head_left_sensors = Vec::new();
        head_left_sensors.resize(head_left_rays.dim(0), 0.0f32);

        let mut head_right_sensors = Vec::new();
        head_right_sensors.resize(head_right_rays.dim(0), 0.0f32);

        let mut tail_left_sensors = Vec::new();
        tail_left_sensors.resize(tail_left_rays.dim(0), 0.0f32);

        let mut tail_right_sensors = Vec::new();
        tail_right_sensors.resize(tail_right_rays.dim(0), 0.0f32);

        Self {
            head_rays: head.rays,
            tail_rays: tail.rays,

            head_left_points: head_left_rays,
            head_right_points: head_right_rays,

            tail_left_points: tail_left_rays,
            tail_right_points: tail_right_rays,

            head_left_sensors,
            head_right_sensors,

            tail_left_sensors,
            tail_right_sensors,
        }
    }
}

pub struct Ray {
    y: f32,
    heading: Heading,
    n: usize
}

impl Ray {
    fn new(y: f32, heading: Heading, n: usize) -> Ray {
        Ray {
            y,
            heading,
            n
        }
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    /// Heading is represented for right side
    pub fn heading(&self) -> Heading {
        self.heading
    }

    pub fn n(&self) -> usize {
        self.n
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    HeadLeft,
    HeadRight,
    TailLeft,
    TailRight,
}

pub struct LateralLine2Plugin;

impl Plugin for LateralLine2Plugin {
    fn build(&self, app: &mut App) {
        let mut head = SensorBuilder::new(0.25, 2., 3); 
        head.add_ray(0., Heading::Unit(0.));
        head.add_ray(0., Heading::Unit(0.125));
        head.add_ray(0., Heading::Unit(0.25));

        let mut tail = SensorBuilder::new(0.75, 2., 3); 
        tail.add_ray(0., Heading::Unit(0.25));
        tail.add_ray(0., Heading::Unit(0.375));
        tail.add_ray(0., Heading::Unit(0.50));

        let lateral_line = LateralLine::from((head, tail));

        app.insert_resource(lateral_line);

        app.system(Tick, update_lateral_line);
    }
}

#[cfg(test)]
mod test {
}