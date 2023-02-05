mod builder;
pub mod fiber;
use fiber::*;
use builder::*;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

pub struct Ticker {
    pub name : String
}

impl Ticker {
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker[{}]", self.name)
    }
}

pub struct TickerSystem {
}

impl TickerSystem {
}

pub struct TickerSystemBuilder {
    data : Rc<RefCell<TickerBuilderData>>,
}

impl TickerSystemBuilder {
    pub fn new() -> TickerSystemBuilder {
        Self {
            data: Rc::new(RefCell::new(TickerBuilderData { 
                is_built: false,
                fiber_id: 0,
            }))
        }
    }

    pub fn fiber<T>(&mut self, name: &str) -> FiberBuilder<T> {
        assert!(! self.data.borrow().is_built);

        FiberBuilder::new(&self.data, name)
    }

    pub fn build(&mut self) -> TickerSystem {
        self.data.borrow_mut().build();

        TickerSystem {
        }
    }
}