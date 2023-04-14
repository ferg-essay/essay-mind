//use essay_ecs_macros::ScheduleLabel;

use crate::prelude::ScheduleLabel;

mod plugin;
mod app;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoreSchedule {
    Startup,
    Main,
    Outer,
}

pub(crate) mod prelude {
    pub use super::app::{App, Tick};
}