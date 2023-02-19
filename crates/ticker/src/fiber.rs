//! Fibers communicate between tickers.

use std::{fmt, rc::Rc, cell::{RefCell, Ref}};

//use log::info;

use crate::{ticker::{ToTicker}};

//pub type FiberRef<M> = Rc<RefCell<Option<Box<dyn FiberInner<M>>>>>;
pub(crate) type FiberRef<M> = Rc<RefCell<Box<dyn FiberInner<M>>>>;

/// Message channel to `Ticker` targets, where each target is
/// a callback in a Ticker's context.
pub struct Fiber<M:Clone>
{
    //pub id: usize,
    //pub name: String,

    pub(crate) fiber_ref: FiberRef<M>,
}

pub(crate) trait FiberInner<M> {
    /// Sends a message to target `Ticker` on_fiber closures
    fn send(&mut self, args: M);
}

struct FiberZero {
}

struct FiberOne<M> {
    to: ToTicker<M>,
    on_fiber: usize,
}

pub struct FiberMany<M> {
    to: Vec<(ToTicker<M>,usize)>,
    to_tail: ToTicker<M>,
    on_fiber_tail: usize,
}

struct FiberPanic {

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
    pub(crate) fn new() -> FiberRef<M> {
        Rc::new(RefCell::new(Box::new(FiberPanic {})))
    }

    pub(crate) fn fill_ptr(
        fiber_ref: &Rc<RefCell<Box<dyn FiberInner<M>>>>,
        to: Vec<(ToTicker<M>,usize)>
    ) {
        fiber_ref.replace(Self::new_inner(to));
    }

    fn new_inner(mut to: Vec<(ToTicker<M>,usize)>) -> Box<dyn FiberInner<M>> {
        match to.len() {
        0 => Box::new(FiberZero {}),
        1 => Box::new(FiberOne { to: to[0].0.clone(), on_fiber: to[0].1 }),
        _ => { match to.pop() {
                    Some((to_tail, on_fiber_tail))  => {
                        Box::new(FiberMany {
                            to,
                            to_tail,
                            on_fiber_tail,
                        })
                    }
                    _ => panic!("missing pair from fiber"),
                }
            }
        }
    }

    /// send a message to fiber targets, on_fiber closures of target tickers.
    pub fn send(&self, args: M) {
        self.fiber_ref.borrow_mut().send(args);

        //self.fiber_ref.borrow().expect("unconfigured fiber").send(args);
    }
}

impl<M:Clone> Clone for Fiber<M> {
    fn clone(&self) -> Self {
        Self {
            //id: self.id,
            //name: self.name.clone(),
            fiber_ref: Rc::clone(&self.fiber_ref),
        }
    }
}

impl<M> FiberInner<M> for FiberPanic {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
        panic!("sending message to unconfigured fiber")
    }
}

impl<M> FiberInner<M> for FiberZero {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberOne<M> {
    fn send(&mut self, args: M) {
        self.to.send(self.on_fiber, args);
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberMany<M> {
    fn send(&mut self, args: M) {
        for (to, on_fiber) in &mut self.to {
            to.send(*on_fiber, args.clone());
        }

        self.to_tail.send(self.on_fiber_tail, args);
    }
}


