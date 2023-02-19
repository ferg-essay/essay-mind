//! Single ticking node

use crate::system::{ToThreadRef, ThreadInner};

//use log::{log};

use std::cell::RefCell;
use std::io::Error;
use std::result;
use std::{fmt, rc::Rc};

pub type OnBuild<T> = dyn FnMut(&T)->();
pub type OnTickFn<T> = dyn Fn(&T, u64)->();
pub type OnFiber<M,T> = dyn Fn(&mut T, M)->();

pub type ToTickerRef<T> = Rc<RefCell<ToTickerInner<T>>>;
#[allow(dead_code)]
pub type Result<T> = result::Result<T, Error>;

pub trait Ticker {
    #[allow(unused_variables)]
    fn tick(&mut self, ticks: u64) {}
    fn build(&mut self) {}
}

pub trait TickerCall<M> {
    fn id(&self)->usize;

    fn tick(&mut self, ticks: u64);
    // {
    //    (self.on_tick)(&self.ticker, ticks);
    //}

    fn on_build(&mut self);
    // {
    //    (self.on_build)(&self.ticker);
    //}

    fn send(&mut self, on_fiber: usize, args: M);
    // {
    //    self.on_fibers[on_fiber](&mut self.ticker, args);
    //}
   fn update_to_tickers(&self, thread: &ThreadInner<M>) -> Vec<usize>;

   fn from_ticker_ids(&self) -> Vec<usize>;
}


pub struct TickerOuter {
    pub id: usize,
    pub name: String,
}

pub(crate) struct TickerInner<M,T> {
    pub id: usize,
    name: String,

    ticker: Box<T>,

    to_tickers: Vec<ToTicker<M>>,
    from_tickers: Vec<ToTicker<M>>,

    pub on_build: Option<Box<OnBuild<T>>>,
    pub on_tick: Option<Box<OnTickFn<T>>>,
    on_fiber: Vec<Box<OnFiber<M,T>>>,
}

pub trait OnTick<T> {
    fn tick(t: &T, ticks: u32) -> Result<()>;
}


pub struct ToTicker<M> {
    pub from_ticker: usize,
    pub to_ticker: usize,
    pub(crate) to: ToTickerRef<M>,
}

pub struct ToTickerInner<M> {
    pub to: ToThreadRef<M>,
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

impl<M:'static> ToTicker<M> {
    pub fn new(
        from_ticker: usize, 
        to_ticker: usize, 
        to: &ToThreadRef<M>
    ) -> ToTicker<M> {
        ToTicker {
            from_ticker,
            to_ticker,
            to: Rc::new(RefCell::new(ToTickerInner { to: Rc::clone(&to) })),
        }
    }

    pub fn send(&mut self, on_fiber: usize, args: M) {
        self.to.borrow().to.borrow_mut().send(self.to_ticker, on_fiber, args);
    }
}

impl<M> Clone for ToTicker<M> {
    fn clone(&self) -> Self {
        Self {
            from_ticker: self.from_ticker,
            to_ticker: self.to_ticker,
            to: Rc::clone(&self.to),
        }
    }
}


impl<M,T:'static> TickerInner<M,T> {
    pub fn new(
        ticker: Box<T>,

        id: usize,
        name: String,

        to_tickers: Vec<ToTicker<M>>,
        from_tickers: Vec<ToTicker<M>>,

        on_tick: Option<Box<OnTickFn<T>>>,
        on_build: Option<Box<OnBuild<T>>>,
        on_fibers: Vec<Box<OnFiber<M,T>>>
    ) -> TickerInner<M,T> {
        TickerInner {
            id,
            name: name,
            ticker: ticker,
            to_tickers: to_tickers,
            from_tickers: from_tickers,
            on_tick,
            on_build,
            on_fiber: on_fibers,
        }
    }

    pub fn tick(&mut self, ticks: u64) {
        if let Some(on_tick) = &mut self.on_tick {
            on_tick(&self.ticker, ticks);
        }
    }

    pub fn on_build(&mut self) {
        if let Some(on_build) = &mut self.on_build {
            on_build(&self.ticker);
        }
    }

    pub fn send(&mut self, on_fiber: usize, args: M) {
        self.on_fiber[on_fiber](&mut self.ticker, args);
    }
}

impl<M:'static,T> TickerCall<M> for TickerInner<M,T> {
    fn id(&self) -> usize {
        self.id
    }

    fn tick(&mut self, ticks: u64) {
        if let Some(on_tick) = &mut self.on_tick {
            on_tick(&self.ticker, ticks);
        }
    }

    fn on_build(&mut self) {
        if let Some(on_build) = &mut self.on_build {
            on_build(&self.ticker);
        }
    }

    fn send(&mut self, on_fiber: usize, args: M) {
        self.on_fiber[on_fiber](&mut self.ticker, args);
    }
    
    fn update_to_tickers(&self, thread: &ThreadInner<M>) -> Vec<usize> {
        for to_ticker in &self.to_tickers {
            //let mut to_ticker_inner: &ToTickerInner<T> = &
            to_ticker.to.borrow_mut().to = thread.to_thread(to_ticker.to_ticker);
        }

        self.from_ticker_ids()
    }

    fn from_ticker_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        for from in &self.from_tickers {
            let from_id = from.from_ticker;

            if from_id != from.to_ticker && ! ids.contains(&from_id) {
                ids.push(from_id);
            }
        }

        ids
    }

}

impl<T,M> fmt::Display for TickerInner<T,M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}
