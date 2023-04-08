pub(crate) mod row_meta;
pub(crate) mod row;
pub(crate) mod ptr;
mod table;
mod type_meta;

pub mod prelude {
    pub use super::row::{RowMeta, RowId, Row};
    pub use super::table::{Table, RowRef, EntityMutIterator};
    pub use super::type_meta::{TypeMetas, TypeIndex};
    pub use super::row_meta::{ViewTypeId, RowTypeId};
}