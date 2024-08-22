use mind_ecs::AppTick;

use crate::util::{DecayValue, Seconds};

pub struct StriatumTimeout {
    ltd_buildup: DecayValue,
    ltd_decay: DecayValue,
}

impl StriatumTimeout {
    const BUILDUP : f32 = 25.;

    pub fn new() -> Self {
        Self {
            ltd_buildup: DecayValue::new(Seconds(Self::BUILDUP)),
            ltd_decay: DecayValue::new(Seconds(1.5 * Self::BUILDUP)),
        }
    }

    pub fn update(&mut self, tick: &AppTick) -> TimeoutState {
        self.ltd_buildup.update_ticks(tick.ticks());
        self.ltd_decay.update_ticks(tick.ticks());
        
        // avoid timeout (adenosine in striatum) with hysteresis
        let state = if self.ltd_decay.value() < 0.2 {
            TimeoutState::Active
        } else if self.ltd_decay.value() > 0.9 {
            TimeoutState::Timeout
        } else if self.ltd_buildup.value() > 0.05 {
            // hysteresis
            TimeoutState::Active
        } else {
            TimeoutState::Timeout
        };

        match state {
            TimeoutState::Active => {
                self.ltd_buildup.add(1.);
                self.ltd_decay.set_max(self.ltd_buildup.value());
            }
            TimeoutState::Timeout => {
                self.ltd_buildup.set(0.);
            }
        }

        state
    }
}

#[derive(Debug)]
pub enum TimeoutState {
    Active,
    Timeout
}
