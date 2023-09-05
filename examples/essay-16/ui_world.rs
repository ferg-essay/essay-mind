use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::paths};
use essay_tensor::Tensor;
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::world::{World, OdorType};

use super::world::{SlugWorldPlugin, WorldCell};

#[derive(Component)]
pub struct UiWorld {
    id: BoxId,
    pos: Bounds<Canvas>,
    bounds: Bounds<World>,
    width: usize,
    height: usize,
    image: Option<ImageId>,
}

impl UiWorld {
    pub fn new(id: BoxId, width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldCell::Empty);

        Self {
            id,
            pos: Bounds::zero(),
            width,
            height,
            bounds: Bounds::from([width as f32, height as f32]),
            image: None,
        }
    }

    pub fn _extent(&self) -> (f32, f32) {
        (self.bounds.xmax(), self.bounds.ymax())
    }

    pub fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        let aspect = self.bounds.width() / self.bounds.height();

        let (c_width, c_height) = if aspect * set_pos.height() <= set_pos.width() {
            (aspect * set_pos.height(), set_pos.height())
        } else {
            (set_pos.width(), set_pos.width() / aspect)
        };

        let pos = Bounds::<Canvas>::new(
            Point(set_pos.xmin(), set_pos.ymin()),
            Point(set_pos.xmin() + c_width, set_pos.ymin() + c_height),
        );

        self.pos = pos;
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
}

impl Coord for World {}


pub fn world_resize(
    mut ui_world: ResMut<UiWorld>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_world.id;
        ui_world.set_pos(ui_layout.get_box(id));
    }
}

pub fn draw_world(
    world: Res<World>, 
    mut ui_world: ResMut<UiWorld>, 
    mut ui: ResMut<UiCanvas>
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

        ui_world.image = ui.create_image(colors);
    }

    // TODO: cache texture when unmodified

    if let Some(image) = &ui_world.image {
        ui.draw_image(&ui_world.pos, image.clone());
    }

    let circle = paths::circle().transform(&ui_world.to_canvas());
    let mut xy : Vec<[f32; 2]> = Vec::new();
    let mut sizes : Vec<[f32; 2]> = Vec::new();
    let mut colors : Vec<Color> = Vec::new();

    for odor in world.odors() {
        xy.push([odor.x(), odor.y()]);
        sizes.push([odor.r(), odor.r()]);

        colors.push(Color::from(odor.odor()).set_alpha(0.2));
    }

    let xy = ui_world.to_canvas().transform(&Tensor::from(xy));

    ui.draw_markers(&circle, xy, sizes, &colors);
}

impl From<&WorldCell> for Color {
    fn from(value: &WorldCell) -> Self {
        match value {
            WorldCell::Empty => Color::from("black"),
            WorldCell::Food => Color::from("amber"),
            WorldCell::Wall => Color(0xbfbfbfff),
        }
    }
}

impl From<OdorType> for Color {
    fn from(value: OdorType) -> Self {
        match value {
            OdorType::FoodA => Color::from("amber"),
            OdorType::FoodB => Color::from("red"),
            OdorType::OtherA => Color::from("azure"),
            OdorType::OtherB => Color::from("blue"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawAgent;

pub struct UiSlugWorldPlugin {
    bounds: Bounds::<UiLayout>,
}

impl UiSlugWorldPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
        }
    }
}

impl Plugin for UiSlugWorldPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<SlugWorldPlugin>());

            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
            let ui_world = UiWorld::new(box_id, 30, 20);
            app.insert_resource(ui_world);

            app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            app.system(Update, draw_world.phase(DrawWorld));
            app.system(PreUpdate, world_resize);

            // app.system(Startup, spawn_ui_world);
        }
    }
}