//! Fibers communicate between tickers.

use std::{fmt, cell::RefCell, rc::Rc, error::Error};

use crate::{Ticker, ticker::TickerImpl};

pub type FiberFn<T> = dyn Fn(&FiberId,&T)->() + Send;

/// Unique identifier for a fiber.
pub struct FiberId {
    pub id: i32,
    pub name: String,
}

impl Clone for FiberId {
    fn clone(&self) -> FiberId {
        FiberId { id: self.id, name: self.name.clone(), }
    }
}

impl fmt::Display for FiberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberId[{},{}]", self.id, self.name)
    }
}

/// Message channel to `Ticker` targets, where each target is
/// a callback in a Ticker's context.
pub struct Fiber<T>
{
    pub id: FiberId,

    fiber: Rc<RefCell<FiberImpl<T>>>,
}

impl<T> Fiber<T> {
    /// send a message to the fiber targets.
    pub fn send(&self, args: &T) {
        self.fiber.borrow().send(args);
    }
}

impl<T> fmt::Display for Fiber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fiber[{},{}]", self.id.id, self.id.name)
    }
}

pub fn new_fiber<T>(fiber_ref: &Rc<RefCell<FiberImpl<T>>>) -> Fiber<T>
{
    Fiber {
        id: fiber_ref.borrow().id.clone(),
        fiber: fiber_ref.clone(),
    }
}

pub struct FiberImpl<T>
{
    pub id: FiberId,

    to: Vec<(Rc<RefCell<TickerImpl>>,Box<FiberFn<T>>)>,
}

impl<T> FiberImpl<T> {
    pub fn new(id: i32, name: String, to: Vec<(Rc<RefCell<TickerImpl>>,Box<FiberFn<T>>)>) -> Self {
        Self {
            id: FiberId {
                id,
                name,
            },
            to,
        }
    }
    /// send a message to the fiber targets.
    pub fn send(&self, args: &T) {
        for (ticker, cb) in &self.to {
            cb(&self.id, args)
        }
    }
}
