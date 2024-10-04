use essay_ecs::{app::{App, Plugin, Startup}, core::Commands};

use crate::world::odor::Odor;

use super::{food::FoodKind, FloorType, Food, OdorKind, OdorType, World, WorldCell, WorldHex};

pub struct WorldPlugin {
    width: usize,
    height: usize,

    walls: Vec<(usize, usize)>,
    food: Vec<Food>,
    odors: Vec<OdorItem>,
    floor: Vec<FloorItem>,

    loc_odor: Vec<LocOdorItem>,
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

            loc_odor: Vec::new(),
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

        self.food.push(Food::new((x as f32 + 0.5, y as f32 + 0.5)));

        self
    }

    pub fn food_kind(mut self, x: usize, y: usize, kind: impl Into<FoodKind>) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        self.food.push(Food::new((x as f32 + 0.5, y as f32 + 0.5)).set_kind(kind.into()));

        self
    }

    pub fn food_odor(self, x: usize, y: usize, odor: OdorType) -> Self {
        self.food(x, y).odor(x, y, odor)
    }

    pub fn food_odor_r(self, x: usize, y: usize, food: FoodKind, r: usize, odor: OdorType) -> Self {
        self.food_kind(x, y, food).odor_r(x, y, r, odor)
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

    pub fn loc_odor(mut self, x: usize, y: usize, r: usize, odor: OdorKind) -> Self {
        assert!(x < self.width);
        assert!(y < self.height);

        self.loc_odor.push(LocOdorItem::new(x, y, r, odor));

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

        //for food in &self.food {
        //    world[*food] = WorldCell::Food;
        //}

        for wall in &self.walls {
            world[*wall] = WorldCell::Wall;
        }

        world
    }

    fn create_world_hex(&self) -> WorldHex<OdorKind> {
        let mut world = WorldHex::<OdorKind>::new(self.width, self.height, 1.);

        for item in &self.loc_odor {
            item.fill(&mut world);
        }

        world
    }

    fn create_food(&self, app: &mut App) {
        let mut foods : Vec<Food> = self.food.clone();

        app.system(Startup, move |mut cmd: Commands| {
            for food in foods.drain(..) {
                cmd.spawn(food);
            }
        });
    }

    fn create_odors(&self, app: &mut App) {
        let mut odors : Vec<Odor> = self.odors.iter().map(|odor| {
            Odor::new_r(odor.pos.0, odor.pos.1, odor.r, odor.odor)
        }).collect();

        app.system(Startup, move |mut cmd: Commands| {
            for odor in odors.drain(..) {
                cmd.spawn(odor);
            }
        });
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.create_world());

        app.insert_resource(self.create_world_hex());

        self.create_odors(app);

        self.create_food(app);
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

struct LocOdorItem {
    pos: (usize, usize),
    r: usize,
    odor: OdorKind,
}

impl LocOdorItem {
    fn new(x: usize, y: usize, r: usize, odor: OdorKind) -> Self {
        Self { 
            pos: (x, y), 
            r,
            odor 
        }
    }

    fn fill(&self, world: &mut WorldHex<OdorKind>) {
        let x = self.pos.0 as u32;
        let y = self.pos.1 as u32;
        let r = self.r as u32;

        let x0 = x as f32;
        let y0 = y as f32 + if x % 2 == 0 { 0.5 } else { 0. };

        for j in (y - r).max(0)..(y + r).min(world.height() as u32) {
            for i in (x - r).max(0)..(x + r).min(world.width() as u32) {
                let x1 = i as f32;
                let y1 = j as f32 + if i % 2 == 0 { 0.5 } else { 0. };

                if (x1 - x0).hypot(y1 - y0) < r as f32 - 0.5 {
                    world[(i as usize, j as usize)] = self.odor.clone();
                }
            }
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
