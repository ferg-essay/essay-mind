use std::{cell::RefCell, rc::Rc};

use crate::{fiber::*, ticker::*};

pub struct TickerBuilderData {
    is_built : bool,

    fiber_id : i32,

    ticker_id : i32,
}

impl TickerBuilderData {
    fn next_fiber_id(&mut self) -> i32 {
        let id = self.fiber_id;
        self.fiber_id += 1;
        id
    }

    fn next_ticker_id(&mut self) -> i32 {
        let id = self.ticker_id;
        self.ticker_id += 1;
        id
    }

    fn build(&mut self) {
        assert!(! self.is_built);
        self.is_built = true;
    }
}

pub struct TickerBuilder {
    builder: Rc<RefCell<TickerBuilderImpl>>,
}

impl TickerBuilder {
    pub fn name(&self, name: &str) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().name(name);

        self
    }

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

    id: i32,

    name: Option<String>,

    on_tick: Option<Box<TickFn>>,

    /// the built ticker
    ticker: Option<Rc<RefCell<TickerImpl>>>,
}

impl TickerBuilderImpl {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>)->Self {
        let mut parent_ref = parent.borrow_mut();
        assert!(! parent_ref.is_built);

        Self {
            parent: Rc::clone(&parent),
            id: parent_ref.next_ticker_id(),
            name: None,
            ticker: None,
            on_tick: None,
        }
    }

    fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(String::from(name));

        self
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

        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("ticker-{}", self.id),
        };

        let ticker_impl = TickerImpl {
            name: name,
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
    pub fn name(&self, name: &str) -> &Self {
        self.builder.borrow_mut().name(name);

        self
    }

    pub fn to(&self, ticker: &TickerBuilder, callback: Box<FiberFn<T>>) -> &Self {
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

    id: i32,

    name: Option<String>,

    //to: Vec<FiberToBind<T>>,
    to: Vec<(Rc<RefCell<TickerBuilderImpl>>,Box<FiberFn<T>>)>,

    fiber_ref: Option<Rc<RefCell<FiberImpl<T>>>>,
}

impl<T> FiberBuilderImpl<T> {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>,)->Self {
        assert!(! parent.borrow().is_built);

        Self {
            parent: Rc::clone(&parent),
            id: parent.borrow_mut().next_fiber_id(),
            name: None,
            to: Vec::new(),
            fiber_ref: None,
        }
    }
    
    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
    
    fn to(&mut self, ticker: &Rc<RefCell<TickerBuilderImpl>>, callback: Box<FiberFn<T>>) {
        self.to.push((ticker.clone(), callback));
    }

    fn fiber(&self) -> Fiber<T> {
        match &self.fiber_ref {
            Some(fiber) => {
                new_fiber(fiber)
            }
            None => {
                panic!("fiber has not been built");
            }
        }
    }

    fn build(&mut self) {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", self.id),
        };

        let mut fiber_vec : Vec<(Rc<RefCell<TickerImpl>>,Box<FiberFn<T>>)> = Vec::new();

        for (builder, cb) in self.to.drain(..) {
            let ticker = match &builder.borrow().ticker {
                Some(ticker) => ticker.clone(),
                None => panic!("ticker was not built"),
            };

            fiber_vec.push((ticker, cb));
        }

        let fiber = FiberImpl::new(self.id, name, fiber_vec);

        self.fiber_ref = Some(Rc::new(RefCell::new(fiber)));
    }
}

pub struct TickerSystemBuilder {
    data: Rc<RefCell<TickerBuilderData>>,

    tickers: Vec<Rc<RefCell<TickerBuilderImpl>>>,

    //fiber_build: Vec<Box<dyn Builder>>,
    fiber_build: Vec<Box<dyn FnMut ()>>,
}

impl TickerSystemBuilder {
    pub fn new() -> TickerSystemBuilder {
        let data = Rc::new(RefCell::new(TickerBuilderData {
            is_built: false,
            ticker_id: 0,
            fiber_id: 0,
        }));

        let mut builder = Self {
            data: data,
            tickers: Vec::new(),
            fiber_build: Vec::new(),
        };

        builder.ticker().name("essay::system");
        let fiber: FiberBuilder<()> = builder.fiber();
        fiber.name("essay::system");

        //let fiber2 = builder.fiber().name("test");

        builder
    }

    pub fn fiber<T:'static>(&mut self) -> FiberBuilder<T> {
        assert!(! self.data.borrow().is_built);

        let builder_impl = Rc::new(RefCell::new(FiberBuilderImpl::new(&self.data)));
        let builder2 = builder_impl.clone();
        //self.fiber_build.push(Box::new(FiberBuilderWrapper { builder: builder_impl.clone() }));
        self.fiber_build.push(Box::new(move || builder2.borrow_mut().build()));

        FiberBuilder {
            builder: builder_impl,
        }
    }

    pub fn ticker(&mut self) -> TickerBuilder {
        assert!(! self.data.borrow().is_built);

        let ticker = Rc::new(RefCell::new(TickerBuilderImpl::new(&self.data)));

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

        for mut fiber_build in self.fiber_build.drain(..) {
            fiber_build();
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