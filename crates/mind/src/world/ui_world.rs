use essay_ecs::prelude::*;
use essay_plot::prelude::*;
use essay_tensor::Tensor;
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::world::ApicalWorldPlugin;

use super::{World, world::WorldItem};

#[derive(Component)]
pub struct UiWorld {
    id: BoxId,
    pos: Bounds<Canvas>,
    bounds: Bounds<World>,
    width: usize,
    height: usize,
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
        }
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
        ui_world.pos = ui_layout.get_box(ui_world.id).clone();
    }
}

pub fn draw_world(
    world: Res<World>, 
    ui_world: Res<UiWorld>, 
    mut ui: ResMut<UiCanvas>
) {
    let mut vec = Vec::<[u8; 4]>::new();

    for j in 0..ui_world.height {
        for i in 0..ui_world.width {
            match world[(i, j)] {
                WorldItem::Empty => vec.push(Color::from("black").to_rgba_vec()),
                WorldItem::Wall => vec.push(Color::from("beige").to_rgba_vec()),
            }
            
        }
    }

    let colors = Tensor::from(&vec);
    let colors = colors.reshape([ui_world.height as usize, ui_world.width as usize, 4]);

    // TODO: cache texture when unmodified

    ui.draw_image(&ui_world.pos, colors);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

pub fn spawn_ui_world(
    mut commands: Commands,
    mut ui_layout: ResMut<UiLayout>,
) {
    // spawn_world(commands);

    let id = ui_layout.add_box(Bounds::from([1., 1.]));

    let mut ui_world = UiWorld::new(id, 15, 10);

    commands.insert_resource(ui_world);
}

pub struct UiApicalWorldPlugin;

impl Plugin for UiApicalWorldPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<UiCanvasPlugin>());
        assert!(app.contains_plugin::<ApicalWorldPlugin>());

        if ! app.contains_plugin::<UiLayoutPlugin>() {
            app.plugin(UiLayoutPlugin);
        }

        app.phase(Update, (DrawWorld, DrawItem).chained());
        app.system(Update, draw_world.phase(DrawWorld));
        app.system(PreUpdate, world_resize);

        app.system(Startup, spawn_ui_world);
    }
}