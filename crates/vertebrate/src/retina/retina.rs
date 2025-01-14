use essay_ecs::{app::{App, Plugin, Startup}, core::{Res, ResMut}};
use essay_graphics::api;
use essay_plot::{
    api::{
        form::{Form, FormId, Matrix4}, 
        renderer::{self, Canvas, Drawable, Event, Renderer, Result}, 
        Bounds, Color, Point 
    },
    wgpu::{wgpu::hardcopy::SurfaceId, WgpuHardcopy},
};
use essay_tensor::Tensor;
use mind_ecs::Tick;
use image::Pixel;

use crate::{body::Body, util::{Angle, Heading}, world::{World, Wall}};

fn retina_update(
    body: Res<Body>,
    mut retina: ResMut<Retina>
) {
    retina.draw_and_load(body.get());

    let light_left = if let Some(tensor) = &retina.data_left {
        tensor.reduce_mean(())[0]
    } else {
        0.
    };

    let light_right = if let Some(tensor) = &retina.data_right {
        tensor.reduce_mean(())[0]
    } else {
        0.
    };

    retina.brighten_left = (light_left - retina.light_left) / light_left.max(0.01);
    retina.brighten_right = (light_right - retina.light_right) / light_right.max(0.01);
    retina.light_left = light_left;
    retina.light_right = light_right;

    // println!("Avg {:.2}({:.2}) {:.2}({:.2})", 
    //    retina.light_left, retina.brighten_left, 
    //    retina.light_right, retina.brighten_right
    //);
}

pub struct Retina {
    width: u32,
    size: u32,
    wgpu: WgpuHardcopy,
    id_left: SurfaceId,
    _id_right: SurfaceId,
    form_id: Option<FormId>,

    fov: Angle,
    eye_angle: Angle,

    data_left: Option<Tensor>,
    data_right: Option<Tensor>,

    light_left: f32,
    light_right: f32,

    brighten_left: f32,
    brighten_right: f32,
}

impl Retina {
    pub const HEIGHT : f32 = 0.3;
    pub const SIZE : usize = 8;

    fn new(size: u32) -> Self {
        let width = (2 * size).max(64);

        let mut wgpu = WgpuHardcopy::new(width, size);

        Self {
            width,
            size,
            id_left: wgpu.add_surface(),
            _id_right: wgpu.add_surface(),
            wgpu,
            form_id: None,
            fov: Angle::Deg(90.),
            eye_angle: Angle::Deg(90.),

            data_left: None,
            data_right: None,

            light_left: 0.,
            light_right: 0.,

            brighten_left: 0.,
            brighten_right: 0.,
        }
    }

    pub fn fov(mut self, fov: Angle) -> Self {
        self.fov = fov;

        self
    }

    pub fn get_size(&self) -> usize {
        self.size as usize
    }

    pub fn data_left(&self) -> Option<Tensor> {
        self.data_left.clone()
    }

    pub fn data_right(&self) -> Option<Tensor> {
        self.data_right.clone()
    }

    pub fn light_left(&self) -> f32 {
        self.light_left
    }

    pub fn light_right(&self) -> f32 {
        self.light_right
    }

    pub fn brighten_left(&self) -> f32 {
        self.brighten_left
    }

    pub fn brighten_right(&self) -> f32 {
        self.brighten_right
    }

    pub fn dim_left(&self) -> f32 {
        (- self.brighten_left).max(0.)
    }

    pub fn dim_right(&self) -> f32 {
        (- self.brighten_right).max(0.)
    }

    fn startup(&mut self, world: &World) {
        let mut startup = RetinaStartup {
            world,
            form_id: None,
        };

        let mut renderer = self.wgpu.renderer_viewless();

        startup.event(
            &mut renderer, 
            &Event::Resize(Bounds::from([self.size as f32, self.size as f32]))
        );

        self.form_id = startup.form_id;

        assert!(self.form_id.is_some());
    }

    fn draw_and_load(&mut self, body: &Body) {
        let (left, right) = self.wgpu.draw_and_read(self.id_left, 
            &mut DoubleDrawable {
                width: self.width as f32,
                size: self.size as f32,
                form_id: self.form_id.unwrap(),
                eye_angle: self.eye_angle,
                head_pos: body.head_pos().into(),
                head_dir: body.head_dir(),
                fov: self.fov,
            }, |buf| {
                let mut vec = Vec::<f32>::new();

                for j in 0..self.size {
                    for i in 0..self.size {
                        vec.push(buf.get_pixel(i, j).to_luma().0[0] as f32 / 255.);
                    }
                }

                let left = Tensor::from(vec).reshape([self.size as usize, self.size as usize]);

                let mut vec = Vec::<f32>::new();
                for j in 0..self.size {
                    for i in self.size..2 * self.size {
                        vec.push(buf.get_pixel(i, j).to_luma().0[0] as f32 / 255.);
                    }
                }

                let right = Tensor::from(vec).reshape([self.size as usize, self.size as usize]);

                (left, right)
            }
        );

        self.data_left = Some(left);
        self.data_right = Some(right);
    }
}

struct RetinaStartup<'a> {
    world: &'a World,
    form_id: Option<FormId>,
}

impl Drawable for RetinaStartup<'_> {
    fn draw(&mut self, _renderer: &mut dyn Renderer) -> Result<()> {
        Ok(())
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        if let Event::Resize(_pos) = event {
            self.form_id = Some(world_form(renderer, self.world));
        }
    }
}

pub fn world_form(renderer: &mut dyn Renderer, world: &World) -> FormId {
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
        Color::black(),
    ])));

    let (width, height) = world.extent();

    let c_n = 0.05;
    let c_s = 0.15;
    let c_e = 0.25;
    let c_w = 0.35;

    let c_gl = 0.45;
    let c_gd = 0.55;
    let c_food = 0.65;

    let c_k = 0.95;

    let (w_f32, h_f32) = (width as f32, height as f32);

    for y in 0..height {
        wall(&mut form, (0., y as f32), (0., y as f32 + 1.), c_n);

        wall(&mut form, (w_f32, y as f32), (w_f32, y as f32 + 1.), c_s);
    }

    // out of bounds walls for clipping
    wall(&mut form, (-1., -1.), (-1., h_f32 + 1.), c_k);                    
    floor(&mut form, (-1., -1.), (0., h_f32 + 1.), c_k);                    
    roof(&mut form, (-1., -1.), (0., h_f32 + 1.), c_k);                    

    wall(&mut form, (w_f32 + 1., -1.), (w_f32 + 1., h_f32 + 1.), c_k);                    
    floor(&mut form, (w_f32, -1.), (w_f32 + 1., h_f32 + 1.), c_k);                    
    roof(&mut form, (w_f32, -1.), (w_f32 + 1., h_f32 + 1.), c_k);                    

    for x in 0..width {
        wall(&mut form, (x as f32, 0.), (x as f32 + 1., 0.), c_e);

        wall(&mut form, (x as f32, height as f32), (x as f32 + 1., height as f32), c_w);
        //wall(&mut form, (x as f32, 1.), (x as f32 + 1., 1.), 0.9);
    }

    // out of bounds walls for clipping
    wall(&mut form, (-1., -1.), (w_f32 + 1., -1.), c_k);                    
    floor(&mut form, (-1., -1.), (w_f32 + 1., 0.), c_k);                    
    roof(&mut form, (-1., -1.), (w_f32 + 1., 0.), c_k);                    

    wall(&mut form, (-1., h_f32 + 1.), (w_f32 + 1., h_f32 + 1.), c_k);                    
    floor(&mut form, (-1., h_f32), (w_f32 + 1., h_f32 + 1.), c_k);                    
    roof(&mut form, (-1., h_f32), (w_f32 + 1., h_f32 + 1.), c_k);                    

    for j in 0..height {
        for i in 0..width {
            match world[(i, j)] {
                Wall::Food => {
                    floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_food);                    
                },
                Wall::Wall => {
                    wall(&mut form, (i as f32, j as f32), (i as f32, j as f32 + 1.), c_n);                    
                    wall(&mut form, (i as f32 + 1., j as f32), (i as f32 + 1., j as f32 + 1.), c_s);                    
                    wall(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32), c_e);                    
                    wall(&mut form, (i as f32, j as f32 + 1.), (i as f32 + 1., j as f32 + 1.), c_w);                    

                    floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_k);                    
                    roof(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_k);                    
                },
                Wall::Empty => {
                    if (i + j) % 2 == 0 {
                        floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_gl);                    
                    } else {
                        floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_gd);                    

                    }
                },
                Wall::FloorLight => {},
                Wall::FloorDark => {},
            }
        }
    }

    renderer.create_form(&form)
}

fn retina_startup(
    world: Res<World>,
    mut retina: ResMut<Retina>,
) {
    retina.startup(world.as_ref());
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

fn roof(form: &mut Form, p0: impl Into<Point>, p1: impl Into<Point>, v: f32) {
    let Point(x0, z0) = p0.into();
    let Point(x1, z1) = p1.into();

    let y0 = 1.;
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


struct DoubleDrawable {
    width: f32,
    size: f32,
    form_id: FormId,
    head_pos: Point,
    head_dir: Heading,
    eye_angle: Angle,
    fov: Angle,
}

impl Drawable for DoubleDrawable {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        //let x0 = - self.size / self.width;
        let x0 = -(self.width - self.size) / self.width;
        let dw = 2. * self.size / self.width;
        let left_camera = camera(self.head_pos, self.head_dir, self.eye_angle, self.fov)
            .scale(self.size / self.width, 1., 1.)
            .translate(x0, 0., 0.);

        let pos = Bounds::<Canvas>::from([self.size, self.size]);

        renderer.draw_with(&pos, &mut RetinaDraw {
            form_id: self.form_id,
            camera: left_camera,
        })?;

        let eye_angle = Angle::Unit(- self.eye_angle.to_unit());
        let right_camera = camera(self.head_pos, self.head_dir, eye_angle, self.fov)
            .scale(self.size / self.width, 1., 1.)
            .translate(x0 + dw, 0., 0.);

        let pos = Bounds::<Canvas>::from(((self.size, 0.), [self.size, self.size]));

        renderer.draw_with(&pos, &mut RetinaDraw {
            form_id: self.form_id,
            camera: right_camera,
        })?;

        Ok(())
    }
}

struct RetinaDraw {
    form_id: FormId,
    camera: Matrix4,
}

impl Drawable for RetinaDraw {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        renderer.draw_form(self.form_id, &self.camera)
    }
}


fn camera(pos: Point, dir: Heading, eye_angle: Angle, fov: Angle) -> Matrix4 {
    let mut camera = Matrix4::eye();

    camera = camera.translate(- pos.x(), - Retina::HEIGHT, pos.y());
    camera = camera.rot_xz(api::Angle::Unit(- dir.to_unit()));

    camera = camera.rot_xz(api::Angle::Unit(eye_angle.to_unit()));
    
    //let fov = 120.0f32;
    camera = camera.projection(fov.to_radians(), 1., 0.01, 100.);

    camera
}

pub struct RetinaPlugin {
    size: u32,
    fov: Angle,
    eye_angle: Angle,

    is_enable: bool,
}

impl RetinaPlugin {
    pub fn new() -> Self {
        Self {
            size: Retina::SIZE as u32,
            fov: Angle::Deg(90.),
            eye_angle: Angle::Deg(90.),

            is_enable: true,
        }
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }

    pub fn size(&mut self, size: u32) -> &mut Self {
        self.size = size;

        self
    }

    pub fn fov(&mut self, fov: Angle) -> &mut Self {
        self.fov = fov;

        self
    }

    pub fn eye_angle(&mut self, angle: Angle) -> &mut Self {
        self.eye_angle = angle;

        self
    }
}

impl Plugin for RetinaPlugin {
    fn build(&self, app: &mut App) {
        if ! self.is_enable {
            return;
        }

        let mut retina = Retina::new(self.size);
        retina.fov = self.fov;
        retina.eye_angle = self.eye_angle;

        app.insert_resource(retina);

        app.system(Startup, retina_startup);
        app.system(Tick, retina_update);
    }
}
