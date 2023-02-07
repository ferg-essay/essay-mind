//! Fibers communicate between tickers.

use std::{fmt, cell::RefCell, rc::Rc, error::Error};

use crate::{Ticker, ticker::{TickerImpl, TickerShared}};

pub type FiberFn<T> = dyn Fn(&FiberId, &T)->() + Send;

/// Unique identifier for a fiber.
pub struct FiberId {
    pub id: usize,
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
pub struct Fiber<T:Clone>
{
    pub id: FiberId,

    fiber: Rc<RefCell<FiberImpl<T>>>,
}

impl<T:Clone> Fiber<T> {
    /// send a message to the fiber targets.
    pub fn send(&self, args: T) {
        //let box_args: Box<T> = Box::new(args);

        self.fiber.borrow().send(args);
    }
}

impl<T:Clone> fmt::Display for Fiber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fiber[{},{}]", self.id.id, self.id.name)
    }
}

pub fn new_fiber<T:Clone>(fiber_ref: &Rc<RefCell<FiberImpl<T>>>) -> Fiber<T>
{
    Fiber {
        id: fiber_ref.borrow().id.clone(),
        fiber: fiber_ref.clone(),
    }
}

pub struct FiberImpl<T>
{
    pub id: FiberId,

    to: Vec<(Ticker<T>,usize)>,
}

impl<T:Clone> FiberImpl<T> {
    pub fn new(id: usize, name: String, to: Vec<(Ticker<T>,usize)>) -> Self {
        Self {
            id: FiberId {
                id,
                name,
            },
            to,
        }
    }
    /// send a message to the fiber targets.
    pub fn send(&self, args: T) {
        let box_args = Box::new(args);

        for (ticker, on_fiber) in &self.to {
            ticker.send_fiber(*on_fiber, box_args.clone());
        }
    }
}
