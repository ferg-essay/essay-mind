use std::{collections::HashMap, f32::consts::PI};
use core::hash::Hash;

use essay_ecs::{app::{App, Plugin, Update}, core::{Res, ResMut}};
use essay_graphics::layout::View;
use essay_plot::api::{
    form::{Shape, ShapeId}, 
    renderer::{self, Canvas, Drawable, Renderer}, 
    Affine2d, Bounds, Color, TextureId
};
use essay_tensor::Tensor;
use ui_graphics::{HexSliceGenerator, TexId, TextureBuilder, TextureGenerator, Tile, UiCanvas};

use crate::world::{OdorKind, WorldHex};

use super::ui_world_map::{UiWorld, UiWorldPlugin};

fn update_hex_world(
    mut ui_hex: ResMut<UiWorldHex<OdorKind>>,
    world_hex: Res<WorldHex<OdorKind>>,
) {
    if ui_hex.update_count < world_hex.update_count() {
        ui_hex.update_count = world_hex.update_count();

        let mut shape = Shape::new();

        let epsilon = 0.01;
        let hex_gen = HexSliceGenerator::new(
            2. / 3. - epsilon,
            (PI / 6.).cos() * 2. / 3. - 2. * epsilon
        );
    
        for j in 0..world_hex.height() {
            for i in 0..world_hex.width() {
                let key = world_hex[(i, j)];

                let x = i as f32 + 0.5;
                let y = j as f32 + if i % 2 == 0 { 0.5 } else { 0.0 };

                if let Some(tile) = ui_hex.tex_gen.tile(&key) {
                    hex_gen.hex(&mut shape, (x, y), tile);
                }
            }
        }

        ui_hex.view.write(|v| v.shape = Some(shape));
    }
}

pub struct UiWorldHex<K: Eq + Hash> {
    view: View<HexView>,

    shape: Shape,

    shape_id: Option<ShapeId>,

    tex_gen: HexGenerator<K>,

    update_count: usize,
}

impl<K: Eq + Hash> UiWorldHex<K> {
    fn new(
        mut view: View<HexView>,
        mut gen: HexGenerator<K>,
    ) -> Self {
        // let tex = HexBuilder::<K>::new(64, 64);

        //tex.none(OdorKind::None);
        //tex.tile(OdorKind::A).fill("teal");

        //tex.tile(OdorKind::B).fill("orange");

        //tex.tile(OdorKind::C).pattern(Pattern::CheckerBoard(6), "red");

        let tex = gen.texture();

        view.write(|v| v.tex = tex);

        Self {
            view,

            shape: Shape::new(),
            shape_id: None,
            update_count: 0,

            tex_gen: gen,
        }
    }

    /*
    pub fn none(&mut self, key: K) {
        if let Some(builder) = &mut self.tex {
            builder.none(key);
        }
    }

    pub fn tile(&mut self, key: K) -> TileBuilder {
        if let Some(builder) = &mut self.tex {
            builder.tile(key)
        } else {
            panic!()
        }
    }
    */

    pub fn update_render(&mut self, renderer: &mut dyn Renderer, world: &WorldHex<K>) {
        //if let Some(tex) = self.tex.take() {
        //    self.tex_gen = Some(tex.gen(renderer));
        //}

        if self.shape_id.is_none() || self.update_count < world.update_count() {
            self.update_count = world.update_count();

            let mut shape = Shape::new();
            // shape.texture(tex_gen.texture_id());
            let epsilon = 0.01;
            let hex_gen = HexSliceGenerator::new(
                2. / 3. - epsilon,
                (PI / 6.).cos() * 2. / 3. - 2. * epsilon
            );

            for j in 0..world.height() {
                for i in 0..world.width() {
                    let x = i as f32 + 0.5;
                    let y = j as f32 + if i % 2 == 0 { 0.5 } else { 0.0 };

                    if let Some(tile) = self.tex_gen.tile(&world[(i, j)]) {
                        hex_gen.hex(&mut shape, (x, y), tile);
                    }
                }
            }

            self.shape_id = Some(renderer.create_shape(&shape));
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

pub struct HexBuilder<K: Eq + Hash + Clone> {
    tex: TextureBuilder,
    tex_map: HashMap<K, TexId>,
    id_missing: TexId,
}

impl<K: Eq + Hash + Clone> HexBuilder<K> {
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

    fn gen(&self) -> HexGenerator<K> {
        let gen = self.tex.gen();

        // let texture = gen.texture();

        HexGenerator {
            gen,
            tex_map: self.tex_map.clone(),
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

    pub fn pattern(
        self,
        pattern: Pattern,
        color: impl Into<Color>
    ) -> Self {
        pattern.draw(self, color.into())
    }
}

pub enum Pattern {
    Solid,
    CheckerBoard(usize),
}

impl Pattern {
    fn draw<'a>(&self, tile: TileBuilder<'a>, color: Color) -> TileBuilder<'a> {
        match self {
            Pattern::Solid => { 
                tile.fill(color)
            }
            Pattern::CheckerBoard(scale) => { 
                tile.tri_p(color, |u, v| {
                    (*scale as f32 * u) as usize % 2 == (*scale as f32 * v) as usize % 2
                })
            }
        }
    }
}

pub struct HexGenerator<K: Eq + Hash> {
    gen: TextureGenerator,
    tex_map: HashMap<K, TexId>,
    id_missing: TexId,
}

impl<K: Eq + Hash> HexGenerator<K> {
    //fn texture_id(&self) -> TextureId {
    //    self.gen.texture_id()
    //}

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
    
    fn texture(&mut self) -> Option<Tensor<u8>> {
        self.gen.texture()
    }
}

struct HexView {
    bounds: Bounds<UiWorld>,
    
    shape: Option<Shape>,
    shape_id: Option<ShapeId>,

    tex: Option<Tensor<u8>>,
    tex_id: Option<TextureId>,

    camera: Affine2d,
}

impl HexView {
    fn new(bounds: Bounds<UiWorld>) -> Self {
        Self {
            bounds,

            shape: None,
            shape_id: None,
            tex: None,
            tex_id: None,

            camera: Affine2d::eye(),
        }
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.camera = self.bounds.affine_to(pos);
    }
}

impl Drawable for HexView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        if let Some(tex) = self.tex.take() {
            self.tex_id = Some(renderer.create_texture_rgba8(&tex));
        }

        if self.shape_id.is_none() {
            if let Some(mut shape) = self.shape.take() {
                if let Some(tex_id) = &self.tex_id {
                    shape.texture(*tex_id);
                }

                self.shape_id = Some(renderer.create_shape(&shape));
            }
        }

        if let Some(shape_id) = &self.shape_id {
            renderer.draw_shape(*shape_id, &self.camera)
        } else {
            Ok(())
        }
    }

    fn resize(&mut self, _renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) -> Bounds<Canvas> {
        self.set_pos(bounds);

        bounds.clone()
    }
}

pub struct UiWorldHexPlugin {
    builder: HexBuilder<OdorKind>,
}

impl UiWorldHexPlugin {
    pub fn new() -> Self {
        Self {
            builder: HexBuilder::new(64, 64),
        }
    }

    pub fn none(&mut self, key: OdorKind) {
        self.builder.none(key);
    }

    pub fn tile(&mut self, key: OdorKind) -> TileBuilder {
        self.builder.tile(key)
    }
}

impl Plugin for UiWorldHexPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let world_bounds = app.resource::<UiWorld>().bounds();
            let world_id = app.resource::<UiWorld>().view_id();

            let view = HexView::new(world_bounds);

            let view = app.resource_mut::<UiCanvas>().subview(world_id, 2, view);

            // let mut hex_builder = HexBuilder::new(64, 64);
            
            // hex_builder.none(OdorKind::None);

            let gen = self.builder.gen();

            let hex = UiWorldHex::<OdorKind>::new(view, gen);

            app.insert_resource(hex);

            // let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
            // app.insert_resource(ui_world);

            // app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            //app.system(Update, draw_world.phase(DrawWorld));
            // app.system(PreUpdate, world_resize);

            // app.system(Startup, spawn_ui_world);
            app.system(Update, update_hex_world);
        }
    }
}