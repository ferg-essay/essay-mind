mod view;
mod insert;
mod store;
mod column;
pub(crate) mod meta;

pub use store::Component;

pub mod prelude {
    pub use super::store::{
        Store, Component, EntityId,
    };
    pub use super::insert::{
        Insert, InsertBuilder, InsertCursor,
    };
    pub use super::view::{
        View, ViewBuilder, ViewCursor, ViewIterator,
    };
    pub use super::meta::{ViewId};
}