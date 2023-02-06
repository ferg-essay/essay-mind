//! Single ticking node

use crate::builder::*;
use crate::fiber::*;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

pub type TickFn = dyn Fn(i32)->() + Send;

pub struct Ticker {
    pub name: String,

    ticker: Rc<RefCell<TickerImpl>>,
}

impl Ticker {
    fn tick(&self, ticks: i32) {
        self.ticker.borrow().tick(ticks);
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker[{}]", self.name)
    }
}

pub struct TickerImpl {
    pub(crate) name: String,

    pub(crate) on_tick: Option<Box<TickFn>>,
}

impl TickerImpl {
    fn tick(&self, ticks: i32) {
        match &self.on_tick {
            Some(on_tick) => on_tick(ticks),
            _ => panic!("TickerImpl.tick called but on_tick."),
        }
    }
}

pub struct TickerSystem {
    pub(crate) system: Rc<RefCell<TickerSystemImpl>>
}

impl TickerSystem {
    pub fn ticks(&self) -> i32 {
        self.system.borrow().ticks
    }

    pub fn tick(&self) {
        self.system.borrow_mut().tick();
    }
}

pub struct TickerSystemImpl {
    pub ticks: i32,

    pub(crate) tickers: Vec<Rc<RefCell<TickerImpl>>>,

    pub(crate) on_tickers: Vec<Rc<RefCell<TickerImpl>>>,
}

impl TickerSystemImpl {
    pub fn tick(&mut self) {
        let ticks = self.ticks + 1;
        self.ticks = ticks;

        for ticker in &self.on_tickers {
            ticker.borrow().tick(ticks);
        }
    }
}
