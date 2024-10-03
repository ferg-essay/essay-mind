use std::f32::consts::TAU;

use essay_graphics::api::form::Shape;
use essay_graphics::prelude::*;
use essay_tensor::Tensor;
use renderer::Renderer;

pub struct HexSliceGenerator {
    vertices: [[f32; 2]; 6],
    uv: [[f32; 2]; 6],
}

impl HexSliceGenerator {
    pub fn new(r_x: f32, r_y: f32) -> Self {
        let r_u = 0.5;
        let r_v = 0.5 / (TAU / 6.).sin();

        Self {
            vertices: [
                hex_vertex(r_x, r_y, 0. * TAU / 6.),
                hex_vertex(r_x, r_y, 1. * TAU / 6.),
                hex_vertex(r_x, r_y, 2. * TAU / 6.),
                hex_vertex(r_x, r_y, 3. * TAU / 6.),
                hex_vertex(r_x, r_y, 4. * TAU / 6.),
                hex_vertex(r_x, r_y, 5. * TAU / 6.),
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

    pub fn hex(&self, shape: &mut Shape, pos: impl Into<Point>, tile: &Tile) {
        let pos = pos.into();

        self.tri(shape, pos, tile, (2, 4, 5));
        self.tri(shape, pos, tile, (1, 2, 5));
        self.tri(shape, pos, tile, (5, 0, 1));
        self.tri(shape, pos, tile, (2, 3, 4));
    }

    pub fn tri(
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

pub struct TextureBuilder {
    vec: Vec<Vec<[u8; 4]>>,

    dw: usize,
    dh: usize,
}

impl TextureBuilder {
    pub fn new(dw: usize, dh: usize) -> Self {
        let vec = Vec::new();

        Self {
            vec,

            dw,
            dh,
        }
    }

    pub fn create_tile(&mut self) -> TexId {
        let id = TexId(self.vec.len());

        let mut tile = Vec::new();
        tile.resize(self.dw * self.dh, [0, 0, 0, 0]);

        self.vec.push(tile);

        id
    }

    pub fn fill(&mut self, id: TexId, color: Color) {
        let dw = self.dw;
        let dh = self.dh;

        let tile = &mut self.vec[id.0];

        for j in 0..dh {
            for i in 0..dw {
                tile[j * dw + i] = color.to_rgba_vec();
            }
        }
    }

    pub fn tri(
        &mut self, 
        id: TexId, 
        color: impl Into<Color>, 
        a: [f32; 2], 
        b: [f32; 2], 
        c: [f32; 2]
    ) {
        self.tri_p(id, color, |u, v| is_in_triangle([u, v], a, b, c))
    }

    pub fn quad(
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

    pub fn tri_p(
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

    pub fn gen(&self) -> TextureGenerator {
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

        let texture = Tensor::from(tex).reshape([height, width, 4]);

        TextureGenerator {
            texture: Some(texture),
            texture_id: None,
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

pub struct TextureGenerator {
    tile: Vec<Tile>,

    texture: Option<Tensor<u8>>,
    texture_id: Option<TextureId>,
}

impl TextureGenerator {
    pub fn tile(&self, id: TexId) -> &Tile {
        self.tile.get(id.0).unwrap()
    }

    pub fn bind(&mut self, renderer: &mut dyn Renderer) {
        if let Some(texture) = self.texture.take() {
            let id = renderer.create_texture_rgba8(&texture);
            self.texture_id = Some(id);
        }
    }

    pub fn texture(&mut self) -> Option<Tensor<u8>> {
        self.texture.take()
    }

    pub fn texture_id(&self) -> TextureId {
        self.texture_id.unwrap()
    }
}

pub struct Tile {
    u: f32,
    v: f32,

    du: f32,
    dv: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TexId(pub usize);
