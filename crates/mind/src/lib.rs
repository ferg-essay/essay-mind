pub mod action;
pub mod topos;
pub mod gram;

use ticker;

pub use self::topos::{Topos};
pub use self::gram::{Gram,gram};

pub type MindMessage = (gram::Gram, Topos);

pub type MindBuilder = ticker::SystemBuilder<MindMessage>;
pub type TickerBuilder<T> = ticker::TickerBuilder<MindMessage,T>;
pub type TickerPtr<T> = ticker::TickerPtr<MindMessage,T>;
pub type Source = ticker::Source<MindMessage>;
pub type Sink = ticker::Sink<MindMessage>;
pub type Fiber = ticker::Fiber<MindMessage>;