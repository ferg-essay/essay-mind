use std::{collections::HashMap, f32::consts::PI};
use core::hash::Hash;

use essay_ecs::{
    app::{App, Plugin, Update}, 
    core::{Res, ResMut}
};
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::{
    renderer::{self, Canvas, Drawable, Renderer}, 
    Affine2d, Bounds, Color, Mesh2d, TextureId
};
use essay_tensor::tensor::Tensor;
use ui_graphics::{HexSliceGenerator, TexId, TextureBuilder, TextureGenerator, Tile, ViewPlugin};

use crate::world::{World, WorldHex, WorldHexTrait};

use super::ui_world_map::UiWorld;

fn update_hex_world<T: WorldHexTrait + Hash + Eq>(
    mut ui_hex: ResMut<UiWorldHex<T>>,
    world_hex: Res<WorldHex<T>>,
) {
    if ui_hex.update_count < world_hex.update_count() {
        ui_hex.update_count = world_hex.update_count();

        let mut shape = Mesh2d::new();

        let epsilon = 0.01;
        let hex_gen = HexSliceGenerator::new(
            2. / 3. - epsilon,
            (PI / 6.).cos() * 2. / 3. - 2. * epsilon
        );
    
        for j in 0..world_hex.height() {
            for i in 0..world_hex.width() {
                let key = &world_hex[(i, j)];

                let x = i as f32 + 0.5;
                let y = j as f32 + if i % 2 == 0 { 0.5 } else { 0.0 };

                if let Some(tile) = ui_hex.tex_gen.tile(&key) {
                    hex_gen.hex(&mut shape, [x, y], tile);
                }
            }
        }

        ui_hex.view.write(|v| v.shape = Some(shape));
    }
}

pub struct UiWorldHex<K: WorldHexTrait + Eq + Hash> {
    view: View<HexView>,

    shape: Mesh2d,

    tex_gen: HexGenerator<K>,

    update_count: usize,
}

impl<T: WorldHexTrait + Eq + Hash> UiWorldHex<T> {
    fn new(
        mut view: View<HexView>,
        mut gen: HexGenerator<T>,
    ) -> Self {
        let tex = gen.texture();

        view.write(|v| v.tex = tex);

        Self {
            view,

            shape: Mesh2d::new(),
            update_count: 0,

            tex_gen: gen,
        }
    }

    pub fn update_render(&mut self, renderer: &mut dyn Renderer, world: &WorldHex<T>) {
        if self.update_count < world.update_count() {
            self.update_count = world.update_count() + 1;

            let mut shape = Mesh2d::new();
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
                        hex_gen.hex(&mut shape, [x, y], tile);
                    }
                }
            }

            self.shape = shape;
        }
    }

    pub fn draw(&mut self, renderer: &mut dyn Renderer, camera: &Affine2d) -> renderer::Result<()> {
        renderer.draw_mesh2d(&self.shape, TextureId::default(), &[camera.into()])
    }
}

pub struct HexBuilder<T: Eq + Hash + Clone> {
    tex: TextureBuilder,
    tex_map: HashMap<T, TexId>,
    id_missing: TexId,
}

impl<T: Eq + Hash + Clone> HexBuilder<T> {
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

    pub fn none(&mut self, key: T) {
        self.tex_map.insert(key, TexId(0));
    }

    pub fn tile<'a>(&'a mut self, key: T) -> TileBuilder<'a> {
        let id = self.tex.create_tile();
        self.tex_map.insert(key, id);

        TileBuilder {
            builder: &mut self.tex,
            id,
        }
    }

    fn gen(&self) -> HexGenerator<T> {
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

pub struct HexGenerator<T: Eq + Hash> {
    gen: TextureGenerator,
    tex_map: HashMap<T, TexId>,
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

pub struct HexView {
    bounds: Bounds<UiWorld>,
    
    shape: Option<Mesh2d>,
    // shape_id: Option<ShapeId>,

    tex: Option<Tensor<u8>>,
    tex_id: Option<TextureId>,

    camera: Affine2d,
}

impl HexView {
    fn new() -> Self {
        Self {
            bounds: Bounds::none(),

            shape: None,
            tex: None,
            tex_id: None,

            camera: Affine2d::eye(),
        }
    }

    fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.camera = self.bounds.affine_to(pos);
    }
}

impl Drawable for HexView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        self.set_pos(renderer.pos());

        if let Some(tex) = self.tex.take() {
            self.tex_id = Some(renderer.create_texture_rgba8(&tex));
        }

        /*
        if self.shape_id.is_none() {
            if let Some(mut shape) = self.shape.take() {
                if let Some(tex_id) = &self.tex_id {
                    shape.texture(*tex_id);
                }

                self.shape_id = Some(renderer.create_shape(&shape));
            }
        }
        */

        if let Some(texture) = &self.tex_id {
            if let Some(shape) = &self.shape {
                renderer.draw_mesh2d(&shape, *texture, &[(Color::from("azure"), (&self.camera)).into()])
                //renderer.draw_mesh2d(&shape, TextureId::default(), &[(Color::from("azure"), (&self.camera)).into()])
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

pub struct UiWorldHexPlugin<T: WorldHexTrait + Hash + Eq> {
    builder: HexBuilder<T>,

    view: Option<View<HexView>>,
}

impl<T: WorldHexTrait + Hash + Eq> UiWorldHexPlugin<T> {
    pub fn new() -> Self {
        Self {
            builder: HexBuilder::new(64, 64),
            view: None,
        }
    }

    pub fn none(&mut self, key: T) {
        self.builder.none(key);
    }

    pub fn tile(&mut self, key: T) -> TileBuilder {
        self.builder.tile(key)
    }
}

impl<T: WorldHexTrait + Hash + Eq> ViewPlugin for UiWorldHexPlugin<T> {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        self.view = Some(View::from(HexView::new()));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl<T: WorldHexTrait + Hash + Eq> Plugin for UiWorldHexPlugin<T> {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            let (width, height) = app.resource::<World>().extent();
            let world_bounds = Bounds::from([
                width as f32,
                height as f32,
            ]);

            let mut view = view.clone();

            view.write(|v| { v.bounds = world_bounds; });

            let gen = self.builder.gen();

            let hex = UiWorldHex::<T>::new(view.clone(), gen);

            app.insert_resource(hex);

            app.system(Update, update_hex_world::<T>);
        }
    }
}