use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::Tensor;

use crate::world::World;

#[derive(Component)]
pub struct BodyLocomotion {
    pos: Point,

    dir: Angle,

    speed: f32,
    arrest: f32,

    muscle_left: f32,
    muscle_right: f32,

    touch_left: bool,
    touch_right: bool,
}

impl BodyLocomotion {
    const ARREST_DECAY : f32 = -0.1;
    const ARREST_THRESHOLD : f32 = 0.4;

    const MUSCLE_DECAY : f32 = -1.0;
    const MUSCLE_THRESHOLD : f32 = 0.2;

    const SPEED : f32 = 0.1; // speed in head-lengths

    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dir: Angle::Unit(0.),
            speed: 1.,
            arrest: 0.,

            muscle_left: 0.,
            muscle_right: 0.,

            touch_left: false,
            touch_right: false,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn pos_head(&self) -> Point {

        let Point(x, y) = self.pos;

        let (dy, dx) = self.dir.to_radians().sin_cos();

        // head location
        let head = Point(x + dx * 0.5, y + dy * 0.5);

        head
    }

    pub fn dir(&self) -> Angle {
        self.dir
    }

    pub fn muscle_left(&self) -> f32 {
        self.muscle_left
    }

    pub fn set_muscle_left(&mut self, muscle: f32) {
        // simulate refraction by only updating when zero.
        if self.muscle_left <= 0. {
            self.muscle_left = self.muscle_left.max(muscle).clamp(0., 1.);
        }
    }

    pub fn muscle_right(&self) -> f32 {
        self.muscle_right
    }

    pub fn set_muscle_right(&mut self, muscle: f32) {
        // simulate refraction by only updating when zero.
        if self.muscle_right <= 0. {
            self.muscle_right = self.muscle_right.max(muscle).clamp(0., 1.);
        }
    }

    pub fn touch_left(&self) -> bool {
        self.touch_left
    }

    pub fn touch_right(&self) -> bool {
        self.touch_right
    }

    pub fn _speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn arrest(&self) -> f32 {
        self.arrest
    }

    ///
    /// Stop the muco-cilia beating for a period of time
    /// 
    pub fn set_arrest(&mut self, time: f32) {
        if self.arrest <= 0. {
            self.arrest = time;
        }
    }

    ///
    /// Update the slugs's position based on the cilia movement
    /// 
    pub fn update(&mut self, world: &World) {
        let mut dir = self.dir.to_unit();

        if self.muscle_left > BodyLocomotion::MUSCLE_THRESHOLD {
            dir += 0.015 * (1. + 0.2 * (random() - 0.5));
        } else if self.muscle_right > BodyLocomotion::MUSCLE_THRESHOLD {
            dir -= 0.015 * (1. + 0.2 * (random() - 0.5));
        }

        let mut speed = self.speed * BodyLocomotion::SPEED;
        self.speed = 1.;

        // if cilia aren't arrested, move in the direction
        if BodyLocomotion::ARREST_THRESHOLD < self.arrest {
            speed = 0.;
        }

        // random noise into direction
        if speed > 0. && random() < 0.2 {
            if random() < 0.5 {
                dir += 0.005;
            } else {
                dir -= 0.005;
            }
        }

        self.dir = Angle::Unit((dir + 1.) % 1.);

        let Point(mut x, mut y) = self.pos;

        let (dy, dx) = self.dir.to_radians().sin_cos();

        // head location
        let head = Point(x + dx * 0.5, y + dy * 0.5);

        // sensor ahead and to the side
        let sensor_left = (head.0 + dx * 0.1 - dy * 0.1, head.1 + dy * 0.1 + dx * 0.1);
        self.touch_left = world.is_collide(sensor_left);

        let sensor_right = (head.0 + dx * 0.1 + dy * 0.1, head.1 + dy * 0.1 - dx * 0.1);
        self.touch_right = world.is_collide(sensor_right);

        x = (1. - speed) * x + speed * head.0;
        y = (1. - speed) * y + speed * head.1;

        if ! world.is_collide((x, y)) {
            self.pos = Point(x, y);
        } else if ! self.touch_left && ! self.touch_right {
            self.touch_left = true;
            self.touch_right = true;
        }

        self.dir = Angle::Unit((dir + 1.) % 1.);

        self.arrest = (self.arrest + BodyLocomotion::ARREST_DECAY).max(0.);
        self.muscle_left = (self.muscle_left + BodyLocomotion::MUSCLE_DECAY).max(0.);
        self.muscle_right = (self.muscle_right + BodyLocomotion::MUSCLE_DECAY).max(0.);
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}
