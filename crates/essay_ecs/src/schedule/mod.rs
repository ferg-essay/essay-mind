mod schedule;

pub use schedule::ScheduleLabel;

pub mod prelude {
    pub use super::schedule::{Schedules, Schedule, ScheduleLabel, BoxedLabel};
}