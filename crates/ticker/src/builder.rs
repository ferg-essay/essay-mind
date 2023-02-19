use std::{cell::RefCell, rc::Rc, fmt, ops::Deref};

use crate::{fiber::*, ticker::*, system::{TickerSystem, PanicToThread}};

type SystemBuilderRef<M> = Rc<RefCell<SystemBuilderInner<M>>>;
type TickerBuilderRef<M,T> = Rc<RefCell<TickerBuilderInner<M,T>>>;
type FiberBuilderRef<M> = Rc<RefCell<FiberBuilderInner<M>>>;


pub struct SystemBuilder<T:Clone> {
    ptr: SystemBuilderRef<T>,
}

#[derive(Clone)]
pub struct TickerBuilder<M:Clone,T:Ticker> {
    ptr: TickerBuilderRef<M,T>,
}

#[derive(Clone)]
pub struct NodeBuilder<M:Clone,T> {
    ptr: TickerBuilderRef<M,T>,
}

pub struct FiberBuilder<T:Clone> {
    builder: FiberBuilderRef<T>,
}

pub struct OnFiberBuilder<M:Clone> {
    builder: FiberBuilderRef<M>,
}

//
// # inner structures
//

pub struct SystemBuilderInner<M:Clone> {
    is_built : bool,

    tickers: Vec<Box<dyn TickerOnBuild<M>>>,
    external_ticker: Option<TickerBuilder<M,ExternalTicker>>,
    fibers: Vec<FiberBuilderRef<M>>,
}

struct TickerBuilderInner<M:Clone,T> {
    id: usize,

    name: Option<String>,

    ticker: Option<Box<T>>,

    on_build: Option<Box<OnBuild<T>>>,
    on_tick: Option<Box<OnTickFn<T>>>,
    on_fibers: Vec<Box<OnFiber<M,T>>>,

    to_tickers: Rc<RefCell<ToTickerInner<M>>>,

    system: SystemBuilderRef<M>,
}

struct FiberBuilderInner<M:Clone>
{
    id: usize,
    name: Option<String>,

    from_tickers: Vec::<(Rc<RefCell<ToTickerInner<M>>>,FiberRef<M>)>,

    to: Vec<(Rc<RefCell<ToTickerInner<M>>>,usize)>,

    _system: SystemBuilderRef<M>,
}

struct ExternalTicker {
}

//
// # SystemBuilder
//

impl<M:Clone + 'static> SystemBuilder<M> {
    /// Create a new ticker `SystemBuilder`
    /// 
    /// ```ignore
    /// let builder = SystemBuilder::<u32>::new();
    /// let system = builder.build();
    /// ```
    /// 
    pub fn new() -> SystemBuilder<M> {
        let builder_inner: SystemBuilderInner<M> = SystemBuilderInner {
            is_built: false,
            tickers: Vec::new(),
            fibers: Vec::new(),
            external_ticker: None,
        };

        let mut builder = Self {
            ptr: Rc::new(RefCell::new(builder_inner)),
        };

        let ticker = builder.ticker(ExternalTicker {});
        ticker.name("essay::external");

        builder.ptr.borrow_mut().external_ticker = Some(ticker);

        builder
    }

    /// Create a new Ticker
    /// 
    pub fn ticker<T:Ticker + 'static>(&mut self, ticker: T) -> TickerBuilder<M,T> {
        let ptr = self.ptr.borrow_mut().ticker(ticker, &self.ptr);

        let builder = TickerBuilder {
            ptr: ptr,
        };

        builder
    }

    /// Create a non-ticking node
    /// 
    pub fn node<T:'static>(&mut self, node: T) -> NodeBuilder<M,T> {
        NodeBuilder {
            ptr: self.ptr.borrow_mut().ticker(node, &self.ptr)
        }
    }

    /// Create a fiber from an external source. External code must send
    /// messages to a ticker using an external fiber.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let system = SystemBuilder::<i32>::new();
    /// let external = system.external_fiber();
    /// 
    /// let system = system.build();
    /// let external = external.fiber();
    /// 
    /// external.send((23, ));
    /// ```
    /// 
    pub fn external_fiber(&mut self) -> OnFiberBuilder<M> {
        //let system = self.builder.borrow();

        //assert!(! system.is_built);

        //let ticker = Rc::clone(&system.tickers[0]);

        todo!()
        //FiberBuilderInner::new(&self.external_ticker())
    }
/* TODO
    fn external_ticker(&self) -> TickerBuilderRef<M> {
        let system = self.builder.borrow();

        assert!(! system.is_built);

        // Rc::clone(&system.tickers[0])
        todo!();
    }
     */

    pub fn build(&mut self) -> TickerSystem<M> {
        // let builder = self.builder.borrow_mut();
        self.ptr.borrow_mut().build()
    }
}

//
// # TickerBuilder
//

impl<M:Clone + 'static,T:Ticker + 'static> TickerBuilder<M, T> {
    /// Sets a debugging name for the ticker.
    pub fn name(&self, name: &str) -> &Self {
        assert!(! self.ptr.borrow().is_built());

        self.ptr.borrow_mut().name(name);

        self
    }

    /// Creates a new fiber source from this `TickerBuilder`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::new();
    /// 
    /// let src_ticker = system.ticker(MySrc{});
    /// let source = src_ticker.source<M>(move |t, fiber| t.fiber = Some(fiber));
    /// 
    /// let dst_ticker = system.ticker(MyDst{});
    /// let sink = dst_ticker.sink<M>(move |t, msg| t.call(msg));
    /// source.to(sink);
    /// 
    /// ```
    pub fn fiber(&mut self) -> (Fiber<M>, OnFiberBuilder<M>) {
        assert!(! self.ptr.borrow().is_built());

        FiberBuilderInner::new_fiber(
            &self.ptr.borrow().to_tickers,
            &self.ptr.borrow().system,
        )
    }

    pub fn on_fiber(
        &self,
        on_fiber: impl Fn(&mut T, M) + 'static
    ) -> FiberBuilder<M> {
        assert!(! self.ptr.borrow().is_built());

        let on_fiber_id = self.ptr.borrow_mut().on_fiber(Box::new(on_fiber));
    
        FiberBuilderInner::new_on_fiber(
            &self.ptr.borrow().to_tickers, 
            on_fiber_id,
            &self.ptr.borrow().system,
        )
    }
}

//
// FiberBuilder
//

impl<M:Clone + 'static> OnFiberBuilder<M> {
    pub fn on_fiber<T:Ticker+'static>(
        &mut self, 
        ticker: &TickerBuilder<M,T>,
        on_fiber: impl Fn(&mut T, M) + 'static
    ) {
        self.builder.borrow_mut().on_fiber(
            &ticker.ptr.borrow().to_tickers, 
            ticker.ptr.borrow_mut().on_fiber(Box::new(on_fiber)),
        );
    }
}

impl<M:Clone + 'static> FiberBuilder<M> {
    pub fn fiber<T:Ticker>(&mut self, ticker: &TickerBuilder<M,T>) -> Fiber<M> {
        self.builder.borrow_mut().fiber(
            &ticker.ptr.borrow().to_tickers,
        )
    }
}

//
// # Inner implementations
//

impl<M:Clone + 'static> SystemBuilderInner<M> {

    /// Create a new Ticker
    /// 
     fn ticker<T:'static>(
        &mut self, 
        ticker: T,
        system_ref: &SystemBuilderRef<M>,
    ) -> TickerBuilderRef<M,T> {
        assert!(! self.is_built);

        let ticker = Box::new(ticker);

        let id = self.tickers.len();

        let ticker_inner: TickerBuilderInner<M, T> = TickerBuilderInner {
            ticker: Some(ticker),
            name: Some(format!("Ticker-{}", id)),
            id: id,
            on_tick: None,
            on_build: None,
            on_fibers: Vec::new(),
            to_tickers: Rc::new(RefCell::new(ToTickerInner {
                ticker_id: id,
                to_tickers: Vec::new(),
                from_tickers: Vec::new(),
            })),
            system: Rc::clone(system_ref),
        };

        let ticker_ref = Rc::new(RefCell::new(ticker_inner));

        self.tickers.push(Box::new(BuilderHolder {
            ptr: Rc::clone(&ticker_ref)
        }));

        ticker_ref
    }

    fn build(&mut self) -> TickerSystem<M> {
        assert!(! self.is_built);
        self.is_built = true;

        let mut tickers: Vec<Box<dyn TickerCall<M>>> = Vec::new();
        for build in self.tickers.drain(..) {
            tickers.push(build.build());
        }

        for fiber in self.fibers.drain(..) {
            fiber.borrow_mut().build();
        }

        let spawn_threads = 0;

        TickerSystem::new(tickers, spawn_threads)
    }
}

struct BuilderHolder<M:Clone,T> {
    ptr: TickerBuilderRef<M,T>,
}

trait TickerOnBuild<M> {
    fn build(&self) -> Box<dyn TickerCall<M>>;
}

impl<M:Clone+'static,T:'static> TickerOnBuild<M> for BuilderHolder<M, T> {
    fn build(&self) -> Box<dyn TickerCall<M>> {
       self.ptr.borrow_mut().build()
    }
}

impl<T:Clone, E:Ticker> fmt::Debug for TickerBuilder<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TickerBuilder:{}[{}]", 
            self.ptr.borrow().id, 
            match &self.ptr.borrow().name {
                Some(name) => name,
                None => "",
            }
        )
    }
}

struct ToTickerInner<M> {
    ticker_id: usize,

    to_tickers: Vec<ToTicker<M>>,
    from_tickers: Vec<ToTicker<M>>,
}

impl<M:Clone +'static,T:'static> TickerBuilderInner<M,T> {
    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    fn on_fiber(&mut self, on_fiber: Box<OnFiber<M,T>>) -> usize {
        assert!(! self.is_built());

        let on_fiber_id = self.on_fibers.len();

        self.on_fibers.push(on_fiber);

        on_fiber_id
    }
    
    fn is_built(&self) -> bool {
        self.ticker.is_none()
    }

    fn build(&mut self) -> Box<dyn TickerCall<M>> {
        // assert!(match self.ticker_ref { None=>true, _=> false });

        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("ticker-{}", self.id),
        };

        let on_fibers = self.on_fibers.drain(..).collect();
        let to_tickers = self.to_tickers.borrow_mut().to_tickers.drain(..).collect();
        let from_tickers = self.to_tickers.borrow_mut().from_tickers.drain(..).collect();

        let ticker = TickerInner::new(
            self.ticker.take().expect("ticker already built"),
            self.id, 
            name,
            to_tickers,
            from_tickers,    
            self.on_tick.take(),
            self.on_build.take(),
            on_fibers
        );

        Box::new(ticker)
    }
}

impl<M:'static> ToTickerInner<M> {
    fn add_from(&mut self, from_ticker: &ToTicker<M>) {
        match self.from_tickers.iter().filter(|from_ticker|
            from_ticker.from_ticker == self.ticker_id
        ).next() {
            Some(_) => {},
            None => { self.from_tickers.push(from_ticker.clone()); }
        }
    }

    fn to_ticker(&mut self, to_ticker_id: usize) -> ToTicker<M> {
        match self.to_tickers.iter().filter(|to_ticker| to_ticker.to_ticker == to_ticker_id).next() {
            Some(to_ticker) => to_ticker.clone(),
            None => {
                let to_ticker = ToTicker::new(
                    self.ticker_id, 
                    to_ticker_id, 
                    &PanicToThread::new(
                    &format!("{}->{}", self.ticker_id, to_ticker_id)
                    ),
                );
                self.to_tickers.push(to_ticker.clone());

                to_ticker
            }
        }
    }
}

impl<M:Clone + 'static> FiberBuilderInner<M> {
    fn new_fiber(
        to_tickers: &Rc<RefCell<ToTickerInner<M>>>,
        system_ref: &SystemBuilderRef<M>,
    )->(Fiber<M>, OnFiberBuilder<M>) {
        let mut system = system_ref.borrow_mut();

        assert!(! system.is_built);

        let id = system.fibers.len();

        let mut builder = Self {
            id: id,
            name: None,
            from_tickers: Vec::new(),
            to: Vec::new(),
            _system: Rc::clone(system_ref),
        };

        let fiber = builder.fiber(&to_tickers);

        let builder_ptr = Rc::new(RefCell::new(builder));

        system.fibers.push(Rc::clone(&builder_ptr));


        (fiber, OnFiberBuilder { builder: builder_ptr })
    }

    fn new_on_fiber(
        to_tickers: &Rc<RefCell<ToTickerInner<M>>>,
        on_fiber: usize,
        system_ref: &SystemBuilderRef<M>,
    ) -> FiberBuilder<M> {
        let mut system = system_ref.borrow_mut();

        assert!(! system.is_built);

        let id = system.fibers.len();

        let builder = Self {
            id: id,
            name: None,
            from_tickers: Vec::new(),
            to: Vec::new(),
            _system: Rc::clone(system_ref),
        };

        let builder_ptr = Rc::new(RefCell::new(builder));

        system.fibers.push(Rc::clone(&builder_ptr));

        FiberBuilder { builder: builder_ptr }
    }

    fn fiber(&mut self, to_tickers: &Rc<RefCell<ToTickerInner<M>>>) -> Fiber<M> {
        let fiber_ref = Fiber::new();

        self.from_tickers.push((Rc::clone(to_tickers), Rc::clone(&fiber_ref)));

        Fiber {
            fiber_ref: fiber_ref,
        }
    }
    
    fn on_fiber(&mut self, to_ticker: &Rc<RefCell<ToTickerInner<M>>>, on_fiber: usize) {
        //let to_ticker = self.from_ticker.borrow_mut().to_ticker(to_ticker_id);

        self.to.push((Rc::clone(to_ticker), on_fiber));
    }

    fn build(&mut self) {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => format!("fiber-{}", self.id),
        };

        todo!()
        //let fiber = Fiber::new(self.id, name, self.to.drain(..).collect());

        //self.fiber_ref = Some(fiber);
    }
}

impl Ticker for ExternalTicker {
    fn tick(&mut self, _ticks: u64) {
    }

    fn build(&mut self) {
    }
}
