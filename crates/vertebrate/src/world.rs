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

    fn add_odor(&mut self, x: usize, y: usize, odor: OdorType) {
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

pub struct Food {
    x: f32,
    y: f32,
    odor: OdorType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OdorType {
    FoodA,
    FoodB,
    AvoidA,
    AvoidB,
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
            OdorType::AvoidA => 2,
            OdorType::AvoidB => 3,
            OdorType::OtherA => 4,
            OdorType::OtherB => 5,
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
            2 => OdorType::AvoidA,
            3 => OdorType::AvoidB,
            4 => OdorType::OtherA,
            5 => OdorType::OtherB,
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

pub struct WorldPlugin {
    width: usize,
    height: usize,

    walls: Vec<(usize, usize)>,
    food: Vec<(usize, usize)>,
    odors: Vec<OdorItem>,
}

impl WorldPlugin {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            walls: Vec::new(),
            food: Vec::new(),
            odors: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn walls<const N: usize>(mut self, walls: [(usize, usize); N]) -> Self {
        for wall in walls {
            assert!(wall.0 < self.width);
            assert!(wall.1 < self.height);

            self.walls.push(wall);
        }
        self
    }

    pub fn wall(mut self, pos: (usize, usize), extent: (usize, usize)) -> Self {
        assert!(pos.0 + extent.0 < self.width);
        assert!(pos.1 + extent.1 < self.height);

        for j in 0..extent.1 {
            for i in 0..extent.0 {
                self.walls.push((pos.0 + i, pos.1 + j));
            }
        }

        self
    }

    pub fn food(mut self, x: usize, y: usize) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        self.food.push((x, y));
        
        self
    }

    pub fn food_odor(mut self, x: usize, y: usize, odor: OdorType) -> Self {
        self.food(x, y).odor(x, y, odor)
    }

    pub fn odor(mut self, x: usize, y: usize, odor: OdorType) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);
        
        self.odors.push(OdorItem::new(x, y, odor));

        self
    }

    fn create_world(&self) -> World {
        let mut world = World::new(self.width, self.height);

        for food in &self.food {
            world[*food] = WorldCell::Food;
        }

        for wall in &self.walls {
            world[*wall] = WorldCell::Wall;
        }

        for odor in &self.odors {
            world.add_odor(odor.pos.0, odor.pos.1, odor.odor);
        }
    
        world
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.create_world());
    }
}

struct OdorItem {
    pos: (usize, usize),
    odor: OdorType,
}

impl OdorItem {
    fn new(x: usize, y: usize, odor: OdorType) -> Self {
        Self {
            pos: (x, y),
            odor,
        }
    }
}

pub struct SlugWorldPlugin;

impl SlugWorldPlugin {
    pub fn new() -> WorldPlugin {
        let mut world = WorldPlugin::new(30, 20);

        world = world.walls([
            (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (8, 9), (8, 10), (8, 11), (8, 12),
            (18, 6), (19, 6), (20, 6), (25, 6), (26, 6),
            (20, 14), (21, 14), (22, 14), (26, 14), (27, 14)
        ]);

        world = world.food_odor(4, 2, OdorType::FoodA)
            .food_odor(20, 10, OdorType::FoodB);

        world
    } 
}
