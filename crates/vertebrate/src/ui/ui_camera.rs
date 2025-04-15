use essay_ecs::{app::{App, Plugin, Startup, Update}, core::{Res, ResMut}};
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::{
    form::{FormId, Matrix4}, renderer::{Result, Drawable, Renderer}, Angle,
};
use ui_graphics::{UiCanvas, ViewPlugin};

use crate::{
    body::Body, 
    retina::{self, Retina}, 
    world::World
};

fn startup_camera(
    world: Res<World>,
    mut canvas: ResMut<UiCanvas>,
    mut camera: ResMut<UiCamera>,
) {
    canvas.draw_viewless(|ui| {
       camera.view.write(|v| {
            v.form_id = Some(retina::world_form(ui, &world))
        });
    });
}

fn draw_camera(
    body: Res<Body>,
    mut ui_camera: ResMut<UiCamera>,
) {
    let mut camera = Matrix4::eye();

    let head_pos = body.head_pos();

    let head_dir = body.head_dir();

    camera = camera.translate(- head_pos.x(), - Retina::HEIGHT, head_pos.y());
    camera = camera.rot_xz(Angle::Unit(- head_dir.to_unit()));

    let fov = ui_camera.fov.to_radians_arc();
    camera = camera.projection(fov, 1., 0.01, 100.);

    ui_camera.view.write(|v| {
        v.camera = camera;
    });
}

struct UiCamera {
    view: View<UiCameraView>,

    fov: Angle,
}

impl UiCamera {
    fn new(view: View<UiCameraView>) -> Self {
        Self {
            view,
            fov: Angle::Deg(90.),
        }
    }

    fn fov(&mut self, angle: impl Into<Angle>) -> &mut Self {
        self.fov = angle.into();

        self
    }
}

struct UiCameraView {
    form_id: Option<FormId>,
    camera: Matrix4,
}

impl UiCameraView {
    fn new() -> Self {
        Self {
            camera: Matrix4::eye(),
            form_id: None,
        }
    }
}

impl Drawable for UiCameraView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        if let Some(form_id) = self.form_id {
            let pos = renderer.pos();

            let bounds = renderer.extent();
            let to = Matrix4::view_to_canvas_unit(pos, bounds);
    
            let camera = to.matmul(&self.camera);

            renderer.draw_form(form_id, &camera)?;            
        }

        Ok(())
    }
}

pub struct UiCameraPlugin {
    // bounds: Bounds::<Layout>,
    fov: Angle,

    view: Option<View<UiCameraView>>,
}

impl UiCameraPlugin {
    pub fn new() -> Self {
        Self {
            // bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            fov: Angle::Deg(90.),
            view: None,
        }
    }

    pub fn fov(mut self, fov: impl Into<Angle>) -> Self {
        self.fov = fov.into();

        self
    }
}

impl ViewPlugin for UiCameraPlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        self.view = Some(View::from(UiCameraView::new()));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl Plugin for UiCameraPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_resource::<World>());

        if let Some(view) = &self.view {
            let mut ui_camera = UiCamera::new(view.clone());
            ui_camera.fov(self.fov);
            app.insert_resource(ui_camera);

            app.system(Startup, startup_camera);
            app.system(Update, draw_camera);
        }
    }
}
