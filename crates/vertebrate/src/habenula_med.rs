use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, core::{store::FromStore, Store, Local}};
use mind_ecs::Tick;

use crate::{
    mid_locomotor::MidLocomotorPlugin, 
    olfactory::{OlfactoryPlugin, Olfactory}, 
    tectum::TectumLocomotionStn, action::Turn, util::DecayValue, phototaxis::GoalVector
};

pub struct Habenula {
    give_up: DecayValue,

    toward: Vec::<HabenulaItem>,
    away: Vec::<HabenulaItem>,
}

impl Habenula {
    pub fn new(half_life: usize) -> Self {
        Self {
            give_up: DecayValue::new(half_life),

            toward: Vec::new(),
            away: Vec::new(),
        }
    }

    pub fn value(&self) -> f32 {
        self.give_up.value()
    }

    pub fn excite(&mut self, value: f32) -> &mut Self {
        self.give_up.add(value);

        self
    }

    pub fn inhibit(&mut self, value: f32) -> &mut Self {
        self.give_up.subtract(value);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.give_up.update();

        self
    }
}

struct HabenulaItem {
    value: f32,

    average: DecayValue,
    goal_vector: GoalVector,
}
impl HabenulaItem {
    fn update(&mut self, value: f32) {
        self.average.update();
        self.average.add(value);
    }
}

pub struct HabenulaSetter {
    avoid: AvoidType,
    index: usize,
}

impl HabenulaSetter {
    pub fn update(&self, value: f32, mut hb: impl AsMut<Habenula>) {
        let hb = hb.as_mut();

        match self.avoid {
            AvoidType::TOWARD => { hb.toward[self.index].update(value) },
            AvoidType::AWAY => { hb.away[self.index].update(value) },
        }
    }
}

enum AvoidType {
    TOWARD,
    AWAY,
}

pub struct HabenulaMedPlugin;

impl Plugin for HabenulaMedPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "HabenulaMedPlugin requires MidLocomotorPlugin");

        // app.system(Tick, update_habenula_med);
    }
}
