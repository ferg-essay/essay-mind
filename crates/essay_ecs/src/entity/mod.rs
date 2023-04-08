mod component;
mod entity;

pub mod prelude {
    pub use super::entity::{EntityTable, EntityRef, Entity2MutIterator, 
    Entity3MutIterator};
    pub use super::component::{Insert, Component};
}
