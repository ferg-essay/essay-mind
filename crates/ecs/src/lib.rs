mod striatum;
use essay_ecs::{prelude::*, core::{Local, Store, Schedule, schedule::Executors}};

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreTick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostTick;

pub struct TickConfig {
    n_ticks: usize,
}

impl TickConfig {
    pub fn set_n_ticks(&mut self, n_ticks: usize) {
        self.n_ticks = n_ticks;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct TickSchedulePlugin {
    ticks_per_update: usize,
}

impl TickSchedulePlugin {
    pub fn new() -> Self {
        Self {
            ticks_per_update: 1,
        }
    }

    pub fn ticks(self, ticks: usize) -> Self {
        Self {
            ticks_per_update: ticks,
            .. self
        }
    }
}

impl TickSchedulePlugin {
    fn config(&self) -> TickConfig {
        TickConfig {
            n_ticks: self.ticks_per_update,
        }
    }
}

impl Plugin for TickSchedulePlugin {
    fn build(&self, app: &mut App) {
        let mut main_schedule = Schedule::new();
        main_schedule.set_executor(Executors::Single);
        
        app.schedule(Main, main_schedule);

        let tick_cfg = self.config();
        app.insert_resource(tick_cfg);

        app.system(Main, 
            move |w: &mut Store, is_init: Local<bool>| {
                tick_system(w, is_init);
            }
        );
    }
}

fn tick_system(
    store: &mut Store, 
    mut is_init: Local<bool>, 
) {
    if ! *is_init {
        *is_init = true;
        let _ = store.try_run_schedule(PreStartup);
        let _ = store.try_run_schedule(Startup);
        let _ = store.try_run_schedule(PostStartup);
    }

    let _ = store.try_run_schedule(First);
    let _ = store.try_run_schedule(PreUpdate);

    let n_ticks = store.resource::<TickConfig>().n_ticks;
    for _ in 0..n_ticks {
        let _ = store.try_run_schedule(PreTick);
        let _ = store.try_run_schedule(Tick);
        let _ = store.try_run_schedule(PostTick);
    }

    let _ = store.try_run_schedule(Update);
    let _ = store.try_run_schedule(PostUpdate);
    let _ = store.try_run_schedule(Last);
}

