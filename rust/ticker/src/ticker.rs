//! Single ticking node

use crate::builder::*;
use crate::fiber::*;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

pub struct Ticker {
    pub name: String,

    ticker: Rc<RefCell<TickerImpl>>,
}

impl Ticker {
}

pub struct TickerImpl {
    name : String,
}

impl TickerImpl {
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker[{}]", self.name)
    }
}

pub struct TickerSystem {
}

impl TickerSystem {
}
