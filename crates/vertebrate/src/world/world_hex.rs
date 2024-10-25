use std::ops::{Index, IndexMut};

use crate::util::Point;

pub struct WorldHex<K> {
    vec: Vec<HexItem<K>>,

    width: usize,
    height: usize,

    scale: f32,

    update_count: usize,
}

impl<K: Default> WorldHex<K> {
    pub fn new(width: usize, height: usize, scale: f32) -> WorldHex<K> {
        let hex_width = width + 1;

        let hex_height = height + 1;

        let mut vec = Vec::new();

        for _ in 0..hex_height {
            for _ in 0..hex_width {
                vec.push(HexItem::<K>::default());
            }
        }

        Self {
            vec,
            width: hex_width,
            height: hex_height,
            scale: scale,
            update_count: 1,
        }
    }
}

impl<K> WorldHex<K> {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn update_count(&self) -> usize {
        self.update_count
    }
}

impl<K> Index<(usize, usize)> for WorldHex<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        &self.vec[index.1 * self.width + index.0].kind
    }
}

impl<K> Index<Point> for WorldHex<K> {
    type Output = K;

    fn index(&self, index: Point) -> &Self::Output {
        let x = index.x().max(0.).min(self.width as f32 - 1.) as usize;
        let y = (index.y() + if x % 2 == 0 { 0.} else { 0.5 })
            .max(0.)
            .min(self.height as f32 - 1.) as usize;

        &self.vec[y * self.width + x].kind
    }
}

impl<K> IndexMut<(usize, usize)> for WorldHex<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.width);
        assert!(index.1 < self.height);

        &mut self.vec[index.1 * self.width + index.0].kind
    }
}

pub struct HexItem<K> {
    kind: K,
}

impl<K: Default> Default for HexItem<K> {
    fn default() -> Self {
        Self { 
            kind: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OdorKind {
    None,
    A,
    B,
    C,
    D,
    Bogus,
}

impl Default for OdorKind {
    fn default() -> Self {
        OdorKind::None
    }
}
