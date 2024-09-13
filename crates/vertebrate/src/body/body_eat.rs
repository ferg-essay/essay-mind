use essay_ecs::{app::{App, Plugin}, core::{Query, Res, ResMut}};
use log::error;
use mind_ecs::Tick;

use crate::{body::BodyPlugin, util::{DecayValue, Seconds, TimeoutValue}, world::Food};

use super::Body;

pub struct BodyEat {
    is_sweet: DecayValue,
    _is_umami: f32,
    _is_bitter: f32,
    _is_sour: f32,

    glucose: DecayValue,

    is_eating: TimeoutValue<bool>,
}

impl BodyEat {
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
        self.is_eating.value_or(false)
    }

    #[inline]
    pub fn eat(&mut self) {
        self.is_eating.set(true);
    }

    #[inline]
    pub fn stop_eat(&mut self) {
        self.is_eating.set(false);
    }

    pub fn p_food(&self) -> f32 {
        // self.tick_food as f32 / self.ticks.max(1) as f32
        0.
    }

    ///
    /// Update the animal's eating and digestion
    /// 
    fn pre_update(&mut self) {
        self.is_sweet.update();

        self.glucose.update();

        self.is_eating.update();

        /*
        let is_food = world.is_food(body.head_pos());

        if self.is_eating() && is_food {
            body.eat();
            self.glucose.add(1.);
            self.is_sweet.add(1.);
        }
        */
    }
}

impl Default for BodyEat {
    fn default() -> Self {
        Self {
            is_sweet: DecayValue::new(Seconds(1.)),
            _is_umami: 0.,
            _is_bitter: 0.,
            _is_sour: 0.,

            glucose: DecayValue::new(Seconds(40.)).fill_time(Seconds(10.)),

            is_eating: TimeoutValue::default(),
        }
    }
}

fn body_eat_update(
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
    food: Query<&Food>,
) {
    body_eat.pre_update();

    if body_eat.is_eating() {
        if let Some(_food) = food.iter().find(|f| f.is_pos(body.head_pos())) {
            body_eat.glucose.add(1.);
            body_eat.is_sweet.set(1.);
        } else {
            error!("Eating without food");
        }
    }

    /*
    let is_food = world.is_food(body.head_pos());

    if self.is_eating() && is_food {
        body.eat();
        self.glucose.add(1.);
        self.is_sweet.add(1.);
    }
    */
}

pub struct BodyEatPlugin;

impl Plugin for BodyEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "BodyEatPlugin requires BodyPlugin");

        app.init_resource::<BodyEat>();

        app.system(Tick, body_eat_update);
    }
}
