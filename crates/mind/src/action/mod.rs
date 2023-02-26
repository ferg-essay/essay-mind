pub(crate) mod shared_memory;
mod action_group;
mod action;

pub use action_group::{Action, ActionGroup};
pub use shared_memory::{SharedReader, SharedWriter};

#[cfg(test)]
mod tests;
