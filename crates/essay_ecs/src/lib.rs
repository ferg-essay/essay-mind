mod entity;
mod fiber;
mod builtin_param;
mod store;
mod system;
mod app;
mod world;
mod tests;
mod type_meta;

mod prelude {
    pub use crate::app::{App, AppRef};
    pub use crate::system::prelude::*;
    pub use crate::entity::prelude::{Component};
}
