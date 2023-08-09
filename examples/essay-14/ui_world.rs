use essay_ecs::prelude::*;
use essay_plot::prelude::*;
use essay_tensor::Tensor;
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::world::World;

use super::world::{SlugWorldPlugin, WorldItem};

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

        values.resize_with(width * height, || WorldItem::Empty);

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
        let c_height = set_pos.height();
        let c_width = c_height * aspect;
        let pos = Bounds::<Canvas>::new(
            Point(self.pos.xmin(), self.pos.ymin()),
            Point(self.pos.xmin() + c_width, self.pos.ymin() + c_height),
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
                match world[(i, j)] {
                    WorldItem::Empty => vec.push(Color::from("black").to_rgba_vec()),
                    WorldItem::Food => vec.push(Color::from("dark teal").to_rgba_vec()),
                    WorldItem::Wall => vec.push(Color::from("beige").to_rgba_vec()),
                }
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawAgent;

pub fn spawn_ui_world(
    mut commands: Commands,
    mut ui_layout: ResMut<UiLayout>,
) {
    // spawn_world(commands);

    let id = ui_layout.add_box(Bounds::from([1., 1.]));

    let ui_world = UiWorld::new(id, 15, 10);

    commands.insert_resource(ui_world);
}

pub struct UiApicalWorldPlugin;

impl Plugin for UiApicalWorldPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<UiCanvasPlugin>());
        assert!(app.contains_plugin::<SlugWorldPlugin>());

        if ! app.contains_plugin::<UiLayoutPlugin>() {
            app.plugin(UiLayoutPlugin);
        }

        app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
        app.system(Update, draw_world.phase(DrawWorld));
        app.system(PreUpdate, world_resize);

        app.system(Startup, spawn_ui_world);
    }
}