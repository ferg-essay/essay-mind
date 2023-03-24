pub(crate) mod ptr;
mod table;
mod type_meta;

pub mod prelude {
    pub use super::table::{Table, RowMeta, EntityRef, EntityMutIterator};
    pub use super::type_meta::{TypeMetas, TypeIndex};
}