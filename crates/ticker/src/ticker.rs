//! Single ticking node

use crate::fiber::{ThreadChannels, TickerFibers};
use crate::system::{TickerAssignment, Context, ThreadGroup, LazyGroup};
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

pub(crate) struct TickerAccess<M,T>(usize,Rc<RefCell<TickerInner<M, T>>>);

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
    pub(crate) ticker: TickerAccess<M, T>,
    pub(crate) threads: Arc<Mutex<ThreadGroup<M>>>,
}


impl<M, T> TickerAccess<M, T> {
    pub(crate) fn new(
        id: usize,
        ptr: Rc<RefCell<TickerInner<M, T>>>
    ) -> Self {
        TickerAccess(id, ptr)
    }

    pub(crate) fn id(&self) -> usize {
        self.0
    }

    pub(crate) fn read<R>(&self, fun: impl FnOnce(&T)->R) -> R {
        fun(&self.1.borrow().ticker)
    }

    pub(crate) fn write<R>(&self, fun: impl FnOnce(&mut T)->R) -> R {
        fun(&mut self.1.borrow_mut().ticker)
    }
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

    fn is_none(&self)->bool;

    fn next_tick(&self) -> u64;
    fn tick(&mut self, ctx: &mut Context);

    fn step(&self)->usize;
    fn offset(&self)->usize;
    fn is_lazy(&self)->bool;
    fn theta(&self)->f32;

    fn on_build(&mut self);

    fn send(&mut self, on_fiber: usize, args: M);

    fn update_channels(
        &mut self, 
        tickers: &TickerAssignment, 
        channels: &ThreadChannels<M>
    );

    fn update_lazy(
        &mut self,
        lazy_group: &LazyGroup,
    );

    fn clone(&self) -> Box<dyn TickerCall<M>>;
}

pub struct TickerOuter<M, T> {
    pub id: usize,
    pub name: String,

    step: usize,
    offset: usize,
    theta: f32,

    ptr: Rc<RefCell<TickerInner<M, T>>>,
}

pub struct TickerLazy<M, T> {
    pub id: usize,
    pub name: String,

    ptr: Rc<RefCell<TickerInner<M, T>>>,

    lazy_group: LazyGroup,
    is_wake: bool,
    last_tick: u64,
    next_tick: u64,
}

pub(crate) struct TickerInner<M,T> {
    pub id: usize,
    pub name: String,

    ticker: Box<T>,

    on_build: Option<Box<OnBuild<T>>>,
    on_tick: Option<Box<OnTickFn<T>>>,

    on_fiber: Vec<Box<OnFiber<M,T>>>,
    fibers: TickerFibers<M>,
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
        ptr: Rc<RefCell<TickerInner<M, T>>>,
        builder: &mut TickerBuilderInner<M, T>,
    ) -> Box<dyn TickerCall<M>> {
        if builder.is_lazy {
            Box::new(TickerLazy {
                id: id,
                name: name,
                ptr: ptr,

                lazy_group: LazyGroup::new(Vec::new()),
                is_wake: false,
                last_tick: 0,
                next_tick: u64::MAX,
            })
        } else {
            Box::new(TickerOuter {
                id: id,
                name: name,
                ptr: ptr,

                step: builder.step,
                offset: builder.offset,
                theta: builder.theta,
            })
        }
    }       
}

impl<M:Clone + 'static, T> TickerInner<M, T> {
    pub(crate) fn new(
        id: usize,
        name: String,
        ticker: Box<T>,
        on_tick: Option<Box<OnTickFn<T>>>,
        on_build: Option<Box<OnBuild<T>>>,
        fibers: TickerFibers<M>,
        on_fiber: Vec<Box<OnFiber<M,T>>>,
    ) -> Rc<RefCell<TickerInner<M, T>>> {
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

        ptr
    }

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

    fn is_none(&self) -> bool { false }
    fn step(&self) -> usize { self.step }
    fn offset(&self) -> usize { self.offset }
    fn theta(&self) -> f32 { self.theta }

    fn is_lazy(&self) -> bool { false }
    fn next_tick(&self) -> u64 { panic!("next_tick not available") }

    fn on_build(&mut self) {
        self.ptr.borrow_mut().on_build();
    }

    fn update_channels(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        self.ptr.borrow_mut().build_channel(&tickers, &channels);
    }

    fn update_lazy(&mut self, _lazy_group: &LazyGroup) {
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

impl<M:Clone+'static,T:'static> TickerCall<M> for TickerLazy<M,T> {
    fn id(&self) -> usize {
        self.id
    }

    fn tick(&mut self, ctx: &mut Context) {
        self.is_wake = false;
        self.next_tick = u64::MAX;
        self.last_tick = ctx.ticks();
        self.lazy_group.ptr.borrow_mut().push(self.id);

        self.ptr.borrow_mut().tick(ctx);
    }

    fn is_none(&self) -> bool { false }

    fn step(&self) -> usize { 0 }
    fn offset(&self) -> usize { 0 }
    fn theta(&self) -> f32 { -1. }

    fn is_lazy(&self) -> bool { true }
    fn next_tick(&self) -> u64 { self.next_tick }

    fn on_build(&mut self) {
        self.ptr.borrow_mut().on_build();
    }

    fn update_channels(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        self.ptr.borrow_mut().build_channel(&tickers, &channels);
    }

    fn update_lazy(&mut self, lazy_group: &LazyGroup) {
        self.lazy_group = (*lazy_group).clone();
    }

    fn send(&mut self, on_fiber: usize, args: M) {
        self.ptr.borrow_mut().send(on_fiber, args);

        if ! self.is_wake {
            self.is_wake = true;
            self.next_tick = self.last_tick;
            self.lazy_group.push(self.id);
        }
    }

    fn clone(&self) -> Box<dyn TickerCall<M>> {
        Box::new(TickerLazy {
            id: self.id,
            name: self.name.clone(),
            ptr: Rc::clone(&self.ptr),

            lazy_group: LazyGroup::new(Vec::new()),

            ..*self
         })
    }
}

impl<T,M> fmt::Debug for TickerInner<T,M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TickerInner:{}[{}]", self.id, self.name)
    }
}

pub(crate) struct TickerNone {
    id: usize,
}

impl TickerNone {
    pub(crate) fn new<M>(id: usize) -> TickerRef<M> {
        Box::new(TickerNone { id: id })
    }
}

impl<M> TickerCall<M> for TickerNone {

    fn id(&self)->usize { self.id }

    fn is_none(&self) -> bool { true }

    fn step(&self)->usize { 0 }
    fn offset(&self)->usize { 0 }
    fn theta(&self)->f32 { -1. }

    fn is_lazy(&self)->bool { false }
    fn next_tick(&self)->u64 { panic!("next tick called on non-ticker") }

    fn tick(&mut self, _ctx: &mut Context) {
        panic!("tick call on non-ticker #{}", self.id);
    }

    fn on_build(&mut self) {
    }

    fn send(&mut self, _on_fiber: usize, _args: M) {
        panic!("Sending to unconfigured ticker");
    }

    fn update_channels(
        &mut self, 
        _tickers: &TickerAssignment, 
        _channels: &ThreadChannels<M>
    ) {
    }

    fn update_lazy(
        &mut self, 
        _lazy_group: &LazyGroup
    ) {
    }

    fn clone(&self) -> Box<dyn TickerCall<M>> {
        TickerNone::new(self.id)
    }
}
