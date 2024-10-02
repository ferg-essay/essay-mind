use std::ops::{Index, IndexMut};

use essay_ecs::prelude::*;

use crate::util::Point;

use super::{hex_odor::OdorKind, HexOdorWorld};

#[derive(Component)]
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<WorldCell>,
    hex: HexOdorWorld,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldCell::Empty);

        let mut hex = HexOdorWorld::new(width, height, 0.866);
        hex[(3, 5)] = OdorKind::A;
        hex[(3, 6)] = OdorKind::C;
        hex[(4, 5)] = OdorKind::B;
        hex[(5, 5)] = OdorKind::A;
        hex[(7, 5)] = OdorKind::D;

        Self {
            width,
            height,
            cells: values,
            hex,
        }
    }

    pub fn extent(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn hex(&self) -> &HexOdorWorld {
        &self.hex
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

#[derive(Clone, Copy, PartialEq)]
pub enum FloorType {
    Light,
    Dark,
}

#[cfg(test)]
mod test {
    use crate::world::World;

    #[test]
    fn world_extent() {
        assert_eq!(World::new(7, 8).extent(), (7, 8));
    }
}