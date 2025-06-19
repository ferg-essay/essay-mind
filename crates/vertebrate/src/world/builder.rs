use essay_ecs::app::{App, Plugin};

use crate::{util::Point, world::{Food, FoodKind}};

use super::{FloorType, World, Wall};

pub struct WorldPlugin {
    width: usize,
    height: usize,

    walls: Vec<(usize, usize)>,
    floor: Vec<FloorItem>,
}

impl WorldPlugin {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            walls: Vec::new(),
            floor: Vec::new(),
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
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

    fn create_world(&self) -> World {
        let mut world = World::new(self.width, self.height);

        for floor in &self.floor {
            let (x, y) = floor.pos;
            let (w, h) = floor.extent;
            let cell = match floor.floor {
                FloorType::Light => Wall::FloorLight,
                FloorType::Dark => Wall::FloorDark,
            };

            for j in y..y + h {
                for i in x..x + w {
                    world[(i, j)] = cell;
                }
            }
        }

        for wall in &self.walls {
            world[*wall] = Wall::Wall;
        }

        world
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.create_world());
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
