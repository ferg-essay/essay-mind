use std::{collections::HashMap, f32::consts::PI};
use core::hash::Hash;

use essay_plot::api::{form::{Shape, ShapeId}, renderer::{self, Renderer}, Affine2d, Color, TextureId};
use ui_graphics::{HexSliceGenerator, TexId, TextureBuilder, TextureGenerator, Tile};

use crate::world::{HexOdorWorld, OdorKind};

pub struct HexBuilder<K: Eq + Hash> {
    tex: TextureBuilder,
    tex_map: HashMap<K, TexId>,
    id_missing: TexId,
}

impl<K: Eq + Hash> HexBuilder<K> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tex = TextureBuilder::new(width, height);

        // tex 0 reserved
        tex.create_tile();

        let id_missing = tex.create_tile();
        tex.fill(id_missing, "hot pink".into());

        Self {
            tex,
            tex_map: HashMap::default(),
            id_missing,
        }
    }

    pub fn none(&mut self, key: K) {
        self.tex_map.insert(key, TexId(0));
    }

    pub fn tile<'a>(&'a mut self, key: K) -> TileBuilder<'a> {
        let id = self.tex.create_tile();
        self.tex_map.insert(key, id);

        TileBuilder {
            builder: &mut self.tex,
            id,
        }
    }

    fn gen(self, renderer: &mut dyn Renderer) -> HexGenerator<K> {
        let mut gen = self.tex.gen();
        gen.bind(renderer);

        HexGenerator {
            gen,
            tex_map: self.tex_map,
            id_missing: self.id_missing,
        }
    }
}

pub struct TileBuilder<'a> {
    builder: &'a mut TextureBuilder,
    id: TexId,
}

impl<'a> TileBuilder<'a> {
    pub fn fill(self, color: impl Into<Color>) -> Self {
        self.builder.fill(self.id, color.into());
        
        self
    }

    pub fn tri(
        self, 
        color: impl Into<Color>, 
        a: [f32; 2], 
        b: [f32; 2], 
        c: [f32; 2]
    ) -> Self {
        self.builder.tri(self.id, color, a, b, c);

        self
    }

    pub fn quad(
        self, 
        color: impl Into<Color>, 
        a: [f32; 2], 
        b: [f32; 2], 
        c: [f32; 2],
        d: [f32; 2]
    ) -> Self {
        self.builder.quad(self.id, color, a, b, c, d);

        self
    }

    pub fn tri_p(
        self, 
        color: impl Into<Color>, 
        fun: impl Fn(f32, f32)->bool
    ) -> Self {
        self.builder.tri_p(self.id, color, fun);

        self
    }
}

pub struct HexGenerator<K: Eq + Hash> {
    gen: TextureGenerator,
    tex_map: HashMap<K, TexId>,
    id_missing: TexId,
}

impl<K: Eq + Hash> HexGenerator<K> {
    fn texture_id(&self) -> TextureId {
        self.gen.texture_id()
    }

    fn tile(&self, key: &K) -> Option<&Tile> {
        if let Some(id) = self.tex_map.get(key) {
            if id.0 != 0 {
                Some(self.gen.tile(*id))
            } else {
                None
            }
        } else {
            Some(self.gen.tile(self.id_missing))
        }
    }
}

pub(super) struct UiWorldHex {
    shape: Shape,

    shape_id: Option<ShapeId>,

    tex: Option<HexBuilder<OdorKind>>,
    tex_gen: Option<HexGenerator<OdorKind>>,

    update_count: usize,
}

impl UiWorldHex {
    pub fn new() -> Self {
        let mut tex = HexBuilder::<OdorKind>::new(64, 64);

        tex.none(OdorKind::None);
        tex.tile(OdorKind::A).fill("teal");

        tex.tile(OdorKind::B).fill("orange");

        tex.tile(OdorKind::C).fill("red");

        Self {
            shape: Shape::new(),
            shape_id: None,
            update_count: 0,

            tex: Some(tex),
            tex_gen: None,
        }
    }

    pub fn update_render(&mut self, renderer: &mut dyn Renderer, world: &HexOdorWorld) {
        if let Some(tex) = self.tex.take() {
            self.tex_gen = Some(tex.gen(renderer));
        }

        if self.shape_id.is_none() || self.update_count < world.update_count() {
            self.update_count = world.update_count();

            if let Some(tex_gen) = &self.tex_gen {
                let mut shape = Shape::new();
                shape.texture(tex_gen.texture_id());
                let epsilon = 0.01;
                let hex_gen = HexSliceGenerator::new(
                    2. / 3. - epsilon,
                    (PI / 6.).cos() * 2. / 3. - 2. * epsilon
                );

                for j in 0..world.height() {
                    for i in 0..world.width() {
                        let x = i as f32 + 0.5;
                        let y = j as f32 + if i % 2 == 0 { 0.5 } else { 0.0 };

                        if let Some(tile) = tex_gen.tile(&world[(i, j)]) {
                            hex_gen.hex(&mut shape, (x, y), tile);
                        }
                    }
                }

                self.shape_id = Some(renderer.create_shape(&shape));
            }
        }
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
