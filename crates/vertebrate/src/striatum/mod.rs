mod timeout;
mod striatum2;
mod gate;
pub mod striatum;

pub use gate::{Gate, StriatumGate};
pub use striatum2::Striatum2;
pub use timeout::{StriatumTimeout, TimeoutState};