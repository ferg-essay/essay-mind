use std::{ops::{Index, IndexMut}, time::Instant};

use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::PathStyle};

use essay_tensor::Tensor;
use ui_graphics::{UiCanvas, ui_layout::{UiLayout, BoxId, UiLayoutEvent}};

#[derive(Component)]
pub struct World {
    id: BoxId,
    pos: Bounds<Canvas>,

    width: usize,
    height: usize,
    color: Color,
    values: Vec<WorldItem>,
}

impl World {
    pub fn new(id: BoxId, width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldItem::Empty);

        Self {
            id,
            pos: Bounds::zero(),
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

pub fn spawn_world(
    mut commands: Commands,
    mut ui_layout: ResMut<UiLayout>,
) {
    let id = ui_layout.add_box(Bounds::from([1., 1.]));

    let mut world = World::new(id, 15, 10);
    world[(4, 2)] = WorldItem::Wall;
    world[(5, 5)] = WorldItem::Wall;
    world[(6, 6)] = WorldItem::Wall;

    commands.insert_resource(world);
}

pub fn world_resize(
    mut world: ResMut<World>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>) {
    for _ in read.iter() {
        world.pos = ui_layout.get_box(world.id).clone();
    }
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

    // TODO: cache texture when unmodified

    ui.draw_image(&world.pos, colors);
}