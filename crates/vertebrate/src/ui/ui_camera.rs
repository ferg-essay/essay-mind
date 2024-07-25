use essay_ecs::{app::{App, Plugin, Startup, Update}, core::{Res, ResMut}};
use essay_graphics::layout::{Layout, View};
use essay_plot::api::{
    form::{Form, FormId, Matrix4}, renderer::{self, Drawable, Event, Renderer}, Angle, Bounds, Clip, Color, Point
};
use essay_tensor::Tensor;
use ui_graphics::{UiCanvas, UiCanvasPlugin};

use crate::{body::Body, retina, world::{World, WorldCell, WorldPlugin}};

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

    //fn pos(&self) -> Bounds<Canvas> {
    //    self.view.pos()
    //}
}

fn startup_camera(
    world: Res<World>,
    mut canvas: ResMut<UiCanvas>,
    mut camera: ResMut<UiCamera>,
) {
    let mut renderer = canvas.renderer_viewless();

    /*
    let mut form = Form::new();

    form.texture(renderer.create_texture_rgba8(&texture_colors(&[
        Color::from((0x00, 0x40, 0x40)),
        Color::from((0x00, 0x10, 0x10)),
        Color::from((0x00, 0x20, 0x20)),
        Color::from((0x00, 0x20, 0x30)),
        Color::from((0xc0, 0xc0, 0xc0)),

        Color::from((0xd0, 0xd0, 0xd0)),
        Color::from("green"),
        Color::from("green"),
        Color::from("green"),
        Color::from("green"),
    ])));

    let (width, height) = world.extent();

    let c_n = 0.05;
    let c_s = 0.15;
    let c_e = 0.25;
    let c_w = 0.35;

    let c_gl = 0.45;
    let c_gd = 0.55;
    let c_food = 0.65;

    for y in 0..height {
        wall(&mut form, (0., y as f32), (0., y as f32 + 1.), c_n);

        wall(&mut form, (width as f32, y as f32), (width as f32, y as f32 + 1.), c_s);
    }

    for x in 0..width {
        wall(&mut form, (x as f32, 0.), (x as f32 + 1., 0.), c_e);

        wall(&mut form, (x as f32, height as f32), (x as f32 + 1., height as f32), c_w);
        //wall(&mut form, (x as f32, 1.), (x as f32 + 1., 1.), 0.9);
    }

    for j in 0..height {
        for i in 0..width {
            match world[(i, j)] {
                WorldCell::Food => {
                    floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_food);                    
                },
                WorldCell::Wall => {
                    wall(&mut form, (i as f32, j as f32), (i as f32, j as f32 + 1.), c_n);                    
                    wall(&mut form, (i as f32 + 1., j as f32), (i as f32 + 1., j as f32 + 1.), c_s);                    
                    wall(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32), c_e);                    
                    wall(&mut form, (i as f32, j as f32 + 1.), (i as f32 + 1., j as f32 + 1.), c_w);                    
                },
                WorldCell::Empty => {
                    if (i + j) % 2 == 0 {
                        floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_gl);                    
                    } else {
                        floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_gd);                    

                    }
                },
                WorldCell::FloorLight => {},
                WorldCell::FloorDark => {},
            }
        }
    }
    */

    camera.view.write(|v| {
        v.form_id = Some(retina::world_form(&mut renderer, &world))
    });
}

fn wall(form: &mut Form, p0: impl Into<Point>, p1: impl Into<Point>, v: f32) {
    let Point(x0, z0) = p0.into();
    let Point(x1, z1) = p1.into();

    let y0 = 0.;
    let y1 = 1.;

    let u0 = 0.5;
    let u1 = 0.5;
    let v0 = v;
    let v1 = v;

    let vert = [
        form.vertex([x0, y0, -z0], [u0, v0]),
        form.vertex([x0, y1, -z0], [u0, v1]),
        form.vertex([x1, y0, -z1], [u1, v0]),
        form.vertex([x1, y1, -z1], [u1, v1])
    ];

    form.triangle([vert[0], vert[1], vert[3]]);
    form.triangle([vert[3], vert[2], vert[0]]);
}

fn floor(form: &mut Form, p0: impl Into<Point>, p1: impl Into<Point>, v: f32) {
    let Point(x0, z0) = p0.into();
    let Point(x1, z1) = p1.into();

    let y0 = 0.;
    // let y1 = 1.;

    let u0 = 0.5;
    let u1 = 0.5;
    let v0 = v;
    let v1 = v;

    let vert = [
        form.vertex([x0, y0, -z0], [u0, v0]),
        form.vertex([x0, y0, -z1], [u0, v1]),
        form.vertex([x1, y0, -z0], [u1, v0]),
        form.vertex([x1, y0, -z1], [u1, v1])
    ];

    form.triangle([vert[0], vert[1], vert[3]]);
    form.triangle([vert[3], vert[2], vert[0]]);
}

fn draw_camera(
    mut canvas: ResMut<UiCanvas>,
    body: Res<Body>,
    mut ui_camera: ResMut<UiCamera>,
) {
    let mut camera = Matrix4::eye();

    let body_pos = body.pos();
    let head_pos = body.head_pos();

    let body_dir = body.dir();
    let head_dir = body.head_dir();

    // camera = camera.translate(- body_pos.x(), -0.2, body_pos.y());
    camera = camera.translate(- head_pos.x(), -0.2, head_pos.y());
    camera = camera.rot_xz(Angle::Unit(- head_dir.to_unit()));
    // camera = self.mat.matmul(&camera);

    //let fov = 120.0f32;
    let fov = ui_camera.fov.to_radians_arc();
    camera = camera.projection(fov, 1., 0.01, 100.);

    ui_camera.view.write(|v| {
        v.camera = camera;
    });
    /*
    if let Some(mut renderer) = canvas.renderer_draw() {
        if let Some(form_id) = ui_camera.form_id {
            let mut camera = Matrix4::eye();

            let body_pos = body.pos();

            camera = camera.translate(- body_pos.x(), -0.2, body_pos.y());
            camera = camera.rot_xz(Angle::Unit(- body.dir().to_unit()));
            // camera = self.mat.matmul(&camera);

            //let fov = 120.0f32;
            let fov = ui_camera.fov.to_radians_arc();
            camera = camera.projection(fov, 1., 0.1, 100.);

            // let pos = ui_camera.pos();
            let pos = renderer.pos();

            let bounds = renderer.extent();
            let to = Matrix4::view_to_canvas_unit(&pos, bounds);
    
            let camera = to.matmul(&camera);

            renderer.draw_form(form_id, &camera, &Clip::Bounds(pos.p0(), pos.p1())).unwrap();
        }
    }
    */
}

struct UiCameraView {
    form_id: Option<FormId>,
    camera: Matrix4,
}

impl UiCameraView {
    fn new() -> Self {
        Self {
            // cube: cube_view(),
            camera: Matrix4::eye(),
            form_id: None,
        }
    }
}

impl Drawable for UiCameraView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        // self.cube.draw(renderer, pos);
        if let Some(form_id) = self.form_id {
            let pos = renderer.pos();

            let bounds = renderer.extent();
            let to = Matrix4::view_to_canvas_unit(&pos, bounds);
    
            let camera = to.matmul(&self.camera);

            // renderer.draw_form(form_id, &camera, &Clip::Bounds(pos.p0(), pos.p1())).unwrap();

            renderer.draw_form(form_id, &camera, &Clip::Bounds(pos.p0(), pos.p1()))?;            
        }

        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
        // self.cube.event(renderer, event);
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

            // if ! app.contains_plugin::<UiLayoutPlugin>() {
            //    app.plugin(UiLayoutPlugin);
            // }

            let view = app.resource_mut::<UiCanvas>().view(self.bounds.clone(), UiCameraView::new());

            // let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
            
            let mut ui_camera = UiCamera::new(view);
            ui_camera.fov(self.fov);
            app.insert_resource(ui_camera);

            app.system(Startup, startup_camera);
            // app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            app.system(Update, draw_camera);
            // app.system(PreUpdate, world_resize);
        }
    }
}

fn texture_colors(colors: &[Color]) -> Tensor<u8> {
    let mut vec = Vec::<[u8; 4]>::new();

    let size = 8;
    for color in colors {
        for _ in 0..size * size {
            vec.push(color.to_rgba_vec());
        }
    }

    Tensor::from(vec).reshape([colors.len() * size, size, 4])
}