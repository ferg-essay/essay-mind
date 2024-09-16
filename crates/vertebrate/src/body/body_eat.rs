use essay_ecs::{app::{App, Plugin}, core::{Query, Res, ResMut}};
use log::error;
use mind_ecs::Tick;

use crate::{body::BodyPlugin, util::{DecayValue, Point, Seconds, Ticks, TimeoutValue}, world::{Food, FoodKind}};

use super::Body;

pub struct BodyEat {
    is_sweet: DecayValue,
    is_umami: DecayValue,
    is_bitter: DecayValue,

    sated_cck: DecayValue,

    gut_sweet: DecayValue,
    gut_glucose: DecayValue,
    gut_sickness: DecayValue,

    is_eating: TimeoutValue<bool>,

    food_delay: DelayRing<FoodKind>,
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

    /// sated as measured CCK by stomach fullness
    #[inline]
    pub fn sated_cck(&self) -> f32 {
        self.sated_cck.active_value()
    }

    #[inline]
    pub fn sickness(&self) -> f32 {
        self.gut_sickness.active_value()
    }

    /// sweetness as measured in the gut
    #[inline]
    pub fn gut_sweet(&self) -> f32 {
        self.gut_sweet.active_value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.gut_glucose.active_value()
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

    ///
    /// Update the animal's eating and digestion
    /// 
    fn pre_update(&mut self) {
        self.is_sweet.update();
        self.is_umami.update();
        self.is_bitter.update();

        self.sated_cck.update();
        self.gut_sweet.update();
        self.gut_glucose.update();
        self.gut_sickness.update();

        self.is_eating.update();
    }

    fn update(&mut self, head_pos: Point, food: Query<&Food>) {
        self.pre_update();

        match self.food_delay.value() {
            FoodKind::None => {},
            FoodKind::Plain => {
                self.gut_glucose.add(1.);
            },
            FoodKind::Sweet => {
                self.gut_glucose.add(1.);
                self.gut_sweet.add(1.);
            },
            FoodKind::Bitter => {},
            FoodKind::Sick => {
                self.gut_sickness.set(1.);
            },
        }
    
        if self.is_eating() {
            if let Some(food) = food.iter().find(|f| f.is_pos(head_pos)) {
                self.food_delay.set(food.kind());
    
                match food.kind() {
                    FoodKind::None => {
                    }
                    FoodKind::Plain => {
                        self.sated_cck.add(1.);
                    }
                    FoodKind::Sweet => {
                        self.sated_cck.add(1.);
                        self.is_sweet.set(1.);
                    }
                    FoodKind::Bitter => {
                        self.is_bitter.set(1.);
                    }
                    FoodKind::Sick => {
                    }
                }
            } else {
                error!("Eating without food");
            }
        }
    
        self.food_delay.next();
    }
}

impl Default for BodyEat {
    fn default() -> Self {
        Self {
            is_sweet: DecayValue::new(Seconds(1.)),
            is_umami: DecayValue::new(Seconds(1.)),
            is_bitter: DecayValue::new(Seconds(1.)),

            sated_cck: DecayValue::new(Seconds(40.)).fill_time(Seconds(10.)),

            food_delay: DelayRing::new(Seconds(30.)),

            gut_glucose: DecayValue::new(Seconds(40.)).fill_time(Seconds(10.)),
            gut_sweet: DecayValue::new(Seconds(1.)),
            gut_sickness: DecayValue::new(Seconds(60.)),

            is_eating: TimeoutValue::default(),
        }
    }
}

fn body_eat_update(
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
    food: Query<&Food>,
) {
    body_eat.update(body.head_pos(), food);
}

struct DelayRing<V: Clone + Default> {
    vec: Vec<V>,
    i: usize,
}

impl<V: Clone + Default> DelayRing<V> {
    fn new(ticks: impl Into<Ticks>) -> Self {
        let mut vec = Vec::<V>::new();

        vec.resize(ticks.into().ticks().max(1), V::default());

        Self {
            vec,
            i: 0,
        }
    }

    fn value(&mut self) -> V {
        let next = self.vec[self.i].clone();
        self.vec[self.i] = V::default();

        next
    }

    fn set(&mut self, value: V) {
        self.vec[self.i] = value;
    }

    fn next(&mut self) {
        self.i = (self.i + 1) % self.vec.len();
    }
}

pub struct BodyEatPlugin;

impl Plugin for BodyEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "BodyEatPlugin requires BodyPlugin");

        app.init_resource::<BodyEat>();

        app.system(Tick, body_eat_update);
    }
}
