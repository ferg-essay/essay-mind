pub mod touch;
mod locomotion;
mod body;
mod eat;

pub use body::{Body, BodyPlugin};
pub use locomotion::{BodyLocomotion, Action, ActionFactory};
