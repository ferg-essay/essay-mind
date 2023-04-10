mod entity;
mod component;
mod table;

pub mod prelude {
    pub use super::component::{Component};
    pub use super::table::{EntityTable, Entity2MutIterator, 
    Entity3MutIterator};
    pub use super::entity::{EntityRef};
}
