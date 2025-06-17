use mind_ecs::AppTick;

use crate::util::{Seconds, Ticks, TimeoutValue};

pub struct StriatumTimeout {
    ltd_rise: f32,
    ltd_decay: f32,

    // hysteresis
    threshold_high: f32,
    threshold_low: f32,

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
            ltd_rise: 1. / ltd.ticks().max(1) as f32,
            ltd_decay: 1. / decay.ticks().max(1) as f32,
            threshold_high: 0.9,
            threshold_low: 0.1,
            active_gap: Ticks(3),

            ltd: 0.,
            last_active: 0,
            last_time: 0,
        }
    }

    pub fn ltd(mut self, time: impl Into<Ticks>) -> Self {
        let time: Ticks = time.into();
        self.ltd_rise = 1. / time.ticks().max(1) as f32;

        self
    }

    pub fn decay(mut self, time: impl Into<Ticks>) -> Self {
        let time: Ticks = time.into();
        self.ltd_decay = 1. / time.ticks().max(1) as f32;

        self
    }

    pub fn active(&mut self, tick: &AppTick) -> StriatumValue {
        let now = tick.ticks();

        let last_time = self.last_time;
        self.last_time = now;

        let delta = now - last_time;

        let last_active = self.last_active;
        self.last_active = now;

        let is_active = (now - last_active) < self.active_gap.ticks() as u64;

        if is_active {
            // continuation of active
            self.ltd += self.ltd_rise * delta as f32;

            // active not yet timed out
            if self.ltd <= self.threshold_high {
                StriatumValue::Active
            } else {
                StriatumValue::Avoid
            }
        } else {
            // attempted new active
            self.ltd = (self.ltd - self.ltd_decay * delta as f32).max(0.);

            if self.ltd <= self.threshold_low {
                StriatumValue::Active
            } else {
                StriatumValue::None
            }
        }
    }

    pub fn is_active(&mut self, tick: &AppTick) -> bool {
        self.active(tick) == StriatumValue::Active
    } 

    pub fn is_valid(&self, tick: &AppTick) -> bool {
        let delta = (tick.ticks() - self.last_time) as f32;

        delta < self.ltd_decay.recip()
    }
}

#[derive(Debug, PartialEq)]
pub enum StriatumValue {
    None,
    Active,
    Avoid,
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct StriatumId(usize);

pub struct StriatumExclusive {
    active: TimeoutValue<StriatumId>,

    last_id: usize,
}

impl StriatumExclusive {
    pub fn alloc_id(&mut self) -> StriatumId {
        let id = self.last_id + 1;
        self.last_id = id;

        StriatumId(id)
    }

    pub fn active_id(&mut self, tick: &AppTick) -> Option<StriatumId> {
        self.active.update_ticks(tick.ticks());

        self.active.value().clone()
    }

    pub fn is_active(&mut self, id: StriatumId, tick: &AppTick) -> bool {
        self.active.update_ticks(tick.ticks());

        self.active.value().map_or(false, |active_id| active_id == id)
    }

    pub fn is_idle(&self) -> bool {
        self.active.value().is_none()
    }

    pub fn is_update(&mut self, id: StriatumId, tick: &AppTick) -> bool {
        self.active.update_ticks(tick.ticks());

        if let Some(active_id) = self.active.value() {
            *active_id == id
        } else {
            ! self.active.is_active()
        }
    }

    pub fn update_id(&mut self, id: StriatumId, tick: &AppTick) {
        self.active.update_ticks(tick.ticks());

        self.active.set(id);
    }

    pub fn update(&mut self, tick: &AppTick) {
        self.active.update_ticks(tick.ticks());
    }

    pub fn init(&mut self, id: StriatumId, tick: &AppTick) {
        self.active.update_ticks(tick.ticks());

        if ! self.active.is_active() {
            self.active.set(id);
        }
    }
}

impl Default for StriatumExclusive {
    fn default() -> Self {
        Self {
            active: TimeoutValue::new(Seconds(1.)),
            last_id: 0,
        }

    }
}