pub mod action;
pub mod topos;
pub mod gram;

use ticker;

pub use self::topos::{Topos};

#[derive(Clone,Debug)]
struct MindMessage(gram::Gram, Topos);

type MindBuilder = ticker::SystemBuilder<MindMessage>;
type TickerBuilder<T:ticker::Ticker> = ticker::TickerBuilder<MindMessage,T>;
type Source = ticker::Source<MindMessage>;
type Sink = ticker::Sink<MindMessage>;
type Fiber = ticker::Fiber<MindMessage>;