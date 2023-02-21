//! Single ticking node

use crate::fiber::PanicToThread;
use crate::system::{ThreadInner};

//use log::{log};

use std::cell::RefCell;
use std::io::Error;
use std::marker::PhantomData;
use std::result;
use std::{fmt, rc::Rc};

pub type OnBuild<T> = dyn Fn(&mut T)->();
pub type OnTickFn<T> = dyn Fn(&mut T, u64)->();
pub type OnFiber<M,T> = dyn Fn(&mut T, M)->();

//pub type ToTickerRef<T> = Rc<RefCell<ToTickerInner<T>>>;
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

    fn on_build(&mut self);

    fn send(&mut self, on_fiber: usize, args: M);

    /*
    fn update_to_tickers(&self, thread: &ThreadInner<M>) -> Vec<usize>;

    fn from_ticker_ids(&self) -> Vec<usize>;
    */
}


pub struct TickerOuter {
    pub id: usize,
    pub name: String,
}

pub(crate) struct TickerInner<M,T> {
    pub id: usize,
    pub name: String,

    pub ticker: Box<T>,

    //to_tickers: Vec<ToTicker<M>>,
    //from_tickers: Vec<ToTicker<M>>,

    pub on_build: Option<Box<OnBuild<T>>>,
    pub on_tick: Option<Box<OnTickFn<T>>>,
    pub on_fiber: Vec<Box<OnFiber<M,T>>>,
}

pub trait OnTick<T> {
    fn tick(t: &T, ticks: u32) -> Result<()>;
}

/*
pub(crate) struct ToTicker<M> {
    pub from_ticker: usize,
    pub to_ticker: usize,

    pub(crate) to: ToThreadRef<M>,
    //pub channel: usize,
}

pub struct ToTickerInner<M> {
    pub to: ToThreadRef<M>,
}
 */

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
/*
impl<M:'static> ToTicker<M> {
    pub fn new(
        from_ticker: usize, 
        to_ticker: usize, 
        //to: &ToThreadRef<M>
    ) -> ToTicker<M> {
        ToTicker {
            from_ticker,
            to_ticker,
            to: PanicToThread::new("unassigned to-thread"),
            //to: Rc::new(RefCell::new(ToTickerInner { to: Rc::clone(&to) })),
        }
    }

    pub fn send(&mut self, on_fiber: usize, args: M) {
        self.to.borrow_mut().send(self.to_ticker, on_fiber, args);
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
 */


impl<M,T:'static> TickerInner<M,T> {
    pub fn send(&mut self, on_fiber: usize, args: M) {
        self.on_fiber[on_fiber](&mut self.ticker, args);
    }

    fn send2<N>(&mut self, fun: Box<dyn Fn(&mut T, N)>, args: N) {
        fun(&mut self.ticker, args);
    }
}

impl<M:'static,T> TickerCall<M> for TickerInner<M,T> {
    fn id(&self) -> usize {
        self.id
    }

    fn tick(&mut self, ticks: u64) {
        if let Some(on_tick) = &mut self.on_tick {
            on_tick(&mut self.ticker, ticks);
        }
    }

    fn on_build(&mut self) {
        if let Some(on_build) = &mut self.on_build {
            on_build(&mut self.ticker);
        }
    }

    fn send(&mut self, on_fiber: usize, args: M) {
        self.on_fiber[on_fiber](&mut self.ticker, args);
    }
    /*
    fn update_to_tickers(&self, thread: &ThreadInner<M>) -> Vec<usize> {
         self.from_ticker_ids()
    }

    fn from_ticker_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        ids
    }
    */

}

impl<T,M> fmt::Display for TickerInner<T,M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}
