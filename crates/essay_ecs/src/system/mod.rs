mod fun_system;
mod world_fun_system;
mod param;
mod each_system;
mod channel_system;

pub mod prelude {
    pub use super::param::{Param, Local};
    pub use super::fun_system::{Fun};
}
