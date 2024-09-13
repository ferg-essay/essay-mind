use essay_ecs::app::{App, Plugin};

use crate::world::odor::Odor;

use super::{FloorType, OdorType, Odors, World, WorldCell};


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

        world
    }

    fn create_odors(&self) -> Odors {
        let mut odors = Odors::new();

        for odor in &self.odors {
            odors.add_odor(odor.pos.0, odor.pos.1, odor.r, odor.odor);
        }

        odors
    }

    pub fn build(self, app: &mut App) {
        app.insert_resource(self.create_world());
        app.insert_resource(self.create_odors());
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.create_world());
        app.insert_resource(self.create_odors());
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
            odor 
        }
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
