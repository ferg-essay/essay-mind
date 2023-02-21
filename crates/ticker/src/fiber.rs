//! Fibers communicate between tickers.

use std::{fmt, rc::Rc, cell::{RefCell, Ref}};

//use log::info;

use crate::{ticker::{ToTicker}, system::{ToThreadRef, PanicToThread, ThreadInner}};

//pub type FiberRef<M> = Rc<RefCell<Option<Box<dyn FiberInner<M>>>>>;
pub(crate) type FiberSourceRef<M> = Rc<RefCell<Box<dyn FiberInner<M>>>>;

/// Message channel to `Ticker` targets, where each target is
/// a callback in a Ticker's context.
pub struct Fiber<M:Clone>
{
    //pub id: usize,
    //pub name: String,

    ptr: FiberSourceRef<M>,
}

//
// Implementation
//
/*
struct FiberHolder<M> {
    opt: Option<Box<dyn FiberInner<M>>>,
}

impl<M:Clone> FiberHolder<M> {
    fn send(&self, args: M) {
        match &self.opt {
            Some(fiber) => fiber.send(args),
            None => panic!("mismatched holder")
        }
    }

    fn replace(&mut self, fiber: Box<dyn FiberInner<M>>) {
        self.opt.replace(fiber);
    }
}
 */

impl<M:'static + Clone> Fiber<M> {
    pub(crate) fn new(to: &mut Vec<(usize, usize, usize)>) -> Fiber<M> {
        Fiber {
            ptr: Rc::new(RefCell::new(Self::new_inner(to)))
        }
    }
    fn new_inner(to: &mut Vec<(usize, usize,usize)>) -> Box<dyn FiberInner<M>> {
        match to.len() {
            0 => Box::new(FiberZero {}),
            1 => Box::new(FiberOne::new(to[0])),
            _ => Box::new(FiberMany::new(to)),
        }
    }

    /// send a message to fiber targets, on_fiber closures of target tickers.
    pub fn send(&self, args: M) {
        self.ptr.borrow_mut().send(args);

        //self.fiber_ref.borrow().expect("unconfigured fiber").send(args);
    }
}

impl<M:'static + Clone> Clone for Fiber<M> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

//
// # inner


pub trait ToThread<M> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: M);
}

struct TickerSources<M> {
    sources: Vec<FiberSourceRef<M>>,
}

pub(crate) trait FiberInner<M> {
    /// Sends a message to target `Ticker` on_fiber closures
    fn send(&mut self, args: M);

    /// Builds the channel
    fn build_channel(&mut self, thread: &ThreadInner<M>);
}

struct FiberZero {
}

struct FiberOne<M> {
    source: usize,
    sink: usize,

    to: ToThreadRef<M>,
    
    on_fiber: usize,
}

pub struct FiberMany<M> {
    to: Vec<FiberOne<M>>,
    tail: FiberOne<M>,
}

struct FiberPanic {

}

impl<M> TickerSources<M> {
    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        for source in &mut self.sources {
            source.borrow_mut().build_channel(thread)
        }
    }
    
    fn update_sources(&self, thread: &ThreadInner<M>) {
        for source in &self.sources {
            //let mut to_ticker_inner: &ToTickerInner<T> = &
            source.borrow_mut().build_channel(thread);
        }

        //self.from_ticker_ids()
    }

    /*
    fn from_ticker_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        for from in &self.sources {
            let from_id = from.from_ticker;

            if from_id != from.to_ticker && ! ids.contains(&from_id) {
                ids.push(from_id);
            }
        }

        ids
    }
     */
}

/*
impl<M:Clone> Clone for Fiber<M> {
    fn clone(&self) -> Self {
        Self {
            //id: self.id,
            //name: self.name.clone(),
            ptr: Rc::clone(&self.ptr),
        }
    }
}
 */

impl<M> FiberOne<M> {
    fn new(to: (usize, usize, usize)) -> Self {
        FiberOne {
            source: to.0,
            sink: to.1,
            on_fiber: to.2,
            to: PanicToThread::new("unconfigured fiber")
        }
    }
}

impl<M> FiberMany<M> {
    fn new(to: &mut Vec<(usize, usize, usize)>) -> Self {
        let tail = FiberOne::new(to.pop().unwrap());

        let mut head: Vec<FiberOne<M>> = Vec::new();

        for item in to {
            head.push(FiberOne::new(*item));
        }

        Self {
            to: head,
            tail: tail,
        }
    }
}

impl<M> FiberInner<M> for FiberPanic {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
        panic!("sending message to unconfigured fiber")
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        panic!("building channel for unconfigured fiber")
    }
 }

impl<M> FiberInner<M> for FiberZero {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberOne<M> {
    fn send(&mut self, args: M) {
        self.to.borrow_mut().send(self.sink, self.on_fiber, args);
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        todo!()
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberMany<M> {
    fn send(&mut self, args: M) {
        for to in &mut self.to {
            to.send(args.clone());
        }

        self.tail.send(args);
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        for to in &mut self.to {
            to.build_channel(thread);
        }

        self.tail.build_channel(thread);
    }
}


