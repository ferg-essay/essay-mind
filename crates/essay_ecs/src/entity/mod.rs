mod view;
mod insert;
mod table;
mod column;
pub(crate) mod meta;

pub mod prelude {
    pub use super::table::{
        Table, Component, EntityId,
    };
    pub use super::insert::{
        Insert, InsertBuilder, InsertCursor,
    };
    pub use super::view::{
        View, ViewBuilder, ViewCursor, ViewIterator,
    };
    pub use super::meta::{ViewId};
}