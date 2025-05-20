use crate::util::{DecayValue, HalfLife, Side};

pub struct PptSide {
    side: Side,

    active: DecayValue,   
}

impl PptSide {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            active: DecayValue::default(),
        }
    }

    pub fn set_timeout(&mut self, half_life: impl Into<HalfLife>) {
        self.active.set_half_life(half_life);
    }
}
