mod table;
mod entity;
mod cell;
mod column;
pub(crate) mod meta;
pub(crate) mod row;
pub(crate) mod ptr;
mod table2;

pub mod prelude {
    pub use super::row::{RowId, Row};
    pub use super::table2::{Table2, RowRef, QueryIterator};
    pub use super::meta::{ViewTypeId, RowTypeId, Query2, QueryBuilder2, QueryCursor2};
}