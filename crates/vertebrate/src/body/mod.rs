pub mod touch;
mod body;
mod body_eat;

pub use body::{Body, BodyPlugin, BodyAction};
pub use body_eat::{BodyEat, BodyEatPlugin};
//pub use locomotion::{BodyLocomotion, Action, ActionFactory};
