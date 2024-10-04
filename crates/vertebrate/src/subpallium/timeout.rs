use mind_ecs::AppTick;

use crate::util::{Seconds, Ticks};

pub struct StriatumTimeout {
    ltd_factor: f32,
    ltd_decay: f32,
    active_gap: Ticks,

    ltd: f32,

    last_active: u64,
    last_time: u64,
}

impl StriatumTimeout {
    const BUILDUP : f32 = 25.;
    const DECAY : f32 = 1.5 * Self::BUILDUP;

    pub fn new() -> Self {
        let ltd : Ticks = Seconds(Self::BUILDUP).into();
        let decay : Ticks = Seconds(Self::DECAY).into();

        Self {
            ltd_factor: 1. / ltd.ticks().max(1) as f32,
            ltd_decay: 1. / decay.ticks().max(1) as f32,
            active_gap: Ticks(3),

            ltd: 0.,
            last_active: 0,
            last_time: 0,
        }
    }

    pub fn decay(mut self, time: impl Into<Ticks>) -> Self {
        self.ltd_decay = 1. / time.into().ticks().max(1) as f32;

        self
    }

    pub fn active(&mut self, tick: &AppTick) -> StriatumValue {
        let now = tick.ticks();
        let last_time = self.last_time;
        let delta = now - last_time;
        self.last_time = now;
        let last_active = self.last_active;

        if (now - last_active) < self.active_gap.ticks() as u64 {
            // continuation of active
            self.ltd += self.ltd_factor * delta as f32;

            // active not yet timed out
            if self.ltd <= 1. {
                self.last_active = now;

                StriatumValue::Active
            } else {
                StriatumValue::Avoid
            }
        } else {
            // attempted new active
            self.ltd = (self.ltd - self.ltd_decay * delta as f32).max(0.);

            if self.ltd <= 0. {
                self.last_active = now;

                StriatumValue::Active
            } else {
                StriatumValue::None
            }
        }
    }

    pub fn is_valid(&self, tick: &AppTick) -> bool {
        let delta = (tick.ticks() - self.last_time) as f32;

        delta < self.ltd_decay.recip()
    }
}

#[derive(Debug)]
pub enum StriatumValue {
    None,
    Active,
    Avoid,
}
