use std::ops::{Index, IndexMut};

use crate::util::Point;

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Wall>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || Wall::default());

        Self {
            width,
            height,
            cells: values,
        }
    }

    #[inline]
    pub fn extent(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<(usize, usize)> for World {
    type Output = Wall;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        &self.cells[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        &mut self.cells[index.1 * self.width + index.0]
    }
}

impl Index<(f32, f32)> for World {
    type Output = Wall;

    fn index(&self, index: (f32, f32)) -> &Self::Output {
        assert!(index.0 >= 0.);
        assert!(index.1 >= 0.);

        let x = index.0 as usize;
        let y = index.1 as usize;

        assert!(x < self.width);
        assert!(y < self.height);

        &self.cells[y * self.width + x]
    }
}

impl IndexMut<(f32, f32)> for World {
    fn index_mut(&mut self, index: (f32, f32)) -> &mut Self::Output {
        assert!(index.0 >= 0.);
        assert!(index.1 >= 0.);

        let x = index.0 as usize;
        let y = index.1 as usize;

        assert!(x < self.width);
        assert!(y < self.height);

        &mut self.cells[y * self.width + x]
    }
}

pub trait WorldType : Default + Send + Sync + 'static {
    fn is_collide(&self) -> bool;
}

impl World {
    pub fn is_collide(&self, pt: impl Into<Point>) -> bool {
        let Point(x, y) = pt.into();

        if x <= 0. || x >= self.width as f32 || y <= 0. || y >= self.height as f32 {
            return true;
        }

        self[(x.floor() as usize, y.floor() as usize)].is_collide()
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
            Wall::Empty => 1.,
            Wall::Food => 1.,
            Wall::Wall => -1.,
            Wall::FloorLight => 1.,
            Wall::FloorDark => 0.,
        }
    }

    pub fn is_food(&self, pt: impl Into<Point>) -> bool {
        let Point(x, y) = pt.into();

        if x <= 0. || x >= self.width as f32 || y <= 0. || y >= self.height as f32 {
            return false;
        }

        match self[(x.floor() as usize, y.floor() as usize)] {
            Wall::Food => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Wall {
    Empty,
    Food,
    Wall,

    FloorLight,
    FloorDark,
}

impl Default for Wall {
    fn default() -> Self {
        Wall::Empty
    }
}

impl WorldType for Wall {
    fn is_collide(&self) -> bool {
        match self {
            Wall::Wall => true,
            _ => false,
        }
    }
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