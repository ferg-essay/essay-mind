mod state;
mod fun_system;
mod system;
mod param;
mod each_system;
mod channel_system;

pub mod prelude {
    pub use super::system::{System, IntoSystem, SystemMeta, SystemState};
    pub use super::param::{Param, Local};
    pub use super::fun_system::{Fun};
}
