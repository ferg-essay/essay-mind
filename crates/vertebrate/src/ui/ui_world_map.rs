use essay_ecs::core::Query;
use essay_ecs::prelude::*;
use essay_graphics::layout::{Layout, View, ViewId};
use essay_plot::{prelude::*, artist::paths};
use essay_tensor::Tensor;
use renderer::{Canvas, Drawable, Renderer};
use ui_graphics::{ui_layout::UiLayoutPlugin, UiCanvas, UiCanvasPlugin};

use crate::world::{Food, FoodKind, Odor, OdorInnate, OdorKind, World, WorldPlugin};

use crate::world::Wall;

#[derive(Component)]
pub struct UiWorld {
    view: View<UiWorldView>,
    width: usize,
    height: usize,
    image: Option<ImageId>,
    // hex: UiWorldHex<OdorKind>,
}

impl UiWorld {
    fn new(
        view: View<UiWorldView>, 
        width: usize, 
        height: usize,
        // hex: UiWorldHex<OdorKind>,
    ) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || Wall::Empty);

        Self {
            view,
            // pos: Bounds::zero(),
            width,
            height,
            image: None,
            // hex,
        }
    }
    /*
    pub fn hex(&mut self) -> &mut UiWorldHex<OdorKind> {
        &mut self.hex
    }
    */

    fn pos(&self) -> Bounds<Canvas> {
        self.view.read(|v| v.pos.clone())    
    }

    pub fn bounds(&self) -> Bounds<UiWorld> {
        self.view.read(|v| v.bounds.clone())    
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.view.read(|v| v.to_canvas.clone())    
    }

    pub fn to_canvas_scale(&self) -> Affine2d {
        let pos = self.pos();
        let bounds = self.bounds();

        Affine2d::eye().scale(
            pos.width() / bounds.width(), 
            pos.height() / bounds.height(),
        )
    }

    pub fn clip(&self) -> &Clip {
        // &self.clip
        &Clip::None
    }

    pub fn view_id(&self) -> ViewId {
        self.view.id()
    }

    pub fn update(&mut self, _world: &World, _renderer: &mut dyn Renderer) {
        // self.hex.update_render(renderer, world.hex());
    }

    pub fn draw(&mut self, _renderer: &mut dyn Renderer, _camera: &Affine2d) -> renderer::Result<()> {
        // self.hex.draw(renderer, camera)
        Ok(())
    }
}

impl Coord for UiWorld {}

pub fn draw_world(
    world: Res<World>, 
    odors: Query<&Odor<OdorKind>>, 
    foods: Query<&Food>,
    mut ui_world: ResMut<UiWorld>, 
    mut ui_canvas: ResMut<UiCanvas>
) {
    if ui_world.image.is_none() {
        let mut vec = Vec::<[u8; 4]>::new();

        for j in 0..ui_world.height {
            for i in 0..ui_world.width {
                vec.push(Color::from(&world[(i, j)]).to_rgba_vec());
            }
        }

        let colors = Tensor::from(&vec);
        let colors = colors.reshape([ui_world.height as usize, ui_world.width as usize, 4]);

        let image = ui_canvas.create_image(colors);
        ui_world.image = image.clone();
        ui_world.view.write(|v| v.image = image);
    }

    // TODO: cache texture when unmodified
    if let Some(mut ui) = ui_canvas.renderer() {
        //let to_canvas = ui_world.to_canvas();

        //if let Some(image) = &ui_world.image {
        //    ui.draw_image(&ui_world.pos(), image.clone());
        //    ui.flush();
        //}

        //ui.flush();
        //ui_world.update(world.get(), ui.renderer());
        //ui_world.draw(ui.renderer(), &to_canvas).unwrap();
        //ui.flush();

        let circle: Path<Canvas> = paths::circle().transform(&ui_world.to_canvas_scale());
        let mut xy : Vec<[f32; 2]> = Vec::new();
        let mut sizes : Vec<[f32; 2]> = Vec::new();
        let colors : Vec<Color> = Vec::new();

        for odor in odors.iter() {
            xy.push([odor.x(), odor.y()]);
            sizes.push([odor.r(), odor.r()]);

            // colors.push(Color::from(odor.odor()).set_alpha(0.2));
        }

        let xy = ui_world.to_canvas().transform(&Tensor::from(xy));

        if xy.len() > 0 {
            ui.draw_markers(&circle, xy, sizes, &colors);
        }

        let mut xy : Vec<[f32; 2]> = Vec::new();
        let mut sizes : Vec<[f32; 2]> = Vec::new();
        let mut colors : Vec<Color> = Vec::new();

        for food in foods.iter() {
            let pos = food.pos();

            xy.push([pos.x(), pos.y()]);
            sizes.push([food.radius(), food.radius()]);

            let color = match food.kind() {
                FoodKind::None => Color::from("black"),
                // FoodKind::Plain => Color::from("pumpkin orange"),
                FoodKind::Plain => Color::from("teal"),
                FoodKind::Sweet => Color::from("cherry red"),
                FoodKind::Bitter => Color::from("mustard yellow"),
                FoodKind::Sick => Color::from("brownish green"),
            };
            colors.push(color);
        }

        let xy = ui_world.to_canvas().transform(&Tensor::from(xy));

        if xy.len() > 0 {
            // ui.flush();
            let star: Path<Canvas> = paths::unit_star(8, 0.6)
                .transform(&ui_world.to_canvas_scale());
            ui.draw_markers(&star, xy, sizes, &colors);
        }
    }
}

impl From<&Wall> for Color {
    fn from(value: &Wall) -> Self {
        match value {
            Wall::Empty => Color::from_hsv(0.25, 0.0, 0.98),
            Wall::Food => Color::from("green"),
            Wall::Wall => Color::from("dark beige"),

            Wall::FloorLight => Color::from(0xf8f8f8),
            Wall::FloorDark => Color::from(0x606060),
        }
    }
}

impl From<OdorInnate> for Color {
    fn from(value: OdorInnate) -> Self {
        match value {
            OdorInnate::Food => Color::from("green"),
            OdorInnate::Avoid => Color::from("red"),
            OdorInnate::None => Color::from("purple"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawAgent;

struct UiWorldView {
    bounds: Bounds<UiWorld>,
    pos: Bounds<Canvas>,

    clip: Clip,
    to_canvas: Affine2d,

    image: Option<ImageId>,
}

impl UiWorldView {
    fn new(bounds: impl Into<Bounds<UiWorld>>) -> Self {
        Self {
            bounds: bounds.into(),
            pos: Bounds::zero(),

            image: None,

            clip: Clip::None,
            to_canvas: Affine2d::eye(),
        }
    }
}

impl Drawable for UiWorldView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        // todo!()
        if let Some(image) = &self.image {
            renderer.draw_image_ref(&self.pos, image.clone())?;
        }

        Ok(())
    }

    fn resize(
        &mut self, 
        _renderer: &mut dyn Renderer, 
        pos: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        let aspect = self.bounds.width() / self.bounds.height();

        // force bounds to match aspect ratio
        let (c_width, c_height) = if aspect * pos.height() <= pos.width() {
            (aspect * pos.height(), pos.height())
        } else {
            (pos.width(), pos.width() / aspect)
        };

        // center the box
        let xmin = pos.xmin() + 0.5 * (pos.width() - c_width);
        let ymin = pos.ymin() + 0.5 * (pos.height() - c_height);

        let xmin = xmin.max(10.);
        let ymin = ymin.max(10.);

        //let xmin = pos.xmin();
        //let ymin = pos.ymin();

        let c_width = c_width - xmin - pos.xmin();
        let c_height = c_height - xmin - pos.xmin();

        let pos = Bounds::<Canvas>::new(
            Point(xmin, ymin),
            Point(xmin + c_width, ymin + c_height),
        );

        self.pos = pos;
        self.clip = Clip::from(&self.pos);
        self.to_canvas = self.bounds.affine_to(&self.pos);

        self.pos.clone()
    }
}

pub struct UiWorldPlugin {
    bounds: Bounds::<Layout>,

    // hex: UiWorldHex<OdorKind>,
}

impl UiWorldPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            // hex: UiWorldHex::new(),
        }
    }

    pub fn none(&mut self, _key: OdorKind) {
        // self.hex.none(key);
    }

    //pub fn tile(&mut self, key: OdorKind) -> TileBuilder {
        // self.hex.tile(key)
    //}
}

impl Plugin for UiWorldPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<WorldPlugin>());

            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let (width, height) = {
                let world = app.get_plugin::<WorldPlugin>().unwrap();
                (world.width(), world.height())
            };
            let view = UiWorldView::new([width as f32, height as f32]);
            let view = app.resource_mut::<UiCanvas>().view(self.bounds.clone(), view);

            /*
            let mut hex = UiWorldHex::<OdorKind>::new();
            
            hex.none(OdorKind::None);
            hex.tile(OdorKind::A).fill("red");
            hex.tile(OdorKind::B).fill("orange");
            hex.tile(OdorKind::C).fill("teal");
            */
            // let hex = self.hex.gen();


            let ui_world = UiWorld::new(view, width, height);

            //let mut hex = UiWorldHex::<OdorKind>::new(&ui_world);
            
            //hex.none(OdorKind::None);
            //hex.tile(OdorKind::A).fill("red");
            //hex.tile(OdorKind::B).fill("orange");
            //hex.tile(OdorKind::C).fill("teal");

            //app.insert_resource(hex);

            // let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
            app.insert_resource(ui_world);

            app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            app.system(Update, draw_world.phase(DrawWorld));
            // app.system(PreUpdate, world_resize);

            // app.system(Startup, spawn_ui_world);
        }
    }
}