use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::BodyPlugin, util::{DecayValue, Point}, world::World};

use super::Body;

pub struct BodyEat {
    is_sensor_food: bool,

    is_sweet: DecayValue,
    _is_umami: f32,
    _is_bitter: f32,
    _is_sour: f32,

    blood_sugar: DecayValue,

    is_eating: DecayValue,
}

impl BodyEat {
    pub fn is_sensor_food(&self) -> bool {
        self.is_sensor_food
    }

    pub fn set_sensor_food(&mut self, is_food: bool) {
        self.is_sensor_food = is_food;
    }

    #[inline]
    pub fn sweet(&self) -> f32 {
        self.is_sweet.value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.blood_sugar.value()
    }

    #[inline]
    pub fn is_eating(&self) -> bool {
        self.is_eating.value() > 0.25
    }

    pub fn eat(&mut self) {
        self.is_eating.add(1.);
    }

    pub fn stop_eat(&mut self) {
        self.is_eating.set(0.);
    }

    ///
    /// Update the animal's eating and digestion
    /// 
    fn update(&mut self, world: &World, body: &Body) {
        self.is_sweet.update();

        self.blood_sugar.update();

        self.is_eating.update();

        let is_food = world.is_food(body.pos_head());
        self.set_sensor_food(is_food);

        if self.is_eating() && self.is_sensor_food() {
            self.blood_sugar.add(1.);
            self.is_sweet.add(1.);
        }
    }
}

impl Default for BodyEat {
    fn default() -> Self {
        Self {
            is_sensor_food: false,

            is_sweet: DecayValue::new(10),
            _is_umami: 0.,
            _is_bitter: 0.,
            _is_sour: 0.,

            blood_sugar: DecayValue::new(200).fill_time(20),

            is_eating: DecayValue::new(2),
        }
    }
}

fn body_eat_update(
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
    world: Res<World>,
) {
    body_eat.update(world.get(), body.get());
}

pub struct BodyEatPlugin;

impl Plugin for BodyEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "BodyEatPlugin requires BodyPlugin");

        app.init_resource::<BodyEat>();

        app.system(Tick, body_eat_update);
    }
}
