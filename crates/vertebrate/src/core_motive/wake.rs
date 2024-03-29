use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::util::{Seconds, Ticks};

use super::motive::{Motive, MotiveTrait, Motives};

pub struct WakeState {
    circadian: CircadianWake,
}

impl WakeState {
    fn update(&mut self) {
        self.circadian.update();

        
    }
}

pub struct CircadianWake {
    /// number of ticks in the wake phase
    wake_ticks: usize, 
    /// number of ticks in the sleep phase
    sleep_ticks: usize, 

    /// current tick in the cycle
    tick: usize, 

    /// normalized sleep/wake phase where 0.0 starts wake and 0.5 starts sleep
    phase: f32, 
}

impl CircadianWake {
    const WAKE_TIME: Seconds = Seconds(180.);
    const SLEEP_TIME: Seconds = Seconds(30.);

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

    pub fn get_state(&self) -> CircadianState {
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

impl Default for CircadianWake {
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
    mut circadian: ResMut<CircadianWake>,
    mut wake: ResMut<Motive<Wake>>
) {
    circadian.update();

    if circadian.get_state() == CircadianState::Wake {
        wake.add(1.);
    }
}

pub struct Wake;
impl MotiveTrait for Wake {}

pub struct CoreWakePlugin {
    wake: Ticks,
    sleep: Ticks,
}

impl CoreWakePlugin {
    pub fn new() -> Self {
        Self {
            wake: CircadianWake::WAKE_TIME.into(),
            sleep: CircadianWake::SLEEP_TIME.into(),
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
        Motives::insert::<Wake>(app, Seconds(1.));

        let circadian = CircadianWake::new(self.wake, self.sleep);

        app.insert_resource(circadian);

        app.system(Tick, wake_update);
    }
}
