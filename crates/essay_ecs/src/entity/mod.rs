mod table;
mod entity;
mod component;
mod cell;
mod column;
pub(crate) mod meta;
pub(crate) mod row;
pub(crate) mod ptr;

pub mod prelude {
    pub use super::row::{RowId};
    pub use super::table::{
        Table, RowRef, 
        Query, QueryBuilder, QueryCursor, QueryIterator,
        Insert, InsertBuilder, InsertCursor
    };
    pub use super::meta::{ViewTypeId, RowTypeId};
    pub use super::component::{Component};
}