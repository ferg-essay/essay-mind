use std::{cell::RefCell, rc::Rc, fmt};

use crate::{fiber::*, ticker::*, system::{TickerSystem, PanicToThread}};

type SystemBuilderRef<T> = Rc<RefCell<SystemBuilderInner<T>>>;
type TickerBuilderRef<T> = Rc<RefCell<TickerBuilderInner<T>>>;
type FiberBuilderRef<T> = Rc<RefCell<FiberBuilderInner<T>>>;

pub struct SystemBuilder<T:Clone> {
    builder: SystemBuilderRef<T>,
}

pub struct SystemBuilderInner<T:Clone> {
    is_built : bool,

    tickers: Vec<TickerBuilderRef<T>>,
    fibers: Vec<FiberBuilderRef<T>>,
}

pub struct TickerBuilder<T:Clone,E:Ticker> {
    builder: TickerBuilderRef<T>,
    ptr: Rc<RefCell<Box<E>>>,
}

struct TickerBuilderInner<T:Clone> {
    id: usize,

    name: Option<String>,

    //ptr: Rc<RefCell<Box<dyn Ticker>>>,

    on_build: Option<Box<OnBuild>>,
    on_tick: Option<Box<OnTickFn>>,
    on_fibers: Vec<Box<OnFiber<T>>>,

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

    _system: SystemBuilderRef<T>,

    fiber_ref: Option<Fiber<T>>,
}

struct ExternalTicker {
}

//
// Implementation
//

impl<T:Clone + 'static> SystemBuilder<T> {
    /// Create a new ticker `SystemBuilder`
    /// 
    /// ```rust
    /// let builder = SystemBuilder::<u32>::new();
    /// let system = builder.build();
    /// ```
    /// 
    pub fn new() -> SystemBuilder<T> {
        let builder_ref = Rc::new(RefCell::new(SystemBuilderInner {
            is_built: false,
            tickers: Vec::new(),
            fibers: Vec::new(),
        }));

        let mut builder = Self {
            builder: builder_ref,
        };

        let ticker = builder.ticker(ExternalTicker {});
        ticker.name("essay::external");

        builder
    }

    /// Create a new Ticker
    /// 
    pub fn ticker<E:Ticker + 'static>(&mut self, ticker: E) -> TickerBuilder<T,E> {
        assert!(! self.builder.borrow().is_built);

        let p1 = Box::new(ticker);
        // let p2: Box<dyn Ticker> = p1;

        let ptr = Rc::new(RefCell::new(p1));

        let ptr2: Rc<RefCell<Box<E>>> = ptr.clone();
        let ptr3: Rc<RefCell<Box<E>>> = ptr.clone();

        let ticker: TickerBuilderRef<T> = TickerBuilderInner::new(
            &self.builder,
            Box::new(move || ptr2.borrow_mut().build()),
            Box::new(move |ticks| ptr3.borrow_mut().tick(ticks)),
        );

        //let mut builder = self.builder.borrow_mut();

        //builder.tickers.push(Rc::clone(&ticker));

        TickerBuilder {
            builder: ticker,
            ptr: ptr,
        }
    }

    /// Create a fiber from an external source. External code must send
    /// messages to a ticker using an external fiber.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// let external = system.external_fiber();
    /// 
    /// let system = system.build();
    /// let external = external.fiber();
    /// 
    /// external.send((23, ));
    /// ```
    /// 
    pub fn external_fiber(&mut self) -> FiberBuilder<T> {
        //let system = self.builder.borrow();

        //assert!(! system.is_built);

        //let ticker = Rc::clone(&system.tickers[0]);

        FiberBuilderInner::new(&self.external_ticker())
    }

    fn external_ticker(&self) -> TickerBuilderRef<T> {
        let system = self.builder.borrow();

        assert!(! system.is_built);

        Rc::clone(&system.tickers[0])
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

        let mut tickers: Vec<TickerInner<T>> = Vec::new();
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

impl<T:Clone + 'static,E:Ticker + 'static> TickerBuilder<T, E> {
    /// Sets a debugging name for the ticker.
    pub fn name(&self, name: &str) -> &Self {
        assert!(! self.builder.borrow().is_built());

        self.builder.borrow_mut().name(name);

        self
    }

    /// Returns a shared reference to the underlying ticker implementation.
    /// Application builders build `on_fiber` callbacks using the pointer.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let ticker = system.ticker(MyTicker {});
    /// let ptr = ticker.ptr();
    /// ticker.on_fiber(fiber, move |id,args| ptr.borrow().my_call(id, args))
    /// ```
    pub fn ptr(&self) -> Rc<RefCell<Box<E>>> {
        Rc::clone(&self.ptr)
    }

    /// Creates a new fiber builder from this `TickerBuilder`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let fiber = src_ticker.fiber();
    /// let ptr = dst.ticker.ptr();
    /// dst_ticker.on_fiber(fiber, |(id, args)| ptr.borrow().call(id, args); );
    /// ```
    pub fn fiber(&mut self) -> FiberBuilder<T> {
        assert!(! self.builder.borrow().is_built());

        FiberBuilderInner::new(&self.builder)
    }

    pub fn on_fiber(
        &self, 
        fiber: &mut FiberBuilder<T>, 
        on_fiber: impl Fn(usize, T) + 'static
    ) -> &Self {
        assert!(! self.builder.borrow().is_built());

        let to_ticker_id = self.builder.borrow().id;
        let to = fiber.to_ticker(to_ticker_id);

        let on_fiber_id = self.add_fiber(Box::new(on_fiber));

        fiber.on_fiber(to_ticker_id, on_fiber_id);
        /*
        let mut fiber_inner = fiber.builder.borrow_mut();
        let mut from_ticker = fiber_inner.from_ticker.borrow_mut();
        */


        //let from_ticker = fiber.to(to_ticker, on_fiber_id);
        self.builder.borrow_mut().add_from(&to);

        self
    }

    fn add_fiber(&self, on_fiber: Box<OnFiber<T>>) -> usize {
        self.builder.borrow_mut().on_fiber(on_fiber)
    }
}

impl<T:Clone, E:Ticker> fmt::Display for TickerBuilder<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TickerBuilder:{}[{}]", 
            self.builder.borrow().id, 
            match &self.builder.borrow().name {
                Some(name) => name,
                None => "",
            }
        )
    }
}

impl<T:Clone +'static> TickerBuilderInner<T> {
    fn new(
        system_ref: &SystemBuilderRef<T>, 
        on_build: Box<OnBuild>,
        on_tick: Box<OnTickFn>,
    )->TickerBuilderRef<T> {
        let mut system = system_ref.borrow_mut();

        let id = system.tickers.len();

        let ticker = Rc::new(RefCell::new(Self {
            id: id,
            name: None,
            on_build: Some(on_build),
            on_tick: Some(on_tick),
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

    fn on_fiber(&mut self, on_fiber: Box<OnFiber<T>>) -> usize {
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
            Some(_) => {},
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

    fn build(&mut self) -> TickerInner<T> {
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
            self.on_tick.take().expect("on_tick already remove"),
            self.on_build.take().expect("on_build already removed"),
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

    /// Register a fiber callback to a target ticker.
    pub fn to<E:Ticker + 'static>(
        &mut self, 
        target: &TickerBuilder<T,E>, 
        on_fiber: impl Fn(usize, T) + 'static,
    ) -> &Self {
        target.on_fiber(self, on_fiber);

        self
    }

    fn from_ticker(&self) -> TickerBuilderRef<T> {
         self.builder.borrow().from_ticker.clone()
    }

    fn to_ticker(&mut self, to_ticker_id: usize) -> ToTicker<T> {
        return self.from_ticker().borrow_mut().to_ticker(to_ticker_id);
    }

    fn on_fiber(&mut self, to_ticker_id: usize, on_fiber: usize) {
        let to_ticker = self.to_ticker(to_ticker_id);

        return self.builder.borrow_mut().to(to_ticker, on_fiber);
    }

    pub fn fiber(&self) -> Fiber<T> {
        self.builder.borrow().fiber()
    }
}

impl<T:Clone + 'static> FiberBuilderInner<T> {
    fn new(ticker_ref: &TickerBuilderRef<T>)->FiberBuilder<T> {
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
            _system: Rc::clone(system_ref),

            fiber_ref: None,
        }));

        system.fibers.push(Rc::clone(&fiber));

        FiberBuilder { builder: fiber }
    }
    
    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
    
    fn to(&mut self, to_ticker: ToTicker<T>, on_fiber: usize) {
        //let to_ticker = self.from_ticker.borrow_mut().to_ticker(to_ticker_id);

        self.to.push((to_ticker, on_fiber));
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

impl Ticker for ExternalTicker {
    fn tick(&mut self, _ticks: u64) {
    }

    fn build(&mut self) {
    }
}
/* 
impl Ticker for Box<dyn Ticker> {
    fn tick(&mut self, ticks: u64) {
        self.as_mut().tick(ticks);
    }

    fn build(&mut self) {
        self.as_mut().build();
    }
}
*/