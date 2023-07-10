use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::PathStyle};

use essay_tensor::Tensor;
use ui_graphics::UiCanvas;

#[derive(Component)]
pub struct World {
    width: usize,
    height: usize,
    color: Color,
    values: Vec<WorldItem>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldItem::Empty);

        Self {
            width,
            height,
            color: Color::from("dark teal"),
            values,
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
    Wall
}

pub fn spawn_world(mut commands: Commands) {
    let mut world = World::new(15, 10);
    world[(4, 2)] = WorldItem::Wall;
    world[(5, 5)] = WorldItem::Wall;
    world[(6, 6)] = WorldItem::Wall;

    commands.insert_resource(world);
}

pub fn draw_world(world: Res<World>, mut ui: ResMut<UiCanvas>) {
    let mut vec = Vec::<[u8; 4]>::new();

    for j in 0..world.height {
        for i in 0..world.width {
            match world[(i, j)] {
                WorldItem::Empty => vec.push(Color::from("black").to_rgba_vec()),
                WorldItem::Wall => vec.push(Color::from("beige").to_rgba_vec()),
            }
            
        }
    }

    let colors = Tensor::from(&vec);
    let colors = colors.reshape([world.height as usize, world.width as usize, 4]);

    ui.draw_image(&Bounds::<Canvas>::from([200., 200., 600., 600.]), colors);
}