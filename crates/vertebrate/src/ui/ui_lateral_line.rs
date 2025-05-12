use renderer::{Canvas, Drawable, Renderer};
use essay_ecs::prelude::*;
use essay_graphics::{api, layout::{View, ViewArc}};
use essay_plot::prelude::*;

use ui_graphics::ViewPlugin;

use crate::{
    hind_brain::lateral_line::{LateralLine, Ray, Segment},
    util::Heading
};

fn update_ui_lateral_line(
    mut ui_lateral_line: ResMut<View<UiLateralLine>>,
    lateral_line: Res<LateralLine>,
) {
    ui_lateral_line.write(|v| {
        v.head_left_sensors = lateral_line.sensors(Segment::HeadLeft).clone();
        v.head_right_sensors = lateral_line.sensors(Segment::HeadRight).clone();
        v.tail_left_sensors = lateral_line.sensors(Segment::TailLeft).clone();
        v.tail_right_sensors = lateral_line.sensors(Segment::TailRight).clone();
    });
}

struct UiLateralLine {
    head_left_rays: Vec<UiRay>,
    head_right_rays: Vec<UiRay>,

    tail_left_rays: Vec<UiRay>,
    tail_right_rays: Vec<UiRay>,

    head_left_sensors: Vec<f32>,
    head_right_sensors: Vec<f32>,

    tail_left_sensors: Vec<f32>,
    tail_right_sensors: Vec<f32>,

    /// cached canvas bounds
    pos: Bounds<Canvas>,
}

impl UiLateralLine {
    fn new(
        head_rays: &Vec<Ray>,
        tail_rays: &Vec<Ray>,
    ) -> Self {
        let mut head_left_rays = Vec::<UiRay>::new();
        let mut head_right_rays = Vec::<UiRay>::new();

        for ray in head_rays {
            let heading = ray.heading();
            head_right_rays.push(UiRay::new(heading));

            let heading = Heading::Unit(- heading.to_unit());
            head_left_rays.push(UiRay::new(heading));
        }

        let head_left_sensors = head_left_rays.iter().map(|_| 0.).collect();
        let head_right_sensors = head_right_rays.iter().map(|_| 0.).collect();

        let mut tail_left_rays = Vec::<UiRay>::new();
        let mut tail_right_rays = Vec::<UiRay>::new();

        for ray in tail_rays {
            let heading = ray.heading();
            tail_right_rays.push(UiRay::new(heading));

            let heading = Heading::Unit(- heading.to_unit());
            tail_left_rays.push(UiRay::new(heading));
        }

        let tail_left_sensors = tail_left_rays.iter().map(|_| 0.).collect();
        let tail_right_sensors = tail_right_rays.iter().map(|_| 0.).collect();

        Self {
            head_left_rays,
            head_right_rays,

            tail_left_rays,
            tail_right_rays,

            head_left_sensors,
            head_right_sensors,

            tail_left_sensors,
            tail_right_sensors,

            pos: Bounds::zero(),
        }


    }
    
    fn resize(&mut self, pos: Bounds<Canvas>) {
        for ray in &mut self.head_left_rays {
            ray.resize(pos);
        }

        for ray in &mut self.head_right_rays {
            ray.resize(pos);
        }

        for ray in &mut self.tail_left_rays {
            ray.resize(pos);
        }

        for ray in &mut self.tail_right_rays {
            ray.resize(pos);
        }
    }
}

impl Drawable for UiLateralLine {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        if ui.pos() != self.pos {
            self.resize(ui.pos().with_aspect(1.));
        }

        let center = api::Point(ui.pos().xmid(), ui.pos().ymid());

        let head_color = Color::from("azure").with_alpha(0.25);
        let tail_color = Color::from("orange").with_alpha(0.25);

        let mut mesh = Mesh2d::new();
        draw_rays(&mut mesh, center, &self.head_left_rays, &self.head_left_sensors);
        ui.draw_mesh2d(&mesh, TextureId::default(), &[head_color.into()])?;

        let mut mesh = Mesh2d::new();
        draw_rays(&mut mesh, center, &self.head_right_rays, &self.head_right_sensors);
        ui.draw_mesh2d(&mesh, TextureId::default(), &[head_color.into()])?;

        let mut mesh = Mesh2d::new();
        draw_rays(&mut mesh, center, &self.tail_left_rays, &self.tail_left_sensors);
        ui.draw_mesh2d(&mesh, TextureId::default(), &[tail_color.into()])?;

        let mut mesh = Mesh2d::new();
        draw_rays(&mut mesh, center, &self.tail_right_rays, &self.tail_right_sensors);
        ui.draw_mesh2d(&mesh, TextureId::default(), &[tail_color.into()])?;

        //if self.cache_pos != ui.pos() {
        //    self.cache_pos = ui.pos().clone();
        //    self.resize(ui);
        //}
        Ok(())
    }
}

fn draw_rays(
    mesh: &mut Mesh2d,
    center: Point, 
    rays: &Vec<UiRay>,
    sensors: &Vec<f32>,
) {
    let mut prev: Option<(Point, Point)> = None; 

    for (ray, value) in rays.iter().zip(sensors.iter()) {
        let point = ray.value(center, *value);

        if let Some((prev_point, prev_max)) = prev {
            mesh.triangle(
                ray.point,
                point,
                prev_point,
            );

            mesh.triangle(
                ray.point,
                prev_max,
                prev_point,
            );
        }

        prev = Some((point, ray.point));
    }
}

struct UiRay {
    heading: Heading,

    point: api::Point,
}

impl UiRay {
    fn new(heading: Heading) -> Self {
        let (sin, cos) = heading.sin_cos();

        Self {
            heading,
            point: api::Point(cos, sin),
        }
    }

    fn resize(&mut self, pos: Bounds<Canvas>) {
        let (sin, cos) = self.heading.sin_cos();
        let (w2, h2) = (0.5 * pos.width(), 0.5 * pos.height());
        let (x0, y0) = (pos.xmid(), pos.ymid());

        self.point = [x0 + w2 * cos, y0 + h2 * sin].into();
    }

    fn value(&self, center: Point, value: f32) -> Point {
        self.point.interpolate(value, center)
    }
}

//
// UiLateralLinePlugin
//

pub struct UiLateralLinePlugin {
    view: Option<View<UiLateralLine>>,
}

impl UiLateralLinePlugin {
    pub fn new() -> Self {
        Self {
            view: None
        }
    }
}

impl ViewPlugin for UiLateralLinePlugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc> {
        if let Some(lateral_line) = app.get_resource::<LateralLine>() {
            self.view = Some(View::from(UiLateralLine::new(
                lateral_line.head_rays(),
                lateral_line.tail_rays()
            )));

            self.view.as_ref().map(|v| v.arc())
        } else {
            None
        }
    }
}

impl Plugin for UiLateralLinePlugin {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            app.insert_resource(view.clone());

            app.system(Update, update_ui_lateral_line);
        }
    }
}
