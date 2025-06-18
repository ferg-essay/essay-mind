use essay_ecs::{
    app::{App, Plugin},
    core::{Query, Res, ResMut},
};
use mind_ecs::Tick;

use crate::{
    body::BodyPlugin,
    util::{DecayValue, Point, Seconds, Ticks, TimeoutValue},
    world::{Food, FoodKind, World},
};

use super::Body;

fn body_eat_update(
    mut body_eat: ResMut<BodyEat>, 
    body: Res<Body>, 
    world: Res<World>,
    food: Query<&mut Food>
) {
    body_eat.update(body.head_pos(), world.get(), food);
}

// BodyEat is physical simulation of eating and sensors
// It includes a gut delay for sickness, glucose and gut sensors
//
// TODO: note missing N5 including capiscin
// TODO: missing salt

// [Ahn et al 2020] Gut(fat) -> N10 -> R.nts -> R.pb -> Snc.da
// [Essner et al 2017] Amylin (pancreas), CCK (small intestine),
//   LiCl (gastric discomfort), LPS (bacterial inflammation)
// [Han W et al 2018] Gut -> N10 -> R.nts -> R.pb.dl -> Snc
// [Kraus et al 2021] Ascidian filter feeders regularly exposed to microbes,
//   neuroimmune interactions. TRP to CGRP immune avoidance.
// [Li J et al 2019] R.pb.el converge bitter, capsaicin (N5 pain), heat
// [Palmiter 2018] N10 for CCK, GLP-1, LPS / LiCl
// [Rosen et al 2010] Specific water response in R.nts and R.pb
// [Torruella-Suárez et al 2020] Sa.nts, R.pb activated by ethanol
// [Weiss et al 2014] R.pb some taste long-latency 1.5s possibly N10 gut.
//    water as independent taste

pub struct BodyEat {
    taste_food: DecayValue,

    taste_sweet: DecayValue,
    is_umami: DecayValue,
    taste_bitter: DecayValue,

    sated_cck: DecayValue,
    sated_stretch: DecayValue,

    gut_food: DecayValue,
    gut_sweet: DecayValue,
    gut_fat: DecayValue,
    gut_glucose: DecayValue,
    gut_sickness: DecayValue,

    is_eating: TimeoutValue<bool>,

    gut_delay: DelayRing<FoodKind>,
}

impl BodyEat {
    #[inline]
    pub fn taste_food(&self) -> f32 {
        self.taste_food.active_value()
    }

    #[inline]
    pub fn taste_sweet(&self) -> f32 {
        self.taste_sweet.active_value()
    }

    #[inline]
    pub fn taste_umami(&self) -> f32 {
        self.is_umami.active_value()
    }

    #[inline]
    pub fn taste_bitter(&self) -> f32 {
        self.taste_bitter.active_value()
    }

    /// sated as measured by gut stretch
    #[inline]
    pub fn sated_stretch(&self) -> f32 {
        self.sated_stretch.active_value()
    }

    /// sated as measured by gut stretch
    #[inline]
    pub fn is_sated_stretch(&self) -> bool {
        self.sated_stretch.is_active()
    }

    /// sated as measured CCK by stomach fullness
    #[inline]
    pub fn sated_cck(&self) -> f32 {
        self.sated_cck.active_value()
    }

    /// sated as measured by leptin
    #[inline]
    pub fn sated_leptin(&self) -> f32 {
        // TODO: currently use single satiety measure. Multiple names
        // to match actual receptors.
        self.sated_cck()
    }

    #[inline]
    pub fn sickness(&self) -> f32 {
        self.gut_sickness.active_value()
    }

    /// food as measured in the gut
    #[inline]
    pub fn gut_food(&self) -> f32 {
        self.gut_food.active_value()
    }

    /// sweetness as measured in the gut
    #[inline]
    pub fn gut_sweet(&self) -> f32 {
        self.gut_sweet.active_value()
    }

    /// fat as measured in the gut
    #[inline]
    pub fn gut_fat(&self) -> f32 {
        self.gut_fat.active_value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.gut_glucose.active_value()
    }

    #[inline]
    pub fn glucose_low(&self) -> bool {
        self.gut_glucose.value() < 0.1
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

    fn update(
        &mut self, 
        head_pos: Point, 
        world: &World,
        food: Query<&mut Food>
    ) {
        self.pre_update();

        self.update_gut();

        if self.is_eating() {
            if let Some(food) = food.iter().find(|f| f.is_pos(head_pos)) {
                self.update_food(food);
            } else if let Some(food) = world.base_food() {
                let mut food = food.clone();
                self.update_food(&mut food);
            }
        }

        self.gut_delay.next();
    }

    ///
    /// Update the animal's eating and digestion
    ///
    fn pre_update(&mut self) {
        self.taste_food.update();
        self.taste_sweet.update();
        self.is_umami.update();
        self.taste_bitter.update();

        self.sated_stretch.update();
        self.sated_cck.update();

        self.gut_food.update();
        self.gut_sweet.update();
        self.gut_fat.update();
        self.gut_glucose.update();
        self.gut_sickness.update();

        self.is_eating.update();
    }

    fn update_gut(&mut self) {
        // update gut values
        match self.gut_delay.take() {
            FoodKind::None => {}
            FoodKind::Poor => {}
            FoodKind::Plain => {
                self.gut_food.set_max(1.);
                self.gut_glucose.set_max(1.);
            }
            FoodKind::Sweet => {
                self.gut_food.add(1.);
                self.gut_glucose.add(1.);
                self.gut_sweet.add(1.);
            }
            FoodKind::Bitter => {}
            FoodKind::Sick => {
                self.gut_sickness.set(1.);
            }
        }
    }

    fn update_food(&mut self, food: &mut Food) {
        if food.eat_probability() {
            self.gut_delay.set(food.kind());

            match food.kind() {
                FoodKind::None => {}
                FoodKind::Poor => {
                    self.sated_stretch.add(1.);
                }
                FoodKind::Plain => {
                    self.sated_stretch.add(1.);
                    self.sated_cck.add(1.);
                    self.taste_food.set(1.);
                }
                FoodKind::Sweet => {
                    self.sated_stretch.add(1.);
                    self.sated_cck.add(1.);
                    self.taste_sweet.set(1.);
                    self.taste_food.set(1.);
                }
                FoodKind::Bitter => {
                    self.taste_bitter.set(1.);
                }
                FoodKind::Sick => {}
            }
        }
    }
}

impl Default for BodyEat {
    fn default() -> Self {
        Self {
            taste_food: DecayValue::new(Seconds(1.)),
            taste_sweet: DecayValue::new(Seconds(1.)),
            is_umami: DecayValue::new(Seconds(1.)),
            taste_bitter: DecayValue::new(Seconds(1.)),

            sated_stretch: DecayValue::new(Seconds(40.)).fill_time(Seconds(20.)),
            sated_cck: DecayValue::new(Seconds(40.)).fill_time(Seconds(20.)),

            gut_delay: DelayRing::new(Seconds(30.)),

            gut_food: DecayValue::new(Seconds(40.)).fill_time(Seconds(20.)),
            gut_glucose: DecayValue::new(Seconds(40.)).fill_time(Seconds(20.)),
            gut_sweet: DecayValue::new(Seconds(1.)),
            gut_fat: DecayValue::new(Seconds(1.)),
            gut_sickness: DecayValue::new(Seconds(60.)),

            is_eating: TimeoutValue::default(),
        }
    }
}

struct DelayRing<V: Clone + Default> {
    vec: Vec<V>,
    i: usize,
}

impl<V: Clone + Default> DelayRing<V> {
    fn new(ticks: impl Into<Ticks>) -> Self {
        let mut vec = Vec::<V>::new();

        vec.resize(ticks.into().ticks().max(1), V::default());

        Self { vec, i: 0 }
    }

    fn take(&mut self) -> V {
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

pub struct BodyEatPlugin {
    gut_delay: Ticks,
}

impl BodyEatPlugin {
    pub fn new() -> Self {
        Self {
            gut_delay: Seconds(15.).into(),
        }
    }

    pub fn gut_delay(&mut self, value: impl Into<Ticks>) -> &mut Self {
        self.gut_delay = value.into();

        self
    }
}

impl Plugin for BodyEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.contains_plugin::<BodyPlugin>(),
            "BodyEatPlugin requires BodyPlugin"
        );

        let mut body_eat = BodyEat::default();
        body_eat.gut_delay = DelayRing::new(self.gut_delay);

        app.insert_resource(body_eat);

        app.system(Tick, body_eat_update);
    }
}
