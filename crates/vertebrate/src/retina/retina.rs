use essay_ecs::{app::{App, Plugin, Startup}, core::{Res, ResMut}};
use essay_graphics::api;
use essay_plot::{
    api::{
        form::{Form, FormId, Matrix4}, 
        renderer::{self, Drawable, Event, Renderer}, 
        Clip, Color, Point, 
    },
    wgpu::{wgpu::hardcopy::SurfaceId, WgpuHardcopy},
};
use essay_tensor::Tensor;
use mind_ecs::Tick;
use image::Pixel;

use crate::{body::Body, util::{self, Angle, Heading}, world::{World, WorldCell}};

pub struct Retina {
    size: u32,
    wgpu: WgpuHardcopy,
    surface_id: SurfaceId,
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
    fn new(size: u32) -> Self {
        let mut wgpu = WgpuHardcopy::new(size, size);
        let id = wgpu.add_surface();

        Self {
            size,
            wgpu,
            surface_id: id,
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

    pub fn data_left(&self) -> Option<Tensor> {
        self.data_left.clone()
    }

    pub fn data_right(&self) -> Option<Tensor> {
        self.data_right.clone()
    }

    fn startup(&mut self, world: &World) {
        let mut startup = RetinaStartup {
            world,
            form_id: None,
        };

        self.wgpu.draw(self.surface_id, &mut startup);

        self.form_id = startup.form_id;

        assert!(self.form_id.is_some());
    }

    fn draw(&mut self, pos: Point, dir: Heading, eye_angle: Angle) -> Tensor {
        let camera = camera(pos, dir, eye_angle);

        let mut draw = RetinaDraw {
            form_id: self.form_id.unwrap(),
            camera,
        };
    
        self.wgpu.draw(self.surface_id, &mut draw);
        self.wgpu.read_into(self.surface_id, |buf| {
            let mut vec = Vec::<f32>::new();

            for p in buf.pixels() {
                vec.push(p.to_luma().0[0] as f32 / 255.);
            }

            Tensor::from(vec).reshape([self.size as usize, self.size as usize])
        })
    }
}

struct RetinaStartup<'a> {
    world: &'a World,
    form_id: Option<FormId>,
}

impl Drawable for RetinaStartup<'_> {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
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
    
        let (width, height) = self.world.extent();
    
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
                match self.world[(i, j)] {
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
    
        self.form_id = Some(world_form(renderer, self.world));

        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
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
                WorldCell::Food => {
                    floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_food);                    
                },
                WorldCell::Wall => {
                    wall(&mut form, (i as f32, j as f32), (i as f32, j as f32 + 1.), c_n);                    
                    wall(&mut form, (i as f32 + 1., j as f32), (i as f32 + 1., j as f32 + 1.), c_s);                    
                    wall(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32), c_e);                    
                    wall(&mut form, (i as f32, j as f32 + 1.), (i as f32 + 1., j as f32 + 1.), c_w);                    

                    floor(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_k);                    
                    roof(&mut form, (i as f32, j as f32), (i as f32 + 1., j as f32 + 1.), c_k);                    
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

fn retina_update(
    body: Res<Body>,
    mut retina: ResMut<Retina>
) {
    let util::Point(x, y) = body.head_pos();

    let eye_left = retina.eye_angle;
    let eye_right = Angle::Unit(- eye_left.to_unit());

    retina.data_left = Some(retina.draw(Point(x, y), body.head_dir(), eye_left));
    retina.data_right = Some(retina.draw(Point(x, y), body.head_dir(), eye_right));

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

    retina.brighten_left = light_left - retina.light_left;
    retina.brighten_right = light_right - retina.light_right;
    retina.light_left = light_left;
    retina.light_right = light_right;

    println!("Avg {:.2}({:.2}) {:.2}({:.2})", 
        retina.light_left, retina.brighten_left, 
        retina.light_right, retina.brighten_right
    );
}



fn camera(pos: Point, dir: Heading, eye_angle: Angle) -> Matrix4 {
    let mut camera = Matrix4::eye();

    camera = camera.translate(- pos.x(), -0.2, pos.y());
    camera = camera.rot_xz(api::Angle::Unit(- dir.to_unit()));
    camera = camera.rot_xz(api::Angle::Unit(eye_angle.to_unit()));
    // camera = self.mat.matmul(&camera);

    //let fov = 120.0f32;
    let fov = 120.0f32;
    camera = camera.projection(fov.to_radians(), 1., 0.1, 100.);

    camera
}

struct RetinaDraw {
    form_id: FormId,
    camera: Matrix4,
}

impl Drawable for RetinaDraw {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        renderer.draw_form(self.form_id, &self.camera, &Clip::None)?;

        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
    }
}

pub struct RetinaPlugin {
    size: u32,
    fov: Angle,
    eye_angle: Angle,
}

impl RetinaPlugin {
    pub fn new() -> Self {
        Self {
            size: 16,
            fov: Angle::Deg(90.),
            eye_angle: Angle::Deg(90.),
        }
    }

    pub fn fov(mut self, fov: Angle) -> Self {
        self.fov = fov;

        self
    }

    pub fn eye_angle(mut self, angle: Angle) -> Self {
        self.eye_angle = angle;

        self
    }
}

impl Plugin for RetinaPlugin {
    fn build(&self, app: &mut App) {
        let mut retina = Retina::new(self.size);
        retina.fov = self.fov;
        retina.eye_angle = self.eye_angle;

        app.insert_resource(retina);

        app.system(Startup, retina_startup);
        app.system(Tick, retina_update);
    }
}
