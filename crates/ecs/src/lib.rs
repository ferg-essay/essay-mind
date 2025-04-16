use std::ops::{Deref, DerefMut};

use essay_ecs::{prelude::*, core::{error::Result, Local, Store, Schedule, schedule::Executors}};
use util::random::random_test;

pub struct MindApp {
    app: App
}

impl MindApp {
    pub fn new() -> Self {
        let mut app = App::new();
        app.plugin(TickSchedulePlugin::new());

        Self {
            app
        }
    }

    pub fn test() -> Self {
        let app = Self::new();

        random_test();

        app
    }

    pub fn build(self) -> App {
        self.app
    }
}

impl Deref for MindApp {
    type Target = App;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for MindApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreTick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostTick;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AfterTicks;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreMenu;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Menu;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostMenu;

pub struct TickConfig {
    n_ticks: usize,
    state: TickState,
    is_run: bool,
    one_tick: bool,
}

impl TickConfig {
    pub fn is_run(&mut self) -> bool {
        if self.one_tick {
            self.one_tick = false;
            true
        } else {
            self.is_run
        }
    }

    pub fn set_n_ticks(&mut self, n_ticks: usize) {
        self.n_ticks = n_ticks;
    }

    pub fn toggle_run(&mut self) {
        self.is_run = ! self.is_run;
    }

    pub fn one_tick(&mut self) {
        self.is_run = false;
        self.one_tick = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TickState {
    Default,
    Menu
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
            state: TickState::Default,
            is_run: true,
            one_tick: false,
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

        app.init_resource::<AppTick>();

        app.system(Main, 
            move |store: &mut Store, is_init: Local<bool>| {
                tick_system(store, is_init)
            }
        );

        app.system(PreTick, 
            |mut ticks: ResMut<AppTick>| {
                ticks.update();
            }
        );
    }
}

pub struct AppTick(u64);

impl AppTick {
    #[inline]
    pub fn ticks(&self) -> u64 {
        self.0
    }

    #[inline]
    fn update(&mut self) {
        self.0 += 1;
    }
}

impl Default for AppTick {
    fn default() -> Self {
        Self(Default::default())
    }
}

fn tick_system(
    store: &mut Store, 
    mut is_init: Local<bool>, 
) -> Result<()> {
    if ! *is_init {
        *is_init = true;
        store.run_schedule_optional(PreStartup)?;
        store.run_schedule_optional(Startup)?;
        store.run_schedule_optional(PostStartup)?;
    }

    store.run_schedule_optional(First)?;

    match store.resource::<TickConfig>().state {
        TickState::Default => {
            store.run_schedule_optional(PreUpdate)?;
            
            let is_run = store.resource_mut::<TickConfig>().is_run();
            if is_run {
                let n_ticks = store.resource::<TickConfig>().n_ticks;
                for _ in 0..n_ticks {
                    store.run_schedule_optional(PreTick)?;
                    store.run_schedule_optional(Tick)?;
                    store.run_schedule_optional(PostTick)?;
                }
            }

            store.run_schedule_optional(AfterTicks)?;

            store.run_schedule_optional(Update)?;
            store.run_schedule_optional(PostUpdate)?;
        },
        TickState::Menu => {
            store.run_schedule_optional(PreMenu)?;
            store.run_schedule_optional(Menu)?;
            store.run_schedule_optional(PostMenu)?;
        }
    }
    store.run_schedule_optional(Last)?;

    Ok(())
}


