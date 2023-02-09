//! Fibers communicate between tickers.

use std::{fmt, cell::RefCell, rc::Rc, error::Error};

//use log::info;

use crate::{ticker::{ToTicker, ToTickerRef}, system::ToThreadRef};

pub type OnFiberFn<T> = dyn Fn(usize, T)->() + Send;
type FiberRef<T> = Rc<Box<dyn FiberInner<T>>>;

/// Message channel to `Ticker` targets, where each target is
/// a callback in a Ticker's context.
pub struct Fiber<T:Clone>
{
    pub id: usize,
    pub name: String,

    fiber_ref: FiberRef<T>,
}

trait FiberInner<T> {
    /// Sends a message to target `Ticker` on_fiber closures
    fn send(&self, args: T);
}

struct FiberZero {
}

struct FiberOne<T> {
    to: ToTicker<T>,
    on_fiber: usize,
}

pub struct FiberMany<T>
{
    to: Vec<(ToTicker<T>,usize)>,
    to_tail: ToTicker<T>,
    on_fiber_tail: usize,
}

//
// Implementation
//

impl<T:'static + Clone> Fiber<T> {
    pub(crate) fn new(id: usize, name: String, to: Vec<(ToTicker<T>,usize)>) -> Self {
        Fiber {
            id,
            name,
            fiber_ref: Rc::new(Fiber::new_inner(to)),
        }
    }

    fn new_inner(mut to: Vec<(ToTicker<T>,usize)>) -> Box<dyn FiberInner<T>> {
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
    pub fn send(&self, args: T) {
        self.fiber_ref.send(args);
    }
}

impl<T:Clone> fmt::Display for Fiber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fiber:{}[{}]", self.id, self.name)
    }
}

impl<T:Clone> Clone for Fiber<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            fiber_ref: self.fiber_ref.clone(),
        }
    }
}

impl<T> FiberInner<T> for FiberZero {
    /// send a message to the fiber targets.
    fn send(&self, args: T) {
    }
}

impl<T:Clone + 'static> FiberInner<T> for FiberOne<T> {
    fn send(&self, args: T) {
        self.to.send(self.on_fiber, args);
    }
}

impl<T:Clone + 'static> FiberInner<T> for FiberMany<T> {
    fn send(&self, args: T) {
        for (to, on_fiber) in &self.to {
            to.send(*on_fiber, args.clone());
        }

        self.to_tail.send(self.on_fiber_tail, args);
    }
}


