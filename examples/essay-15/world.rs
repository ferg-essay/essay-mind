use core::fmt;
use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;
use essay_plot::prelude::*;

#[derive(Component)]
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<WorldCell>,
    food: Vec<Food>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldCell::Empty);

        Self {
            width,
            height,
            cells: values,
            food: Vec::new(),
        }
    }

    /*
    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
    */

    pub fn extent(&self) -> (usize, usize) {
        (self.width, self.height)
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
            WorldCell::Wall => true,
            _ => false,
        }
    }

    fn add_food(&mut self, x: usize, y: usize) {
        self[(x, y)] = WorldCell::Food;

        self.food.push(Food::new(x, y));
    }

    pub(crate) fn food_dir(&self, pt: Point, dist: f32) -> Option<Angle> {
        let Point(x, y) = pt;

        for food in &self.food {
            let dx = food.x - x;
            let dy = food.y - y;

            if dx.hypot(dy) <= dist {
                let angle = dy.atan2(dx);

                return Some(Angle::Rad(angle));
            }
        }

        None
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
            WorldCell::Food => true,
            _ => false,
        }
    }
}

impl Index<(usize, usize)> for World {
    type Output = WorldCell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.1 * self.width + index.0]
    }
}

pub enum WorldCell {
    Empty,
    Food,
    Wall
}

fn create_world() -> World {
    let mut world = World::new(15, 10);

    // sparse_food(&mut world);
    dense_food(&mut world);

    for (x, y) in vec![
        //(0, 0), (10, 0), (14, 0), (0, 9), (7, 9), (14, 9),
        (4, 2), (4, 3), (4, 4), (4, 5), (4, 6),
        // (6, 0), (6, 1), (6, 2), (6, 7), (6, 8),
        (9, 3), (10, 3), (13, 3),
        (10, 7), (11, 7), (13, 7),
    ] {
        world[(x, y)] = WorldCell::Wall;
    }

    world
}

fn dense_food(world: &mut World) {
    for (x, y) in vec![
        (8, 1),
        (3, 3), (6, 3),
        (9, 5), (13, 6),
        (6, 7), (10, 7),
        (2, 6),
        (11, 9),
    ] {
        world.add_food(x, y);
    }
}

fn sparse_food(world: &mut World) {
    for (x, y) in vec![
        (8, 1),
        (3, 3),
        (13, 6),
        (6, 7),
        (2, 6),
    ] {
        world.add_food(x, y);
    }
}

struct Food {
    x: f32,
    y: f32,
}

impl Food {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5
        }
    }
}

impl fmt::Debug for Food {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Food").field(&self.x).field(&self.y).finish()
    }
}

pub struct SlugWorldPlugin;

impl Plugin for SlugWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_world());
    }
}