pub mod body;
pub mod world;
pub mod attention;
pub mod action;
pub mod topos;
pub mod gram;

#[cfg(ticker)]
use ticker;

pub use self::topos::{Topos};
pub use self::gram::{Gram,gram};

#[cfg(ticker)]
pub use self::ticker::{Ticker,Context};

pub type MindMessage = (gram::Gram, Topos);

#[cfg(ticker)]
pub type MindBuilder = ticker::SystemBuilder<MindMessage>;
#[cfg(ticker)]
pub type TickerBuilder<T> = ticker::TickerBuilder<MindMessage,T>;
#[cfg(ticker)]
pub type TickerPtr<T> = ticker::TickerPtr<MindMessage,T>;
#[cfg(ticker)]
pub type Source = ticker::Source<MindMessage>;
#[cfg(ticker)]
pub type Sink = ticker::Sink<MindMessage>;
#[cfg(ticker)]
pub type Fiber = ticker::Fiber<MindMessage>;

#[cfg(ticker)]
pub use self::action::shared_memory::SharedWriter;
