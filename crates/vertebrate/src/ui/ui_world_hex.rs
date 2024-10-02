use std::f32::consts::PI;

use essay_plot::api::{form::{Shape, ShapeId}, renderer::{self, Renderer}, Affine2d};
use ui_graphics::{HexSliceGenerator, TexId, TextureBuilder, TextureGenerator};

use crate::world::HexOdorWorld;

pub(super) struct UiWorldHex {
    shape: Shape,

    shape_id: Option<ShapeId>,
    camera: Affine2d,

    tex: Option<TextureBuilder>,
    tex_gen: Option<TextureGenerator>,

    update_count: usize,
}

impl UiWorldHex {
    pub fn new() -> Self {
        let mut tex = TextureBuilder::new(64, 64);

        let _tex_0 = tex.create_tile();

        let tex_1 = tex.create_tile();
        tex.fill(tex_1, "teal".into());

        Self {
            shape: Shape::new(),
            shape_id: None,
            camera: Affine2d::eye(),
            update_count: 0,

            tex: Some(tex),
            tex_gen: None,
        }
    }

    pub fn update_render(&mut self, renderer: &mut dyn Renderer, world: &HexOdorWorld) {
        if let Some(tex) = self.tex.take() {
            let mut gen = tex.gen();
            gen.bind(renderer);
            self.tex_gen = Some(gen);
        }

        if self.shape_id.is_none() {
            if let Some(tex_gen) = &self.tex_gen {
                let mut shape = Shape::new();
                shape.texture(tex_gen.texture_id());
                let hex_gen = HexSliceGenerator::new(1.);

                hex_gen.hex(&mut shape, (1.5, 1.5), tex_gen.tile(TexId(1)));

                self.shape_id = Some(renderer.create_shape(&shape));
            }
        }
    }

    pub fn update(&mut self, world: &HexOdorWorld, tiles: TextureGenerator) {
        if world.update_count() <= self.update_count {
            return;
        }
        self.update_count = world.update_count();

        let scale = world.scale();
        let y_scale = scale * (PI / 0.6).cos();
        let x_scale = scale * 1.5;

        let mut shape = Shape::new();
        let gen = HexSliceGenerator::new(scale);

        for j in 0..world.height() {
            for i in 0..world.width() {
                let y = (j as f32 + 0.5 * (i % 2) as f32) * y_scale;
                let x = i as f32 * x_scale;

                let tile = tiles.tile(TexId(1));

                gen.hex(&mut shape, (x, y), tile);
            }
        }

        self.shape = shape;
        self.shape_id = None;
    }

    pub fn draw(&mut self, renderer: &mut dyn Renderer, camera: &Affine2d) -> renderer::Result<()> {
        if self.shape_id.is_none() {
            self.shape_id = Some(renderer.create_shape(&self.shape));
        }

        if let Some(shape_id) = &self.shape_id {
            renderer.draw_shape(*shape_id, camera)
        } else {
            Ok(())
        }
    }
}
