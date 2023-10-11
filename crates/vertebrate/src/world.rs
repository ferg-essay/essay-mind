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

    fn add_food(&mut self, x: usize, y: usize, odor: OdorType) {
        if odor.is_food() {
            self[(x, y)] = WorldCell::Food;
        }

        self.food.push(Food::new(x, y, odor));
    }

    pub fn odor(&self, pt: Point) -> Option<(OdorType, Angle)> {
        let Point(x, y) = pt;

        let mut best_odor : Option<(OdorType, Angle)> = None;
        let mut best_dist = f32::MAX;

        for food in &self.food {
            let dx = food.x - x;
            let dy = food.y - y;
            let dist = dx.hypot(dy);

            if dist <= food.r() && dist < best_dist {
                let angle = dy.atan2(dx);

                best_odor = Some((food.odor(), Angle::Rad(angle)));
                best_dist = dist;
            }
        }

       best_odor
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

    pub fn odors(&self) -> &Vec<Food> {
        &self.food
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
    let mut world = World::new(30, 20);

    // sparse_food(&mut world);
    dense_food(&mut world);

    for (x, y) in vec![
        (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (8, 9), (8, 10), (8, 11), (8, 12),
        (18, 6), (19, 6), (20, 6), (25, 6), (26, 6),
        (20, 14), (21, 14), (22, 14), (26, 14), (27, 14)
    ] {
        world[(x, y)] = WorldCell::Wall;
    }

    world
}

fn dense_food(world: &mut World) {
    for (x, y) in vec![
        (4, 2),
    ] {
        world.add_food(x, y, OdorType::FoodA);
    }

    for (x, y) in vec![
        (20, 10),
    ] {
        world.add_food(x, y, OdorType::FoodB);
    }

    let is_distractor = true;

    if is_distractor {
        for (x, y) in vec![
            (2, 8),
        ] {
            world.add_food(x, y, OdorType::OtherA);
        }

        for (x, y) in vec![
            (2, 8),
            (13, 9),
            (14, 2),
            (22, 16),
        ] {
            world.add_food(x, y, OdorType::OtherB);
        }
    }
}

pub struct Food {
    x: f32,
    y: f32,
    odor: OdorType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OdorType {
    FoodA,
    FoodB,
    OtherA,
    OtherB,
}

impl OdorType {
    pub fn is_food(&self) -> bool {
        match self {
            OdorType::FoodA => true,
            OdorType::FoodB => true,
            _ => false,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            OdorType::FoodA => 0,
            OdorType::FoodB => 1,
            OdorType::OtherA => 2,
            OdorType::OtherB => 3,
        }
    }

    pub fn count() -> usize {
        Self::OtherB.index() + 1
    }
}

impl From<usize> for OdorType {
    fn from(value: usize) -> Self {
        match value {
            0 => OdorType::FoodA,
            1 => OdorType::FoodB,
            2 => OdorType::OtherA,
            3 => OdorType::OtherB,
            _ => todo!(),
        }
    }
}

impl Food {
    pub const RADIUS : f32 = 3.;

    fn new(x: usize, y: usize, odor: OdorType) -> Self {
        Self {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            odor,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn r(&self) -> f32 {
        Self::RADIUS
    }

    pub fn is_food(&self) -> bool {
        self.odor.is_food()
    }

    pub fn odor(&self) -> OdorType {
        self.odor
    }
}

impl fmt::Debug for Food {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Food").field(&self.x).field(&self.y).finish()
    }
}

pub struct SlugWorldPlugin;

impl SlugWorldPlugin {
    pub fn new() -> WorldPlugin {
        WorldPlugin::new(30, 20)
    } 
    /*
    fn build(&self, app: &mut App) {
        app.insert_resource(create_world());
    }
    */
}

pub struct WorldPlugin {
    width: usize,
    height: usize,
}

impl WorldPlugin {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn create_world(&self) -> World {
        let mut world = World::new(self.width, self.height);
        /*
        // sparse_food(&mut world);
        dense_food(&mut world);
    
        for (x, y) in vec![
            (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (8, 9), (8, 10), (8, 11), (8, 12),
            (18, 6), (19, 6), (20, 6), (25, 6), (26, 6),
            (20, 14), (21, 14), (22, 14), (26, 14), (27, 14)
        ] {
            world[(x, y)] = WorldCell::Wall;
        }
        */
    
        world
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.create_world());
    }
}