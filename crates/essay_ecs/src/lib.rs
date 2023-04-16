mod util;
mod schedule;
pub mod entity;
mod systems;
mod app;
mod world;

pub mod prelude {
    pub use crate::app::prelude::{App};
    pub use crate::world::{Commands, World, Res, ResMut};
    pub use essay_ecs_macros::{Component, ScheduleLabel, Phase};
    pub use crate::systems::{
        Param, Local
    };

    pub use crate::schedule::{
        IntoSystem, IntoSystemConfig,
        IntoPhaseConfig, IntoPhaseConfigs,
    };
}
