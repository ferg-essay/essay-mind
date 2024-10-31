use std::{collections::HashMap, hash::Hash, ops::{Deref, DerefMut, Index, IndexMut}};

use essay_ecs::app::{App, Plugin};

use crate::util::Point;

use super::World;

#[derive(Clone)]
pub struct WorldHex<T: WorldHexTrait> {
    width: usize,
    height: usize,

    vec: Vec<T>,

    update_count: usize,
}

impl<T: WorldHexTrait> WorldHex<T> {
    pub fn new(width: usize, height: usize, kind: T) -> WorldHex<T> {
        let hex_width = width + 1;

        let hex_height = height + 1;

        let mut vec = Vec::new();

        for _ in 0..hex_height {
            for _ in 0..hex_width {
                vec.push(kind.clone());
            }
        }

        Self {
            vec,
            width: hex_width,
            height: hex_height,
            update_count: 1,
        }
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn update_count(&self) -> usize {
        self.update_count
    }

    pub fn circle(&mut self, pos: impl Into<Point>, r: f32, kind: T) {
        let Point(x, y) = pos.into();
        let (x, y) = (x as i32, y as i32);
        let r = r as i32;

        let x0 = x as f32;
        let y0 = y as f32 + if x % 2 == 0 { 0.5 } else { 0. };

        for j in (y - r).max(0)..(y + r).min(self.height() as i32) {
            for i in (x - r).max(0)..(x + r).min(self.width() as i32) {
                let x1 = i as f32;
                let y1 = j as f32 + if i % 2 == 0 { 0.5 } else { 0. };

                if (x1 - x0).hypot(y1 - y0) < r as f32 - 0.5 {
                    self[(i as usize, j as usize)] = kind.clone();
                }
            }
        }
    }
}

impl<T: WorldHexTrait + Default> WorldHex<T> {
    pub fn fill<K>(&mut self, source: &WorldHex<K>, map: &HashMap<K, T>)
    where
        K: WorldHexTrait + Eq + Hash
    {
        assert_eq!(self.width(), source.width());
        assert_eq!(self.height(), source.height());

        for (cell, k) in self.vec.iter_mut().zip(&source.vec) {
            match map.get(k) {
                Some(value) => *cell = value.clone(),
                None => *cell = T::default(),
            }
        }

        self.update_count += 1;
    }
}

impl<T: WorldHexTrait> Index<(usize, usize)> for WorldHex<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        &self.vec[index.1 * self.width + index.0]
    }
}

impl<T: WorldHexTrait> Index<Point> for WorldHex<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let x = index.x().max(0.).min(self.width as f32 - 1.) as usize;
        let y = (index.y() + if x % 2 == 0 { 0.} else { 0.5 })
            .max(0.)
            .min(self.height as f32 - 1.) as usize;

        &self.vec[y * self.width + x]
    }
}

impl<T: WorldHexTrait> IndexMut<(usize, usize)> for WorldHex<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        self.update_count += 1;

        &mut self.vec[index.1 * self.width + index.0]
    }
}

pub struct WorldHexPlugin<T: WorldHexTrait> {
    world: WorldHex<T>,
}

impl<T: WorldHexTrait + Default> WorldHexPlugin<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: WorldHex::new(width, height, T::default()),
        }
    }
}

impl<T: WorldHexTrait + Default> From<World> for WorldHexPlugin<T> {
    fn from(world: World) -> Self {
        Self::new(world.width(), world.height())
    }
}

impl<T: WorldHexTrait + Default, K1: WorldHexTrait> From<&WorldHexPlugin<K1>> for WorldHexPlugin<T> {
    fn from(world: &WorldHexPlugin<K1>) -> Self {
        Self::new(world.width(), world.height())
    }
}

impl<T: WorldHexTrait> Deref for WorldHexPlugin<T> {
    type Target = WorldHex<T>;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl<T: WorldHexTrait> DerefMut for WorldHexPlugin<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}

impl<T: WorldHexTrait> Plugin for WorldHexPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.world.clone());
    }
}

pub trait WorldHexTrait : Send + Sync + Clone + 'static {

}
