mod entity;
mod entity_meta;
mod row_meta;
mod row;
pub(crate) mod ptr;
mod table;
mod type_meta;

pub mod prelude {
    pub use super::row::{RowMeta};
    pub use super::table::{Table, RowRef, EntityMutIterator};
    pub use super::type_meta::{TypeMetas, TypeIndex};
    pub use super::entity::{EntityTable, EntityRef, Entity2MutIterator, Entity3MutIterator};
}