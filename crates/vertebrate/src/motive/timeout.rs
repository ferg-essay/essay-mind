use essay_ecs::prelude::{Plugin, App};

use crate::util::{DecayValue, HalfLife};

pub struct _Timeout {
    persist: DecayValue,
}

impl _Timeout {
    pub fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            persist: DecayValue::new(half_life),
        }
    }

    pub fn value(&self) -> f32 {
        self.persist.value()
    }

    pub fn excite(&mut self, value: f32) -> &mut Self {
        self.persist.add(value);

        self
    }

    pub fn inhibit(&mut self, value: f32) -> &mut Self {
        self.persist.subtract(value);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.persist.update();

        self
    }
}

pub struct CorePersistPlugin;

impl Plugin for CorePersistPlugin {
    fn build(&self, _app: &mut App) {
        // app.system(Tick, update_habenula_med);
    }
}
