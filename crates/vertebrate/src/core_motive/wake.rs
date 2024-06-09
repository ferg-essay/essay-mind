use std::sync::atomic::{AtomicBool, Ordering};

use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::util::{Seconds, Ticks};

use super::motive::{Motive, MotiveTrait, Motives};

pub struct Wake {
    circadian: Circadian,

    state: CircadianState,

    active_wake: AtomicBool,
}

impl Wake {
    fn new(circadian: Circadian) -> Self {
        Self {
            circadian,
            state: CircadianState::Wake,
            active_wake: AtomicBool::new(false),
        }
    }

    ///
    /// Ongoing actions or important wake alarms override the circadian
    /// sleep/wake
    /// 
    pub fn wake(&self) {
        self.active_wake.store(true, Ordering::Relaxed);
    }

    fn get_state(&self) -> CircadianState {
        self.state
    }

    fn update(&mut self) {
        self.circadian.update();

        self.state = self.circadian.get_state();

        // ongoing action forces a wake state
        if let Ok(_) = self.active_wake.compare_exchange(
            true, 
            false, 
            Ordering::Relaxed, 
            Ordering::Relaxed
        ) {
            self.state = CircadianState::Wake;
        }
    }
}

pub struct Circadian {
    /// number of ticks in the wake phase
    wake_ticks: usize, 
    /// number of ticks in the sleep phase
    sleep_ticks: usize, 

    /// current tick in the cycle
    tick: usize, 

    /// normalized sleep/wake phase where 0.0 starts wake and 0.5 starts sleep
    phase: f32, 
}

impl Circadian {
    const WAKE_TIME: Seconds = Seconds(180.);
    const SLEEP_TIME: Seconds = Seconds(30.);

    const WAKE_DECAY: Seconds = Seconds(1.);
    const SLEEP_DECAY: Seconds = Seconds(1.);

    fn new(wake: impl Into<Ticks>, sleep: impl Into<Ticks>) -> Self {
        let wake_ticks = wake.into().ticks();
        assert!(wake_ticks > 0);
        let sleep_ticks = sleep.into().ticks();
        assert!(sleep_ticks > 0);

        Self {
            wake_ticks,
            sleep_ticks,

            tick: 0,
            phase: 0.,
        }
    }

    fn get_state(&self) -> CircadianState {
        if self.phase < 0.5 {
            CircadianState::Wake
        } else {
            CircadianState::Sleep
        }
    }

    fn update(&mut self) {
        self.tick = (self.tick + 1) % (self.wake_ticks + self.sleep_ticks);
    
        if self.tick < self.wake_ticks {
            self.phase = 0.5 * self.tick as f32 / self.wake_ticks as f32;
        } else {
            self.phase = 0.5 + 0.5 * (self.tick - self.wake_ticks) as f32
                / self.sleep_ticks as f32;
        }
    }
}

impl Default for Circadian {
    fn default() -> Self {
        Self::new(Self::WAKE_TIME, Self::SLEEP_TIME)
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CircadianState {
    Sleep,
    Wake
}

fn wake_update(
    mut circadian: ResMut<Wake>,
    mut wake: ResMut<Motive<Wake>>,
    mut sleep: ResMut<Motive<Sleep>>
) {
    circadian.update();

    match circadian.get_state() {
        CircadianState::Sleep => {
            if ! wake.is_active() {
                sleep.set_max(1.);
            }
        },
        CircadianState::Wake => wake.set_max(1.),
    }
}

// pub struct Wake;
impl MotiveTrait for Wake {}

pub struct Sleep;
impl MotiveTrait for Sleep {}

pub struct CoreWakePlugin {
    wake: Ticks,
    sleep: Ticks,
}

impl CoreWakePlugin {
    pub fn new() -> Self {
        Self {
            wake: Circadian::WAKE_TIME.into(),
            sleep: Circadian::SLEEP_TIME.into(),
        }
    }

    pub fn wake(mut self, wake: impl Into<Ticks>) -> Self {
        self.wake = wake.into();

        self
    }

    pub fn sleep(mut self, sleep: impl Into<Ticks>) -> Self {
        self.sleep = sleep.into();

        self
    }
}

impl Plugin for CoreWakePlugin {
    fn build(&self, app: &mut App) {
        Motives::insert::<Wake>(app, Circadian::WAKE_DECAY);
        Motives::insert::<Sleep>(app, Circadian::SLEEP_DECAY);

        let circadian = Circadian::new(self.wake, self.sleep);
        let wake = Wake::new(circadian);

        app.insert_resource(wake);

        app.system(Tick, wake_update);
    }
}
