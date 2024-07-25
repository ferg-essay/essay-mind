use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::BodyPlugin, util::{DecayValue, Seconds}, world::World};

use super::Body;

pub struct BodyEat {
    is_food_zone: bool,

    is_sweet: DecayValue,
    _is_umami: f32,
    _is_bitter: f32,
    _is_sour: f32,

    glucose: DecayValue,

    is_eating: DecayValue,
}

impl BodyEat {
    #[inline]
    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone
    }

    #[inline]
    pub fn set_food_zone(&mut self, is_food: bool) {
        self.is_food_zone = is_food;
    }

    #[inline]
    pub fn sweet(&self) -> f32 {
        self.is_sweet.value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.glucose.value()
    }

    #[inline]
    pub fn is_eating(&self) -> bool {
        self.is_eating.value() > 0.25
    }

    #[inline]
    pub fn eat(&mut self) {
        self.is_eating.set(1.);
    }

    #[inline]
    pub fn stop_eat(&mut self) {
        self.is_eating.set(0.);
    }

    pub fn p_food(&self) -> f32 {
        // self.tick_food as f32 / self.ticks.max(1) as f32
        0.
    }

    ///
    /// Update the animal's eating and digestion
    /// 
    fn update(&mut self, world: &World, body: &mut Body) {
        self.is_sweet.update();

        self.glucose.update();

        self.is_eating.update();

        let is_food = world.is_food(body.head_pos());
        self.set_food_zone(is_food);

        if self.is_eating() && self.is_food_zone() {
            body.eat();
            self.glucose.add(1.);
            self.is_sweet.add(1.);
        }
    }
}

impl Default for BodyEat {
    fn default() -> Self {
        Self {
            is_food_zone: false,

            is_sweet: DecayValue::new(Seconds(1.)),
            _is_umami: 0.,
            _is_bitter: 0.,
            _is_sour: 0.,

            glucose: DecayValue::new(Seconds(20.)).fill_time(Seconds(2.)),

            is_eating: DecayValue::new(Seconds(0.2)),
        }
    }
}

fn body_eat_update(
    mut body_eat: ResMut<BodyEat>,
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    body_eat.update(world.get(), body.get_mut());
}

pub struct BodyEatPlugin;

impl Plugin for BodyEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "BodyEatPlugin requires BodyPlugin");

        app.init_resource::<BodyEat>();

        app.system(Tick, body_eat_update);
    }
}
