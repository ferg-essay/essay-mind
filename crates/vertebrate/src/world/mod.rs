mod builder;
mod food;
mod odor;
mod world;

pub use builder::WorldPlugin;

pub use food::{Food, FoodKind};

pub use odor::{Odor, OdorType};

pub use world::{
    FloorType, World, WorldCell
};