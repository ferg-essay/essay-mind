use core::fmt;
use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;

use crate::util::{Point, Angle, EgoVector};

#[derive(Component)]
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<WorldCell>,
    odors: Vec<Odor>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldCell::Empty);

        Self {
            width,
            height,
            cells: values,
            odors: Vec::new(),
        }
    }

    pub fn extent(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn is_collide(&self, pt: impl Into<Point>) -> bool {
        let Point(x, y) = pt.into();

        if x <= 0. || x >= self.width as f32 || y <= 0. || y >= self.height as f32 {
            return true;
        }

        match self[(x.floor() as usize, y.floor() as usize)] {
            WorldCell::Wall => true,
            _ => false,
        }
    }

    pub fn light(&self, pt: impl Into<Point>) -> f32 {
        let Point(x, y) = pt.into();

        let (x, y) = (x.floor(), y.floor());

        if x < 0. || x > self.width as f32 {
            return -1.;
        }
        if y < 0. || y > self.height as f32 {
            return -1.;
        }

        let x = (x as usize).clamp(0, self.width - 1);
        let y = (y as usize).clamp(0, self.height - 1);

        match self[(x, y)] {
            WorldCell::Empty => 1.,
            WorldCell::Food => 1.,
            WorldCell::Wall => -1.,
            WorldCell::FloorLight => 1.,
            WorldCell::FloorDark => 0.,
        }
    }

    fn add_odor(&mut self, x: usize, y: usize, r: usize, odor: OdorType) {
        self.odors.push(Odor::new_r(x, y, r, odor));
    }

    pub fn odor(&self, pt: Point) -> Option<(OdorType, Angle)> {
        let Point(x, y) = pt;

        let mut best_odor: Option<(OdorType, Angle)> = None;
        let mut best_dist = f32::MAX;

        for food in &self.odors {
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

        if x <= 0. || x >= self.width as f32 || y <= 0. || y >= self.height as f32 {
            return false;
        }

        match self[(x.floor() as usize, y.floor() as usize)] {
            WorldCell::Food => true,
            _ => false,
        }
    }

    pub fn odors(&self) -> &Vec<Odor> {
        &self.odors
    }

    pub fn odors_by_head(&self, point: Point) -> Vec<(OdorType, EgoVector)> {
        let mut odors = Vec::new();

        for odor in &self.odors {
            let dist = point.dist(&odor.pos());

            if dist < odor.r() {
                let angle = point.heading_to(odor.pos());
                let value = 0.5 / dist.max(0.5);

                odors.push((odor.odor(), EgoVector::new(angle, value)));
            }
        }
        
        odors
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

#[derive(Copy, Clone, Debug)]
pub enum WorldCell {
    Empty,
    Food,
    Wall,

    FloorLight,
    FloorDark,
}

pub struct Odor {
    x: f32,
    y: f32,
    r: f32,
    odor: OdorType,
}

impl Odor {
    pub const RADIUS: f32 = 3.;

    fn new_r(x: usize, y: usize, r:usize, odor: OdorType) -> Self {
        Self {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            r: r as f32,
            odor,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn pos(&self) -> Point {
        Point(self.x, self.y)
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn is_food(&self) -> bool {
        self.odor.is_food()
    }

    pub fn odor(&self) -> OdorType {
        self.odor
    }
}

impl fmt::Debug for Odor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Food").field(&self.x).field(&self.y).finish()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum FloorType {
    Light,
    Dark,
}

pub struct WorldPlugin {
    width: usize,
    height: usize,

    walls: Vec<(usize, usize)>,
    food: Vec<(usize, usize)>,
    odors: Vec<OdorItem>,
    floor: Vec<FloorItem>,
}

impl WorldPlugin {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            walls: Vec::new(),
            floor: Vec::new(),
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
        assert!(pos.0 + extent.0 <= self.width);
        assert!(pos.1 + extent.1 <= self.height);

        for j in 0..extent.1 {
            for i in 0..extent.0 {
                self.walls.push((pos.0 + i, pos.1 + j));
            }
        }

        self
    }

    pub fn floor(mut self, pos: (usize, usize), extent: (usize, usize), floor: FloorType) -> Self {
        assert!(pos.0 + extent.0 <= self.width);
        assert!(pos.1 + extent.1 <= self.height);

        self.floor.push(FloorItem::new(pos, extent, floor));

        self
    }

    pub fn food(mut self, x: usize, y: usize) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        self.food.push((x, y));

        self
    }

    pub fn food_odor(self, x: usize, y: usize, odor: OdorType) -> Self {
        self.food(x, y).odor(x, y, odor)
    }

    pub fn food_odor_r(self, x: usize, y: usize, r: usize, odor: OdorType) -> Self {
        self.food(x, y).odor_r(x, y, r, odor)
    }

    pub fn odor(mut self, x: usize, y: usize, odor: OdorType) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        let r = Odor::RADIUS as usize;

        self.odors.push(OdorItem::new(x, y, r, odor));

        self
    }

    pub fn odor_r(mut self, x: usize, y: usize, r: usize, odor: OdorType) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        self.odors.push(OdorItem::new(x, y, r, odor));

        self
    }

    fn create_world(&self) -> World {
        let mut world = World::new(self.width, self.height);

        for floor in &self.floor {
            let (x, y) = floor.pos;
            let (w, h) = floor.extent;
            let cell = match floor.floor {
                FloorType::Light => WorldCell::FloorLight,
                FloorType::Dark => WorldCell::FloorDark,
            };

            for j in y..y + h {
                for i in x..x + w {
                    world[(i, j)] = cell;
                }
            }
        }

        for food in &self.food {
            world[*food] = WorldCell::Food;
        }

        for wall in &self.walls {
            world[*wall] = WorldCell::Wall;
        }

        for odor in &self.odors {
            world.add_odor(odor.pos.0, odor.pos.1, odor.r, odor.odor);
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
    r: usize,
    odor: OdorType,
}

impl OdorItem {
    fn new(x: usize, y: usize, r: usize, odor: OdorType) -> Self {
        Self { 
            pos: (x, y), 
            r,
            odor }
    }
}

struct FloorItem {
    pos: (usize, usize),
    extent: (usize, usize),
    floor: FloorType,
}

impl FloorItem {
    fn new(pos: (usize, usize), extent: (usize, usize), floor: FloorType) -> Self {
        Self { pos, extent, floor }
    }
}

#[cfg(test)]
mod test {
    use crate::world::World;

    #[test]
    fn world_extent() {
        assert_eq!(World::new(7, 8).extent(), (7, 8));
    }
}