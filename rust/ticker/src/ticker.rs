//! Single ticking node

use crate::builder::*;
use crate::fiber::*;
use crate::system::ToThreadRef;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

pub type TickFn = dyn Fn(u32)->() + Send;

pub type TickerRef<T> = Arc<RwLock<TickerInner<T>>>;

pub struct Ticker {
    pub id: usize,
    pub name: String,

    //group: ThreadGroupShared<T>,

    //ticker_ref: TickerRef<T>,
}

pub struct TickerInner<T> {
    id: usize,
    name: String,

    to_tickers: Vec<ToTicker<T>>,
    from_tickers: Vec<ToTicker<T>>,

    on_tick: Option<Box<TickFn>>,
    on_fiber: Vec<Box<OnFiberFn<T>>>,
}

pub struct ToTicker<T> {
    pub from_ticker: usize,
    pub to_ticker: usize,
    to: ToThreadRef<T>
}

//
// Implementations
//

impl Ticker {
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker:{}[{}]", self.id, self.name)
    }
}

impl<T> ToTicker<T> {
    pub fn new(
        from_ticker: usize, 
        to_ticker: usize, 
        to: &ToThreadRef<T>
    ) -> ToTicker<T> {
        ToTicker {
            from_ticker,
            to_ticker,
            to: Rc::clone(to),
        }
    }

    pub fn send(&self, on_fiber: usize, args: T) {
        self.to.send(self.to_ticker, on_fiber, self.from_ticker, args);
    }
}

impl<T> Clone for ToTicker<T> {
    fn clone(&self) -> Self {
        Self {
            from_ticker: self.from_ticker,
            to_ticker: self.to_ticker,
            to: Rc::clone(&self.to),
        }
    }
}

impl<T> TickerInner<T> {
    pub fn new(
        id: usize,
        name: String,
        to_tickers: Vec<ToTicker<T>>,
        from_tickers: Vec<ToTicker<T>>,
        on_tick: Option<Box<TickFn>>,
        on_fibers: Vec<Box<OnFiberFn<T>>>
    ) -> TickerRef<T> {
        let ticker = TickerInner {
            id,
            name: name.clone(),
            to_tickers: to_tickers,
            from_tickers: from_tickers,
            on_tick : on_tick,
            on_fiber: on_fibers,
        };

        let ticker_ref = Arc::new(RwLock::new(ticker));

        ticker_ref
    }

    pub fn tick(&self, ticks: u32) {
        match &self.on_tick {
            Some(on_tick) => on_tick(ticks),
            None => panic!("{}.tick called but no on_tick was defined.", self),
        }
    }

    pub fn send(&self, on_fiber: usize, from_ticker: usize, args: T) {
        self.on_fiber[on_fiber](from_ticker, args);
    }
}

impl<T> fmt::Display for TickerInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker:{}[{}]", self.id, self.name)
    }
}
