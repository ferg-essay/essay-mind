//! Single ticking node

use crate::{fiber::*};
use crate::system::{ToThreadRef, ThreadInner};

//use log::{log};

use std::cell::RefCell;
use std::io::Error;
use std::result;
use std::{fmt, rc::Rc};

pub type OnBuild = dyn FnMut()->();
pub type OnTickFn = dyn Fn(u64)->();

pub type ToTickerRef<T> = Rc<RefCell<ToTickerInner<T>>>;
#[allow(dead_code)]
pub type Result<T> = result::Result<T, Error>;

pub trait Ticker {
    #[allow(unused_variables)]
    fn tick(&mut self, ticks: u64) {}
    fn build(&mut self) {}
}

pub struct TickerOuter {
    pub id: usize,
    pub name: String,
}

pub struct TickerInner<T> {
    pub id: usize,
    name: String,

    to_tickers: Vec<ToTicker<T>>,
    from_tickers: Vec<ToTicker<T>>,

    pub on_build: Box<OnBuild>,
    pub on_tick: Box<OnTickFn>,
    on_fiber: Vec<Box<OnFiber<T>>>,
}

pub trait OnTick {
    fn tick(ticks: u32) -> Result<()>;
}


pub struct ToTicker<T> {
    pub from_ticker: usize,
    pub to_ticker: usize,
    pub(crate) to: ToTickerRef<T>,
}

pub struct ToTickerInner<T> {
    pub to: ToThreadRef<T>,
}

//
// Implementations
//

impl TickerOuter {
}

impl fmt::Display for TickerOuter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker:{}[{}]", self.id, self.name)
    }
}

impl<T:'static> ToTicker<T> {
    pub fn new(
        from_ticker: usize, 
        to_ticker: usize, 
        to: &ToThreadRef<T>
    ) -> ToTicker<T> {
        ToTicker {
            from_ticker,
            to_ticker,
            to: Rc::new(RefCell::new(ToTickerInner { to: Rc::clone(&to) })),
        }
    }

    pub fn send(&self, on_fiber: usize, args: T) {
        self.to.borrow().to.send(self.to_ticker, on_fiber, self.from_ticker, args);
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

impl<T:'static> TickerInner<T> {
    pub fn new(
        id: usize,
        name: String,
        to_tickers: Vec<ToTicker<T>>,
        from_tickers: Vec<ToTicker<T>>,
        on_tick: Box<OnTickFn>,
        on_build: Box<OnBuild>,
        on_fibers: Vec<Box<OnFiber<T>>>
    ) -> TickerInner<T> {
        TickerInner {
            id,
            name: name,
            to_tickers: to_tickers,
            from_tickers: from_tickers,
            on_tick,
            on_build,
            on_fiber: on_fibers,
        }
    }
    
    pub fn update_to_tickers(&self, thread: &ThreadInner<T>) -> Vec<usize> {
        for to_ticker in &self.to_tickers {
            //let mut to_ticker_inner: &ToTickerInner<T> = &
            to_ticker.to.borrow_mut().to = thread.to_thread(to_ticker.to_ticker);
        }

        self.from_ticker_ids()
    }

    pub fn from_ticker_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        for from in &self.from_tickers {
            let from_id = from.from_ticker;

            if from_id != from.to_ticker && ! ids.contains(&from_id) {
                ids.push(from_id);
            }
        }

        ids
    }

    pub fn tick(&self, ticks: u64) {
        (self.on_tick)(ticks);
    }

    pub fn on_build(&mut self) {
        (self.on_build)();
    }

    pub fn send(&self, on_fiber: usize, from_ticker: usize, args: T) {
        self.on_fiber[on_fiber](from_ticker, args);
    }
}

impl<T> fmt::Display for TickerInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}
