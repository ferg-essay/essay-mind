#[cfg(ticker)]
pub(crate) mod shared_memory;
#[cfg(ticker)]
mod action_group;
#[cfg(ticker)]
mod action;

#[cfg(ticker)]
pub use action_group::{Action, ActionGroup};
#[cfg(ticker)]
pub use shared_memory::{SharedReader, SharedWriter};

#[cfg(test)]
mod tests;
