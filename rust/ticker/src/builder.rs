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
    builder: Rc<RefCell<TickerBuilderImpl>>,
}

impl TickerBuilder {
    pub fn on_tick(&self, on_tick: Box<TickFn>) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().on_tick(on_tick);

        self
    }

    pub fn on_fiber<T>(&self, fiber: &FiberBuilder<T>, cb: Box<FiberFn<T>>) -> &Self {
        assert!(! self.builder.borrow().is_built());

        fiber.builder.borrow_mut().to(&self.builder, cb);

        self
    }
}

struct TickerBuilderImpl {
    parent: Rc<RefCell<TickerBuilderData>>,

    name: String,

    ticker: Option<Rc<RefCell<TickerImpl>>>,

    on_tick: Option<Box<TickFn>>,
}

impl TickerBuilderImpl {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>, name: &str)->Self {
        assert!(! parent.borrow().is_built);

        Self {
            parent: Rc::clone(&parent),
            name: String::from(name),
            ticker: None,
            on_tick: None,
        }
    }

    fn on_tick(&mut self, on_tick: Box<TickFn>) -> &mut Self {
        assert!(! self.is_built());

        self.on_tick = Some(on_tick);

        self
    }
    
    fn is_built(&self) -> bool {
        self.parent.borrow().is_built
    }

    fn build(&mut self) -> Rc<RefCell<TickerImpl>> {
        assert!(match self.ticker { None=>true, _=> false });

        let ticker_impl = TickerImpl {
            name: self.name.clone(),
            on_tick: self.on_tick.take(),
        };

        let ticker_ref = Rc::new(RefCell::new(ticker_impl));

        self.ticker = Some(ticker_ref.clone());

        ticker_ref
    }

}

pub struct FiberBuilder<T> {
    builder: Rc<RefCell<FiberBuilderImpl<T>>>,
}

impl<T> FiberBuilder<T> {
    pub fn to(&mut self, ticker: &TickerBuilder, callback: Box<FiberFn<T>>) -> &mut Self {
        self.builder.borrow_mut().to(&ticker.builder, callback);

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
    data: Rc<RefCell<TickerBuilderData>>,

    tickers: Vec<Rc<RefCell<TickerBuilderImpl>>>,
}

impl TickerSystemBuilder {
    pub fn new() -> TickerSystemBuilder {
        Self {
            data: Rc::new(RefCell::new(TickerBuilderData { 
                is_built: false,
                fiber_id: 0,
            })),
            tickers: Vec::new(),
        }
    }

    pub fn fiber<T>(&mut self, name: &str) -> FiberBuilder<T> {
        assert!(! self.data.borrow().is_built);

        FiberBuilder {
            builder: Rc::new(RefCell::new(FiberBuilderImpl::new(&self.data, name))),
        }
    }

    pub fn ticker(&mut self, name: &str) -> TickerBuilder {
        assert!(! self.data.borrow().is_built);

        let ticker = Rc::new(RefCell::new(TickerBuilderImpl::new(&self.data, name)));

        self.tickers.push(ticker.clone());

        TickerBuilder {
            builder: ticker,
        }
    }

    pub fn build(&mut self) -> TickerSystem {
        self.data.borrow_mut().build();

        let mut tickers: Vec<Rc<RefCell<TickerImpl>>> = Vec::new();
        let mut on_tickers: Vec<Rc<RefCell<TickerImpl>>> = Vec::new();

        for builder in self.tickers.drain(..) {
            let mut builder_impl = builder.borrow_mut();
            let ticker: Rc<RefCell<TickerImpl>> = builder_impl.build();

            match &ticker.borrow().on_tick {
                Some(_) => { on_tickers.push(ticker.clone()) },
                _ => {},
            }

            tickers.push(ticker);
        }

        let system_impl = TickerSystemImpl {
            ticks: 0,
            tickers: tickers,
            on_tickers: on_tickers,
        };

        TickerSystem {
            system: Rc::new(RefCell::new(system_impl)),
        }
    }
}