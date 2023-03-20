mod system;
mod schedule;
mod app;
mod ptr;
mod env;
mod type_info;
mod mind_ecs;
mod tests;

mod prelude {
    pub use crate::app::{App};
    pub use crate::system::{
        System, IntoSystem, SystemFunction,
        Res
    };
}