//! Single ticking node

use crate::fiber::{ThreadChannels, TickerFibers};
use crate::system::{TickerAssignment, Context, ThreadGroup};
use crate::builder::{TickerBuilderInner};

//use log::{log};

use std::cell::RefCell;
use std::io::Error;
use std::rc::Rc;
use std::result;
use std::sync::{Mutex, Arc};
use std::{fmt};

pub type OnBuild<T> = dyn Fn(&mut T)->();
pub type OnTickFn<T> = dyn Fn(&mut T, &mut Context)->();
pub type OnFiber<M,T> = dyn Fn(&mut T, M)->();

pub(crate) type TickerRef<M> = Box<dyn TickerCall<M>>;

//pub type ToTickerRef<T> = Rc<RefCell<ToTickerInner<T>>>;
#[allow(dead_code)]
pub type Result<T> = result::Result<T, Error>;

pub trait Ticker {
    #[allow(unused_variables)]
    fn tick(&mut self, ctx: &mut Context) {}
    fn build(&mut self) {}
}

pub struct TickerPtr<M, T> {
    pub(crate) ticker: TickerOuter<M, T>,
    pub(crate) threads: Arc<Mutex<ThreadGroup<M>>>,
}

impl<M:Clone + 'static, T:'static> TickerPtr<M, T> {
    pub fn read<R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        self.threads.lock().unwrap().read(&self.ticker, fun)
    }

    pub fn write<R>(&self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.threads.lock().unwrap().write(&self.ticker, fun)
    }
}

pub(crate) trait TickerCall<M> {
    fn id(&self)->usize;

    fn tick(&mut self, ctx: &mut Context);

    fn step(&self)->usize;
    fn offset(&self)->usize;
    fn is_lazy(&self)->bool;

    fn on_build(&mut self);

    fn send(&mut self, on_fiber: usize, args: M);

    fn update(
        &mut self, 
        tickers: &TickerAssignment, 
        channels: &ThreadChannels<M>
    );

    fn clone(&self) -> Box<dyn TickerCall<M>>;
}


pub struct TickerOuter<M, T> {
    pub id: usize,
    pub name: String,

    step: usize,
    offset: usize,
    is_lazy: bool,

    ptr: Rc<RefCell<TickerInner<M, T>>>,
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
    pub fibers: TickerFibers<M,T>,
}

pub trait OnTick<T> {
    fn tick(t: &T, ticks: u32) -> Result<()>;
}

//
// Implementations
//

impl<M:Clone + 'static, T:'static> TickerOuter<M, T> {
    pub(crate) fn new(
        id: usize,
        name: String,
        ticker: Box<T>,
        on_tick: Option<Box<OnTickFn<T>>>,
        on_build: Option<Box<OnBuild<T>>>,
        fibers: TickerFibers<M,T>,
        on_fiber: Vec<Box<OnFiber<M,T>>>,
        builder: &mut TickerBuilderInner<M, T>,
    ) -> TickerOuter<M, T> {
        let inner = TickerInner {
            id,
            name: name.clone(),
            ticker,

            on_tick,
            on_build,

            fibers,
            on_fiber,
        };

        let ptr = Rc::new(RefCell::new(inner));

        TickerOuter {
            id: id,
            name: name,
            ptr: ptr,

            step: builder.step,
            offset: builder.offset,
            is_lazy: builder.is_lazy,
        }
    }       

    pub(crate) fn to_box(self) -> Box<dyn TickerCall<M>> {
        Box::new(self)
    }

    pub fn read<R>(&self, fun: impl FnOnce(&T)->R) -> R {
        fun(&self.ptr.borrow().ticker)
    }

    pub fn write<R>(&self, fun: impl FnOnce(&mut T)->R) -> R {
        fun(&mut self.ptr.borrow_mut().ticker)
    }
}

impl<M:Clone + 'static, T> TickerInner<M, T> {
    fn tick(&mut self, ctx: &mut Context) {
        if let Some(on_tick) = &self.on_tick {
            on_tick(&mut self.ticker, ctx);
        }
    }

    fn on_build(&mut self) {
        if let Some(on_build) = &mut self.on_build {
            on_build(&mut self.ticker);
        }
    }

    fn build_channel(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        self.fibers.build_channel(tickers, channels);
    }

    fn send(&mut self, on_fiber: usize, args: M) {
        self.on_fiber[on_fiber](&mut self.ticker, args);
    }
}

impl<M,T> fmt::Debug for TickerOuter<M,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker:{}[{}]", self.id, self.name)
    }
}

impl<M:Clone+'static,T:'static> TickerCall<M> for TickerOuter<M,T> {
    fn id(&self) -> usize {
        self.id
    }

    fn tick(&mut self, ctx: &mut Context) {
        self.ptr.borrow_mut().tick(ctx);
    }

    fn step(&self) -> usize {
        self.step
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn is_lazy(&self) -> bool {
        self.is_lazy
    }

    fn on_build(&mut self) {
        self.ptr.borrow_mut().on_build();
    }

    fn update(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        self.ptr.borrow_mut().build_channel(&tickers, &channels);
    }

    fn send(&mut self, on_fiber: usize, args: M) {
        self.ptr.borrow_mut().send(on_fiber, args);
    }

    fn clone(&self) -> Box<dyn TickerCall<M>> {
        Box::new(TickerOuter {
            id: self.id,
            name: self.name.clone(),
            ptr: Rc::clone(&self.ptr),

            ..*self
         })
    }
}

impl<M,T> Clone for TickerOuter<M, T> {
    fn clone(&self) -> Self {
        Self { 
            id: self.id.clone(), 
            name: self.name.clone(), 
            ptr: Rc::clone(&self.ptr),

            ..*self
         }
    }
}

impl<T,M> fmt::Debug for TickerInner<T,M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}
