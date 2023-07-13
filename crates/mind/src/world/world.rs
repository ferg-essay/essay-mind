use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::Tensor;
use ui_graphics::{UiCanvas, ui_layout::{UiLayout, BoxId, UiLayoutEvent, UiLayoutPlugin}, UiCanvasPlugin};

use super::ui_world::{UiWorld, UiApicalWorldPlugin};

#[derive(Component)]
pub struct World {
    width: usize,
    height: usize,
    values: Vec<WorldItem>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldItem::Empty);

        Self {
            width,
            height,
            values,
        }
    }

    /*
    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
    */
}

impl Index<(usize, usize)> for World {
    type Output = WorldItem;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.1 * self.width + index.0]
    }
}

pub enum WorldItem {
    Empty,
    Wall
}

pub fn spawn_world(
    mut commands: Commands,
) {
    let id = BoxId::new(0);

    let mut world = World::new(15, 10);
    world[(4, 2)] = WorldItem::Wall;
    world[(5, 5)] = WorldItem::Wall;
    world[(6, 6)] = WorldItem::Wall;

    commands.insert_resource(world);
}

pub struct ApicalWorldPlugin;

impl Plugin for ApicalWorldPlugin {
    fn build(&self, app: &mut App) {
        app.system(Startup, spawn_world);

        if app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiApicalWorldPlugin);

            /*
            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            app.phase(Update, (DrawWorld, DrawItem).chained());
            app.system(Update, draw_world.phase(DrawWorld));
            app.system(PreUpdate, world_resize);

            app.system(Startup, spawn_world_ui);
            */
        }
    }
}