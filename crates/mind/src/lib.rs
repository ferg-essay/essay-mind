pub mod attention;
pub mod action;
pub mod topos;
pub mod gram;

pub use self::topos::{Topos};
pub use self::gram::{Gram,gram};

pub type MindMessage = (gram::Gram, Topos);
