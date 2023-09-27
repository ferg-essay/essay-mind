use essay_ecs::core::{store::FromStore, Store};

pub struct Habenula {
    cost: f32,
}

impl Habenula {
    const COST : f32 = 0.005;
    const DECAY : f32 = Habenula::COST * 0.5;
    const THRESHOLD : f32 = 0.5;

    pub fn persist(&mut self) -> bool {
        self.cost = (self.cost + Habenula::COST).min(1.);

        if true {
            return true;
        }
        
        self.cost < Habenula::THRESHOLD
    }

    pub fn decay(&mut self) {
        self.cost = (self.cost - Habenula::DECAY).max(0.);
    }
}

impl FromStore for Habenula {
    fn init(_store: &mut Store) -> Self {
        Habenula {
            cost: 0.
        }
    }
}
