use std::f32::consts::TAU;

use essay_plot::wgpu::WgpuMainLoop;
use essay_graphics::api::form::{Shape, ShapeId};
use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::{layout::Layout, prelude::*};
use essay_tensor::Tensor;

fn main() { 
    let mut layout = Layout::new();

    let colors = [
        Color::from("red"),
        Color::from("blue"),
        Color::from("orange"),
        Color::from("teal"),
    ];

    let s = 64;
    let mut tex_gen = TextureBuilder::new(s, s);

    for color in colors.iter() {
        let tile = tex_gen.create_tile();

        tex_gen.fill(tile, *color);

        tex_gen.tri_p(tile, "white", |u, v| {
            (u * 32.) as u32 % 2 == (v * 32.) as u32 % 2
        });

        let w = (10. * s as f32).recip();
        //tex_gen.tri(tile, "black", [0., 0.5], [1., 0.5], [1., 0.5 + w]);
        //tex_gen.tri(tile, "black", [0., 0.5], [0., 0.5 + w], [1., 0.5 + w]);

        tex_gen.quad(tile, "black", [0., 0.5], [1., 0.5], [1., 0.5 + w], [0., 0.5 + w]);

        tex_gen.tri(tile, "black", [0., 0.0], [1., 0.0], [1., 0.0 + w]);
        tex_gen.tri(tile, "black", [0., 0.0], [0., 0.0 + w], [1., 0.0 + w]);
    }

    let tex = tex_gen.gen();

    let mut form = Shape::new();

    let gen = HexSliceGenerator::new(0.1);

    gen.gen(&mut form, (0.251, 0.25), tex.tile(TexId(0)));
    gen.gen(&mut form, (0.40, 0.25 + 0.0866), tex.tile(TexId(1)));
    gen.gen(&mut form, (0.55, 0.25), tex.tile(TexId(2)));
    gen.gen(&mut form, (0.40, 0.25 - 0.0866), tex.tile(TexId(3)));

    layout.view(((0.5, 0.5), [0.5, 0.5]),
        ShapeView::new(form, tex.texture),
    );
    /*
    texture_colors(&[
        Color::from("red"),
        Color::from("blue"),
        Color::from("orange"),
        Color::from("teal"),
    ]))
    */

    WgpuMainLoop::new().main_loop(Box::new(layout)).unwrap();
}

struct HexSliceGenerator {
    vertices: [[f32; 2]; 6],
    uv: [[f32; 2]; 6],
}

impl HexSliceGenerator {
    fn new(r: f32) -> Self {
        let r_u = 0.5;
        let r_v = 0.5 / (TAU / 6.).sin();

        Self {
            vertices: [
                hex_vertex(r, r, 0. * TAU / 6.),
                hex_vertex(r, r, 1. * TAU / 6.),
                hex_vertex(r, r, 2. * TAU / 6.),
                hex_vertex(r, r, 3. * TAU / 6.),
                hex_vertex(r, r, 4. * TAU / 6.),
                hex_vertex(r, r, 5. * TAU / 6.),
            ],
            uv: [
                hex_vertex(r_u, r_v, 0. * TAU / 6.),
                hex_vertex(r_u, r_v, 1. * TAU / 6.),
                hex_vertex(r_u, r_v, 2. * TAU / 6.),
                hex_vertex(r_u, r_v, 3. * TAU / 6.),
                hex_vertex(r_u, r_v, 4. * TAU / 6.),
                hex_vertex(r_u, r_v, 5. * TAU / 6.),
            ]
        }
    }

    fn gen(&self, shape: &mut Shape, pos: impl Into<Point>, tile: &Tile) {
        let pos = pos.into();

        self.tri(shape, pos, tile, (2, 4, 5));
        self.tri(shape, pos, tile, (1, 2, 5));
        self.tri(shape, pos, tile, (5, 0, 1));
        self.tri(shape, pos, tile, (2, 3, 4));
    }

    fn tri(
        &self, 
        shape: &mut Shape, 
        pos: Point, 
        tile: &Tile, 
        (a, b, c): (usize, usize, usize)
    ) {
        let Point(x, y) = pos;
        let vs = &self.vertices;
        let uvs = &self.uv;

        let du = tile.du;
        let dv = tile.dv;

        let u = tile.u + 0.5 * du;
        let v = tile.v + 0.5 * dv;

        shape.vertex(
            [x + vs[a][0], y + vs[a][1]], 
            [u + du * uvs[a][0], v + dv * uvs[a][1]]
        );
        shape.vertex(
            [x + vs[b][0], y + vs[b][1]], 
            [u + du * uvs[b][0], v + dv * uvs[b][1]]
        );
        shape.vertex(
            [x + vs[c][0], y + vs[c][1]], 
            [u + du * uvs[c][0], v + dv * uvs[c][1]]
        );
    }
}

fn hex_vertex(rx: f32, ry: f32, angle: f32) -> [f32; 2] {
    let (dy, dx) = angle.sin_cos();

    [dx * rx, dy * ry]
}

struct TextureBuilder {
    vec: Vec<Vec<[u8; 4]>>,

    dw: usize,
    dh: usize,
}

impl TextureBuilder {
    fn new(dw: usize, dh: usize) -> Self {
        let vec = Vec::new();

        Self {
            vec,

            dw,
            dh,
        }
    }

    fn create_tile(&mut self) -> TexId {
        let id = TexId(self.vec.len());

        let mut tile = Vec::new();
        tile.resize(self.dw * self.dh, [0, 0, 0, 0]);

        self.vec.push(tile);

        id
    }

    fn fill(&mut self, id: TexId, color: Color) {
        let dw = self.dw;
        let dh = self.dh;

        let tile = &mut self.vec[id.0];

        for j in 0..dh {
            for i in 0..dw {
                tile[j * dw + i] = color.to_rgba_vec();
            }
        }
    }

    fn tri(
        &mut self, 
        id: TexId, 
        color: impl Into<Color>, 
        a: [f32; 2], 
        b: [f32; 2], 
        c: [f32; 2]
    ) {
        self.tri_p(id, color, |u, v| is_in_triangle([u, v], a, b, c))
    }

    fn quad(
        &mut self, 
        id: TexId, 
        color: impl Into<Color>, 
        a: [f32; 2], 
        b: [f32; 2], 
        c: [f32; 2],
        d: [f32; 2]
    ) {
        self.tri_p(id, color, |u, v| is_in_quad([u, v], a, b, c, d))
    }

    fn tri_p(
        &mut self, 
        id: TexId, 
        color: impl Into<Color>, 
        fun: impl Fn(f32, f32)->bool
    ) {
        let color = color.into();
        let dw = self.dw;
        let dh = self.dh;

        let tile = &mut self.vec[id.0];

        for j in 0..dh {
            for i in 0..dw {
                let v = (j as f32) / dh as f32;
                let u = (i as f32) / dw as f32;

                if fun(u, v) {
                    tile[j * dw + i] = color.to_rgba_vec();
                }
            }
        }
    }

    fn gen(&self) -> TextureGenerator {
        // let mut tiles = Vec::new();

        let n = self.vec.len();
        let cols = (n as f32).max(1.).sqrt().max(4.).floor() as usize;
        let rows = (n as f32 / cols as f32).ceil().max(1.) as usize;

        let pad = 4;
        let width = cols * (self.dw + pad);
        let height = rows * (self.dh + pad);

        let mut tex = Vec::<[u8; 4]>::new();
        tex.resize(width * height, [0, 0, 0, 0]);

        let mut tiles = Vec::new();

        for (id, tile) in self.vec.iter().enumerate() {
            let y0 = (id / cols) * (self.dh + pad);
            let x0 = (id % cols) * (self.dw + pad);

            for j in 0..self.dw {
                let y = (y0 + j) * width;

                for i in 0..self.dh {
                    tex[y + x0 + i] = tile[j * self.dw + i];
                }
            }

            tiles.push(Tile {
                u: (x0 as f32) / width as f32,
                v: (y0 as f32) / height as f32,

                du: self.dw as f32 / width as f32,
                dv: self.dh as f32 / height as f32,
            });
        }

        let texture = Tensor::from(tex);
        
        TextureGenerator {
            texture: texture.reshape([height, width, 4]),
            tile: tiles,
        }
    }
}

fn is_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);

    ! (
        (d1 < 0. || d2 < 0. || d3 < 0.)
        && (d1 > 0. || d2 > 0. || d3 > 0.)
    )
}

fn is_in_quad(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2], d: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, d);
    let d4 = sign(p, d, a);

    ! (
        (d1 < 0. || d2 < 0. || d3 < 0. || d4 < 0.)
        && (d1 > 0. || d2 > 0. || d3 > 0. || d4 > 0.)
    )
}

fn sign(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    (p[0] - b[0]) * (a[1] - b[1]) - (a[0] - b[0]) * (p[1] - b[1])
}

struct TextureGenerator {
    texture: Tensor<u8>,

    tile: Vec<Tile>,
}

impl TextureGenerator {
    fn tile(&self, id: TexId) -> &Tile {
        self.tile.get(id.0).unwrap()
    }
}

struct Tile {
    u: f32,
    v: f32,

    du: f32,
    dv: f32,
}

#[derive(Clone, Copy, Debug)]
struct TexId(usize);

struct ShapeView {
    form: Shape,
    form_id: Option<ShapeId>,
    texture: Tensor<u8>,

    is_dirty: bool,
}

impl ShapeView {
    fn new(form: Shape, texture: Tensor<u8>) -> Self {
        Self {
            form,
            form_id: None,
            texture,
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {
        let texture = renderer.create_texture_rgba8(&self.texture);

        self.form.texture(texture);

        self.form_id = Some(renderer.create_shape(&self.form));
    }
}

impl Drawable for ShapeView {
    // fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    // }

    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        if self.is_dirty {
            self.is_dirty = false;
            self.fill_model(renderer);
        }

        if let Some(id) = self.form_id {
            let canvas = renderer.pos().clone();
            let bounds = Bounds::<Canvas>::from(((0., 0.), [1., 1.]));
            let camera = bounds.affine_to(&canvas);
            
            renderer.draw_shape(
                id,
                &camera,
            )?;
        }

        Ok(())
    }
}
