use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::util::{Seconds, Ticks};

pub struct Sleep {
    wake_ticks: usize,
    sleep_ticks: usize,

    tick: usize,
    phase: f32,
}

impl Sleep {
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

    pub fn get_state(&self) -> SleepState {
        if self.phase < 0.5 {
            SleepState::Wake
        } else {
            SleepState::Sleep
        }
    }
}

impl Default for Sleep {
    fn default() -> Self {
        Self::new(Seconds(180.), Seconds(30.))
    }
}

fn update_sleep(mut sleep: ResMut<Sleep>) {
    sleep.tick = (sleep.tick + 1) % (sleep.wake_ticks + sleep.sleep_ticks);

    if sleep.tick < sleep.wake_ticks {
        sleep.phase = 0.5 * sleep.tick as f32 / sleep.wake_ticks as f32;
    } else {
        sleep.phase = 0.5 + 0.5 * (sleep.tick - sleep.wake_ticks) as f32
            / sleep.sleep_ticks as f32;
    }
}

pub enum SleepState {
    Sleep,
    Wake
}

pub struct SleepPlugin;

impl Plugin for SleepPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sleep>();

        app.system(Tick, update_sleep);
    }
}
