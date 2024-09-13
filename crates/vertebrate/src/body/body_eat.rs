use essay_ecs::{app::{App, Plugin}, core::{Query, Res, ResMut}};
use log::error;
use mind_ecs::Tick;

use crate::{body::BodyPlugin, util::{DecayValue, Seconds, TimeoutValue}, world::{Food, FoodKind}};

use super::Body;

pub struct BodyEat {
    is_sweet: DecayValue,
    is_umami: DecayValue,
    is_bitter: DecayValue,
    is_sickness: DecayValue,

    sated_cck: DecayValue,
    glucose: DecayValue,

    is_eating: TimeoutValue<bool>,
}

impl BodyEat {
    #[inline]
    pub fn sweet(&self) -> f32 {
        self.is_sweet.active_value()
    }

    #[inline]
    pub fn umami(&self) -> f32 {
        self.is_umami.active_value()
    }

    #[inline]
    pub fn bitter(&self) -> f32 {
        self.is_bitter.active_value()
    }

    #[inline]
    pub fn sickness(&self) -> f32 {
        self.is_sickness.active_value()
    }

    #[inline]
    pub fn sated_cck(&self) -> f32 {
        self.sated_cck.active_value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.glucose.active_value()
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
        self.is_umami.update();
        self.is_bitter.update();
        self.is_sickness.update();

        self.sated_cck.update();
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
            is_umami: DecayValue::new(Seconds(1.)),
            is_bitter: DecayValue::new(Seconds(1.)),
            is_sickness: DecayValue::new(Seconds(60.)),

            sated_cck: DecayValue::new(Seconds(40.)).fill_time(Seconds(10.)),
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
        if let Some(food) = food.iter().find(|f| f.is_pos(body.head_pos())) {
            match food.kind() {
                FoodKind::Plain => {
                    body_eat.glucose.add(1.);
                    body_eat.sated_cck.add(1.);
                },
                FoodKind::Sweet => {
                    body_eat.glucose.add(1.);
                    body_eat.sated_cck.add(1.);
                    body_eat.is_sweet.set(1.);
                },
                FoodKind::Bitter => {
                    body_eat.is_bitter.set(1.);
                },
                FoodKind::Sick => {
                    body_eat.is_sickness.set(1.);
                },
            }
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
