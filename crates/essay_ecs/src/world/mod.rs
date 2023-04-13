mod params;
mod resource;
mod world;
mod cell;

pub mod prelude {
    pub use super::world::{World};
    pub use super::params::{Res, ResMut};
}