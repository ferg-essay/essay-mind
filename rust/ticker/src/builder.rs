use std::{cell::RefCell, rc::Rc, sync::{Mutex, Arc}};

use crate::{fiber::*, ticker::*};

type TickerBuilderShared<T> = Rc<RefCell<TickerBuilderImpl<T>>>;
type FiberBuilderShared<T> = Rc<RefCell<FiberBuilderImpl<T>>>;

pub struct TickerBuilderData {
    is_built : bool,

    fiber_id : usize,

    ticker_id : usize,
}

impl TickerBuilderData {
    fn next_fiber_id(&mut self) -> usize {
        let id = self.fiber_id;
        self.fiber_id += 1;
        id
    }

    fn next_ticker_id(&mut self) -> usize {
        let id = self.ticker_id;
        self.ticker_id += 1;
        id
    }

    fn build(&mut self) {
        assert!(! self.is_built);
        self.is_built = true;
    }
}

pub struct TickerBuilder<T:Clone> {
    builder: TickerBuilderShared<T>,
}

impl<T:Clone> TickerBuilder<T> {
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

    pub fn on_fiber(&self, fiber: &FiberBuilder<T>, on_fiber: Box<FiberFn<T>>) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().on_fiber(fiber, on_fiber);

        self
    }
}

struct TickerBuilderImpl<T:Clone> {
    parent: Rc<RefCell<TickerBuilderData>>,

    id: usize,

    name: Option<String>,

    on_tick: Option<Box<TickFn>>,

    on_fibers: Vec<(usize, Box<FiberFn<T>>)>,

    /// the built ticker
    ticker: Option<Ticker<T>>,
}

struct OnFiberBuilder<T> {
    on_fiber_i: usize,

    on_fiber: Box<FiberFn<T>>,
}

impl<T:Clone> TickerBuilderImpl<T> {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>)->Self {
        let mut parent_ref = parent.borrow_mut();
        assert!(! parent_ref.is_built);

        Self {
            parent: Rc::clone(&parent),
            id: parent_ref.next_ticker_id(),
            name: None,
            ticker: None,
            on_tick: None,
            on_fibers: Vec::new(),
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

    fn on_fiber(&mut self, fiber: &FiberBuilder<T>, on_fiber: Box<FiberFn<T>>) -> &mut Self {
        assert!(! self.is_built());

        let on_fiber_i = self.on_fibers.len();

        self.on_fibers.push((fiber.builder.borrow().id, on_fiber));

        fiber.builder.borrow_mut().to(self.id, on_fiber_i);

        self
    }

    fn ticker(&self) -> Ticker<T> {
        match &self.ticker {
            Some(ticker) => ticker.clone(),
            _ => panic!("ticker() is not available because the system is not built yet"),
        }
    }
    
    fn is_built(&self) -> bool {
        self.parent.borrow().is_built
    }

    fn build(&mut self, system: &mut TickerSystemBuilder<T>) -> Ticker<T> {
        assert!(match self.ticker { None=>true, _=> false });

        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("ticker-{}", self.id),
        };

        let mut on_fibers: Vec<(FiberId, Box<FiberFn<T>>)> = Vec::new();

        for (id, fun) in self.on_fibers.drain(..) {
            on_fibers.push((system.fiber_id(id), fun));
        }

        let ticker = TickerImpl::new(self.id, name, self.on_tick.take(), on_fibers);

        self.ticker = Some(ticker.clone());

        ticker
    }

}

pub struct FiberBuilder<T> {
    builder: Rc<RefCell<FiberBuilderImpl<T>>>,
}

impl<T:Clone> FiberBuilder<T> {
    pub fn name(&self, name: &str) -> &Self {
        self.builder.borrow_mut().name(name);

        self
    }

    pub fn to(&self, ticker: &TickerBuilder<T>, on_fiber: Box<FiberFn<T>>) -> &Self {
        ticker.on_fiber(self, on_fiber);

        self
    }

    pub fn fiber(&self) -> Fiber<T> {
        self.builder.borrow().fiber()
    }
}

struct FiberBuilderImpl<T>
{
    parent: Rc<RefCell<TickerBuilderData>>,

    id: usize,

    name: Option<String>,

    //to: Vec<FiberToBind<T>>,
    to: Vec<(usize,usize)>,

    fiber_ref: Option<Rc<RefCell<FiberImpl<T>>>>,
}

impl<T:Clone> FiberBuilderImpl<T> {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>)->Self {
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

    fn fiber_id(&self) -> FiberId {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", self.id),
        };

        FiberId {
            id: self.id,
            name: name,
        }
    }
    
    fn to(&mut self, ticker_i: usize, on_fiber_i: usize) {
        self.to.push((ticker_i, on_fiber_i));
    }

    fn fiber(&self) -> Fiber<T> {
        match &self.fiber_ref {
            Some(fiber) => {
                new_fiber(fiber)
            }
            None => {
                panic!("fiber() is not available because the system is not built yet");
            }
        }
    }

    fn build(&mut self, system: &mut TickerSystemBuilder<T>) {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", self.id),
        };

        let mut fiber_vec : Vec<(Ticker<T>, usize)> = Vec::new();

        for (ticker_i, on_fiber_i) in self.to.drain(..) {
            let ticker = system.get_ticker(ticker_i);

            fiber_vec.push((ticker, on_fiber_i));
        }

        let fiber = FiberImpl::new(self.id, name, fiber_vec);

        self.fiber_ref = Some(Rc::new(RefCell::new(fiber)));
    }
}

pub struct TickerSystemBuilder<T:Clone> {
    data: Rc<RefCell<TickerBuilderData>>,

    tickers: Vec<Rc<RefCell<TickerBuilderImpl<T>>>>,
    fibers: Vec<Rc<RefCell<FiberBuilderImpl<T>>>>,

    //fiber_build: Vec<Box<dyn Builder>>,
    //fiber_build: Vec<Box<dyn FnMut ()>>,
}

impl<T:Clone> TickerSystemBuilder<T> {
    pub fn new() -> TickerSystemBuilder<T> {
        let data = Rc::new(RefCell::new(TickerBuilderData {
            is_built: false,
            ticker_id: 0,
            fiber_id: 0,
        }));

        let mut builder = Self {
            data: data,
            tickers: Vec::new(),
            fibers: Vec::new(),
      //      fiber_build: Vec::new(),
        };

        builder.ticker().name("essay::system");
        builder.fiber().name("essay::system");

        //let fiber2 = builder.fiber().name("test");

        builder
    }

    fn fiber_id(&self, fiber_i: usize) -> FiberId {
        self.fibers[fiber_i].borrow().fiber_id()
    }

    pub fn fiber(&mut self) -> FiberBuilder<T> {
        assert!(! self.data.borrow().is_built);

        let fiber = Rc::new(RefCell::new(FiberBuilderImpl::new(&self.data)));
        
        self.fibers.push(fiber.clone());

        FiberBuilder {
            builder: fiber,
        }
    }

    pub fn ticker(&mut self) -> TickerBuilder<T> {
        assert!(! self.data.borrow().is_built);

        let ticker = Rc::new(RefCell::new(TickerBuilderImpl::new(&self.data)));

        self.tickers.push(ticker.clone());

        TickerBuilder {
            builder: ticker,
        }
    }

    fn get_ticker(&self, ticker_i: usize) -> Ticker<T> {
        self.tickers[ticker_i].borrow().ticker()
    }

    pub fn build(&mut self) -> TickerSystem<T> {
        self.data.borrow_mut().build();

        let mut ticker_builders : Vec<TickerBuilderShared<T>> = self.tickers.clone();

        //let mut on_tickers: Vec<Rc<RefCell<TickerImpl<T>>>> = Vec::new();

        /*
        for ticker in self.tickers.drain(..) {
            ticker_builders.push(ticker);
        }
        */

        let mut tickers: Vec<Ticker<T>> = Vec::new();
        for ticker_builder in ticker_builders {
            let mut ticker_impl = ticker_builder.borrow_mut();
            // let ticker: Rc<RefCell<TickerImpl<T>>> = builder_impl.build();

            tickers.push(ticker_impl.build(self));
        }

        //let mut fiber_builders : Vec<FiberBuilderShared<T>> = self.fibers.drain(..).collect();
        let mut fiber_builders : Vec<FiberBuilderShared<T>> = self.fibers.clone();

        for mut fiber_build in fiber_builders {
            fiber_build.borrow_mut().build(self);
        }

        TickerSystemImpl::new(tickers)
    }
}