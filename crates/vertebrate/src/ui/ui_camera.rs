use essay_ecs::{app::{App, Plugin, Startup, Update}, core::{Res, ResMut}};
use essay_graphics::layout::{Layout, View};
use essay_plot::api::{
    form::{FormId, Matrix4}, renderer::{Result, Drawable, Renderer}, Angle, Bounds, Clip,
};
use ui_graphics::{UiCanvas, UiCanvasPlugin};

use crate::{body::Body, retina::{self, Retina}, util::Point, world::{World, WorldPlugin}};

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

fn startup_camera(
    world: Res<World>,
    mut canvas: ResMut<UiCanvas>,
    mut camera: ResMut<UiCamera>,
) {
    let mut renderer = canvas.renderer_viewless();

    camera.view.write(|v| {
        v.form_id = Some(retina::world_form(&mut renderer, &world))
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
            let to = Matrix4::view_to_canvas_unit(&pos, bounds);
    
            let camera = to.matmul(&self.camera);

            renderer.draw_form(form_id, &camera)?;            
        }

        Ok(())
    }
}

pub struct UiCameraPlugin {
    bounds: Bounds::<Layout>,
    fov: Angle,
}

impl UiCameraPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            fov: Angle::Deg(90.),
        }
    }

    pub fn fov(mut self, fov: impl Into<Angle>) -> Self {
        self.fov = fov.into();

        self
    }
}

impl Plugin for UiCameraPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<WorldPlugin>());

            let view = app.resource_mut::<UiCanvas>().view(self.bounds.clone(), UiCameraView::new());
            
            let mut ui_camera = UiCamera::new(view);
            ui_camera.fov(self.fov);
            app.insert_resource(ui_camera);

            app.system(Startup, startup_camera);
            app.system(Update, draw_camera);
        }
    }
}
