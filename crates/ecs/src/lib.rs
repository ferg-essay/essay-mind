use essay_ecs::{prelude::*, core::{Local, Store, Schedule, schedule::Executors}};

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreTick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostTick;

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
    fn tick_system(world: &mut Store, mut is_init: Local<bool>, n_ticks: usize) {
        if ! *is_init {
            *is_init = true;
            let _ = world.try_run_schedule(PreStartup);
            let _ = world.try_run_schedule(Startup);
            let _ = world.try_run_schedule(PostStartup);
        }

        let _ = world.try_run_schedule(First);
        let _ = world.try_run_schedule(PreUpdate);

        for _ in 0..n_ticks {
            let _ = world.try_run_schedule(PreTick);
            let _ = world.try_run_schedule(Tick);
            let _ = world.try_run_schedule(PostTick);
        }

        let _ = world.try_run_schedule(Update);
        let _ = world.try_run_schedule(PostUpdate);
        let _ = world.try_run_schedule(Last);
    }
}

impl Plugin for TickSchedulePlugin {
    fn build(&self, app: &mut App) {
        let mut main_schedule = Schedule::new();
        main_schedule.set_executor(Executors::Single);
        
        app.schedule(Main, main_schedule);
        let n_ticks = self.ticks_per_update;
        app.system(Main, 
            move |w: &mut Store, is_init: Local<bool>| {
                Self::tick_system(w, is_init, n_ticks);
            }
        );
    }
}
