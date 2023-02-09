//! Single ticking node

//use log::info;

use crate::{fiber::*, TickerSystem};
use crate::system::{ToThreadRef, ThreadInner};

//use log::{log};

use std::cell::RefCell;
use std::sync::Arc;
use std::{fmt, rc::Rc};

pub type TickFn = dyn Fn(u64)->() + Send;

//pub type TickerRef<T> = Arc<RwLock<TickerInner<T>>>;
pub type ToTickerRef<T> = Rc<RefCell<ToTickerInner<T>>>;

pub struct Ticker {
    pub id: usize,
    pub name: String,

    //group: ThreadGroupShared<T>,

    //ticker_ref: TickerRef<T>,
}

pub struct TickerInner<T> {
    pub id: usize,
    name: String,

    pub(crate) thread_id: usize,

    to_tickers: Vec<ToTicker<T>>,
    from_tickers: Vec<ToTicker<T>>,

    pub on_tick: Option<Box<TickFn>>,
    on_fiber: Vec<Box<OnFiberFn<T>>>,
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

impl Ticker {
}

impl fmt::Display for Ticker {
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
        on_tick: Option<Box<TickFn>>,
        on_fibers: Vec<Box<OnFiberFn<T>>>
    ) -> TickerInner<T> {
        TickerInner {
            id,
            name: name,
            thread_id: 0,
            to_tickers: to_tickers,
            from_tickers: from_tickers,
            on_tick : on_tick,
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
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}
