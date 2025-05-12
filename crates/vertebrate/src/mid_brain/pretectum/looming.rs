use essay_ecs::app::{App, Plugin};

use super::looming_zebrafish_mtl::LoomingZebrafishMtl;

// Pretectum looming currently same as tectum looming

pub struct PretectumLoomingPlugin {
    strategy: Box<dyn LoomingStrategy>,
    is_enable: bool,
}

impl PretectumLoomingPlugin {
    pub fn new() -> Self {
        Self {
            strategy: Box::new(LoomingZebrafishMtl),
            is_enable: true,
        }
    }

    pub fn strategy(mut self, strategy: impl LoomingStrategy + 'static) -> Self {
        self.strategy = Box::new(strategy);

        self
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }
}

impl Plugin for PretectumLoomingPlugin {
    fn build(&self, app: &mut App) {
        if self.is_enable {
            self.strategy.build(app);
        }
    }
}

pub trait LoomingStrategy {
    fn build(&self, app: &mut App);
}
