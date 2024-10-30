use essay_ecs::{app::{App, Plugin, Startup}, core::{entity::EntityId, Commands, Component, Query}};
use log::warn;
use mind_ecs::Tick;
use util::random::Rand32;

use crate::{util::Point, world::{Wall, World}};

use super::OdorKind;

pub(super) fn update_food(
    query: Query<(EntityId, &Food)>,
    mut command: Commands,
) {
    let mut food_count = 0;

    for (id, food) in query.iter() {
        if food.value <= 0. {
            command.entity(id).despawn();
            food_count += 1;
        }
    }

    if food_count > 0 {
        warn!("Remove food {:?}", food_count);
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

    fn set_kind(&mut self, kind: FoodKind) -> &mut Self {
        self.kind = kind;

        self
    }

    fn set_probability(&mut self, p: f32) -> &mut Self {
        self.probability = p;

        self
    }

    fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;

        self
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
pub struct FoodPlugin {
    food: Vec<Food>,
}

impl FoodPlugin {
    pub fn new() -> Self {
        Self {
            food: Vec::default(),
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
        self.food.last_mut().unwrap().set_kind(kind.into());

        self
    }

    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.food.last_mut().unwrap().set_radius(r);

        self
    }

    pub fn probability(&mut self, p: f32) -> &mut Self {
        self.food.last_mut().unwrap().set_probability(p);

        self
    }

    pub fn odor(&mut self, odor: OdorKind) -> &mut Self {
        todo!();
        // self.food.last_mut().unwrap().odor(odor);

        self
    }

    pub fn odor_r(&mut self, r: usize, odor: OdorKind) -> &mut Self {
        todo!();
        // self.food.last_mut().unwrap().odor_r(odor, r);

        self
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

        app.system(Tick, update_food);
    }
}
