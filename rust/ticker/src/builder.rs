use std::{cell::RefCell, rc::Rc};

use crate::{fiber::*, ticker::*};

pub struct TickerBuilderData {
    pub is_built : bool,

    pub fiber_id : i32,
}

impl TickerBuilderData {
    pub fn fiber_id(&mut self) -> i32 {
        self.fiber_id += 1;
        self.fiber_id
    }

    pub fn build(&mut self) {
        assert!(! self.is_built);
        self.is_built = true;
    }
}


pub struct TickerBuilder {
    pub name: String,

    ticker: Rc<RefCell<TickerBuilderImpl>>,
}

impl TickerBuilder {
    pub fn when<T>(&self, fiber: &FiberBuilder<T>, cb: Box<FiberFn<T>>) -> &Self {
        assert!(! self.ticker.borrow().is_built());

        fiber.builder.borrow_mut().to(&self.ticker, cb);

        self
    }
}

pub struct TickerBuilderImpl {
    parent: Rc<RefCell<TickerBuilderData>>,

    name: String,

    ticker: Option<Rc<RefCell<TickerImpl>>>,
}

impl TickerBuilderImpl {
    pub fn new(parent: &Rc<RefCell<TickerBuilderData>>, name: &str)->Self {
        assert!(! parent.borrow().is_built);

        Self {
            parent: Rc::clone(&parent),
            name: String::from(name),
            ticker: None,
        }
    }
    /*
    pub fn when<T>(&mut self, fiber: FiberBuilder<T>, callback: Box<FiberFn<T>>) -> &mut Self {
        assert!(! self.parent.borrow().is_built);

        fiber.to(self, callback);

        self
    }
    */

    fn is_built(&self) -> bool {
        self.parent.borrow().is_built
    }
}

pub struct FiberBuilder<T> {
    builder: Rc<RefCell<FiberBuilderImpl<T>>>,
}

impl<T> FiberBuilder<T> {
    pub fn to(&mut self, ticker: &TickerBuilder, callback: Box<FiberFn<T>>) -> &mut Self {
        self.builder.borrow_mut().to(&ticker.ticker, callback);

        self
    }

    pub fn fiber(&self) -> Fiber<T> {
        self.builder.borrow().fiber()
    }
}

struct FiberBuilderImpl<T>
{
    parent: Rc<RefCell<TickerBuilderData>>,

    name: Option<String>,

    //to: Vec<FiberToBind<T>>,
    to: Vec<(Rc<RefCell<TickerBuilderImpl>>,Box<FiberFn<T>>)>,

    fiber_ref: Option<Rc<RefCell<FiberImpl<T>>>>,
}

impl<T> FiberBuilderImpl<T> {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>, name: &str)->Self {
        assert!(! parent.borrow().is_built);

        Self {
            parent: Rc::clone(&parent),
            name: None,
            to: Vec::new(),
            fiber_ref: None,
        }
    }
    
    pub fn to(&mut self, ticker: &Rc<RefCell<TickerBuilderImpl>>, callback: Box<FiberFn<T>>) -> &mut Self {
        self.to.push((ticker.clone(), callback));

        self
    }

    pub fn fiber(&self) -> Fiber<T> {
        match &self.fiber_ref {
            Some(fiber) => {
                new_fiber(fiber)
            }
            None => {
                panic!("fiber has not been built");
            }
        }
    }

    pub fn build(&mut self) {
        assert!(! self.parent.borrow().is_built);

        let id = self.parent.borrow_mut().fiber_id();

        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", id),
        };

        let mut fiber_vec : Vec<(Rc<RefCell<TickerImpl>>,Box<FiberFn<T>>)> = Vec::new();

        for (builder, cb) in self.to.drain(..) {
            let ticker = match &builder.borrow().ticker {
                Some(ticker) => ticker.clone(),
                None => panic!("ticker was not built"),
            };

            fiber_vec.push((ticker, cb));
        }

        let fiber = FiberImpl::new(id, name, fiber_vec);

        self.fiber_ref = Some(Rc::new(RefCell::new(fiber)));
    }
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

        FiberBuilder {
            builder: Rc::new(RefCell::new(FiberBuilderImpl::new(&self.data, name))),
        }
    }

    pub fn build(&mut self) -> TickerSystem {
        self.data.borrow_mut().build();

        TickerSystem {
        }
    }
}