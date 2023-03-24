mod fun_system;
mod system;
mod schedule;

pub mod prelude {
    pub use super::system::{System, IntoSystem};
    pub use super::fun_system::{Param};
    pub use super::schedule::{Schedule};
}