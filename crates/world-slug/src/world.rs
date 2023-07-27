use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;
use essay_plot::prelude::*;

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

    /*
    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
    */

    pub fn extent(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    pub fn is_collide(&self, pt: impl Into<Point>) -> bool {
        let Point(x, y) = pt.into();
        
        if x <= 0.
        || x >= self.width as f32
        || y <= 0.
        || y >= self.height as f32 {
            return true;
        }

        match self[(x.floor() as usize, y.floor() as usize)] {
            WorldItem::Wall => true,
            _ => false,
        }
    }

    pub fn is_food(&self, pt: impl Into<Point>) -> bool {
        let Point(x, y) = pt.into();
        
        if x <= 0.
        || x >= self.width as f32
        || y <= 0.
        || y >= self.height as f32 {
            return false;
        }

        match self[(x.floor() as usize, y.floor() as usize)] {
            WorldItem::Food => true,
            _ => false,
        }
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
    Food,
    Wall
}

pub fn spawn_world(
    mut commands: Commands,
) {
    let mut world = World::new(15, 10);
    for (x, y) in vec![
        (1, 1), (8, 1),
        (3, 3), (6, 3),
        (9, 5), (13, 6),
        (6, 7), (10, 7),
        (11, 9), (13, 9),
    ] {
        world[(x, y)] = WorldItem::Food;
    }

    for (x, y) in vec![
        (4, 2), (4, 3), (4, 4), (4, 5), (4, 6),
        (6, 0), (6, 1), (6, 2), (6, 7), (6, 8),
        (9, 3), (10, 3), (11, 3), (13, 3), (14, 3),
        (10, 7), (11, 7), (13, 7),
    ] {
        world[(x, y)] = WorldItem::Wall;
    }

    commands.insert_resource(world);
}

pub struct SlugWorldPlugin;

impl Plugin for SlugWorldPlugin {
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