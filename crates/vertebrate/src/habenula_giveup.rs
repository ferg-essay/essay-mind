use essay_ecs::prelude::{Plugin, App};

use crate::{
    taxis::mid_locomotor::MidLocomotorPlugin, 
    util::DecayValue, 
};

pub struct HabenulaGiveUp {
    give_up: DecayValue,
}

impl HabenulaGiveUp {
    pub fn new(half_life: usize) -> Self {
        Self {
            give_up: DecayValue::new(half_life),
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

pub struct HabenulaMedPlugin;

impl Plugin for HabenulaMedPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "HabenulaMedPlugin requires MidLocomotorPlugin");

        // app.system(Tick, update_habenula_med);
    }
}
