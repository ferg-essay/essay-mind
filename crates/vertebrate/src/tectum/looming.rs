use essay_ecs::app::{App, Plugin};

use super::looming_zebrafish_mtl::LoomingZebrafishMtl;

pub trait LoomingStrategy {
    fn build(&self, app: &mut App);
}

pub struct TectumLoomingPlugin {
    strategy: Box<dyn LoomingStrategy>,
}

impl TectumLoomingPlugin {
    pub fn new() -> Self {
        Self {
            strategy: Box::new(LoomingZebrafishMtl),
        }
    }

    pub fn strategy(mut self, strategy: impl LoomingStrategy + 'static) -> Self {
        self.strategy = Box::new(strategy);

        self
    }
}

impl Plugin for TectumLoomingPlugin {
    fn build(&self, app: &mut App) {
        self.strategy.build(app);
    }
}
