use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;

use ui_graphics::UiCanvasPlugin;

use super::ui_world::UiApicalWorldPlugin;

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

    pub fn extent(&self) -> [usize; 2] {
        [self.width, self.height]
    }
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
    let mut world = World::new(15, 10);
    world[(4, 2)] = WorldItem::Wall;
    world[(5, 5)] = WorldItem::Wall;
    world[(6, 6)] = WorldItem::Wall;

    commands.insert_resource(world);
}

pub struct PlanktonWorldPlugin;

impl Plugin for PlanktonWorldPlugin {
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