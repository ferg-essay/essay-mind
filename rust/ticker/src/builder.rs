use std::{cell::RefCell, rc::Rc, sync::{Mutex, Arc}};

use crate::{fiber::*, ticker::*, system::{TickerSystem, PanicToThread}};

type SystemBuilderRef<T> = Rc<RefCell<SystemBuilderInner<T>>>;
type TickerBuilderRef<T> = Rc<RefCell<TickerBuilderInner<T>>>;
type FiberBuilderRef<T> = Rc<RefCell<FiberBuilderInner<T>>>;

pub struct SystemBuilder<T:Clone> {
    builder: SystemBuilderRef<T>,

    //fiber_build: Vec<Box<dyn Builder>>,
    //fiber_build: Vec<Box<dyn FnMut ()>>,
}

pub struct SystemBuilderInner<T:Clone> {
    is_built : bool,

    tickers: Vec<TickerBuilderRef<T>>,
    fibers: Vec<FiberBuilderRef<T>>,
}

pub struct TickerBuilder<T:Clone> {
    builder: TickerBuilderRef<T>,
}

struct TickerBuilderInner<T:Clone> {
    id: usize,

    name: Option<String>,

    on_tick: Option<Box<TickFn>>,

    on_fibers: Vec<Box<OnFiberFn<T>>>,

    to_tickers: Vec<ToTicker<T>>,
    from_tickers: Vec<ToTicker<T>>,

    system: SystemBuilderRef<T>,

    // the built ticker
    //ticker_ref: Option<Ticker>,
}

pub struct FiberBuilder<T:Clone> {
    builder: FiberBuilderRef<T>,
}

struct FiberBuilderInner<T:Clone>
{
    id: usize,
    name: Option<String>,

    from_ticker: TickerBuilderRef<T>,

    to: Vec<(ToTicker<T>,usize)>,

    system: SystemBuilderRef<T>,

    fiber_ref: Option<Fiber<T>>,
}

//
// Implementation
//

impl<T:Clone + 'static> SystemBuilder<T> {
    pub fn new() -> SystemBuilder<T> {
        let builder_ref = Rc::new(RefCell::new(SystemBuilderInner {
            is_built: false,
            tickers: Vec::new(),
            fibers: Vec::new(),
        }));

        let mut builder = Self {
            builder: builder_ref,
        };

        let mut ticker = builder.ticker();
        ticker.name("essay::system");
        let fiber = ticker.fiber();
        fiber.name("essay::system");

        builder
    }

    pub fn ticker(&mut self) -> TickerBuilder<T> {
        assert!(! self.builder.borrow().is_built);

        let ticker: TickerBuilderRef<T> = TickerBuilderInner::new(&self.builder);

        let mut builder = self.builder.borrow_mut();

        builder.tickers.push(Rc::clone(&ticker));

        TickerBuilder {
            builder: ticker,
        }
    }

    pub fn build(&mut self) -> TickerSystem<T> {
        // let builder = self.builder.borrow_mut();
        self.builder.borrow_mut().build()
    }
}


impl<T:Clone + 'static> SystemBuilderInner<T> {
    fn build(&mut self) -> TickerSystem<T> {
        assert!(! self.is_built);
        self.is_built = true;

        let mut tickers: Vec<TickerRef<T>> = Vec::new();
        for ticker_ref in self.tickers.drain(..) {
            tickers.push(ticker_ref.borrow_mut().build());
        }

        for fiber in self.fibers.drain(..) {
            fiber.borrow_mut().build();
        }

        let spawn_threads = 0;

        TickerSystem::new(tickers, spawn_threads)
    }
}

impl<T:Clone + 'static> TickerBuilder<T> {
    pub fn name(&self, name: &str) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().name(name);

        self
    }

    pub fn fiber(&mut self) -> FiberBuilder<T> {
        assert!(! self.builder.borrow().is_built());

        let fiber: FiberBuilderRef<T> = FiberBuilderInner::new(&self.builder);
        
        //self.ticker.borrow().fibers.push(Rc::clone(&fiber));

        FiberBuilder {
            builder: fiber,
        }
    }

    pub fn on_tick(&self, on_tick: Box<TickFn>) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().on_tick(on_tick);

        self
    }

    pub fn on_fiber(&self, fiber: &mut FiberBuilder<T>, on_fiber: Box<OnFiberFn<T>>) -> &Self {
        assert!(! self.builder.borrow().is_built());

        let to_ticker_id = self.builder.borrow().id;
        let to = fiber.to_ticker(to_ticker_id);

        let on_fiber_id = self.add_fiber(on_fiber);

        /*
        let mut fiber_inner = fiber.builder.borrow_mut();
        let mut from_ticker = fiber_inner.from_ticker.borrow_mut();
        */


        //let from_ticker = fiber.to(to_ticker, on_fiber_id);


        fiber.builder.borrow_mut().to(to, on_fiber_id);

        self
    }

    fn add_fiber(&self, on_fiber: Box<OnFiberFn<T>>) -> usize {
        self.builder.borrow_mut().on_fiber(on_fiber)
    }
}

impl<T:Clone + 'static> TickerBuilderInner<T> {
    fn new(system_ref: &SystemBuilderRef<T>)->TickerBuilderRef<T> {
        let mut system = system_ref.borrow_mut();

        let id = system.tickers.len();

        let ticker = Rc::new(RefCell::new(Self {
            id: id,
            name: None,
            on_tick: None,
            on_fibers: Vec::new(),
            to_tickers: Vec::new(),
            from_tickers: Vec::new(),
            system: Rc::clone(&system_ref),
        }));

        system.tickers.push(Rc::clone(&ticker));

        ticker
    }

    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    fn on_tick(&mut self, on_tick: Box<TickFn>) {
        assert!(! self.is_built());

        self.on_tick = Some(on_tick);
    }

    fn on_fiber(&mut self, on_fiber: Box<OnFiberFn<T>>) -> usize {
        assert!(! self.is_built());

        let on_fiber_id = self.on_fibers.len();

        self.on_fibers.push(on_fiber);

        //let to_ticker = self.from_ticker.borrow_mut().to_ticker(to_ticker_id);
/*
        let to_ticker = from_ticker.to_ticker(self.id);

        let from_ticker = fiber.to(to_ticker, on_fiber_id);
        */

        on_fiber_id
    }

    fn add_from(&mut self, from_ticker: &ToTicker<T>) {
        match self.from_tickers.iter().filter(|from_ticker| from_ticker.from_ticker == self.id).next() {
            Some(to_ticker) => {},
            None => { self.from_tickers.push(from_ticker.clone()); }
        }
    }

    fn to_ticker(&mut self, to_ticker_id: usize) -> ToTicker<T> {
        match self.to_tickers.iter().filter(|to_ticker| to_ticker.to_ticker == to_ticker_id).next() {
            Some(to_ticker) => to_ticker.clone(),
            None => {
                let to_ticker = ToTicker::new(
                    self.id, 
                    to_ticker_id, 
                    &PanicToThread::new(
                        &format!("{}->{}", self.id, to_ticker_id)
                    ),
                );

                self.to_tickers.push(to_ticker.clone());

                to_ticker
            }
        }
    }
    
    fn is_built(&self) -> bool {
        self.system.borrow().is_built
    }

    fn build(&mut self) -> TickerRef<T> {
        // assert!(match self.ticker_ref { None=>true, _=> false });

        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("ticker-{}", self.id),
        };

        let on_fibers = self.on_fibers.drain(..).collect();

        let ticker = TickerInner::new(
            self.id, 
            name,
            self.to_tickers.drain(..).collect(),
            self.from_tickers.drain(..).collect(),
            self.on_tick.take(), 
            on_fibers
        );

        //self.ticker_ref = Some(ticker.clone());

        ticker
    }
}

impl<T:Clone + 'static> FiberBuilder<T> {
    pub fn name(&self, name: &str) -> &Self {
        self.builder.borrow_mut().name(name);

        self
    }

    pub fn to(&mut self, ticker: &TickerBuilder<T>, on_fiber: Box<OnFiberFn<T>>) -> &Self {
        ticker.on_fiber(self, on_fiber);

        self
    }

    fn from_ticker(&self) -> TickerBuilderRef<T> {
         self.builder.borrow().from_ticker.clone()
    }

    fn to_ticker(&mut self, to_ticker_id: usize) -> ToTicker<T>
    {
        return self.from_ticker().borrow_mut().to_ticker(to_ticker_id);
    }

    pub fn fiber(&self) -> Fiber<T> {
        self.builder.borrow().fiber()
    }
}

impl<T:Clone + 'static> FiberBuilderInner<T> {
    fn new(ticker_ref: &TickerBuilderRef<T>)->FiberBuilderRef<T> {
        let ticker = ticker_ref.borrow();
        let system_ref = &ticker.system;
        let mut system = system_ref.borrow_mut();

        assert!(! system.is_built);

        let id = system.fibers.len();

        let fiber = Rc::new(RefCell::new(Self {
            id: id,
            name: None,
            from_ticker: Rc::clone(ticker_ref),
            to: Vec::new(),
            system: Rc::clone(system_ref),

            fiber_ref: None,
        }));

        system.fibers.push(Rc::clone(&fiber));

        fiber
    }
    
    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
    
    fn to(&mut self, to_ticker: ToTicker<T>, on_fiber: usize) {
        //let to_ticker = self.from_ticker.borrow_mut().to_ticker(to_ticker_id);

        self.to.push((to_ticker, on_fiber));
    }

    fn from_ticker_id(&self) -> usize {
        self.from_ticker.borrow().id
    }

    fn fiber(&self) -> Fiber<T> {
        match &self.fiber_ref {
            Some(fiber) => { fiber.clone() },
            None => {
                panic!("fiber() is not available because the system is not built yet");
            }
        }
    }

    fn build(&mut self) {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", self.id),
        };

        let fiber = Fiber::new(self.id, name, self.to.drain(..).collect());

        self.fiber_ref = Some(fiber);
    }
}
