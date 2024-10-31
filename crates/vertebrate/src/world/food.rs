use essay_ecs::{app::{App, Plugin, Startup}, core::{entity::EntityId, Commands, Component, Query, Res}};
use mind_ecs::Tick;
use util::random::Rand32;

use crate::{util::{Point, Ticks}, world::World};

use super::OdorKind;

fn update_food(
    query: Query<(EntityId, &Food)>,
    world: Res<World>,
    gen: Res<FoodGenerator>,
    mut command: Commands,
) {
    let mut food_count = 0;

    for (id, food) in query.iter() {
        if food.value <= 0. {
            command.entity(id).despawn();
        }

        food_count += 1;
    }

    while food_count < gen.count {
        command.spawn(create_food(gen.get(), world.get()));
        food_count += 1;
    }
}

fn create_food(gen: &FoodGenerator, world: &World) -> Food {
    let (width, height) = world.extent();

    let mut rand = Rand32::new();

    loop {
        let x = (width - 1) as f32 * rand.next_uniform() + 0.5; 
        let y = (height - 1) as f32 * rand.next_uniform() + 0.5; 

        if ! world.is_collide((x, y)) {
            let mut food = Food::new((x, y));
            food.value = gen.value;
            food.radius = gen.radius;
            food.kind = gen.kind;

            return food;
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Food {
    pos: Point,
    kind: FoodKind,
    value: f32,
    radius: f32,
    probability: f32,
}

impl Food {
    pub(super) fn new(pos: impl Into<Point>) -> Self {
        Self {
            pos: pos.into(),
            kind: FoodKind::Plain,
            value: f32::MAX,
            radius: 0.4,
            probability: 1.,
        }
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn kind(&self) -> FoodKind {
        self.kind
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    #[inline]
    pub fn is_pos(&self, pos: impl Into<Point>) -> bool {
        self.pos.dist(pos) < self.radius
    }

    ///
    /// Stochastic food eating
    /// 
    #[inline]
    pub fn eat_probability(&mut self) -> bool {
        if self.value >= 1. && Rand32::new().next_uniform() <= self.probability {
            self.value -= 1.;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FoodKind {
    None,
    Poor,
    Plain,
    Sweet,
    Bitter,
    Sick,
}

impl Default for FoodKind {
    fn default() -> Self {
        FoodKind::None
    }
}

#[derive(Clone)]
struct FoodGenerator {
    count: usize,
    radius: f32,
    value: f32,
    kind: FoodKind,
}

impl Default for FoodGenerator {
    fn default() -> Self {
        Self { 
            count: 0, 
            radius: 1.,
            value: f32::MAX,
            kind: FoodKind::Plain,
        }
    }
}

pub struct FoodPlugin {
    food: Vec<Food>,
    gen: FoodGenerator,
}

impl FoodPlugin {
    pub fn new() -> Self {
        Self {
            food: Vec::default(),
            gen: FoodGenerator::default(),
        }
    }

    pub fn food(&mut self, x: usize, y: usize) -> &mut Self {
        // assert!(x < self.width);
        // assert!(y < self.height);

        self.food.push(Food::new((x as f32 + 0.5, y as f32 + 0.5)));

        self
    }

    pub fn food_mut(&mut self, x: usize, y: usize) -> &mut Food {
        // assert!(x < self.width);
        // assert!(y < self.height);

        self.food.push(Food::new((x as f32 + 0.5, y as f32 + 0.5)));

        self.food.last_mut().unwrap()
    }

    pub fn kind(&mut self, kind: impl Into<FoodKind>) -> &mut Self {
        self.food.last_mut().unwrap().kind = kind.into();

        self
    }

    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.food.last_mut().unwrap().radius = r;

        self
    }

    pub fn probability(&mut self, p: f32) -> &mut Self {
        self.food.last_mut().unwrap().probability = p;

        self
    }

    pub fn value(&mut self, value: f32) -> &mut Self {
        self.food.last_mut().unwrap().value = value;

        self
    }

    pub fn gen_count(&mut self, count: usize) -> &mut Self {
        self.gen.count = count;

        self
    }

    pub fn gen_radius(&mut self, r: f32) -> &mut Self {
        self.gen.radius = r;

        self
    }

    pub fn gen_value(&mut self, value: impl Into<Ticks>) -> &mut Self {
        self.gen.value = value.into().ticks() as f32;

        self
    }

    pub fn gen_kind(&mut self, kind: impl Into<FoodKind>) -> &mut Self {
        self.gen.kind = kind.into();

        self
    }

    pub fn odor(&mut self, _odor: OdorKind) -> &mut Self {
        todo!();
        // self.food.last_mut().unwrap().odor(odor);

        //self
    }

    pub fn odor_r(&mut self, _r: usize, _odor: OdorKind) -> &mut Self {
        todo!();
        // self.food.last_mut().unwrap().odor_r(odor, r);

        //self
    }

    fn create_food(&self, app: &mut App) {
        let mut foods : Vec<Food> = self.food.clone();

        app.system(Startup, move |mut cmd: Commands| {
            for food in foods.drain(..) {
                cmd.spawn(food);
            }
        });
    }

}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_resource::<World>(), "FoodPlugin requires World");

        self.create_food(app);

        app.insert_resource(self.gen.clone());

        app.system(Tick, update_food);
    }
}
