use std::{cell::RefCell, rc::Rc, fmt, any::type_name, marker::PhantomData, sync::{Mutex, Arc}};

use crate::{fiber::*, ticker::*, system::{TickerSystem, Context, ThreadGroup, STEP_LIMIT}};

type TickerBuilderRef<M,T> = Rc<RefCell<TickerBuilderInner<M,T>>>;
type SourceRef = Rc<RefCell<Box<SourceInner>>>;
type SetFiber<M,T> = dyn FnOnce(&mut T, Fiber<M>);


pub struct SystemBuilder<M:Clone>(Rc<RefCell<SystemBuilderInner<M>>>);


#[derive(Clone)]
pub struct Source<M:Clone> {
    ptr: Rc<RefCell<Box<SourceInner>>>,

    _marker: PhantomData<M>,
}

#[derive(Clone)]
pub struct Sink<M:Clone> {
    ptr: Rc<RefCell<Box<SinkInner>>>,

    _marker: PhantomData<M>,
}

#[derive(Clone)]
pub struct ExternalSource<M:Clone> {
    ptr: Rc<RefCell<Box<SourceInner>>>,
    fiber: Rc<RefCell<Option<Fiber<M>>>>,
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
            // fibers: Vec::new(),
            external_ticker: None,

            frequency: 64.,
            theta: (4., 8.),
        };

        let mut builder = Self(
            Rc::new(RefCell::new(builder_inner))
        );

        let mut ticker = builder.ticker(ExternalTicker {});
        ticker.name("essay::external");

        builder.0.borrow_mut().external_ticker = Some(ticker);

        builder
    }

    pub fn frequency(&mut self, frequency: f64) -> &Self {
        self.0.borrow_mut().frequency(frequency);

        self
    }

    pub fn theta(&mut self, range: (f64, f64)) -> &Self {
        self.0.borrow_mut().theta(range);

        self
    }

    /// Create a ticker node.
    /// 
    /// To receive on_tick events,the ticker must register and on_tick
    /// callback.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// let ticker = system.node(MyNode {});
    /// ticker.on_tick(move |n, ctx| n.tick());
    /// ```
    pub fn node<T:'static>(&mut self, node: T) -> TickerBuilder<M,T> {
        TickerBuilder(self.0.borrow_mut().ticker(node))
    }

    /// Create a ticker node that implements the ``OnTick`` trait.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// let ticker = system.ticker(MyTicker {});
    /// ```
    /// 
    pub fn ticker<T:Ticker + 'static>(&mut self, ticker: T) -> TickerBuilder<M,T> {
        let ptr = self.0.borrow_mut().ticker(ticker);

        ptr.borrow_mut().on_build(Box::new(move |t| t.build()));
        ptr.borrow_mut().on_tick(Box::new(move |t, ctx| t.tick(ctx)));

        let builder = TickerBuilder(ptr);

        builder
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

    pub fn external_source(&mut self) -> ExternalSource<M> {
        let mut ticker = self.external_ticker();

        let fiber_ref: Rc<RefCell<Option<Fiber<M>>>> = Rc::new(RefCell::new(None));
        let ptr = Rc::clone(&fiber_ref);

        let source = ticker.source(move |_, fiber| {
            ptr.borrow_mut().replace(fiber);
        });

        ExternalSource {
            ptr: Rc::clone(&source.ptr),
            fiber: fiber_ref,
        }
    }

    /// Builds the ticker system
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// system.ticker(MyTicker {});
    /// 
    /// let system = system.build();
    /// system.tick();
    /// ```
    pub fn build(&mut self) -> TickerSystem<M> {
        // let builder = self.builder.borrow_mut();
        self.0.borrow_mut().build()
    }

    fn external_ticker(&self) -> TickerBuilder<M,ExternalTicker> {
        match &self.0.borrow().external_ticker {
            Some(ticker) => ticker.clone(),
            None => panic!("external source isn't available")
        }
    }
}

impl<M:Clone> Clone for SystemBuilder<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

//
// # TickerBuilder
//
pub struct TickerBuilder<M:Clone,T>(Rc<RefCell<TickerBuilderInner<M, T>>>);

impl<M:Clone + 'static,T:'static> TickerBuilder<M, T> {
    /// Sets a debugging name for the ticker.
    pub fn name(&mut self, name: &str) -> &Self {
        self.0.borrow_mut().name(name);

        self
    }

    /// Registers an on_tick callback when the ticker ticks
    ///
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::new();
    /// let ticker = system.node(MyStruct {});
    /// ticker.on_tick(move |t, ctx| t.my_tick(ctx.ticks()));
    /// ```
    pub fn on_tick(&mut self, on_tick: impl Fn(&mut T, &mut Context) + 'static) -> &Self {
        self.0.borrow_mut().on_tick(Box::new(on_tick));

        self
    }

    /// Call on_tick only every ``step`` system ticks.
    /// 
    /// step must be a power of 2 and less or equal to ``STEP_LIMIT`` (64).
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// let ticker = system.node(MyNode {});
    /// ticker.on_tick(move |t, ctx| t.tick());
    /// 
    /// ticker.step(4);
    /// ```
    /// 
    /// # Panics
    /// * step not a power of 2
    /// * step greater than ``STEP_LIMIT`` (64)
    /// 
    /// 
    /// 
    pub fn step(&mut self, step: usize) -> &Self {
        self.0.borrow_mut().step(step);

        self
    }

    /// Call on_tick on an offset within the ticker's steps.
    /// 
    /// Offset must fit in the ticker's step.
    /// 
    /// Steps and offsets can coordinate ticker pipelines to minimize
    /// contention for shared resources.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::<i32>::new();
    /// let ticker = system.node(MyNode {});
    /// ticker.on_tick(move |t, ctx| t.tick());
    /// 
    /// ticker.step(4).offset(1);
    /// ```
    /// 
    /// # Panics
    /// * offset doesn't fit in the ticker's step
    /// 
    pub fn offset(&mut self, offset: usize) -> &Self {
        self.0.borrow_mut().offset(offset);

        self
    }

    pub fn is_lazy(&mut self) -> &Self {
        self.0.borrow_mut().is_lazy();

        self
    }

    pub fn theta(&mut self, phase: f32) -> &Self {
        self.0.borrow_mut().theta(phase);

        self
    }

    pub fn read<R>(&self, read: impl FnOnce(&T) -> R) -> R {
        match &self.0.borrow_mut().ticker {
            Some(ticker) => read(ticker),
            None => panic!("called ticker after build")
        }
    }

    pub fn write<R>(&mut self, write: impl FnOnce(&mut T) -> R) -> R {
        match &mut self.0.borrow_mut().ticker {
            Some(ticker) => write(ticker),
            None => panic!("called ticker after build")
        }
    }

    pub fn on_build(&self, on_build: impl Fn(&mut T) + 'static) -> &Self {
        self.0.borrow_mut().on_build(Box::new(on_build));

        self
    }

    /// Creates a new fiber ``Source`` from this `TickerBuilder`.
    /// 
    /// Sources are connected to sinks to create a ``Fiber.``
    /// When the ``Fiber`` is built, the ``set_fiber`` callback assigns it
    /// to the application's ticker node.
    /// 
    /// The ``Fiber`` must only be used within the ticker's active context
    /// because it assumes it's called in the ticker's thread.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system = SystemBuilder::new();
    /// 
    /// let src_ticker = system.node(MySrc{});
    /// let source = src_ticker.source<M>(move |src, fiber| src.fiber = Some(fiber));
    /// 
    /// let dst_ticker = system.node(MyDst{});
    /// let sink = dst_ticker.sink<M>(move |dst, msg| dst.call(msg));
    /// source.to(sink);
    /// 
    /// ```
    pub fn source(
        &mut self,
        set_fiber: impl FnOnce(&mut T, Fiber<M>) + 'static
    ) -> Source<M> {
        //self.0.borrow_mut().source(Box::new(set_fiber))
        self.0.borrow_mut().source(Box::new(set_fiber))
    }

    /// Creates a new fiber message sink to a callback to the ticker.
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
    pub fn sink(
        &self,
        on_msg: impl Fn(&mut T, M) + 'static
    ) -> Sink<M> {
        self.0.borrow_mut().sink(Box::new(on_msg))
    }

    /// Returns a reference to the application ticker node after building.
    /// 
    /// The reference can read and write to the ticker node between
    /// ticks.
    /// 
    /// # Panics
    /// 
    /// * The ticker system must be built to assign the ticker.
    /// * The ticker must not already be taken.
    /// 

    pub fn unwrap(self) -> TickerPtr<M, T> {
        self.0.borrow_mut().take_ticker()
    }
}

impl<M:Clone,T:Ticker> Clone for TickerBuilder<M, T> {
    fn clone(&self) -> Self {
        TickerBuilder(self.0.clone())
    }
}

//
// Fiber Sources and Sinks.
//

impl<M:Clone + 'static> Sink<M> {
}

impl<M:Clone + 'static> Source<M> {
    pub fn to(&mut self, sink: &Sink<M>) {
        self.ptr.borrow_mut().to(&sink.ptr);
    }
}

impl<M:Clone> ExternalSource<M> {
    pub fn source(&self) -> Source<M> {
        Source {
            ptr: Rc::clone(&self.ptr),
            _marker: Default::default(),
        }
    }

    pub fn fiber(&self) -> Fiber<M> {
        self.fiber.borrow_mut().take().expect("fiber is either unbuilt or already taken")
    }
}
//
// # SystemBuilderInner
//

pub(crate) struct SystemBuilderInner<M:Clone> {
    is_built : bool,

    tickers: Vec<Box<dyn TickerOnBuild<M>>>,
    external_ticker: Option<TickerBuilder<M,ExternalTicker>>,

    pub(crate) frequency: f64,
    pub(crate) theta: (f64, f64),
    // fibers: Vec<Rc<RefCell<Box<SourceInner>>>>,
}

//
// # TickerBuilderInner
//

pub(crate) struct TickerBuilderInner<M:Clone,T> {
    id: usize,

    name: Option<String>,

    ticker: Option<Box<T>>,

    on_build: Option<Box<OnBuild<T>>>,

    on_tick: Option<Box<OnTickFn<T>>>,
    pub(crate) step: usize,
    pub(crate) offset: usize,
    pub(crate) is_lazy: bool,
    pub(crate) theta: f32,

    sources: Vec<SourceRef>,
    set_fibers: Vec<Box<SetFiber<M,T>>>,

    on_fibers: Vec<Box<OnFiber<M,T>>>,

    ticker_access: Option<TickerAccess<M,T>>,
    ticker_ptr: Option<TickerPtr<M,T>>,
}

//
// # SourceInner
//

struct SourceInner {
    // name: Option<String>,

    // set_fiber: usize,

    ticker: usize,

    to: Vec::<Rc<RefCell<Box<SinkInner>>>>,
}

struct SinkInner {
    // name: Option<String>,

//    to_ticker: Rc<RefCell<ToTickerInner<M>>>,
    ticker: usize,

    on_fiber: usize,
}

struct BuilderHolder<M:Clone,T>(TickerBuilderRef<M,T>);

trait TickerOnBuild<M> {
    fn build(&self) -> Box<dyn TickerCall<M>>;
    fn set_ptr(&mut self, thread_group: Arc<Mutex<ThreadGroup<M>>>);
}

struct ExternalTicker {
}
//
// # Inner implementations
//

//
// # SystemBuilderInner
//
impl<M:Clone + 'static> SystemBuilderInner<M> {

    fn frequency(&mut self, frequency: f64) {
        assert!(! self.is_built);
        assert!(frequency > 0.);

        self.frequency = frequency;
    }

    fn theta(&mut self, theta_range: (f64, f64)) {
        assert!(! self.is_built);

        let (min, max) = theta_range;

        assert!(min > 0.);
        assert!(max > 0.);
        assert!(min <= max);

        self.theta = (min, max);
    }

    /// Create a new Ticker
    /// 
     fn ticker<T:'static>(
        &mut self, 
        ticker: T,
    ) -> TickerBuilderRef<M,T> {
        assert!(! self.is_built);

        let id = self.tickers.len();
        let name = format!("{}:{}", type_name::<T>(), self.tickers.len());

        let ticker = Box::new(ticker);
        let ticker_inner: TickerBuilderInner<M, T> = TickerBuilderInner {
            ticker: Some(ticker),
            name: Some(name),
            id: id,
            on_build: None,

            on_tick: None,
            step: 0,
            offset: 0,
            is_lazy: false,
            theta: -1.,

            sources: Vec::new(),
            set_fibers: Vec::new(),

            on_fibers: Vec::new(),

            ticker_access: None,
            ticker_ptr: None,
        };

        let ticker_ref = Rc::new(RefCell::new(ticker_inner));

        self.tickers.push(Box::new(BuilderHolder(Rc::clone(&ticker_ref))));

        ticker_ref
    }

    fn build(&mut self) -> TickerSystem<M> {
        assert!(! self.is_built);
        self.is_built = true;

        let mut tickers: Vec<TickerRef<M>> = Vec::new();
        for ticker in &self.tickers {
            tickers.push(ticker.build());
        }

        let spawn_threads = 0;

        let system = TickerSystem::new(tickers, spawn_threads, self);

        for ticker in &mut self.tickers {
            // tickers.push(ticker.build());
            ticker.set_ptr(system.thread_group());
        }


        system
    }
}

impl<M:Clone+'static,T:'static> TickerOnBuild<M> for BuilderHolder<M, T> {
    fn build(&self) -> Box<dyn TickerCall<M>> {
        self.0.borrow_mut().build()
    }

    fn set_ptr(&mut self, group: Arc<Mutex<ThreadGroup<M>>>) {
        self.0.borrow_mut().set_ptr(group);
    }
}

impl<M:Clone, T:Ticker> fmt::Debug for TickerBuilder<M, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TickerBuilder:{}[{}]", 
            self.0.borrow().id, 
            match &self.0.borrow().name {
                Some(name) => name,
                None => "",
            }
        )
    }
}

//
// # TickerBuilderInner
//


impl<M,T:'static> TickerBuilderInner<M,T>
    where M:Clone + 'static
{
    fn name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    fn on_build(&mut self, on_build: Box<OnBuild<T>>) {
        assert!(! self.is_built());

        self.on_build = Some(on_build);
    }

    fn on_tick(&mut self, on_tick: Box<OnTickFn<T>>) {
        assert!(! self.is_built());

        self.on_tick = Some(on_tick);
        self.step = 1;
    }

    fn step(&mut self, step: usize) {
        assert!(! self.is_built());
        assert!(! self.on_tick.is_none(), "step requires an on_tick callback");
        assert!(step.count_ones() == 1, "step must be a power of 2.");
        assert!(step <= STEP_LIMIT, "step must be less than STEP_LIMIT.");
        assert!(! self.is_lazy, "ticker must not have both step and is_lazy.");

        self.step = step;
    }

    fn offset(&mut self, offset: usize) {
        assert!(! self.is_built());
        assert!(offset < self.step, "Offset must fit in the ticker's step.");

        self.offset = offset;
    }

    fn is_lazy(&mut self) {
        assert!(! self.is_built());
        assert!(! self.on_tick.is_none(), "is_lazy requires an on_tick callback.");
        assert!(self.step == 1, "is_lazy must not be used with step.");

        self.is_lazy = true;
        self.step = 0;
    }

    fn theta(&mut self, phase: f32) {
        assert!(! self.is_built());
        assert!(! self.on_tick.is_none(), "is_lazy requires an on_tick callback.");
        assert!(self.step == 1, "theta must not be used with step or is_lazy.");
        assert!(0. <= phase && phase < 1., "theta phase must be between 0.0 and 1.0");

        self.theta = phase;
        self.step = 0;
    }

    pub fn source(
        &mut self,
        set_fiber: Box<SetFiber<M,T>>
    ) -> Source<M> {
        assert!(! self.is_built());

        //let set_fiber_id = self.set_fiber(Box::new(set_fiber));
        self.set_fiber(Box::new(set_fiber));

        //let name = match &self.name {
        //    Some(name) => Some(String::from(name)),
        //    None => None,
        //};

        let inner = SourceInner {
            //name: name,
            //set_fiber: set_fiber_id,
            ticker: self.id,
            
            to: Vec::new(),
        };

        let source = Rc::new(RefCell::new(Box::new(inner)));

        self.sources.push(Rc::clone(&source));
        // self.system.borrow_mut().fiber(&source);

        Source {
            ptr: source,
            _marker: Default::default(),
        }
    }

    pub fn sink(
        &mut self,
        on_msg: Box<OnFiber<M,T>>,
    ) -> Sink<M> {
        assert!(! self.is_built());

        let on_fiber_id = self.on_fiber(on_msg);

        //let name = match &self.name {
        //    Some(name) => Some(String::from(name)),
        //    None => None,
        //};

        let inner = SinkInner {
            //name: name,
            on_fiber: on_fiber_id,
            ticker: self.id,
        };

        let sink = Rc::new(RefCell::new(Box::new(inner)));

        Sink {
            ptr: sink,
            _marker: Default::default(),
        }
    }

    fn set_fiber(&mut self, set_fiber: Box<dyn FnOnce(&mut T, Fiber<M>)>) -> usize {
        assert!(! self.is_built());

        let set_fiber_id = self.set_fibers.len();

        self.set_fibers.push(set_fiber);

        set_fiber_id
    }

    fn on_fiber(&mut self, on_fiber: Box<OnFiber<M,T>>) -> usize {
        assert!(! self.is_built());

        let on_fiber_id = self.on_fibers.len();

        self.on_fibers.push(on_fiber);

        on_fiber_id
    }

    fn take_ticker(&mut self) -> TickerPtr<M, T> {
        self.ticker_ptr.take().expect("ticker either unbuilt or already taken")
    }

    fn set_ptr(&mut self, threads: Arc<Mutex<ThreadGroup<M>>>) {
        self.ticker_ptr = Some(TickerPtr { 
            ticker: self.ticker_access.take().expect("ticker outer unassigned"),
            threads,
        });
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

        let mut ticker = self.ticker.take().expect("ticker already built");

        let mut fibers = TickerFibers::<M>::new();

        for (source, set_fiber) in self.sources.drain(..).zip(self.set_fibers.drain(..)) {
            let fiber = source.borrow_mut().build(&mut fibers);

            set_fiber(&mut ticker, fiber);
        }

        let on_fibers = self.on_fibers.drain(..).collect();
        
        let inner_ptr = TickerInner::new(
            self.id,
            name.clone(),
            ticker, 
            self.on_tick.take(),
            self.on_build.take(),
            fibers,
            on_fibers,
        );

        self.ticker_access = Some(TickerAccess::new(self.id, Rc::clone(&inner_ptr)));

        TickerOuter::new(
            self.id, 
            name, 
            inner_ptr,
            self
        )
    }
}

impl SourceInner {
    fn to(&mut self, sink: &Rc<RefCell<Box<SinkInner>>>) {
        self.to.push(Rc::clone(&sink));
    }

    fn build<M:Clone + 'static>(&mut self, fibers: &mut TickerFibers<M>) -> Fiber<M> {
        let mut targets = Vec::<(usize, usize, usize)>::new();

        for to in self.to.drain(..) {
            // let to_ticker = to.borrow().to_ticker.clone();
            //let to_ticker = system.to_ticker(self.from_ticker, to.borrow().to_ticker);
            let source_ticker = self.ticker;
            let sink_ticker = to.borrow().ticker;
            let on_fiber = to.borrow().on_fiber;

            targets.push((
                source_ticker,
                sink_ticker,
                on_fiber
            ));
        }

        fibers.new_fiber(&mut targets)
    }
}

impl SinkInner {
}

impl Ticker for ExternalTicker {
    fn tick(&mut self, _ticks: &mut Context) {
    }

    fn build(&mut self) {
    }
}
