mod entity;
mod component;

pub mod prelude {
    pub use super::component::{Component};
    pub use super::entity::{EntityRef};
}
