use std::{cell::RefCell, rc::Rc, fmt, ops::Deref, any::type_name, marker::PhantomData};

use crate::{fiber::*, ticker::*, system::{TickerSystem}};

type SystemBuilderRef<M> = Rc<RefCell<SystemBuilderInner<M>>>;
type TickerBuilderRef<M,T> = Rc<RefCell<TickerBuilderInner<M,T>>>;
//type FiberBuilderRef<M> = Rc<RefCell<SourceInner>>;
type SourceRef = Rc<RefCell<Box<SourceInner>>>;
type SetFiber<M,T> = dyn Fn(&mut T, Fiber<M>);

pub struct SystemBuilder<T:Clone> {
    ptr: SystemBuilderRef<T>,
}

pub struct TickerBuilder<M:Clone,T:Ticker> {
    ptr: TickerBuilderRef<M,T>,
}

#[derive(Clone)]
pub struct NodeBuilder<M:Clone,T> {
    ptr: TickerBuilderRef<M,T>,
}

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

        ptr.borrow_mut().on_build(Box::new(move |t| t.build()));
        ptr.borrow_mut().on_tick(Box::new(move |t, ticks| t.tick(ticks)));

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

    pub fn external_source(&mut self) -> ExternalSource<M> {
        let mut ticker = self.external_ticker();

        let fiber_ref: Rc<RefCell<Option<Fiber<M>>>> = Rc::new(RefCell::new(None));
        let ptr = Rc::clone(&fiber_ref);

        let source = ticker.source(move |t, fiber| {
            ptr.borrow_mut().replace(fiber);
        });

        ExternalSource {
            ptr: Rc::clone(&source.ptr),
            fiber: fiber_ref,
        }
    }

    fn external_ticker(&self) -> TickerBuilder<M,ExternalTicker> {
        match &self.ptr.borrow().external_ticker {
            Some(ticker) => ticker.clone(),
            None => panic!("external source isn't available")
        }
    }

        //let system = self.builder.borrow();

        //assert!(! system.is_built);

        //let ticker = Rc::clone(&system.tickers[0]);

        // todo!()
        //FiberBuilderInner::new(&self.external_ticker())
    //}
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
    pub fn source(
        &mut self,
        set_fiber: impl Fn(&mut T, Fiber<M>) + 'static
    ) -> Source<M> {
        self.ptr.borrow_mut().source(Box::new(set_fiber))
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
        self.ptr.borrow_mut().sink(Box::new(on_msg))
    }
}

impl<M:Clone,T:Ticker> Clone for TickerBuilder<M, T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

//
// FiberBuilder
//

impl<M:Clone + 'static> Sink<M> {
    fn dummy(msg:M) {}
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
// # inner structures
//

pub struct SystemBuilderInner<M:Clone> {
    is_built : bool,

    tickers: Vec<Box<dyn TickerOnBuild<M>>>,
    external_ticker: Option<TickerBuilder<M,ExternalTicker>>,

    // fibers: Vec<Rc<RefCell<Box<SourceInner>>>>,
}

struct TickerBuilderInner<M:Clone,T> {
    id: usize,

    name: Option<String>,

    ticker: Option<Box<T>>,

    on_build: Option<Box<OnBuild<T>>>,
    on_tick: Option<Box<OnTickFn<T>>>,

    sources: Vec<SourceRef>,
    set_fibers: Vec<Box<SetFiber<M,T>>>,

    on_fibers: Vec<Box<OnFiber<M,T>>>,

    //to_tickers: Vec<ToTicker<M>>,
    //from_tickers: Vec<usize>,
    // to_tickers: ToTickerBuilder<M>,
    //fibers: TickerFibers<M, T>,

    system: SystemBuilderRef<M>,
}

struct SourceInner {
    name: Option<String>,

    set_fiber: usize,

    ticker: usize,

    to: Vec::<Rc<RefCell<Box<SinkInner>>>>,
}

struct SinkInner {
    name: Option<String>,

//    to_ticker: Rc<RefCell<ToTickerInner<M>>>,
    ticker: usize,

    on_fiber: usize,
}

struct BuilderHolder<M:Clone,T> {
    ptr: TickerBuilderRef<M,T>,
}

trait TickerOnBuild<M> {
    fn build(&self) -> Box<dyn TickerCall<M>>;
}

struct ExternalTicker {
}
/*
#[derive(Clone)]
struct ToTickerBuilder<M> {
    ptr: Rc<RefCell<ToTickerInnerBuilder<M>>>,
}

struct ToTickerInnerBuilder<M> {
    ticker_id: usize,

    to_tickers: Vec<ToTicker<M>>,
    from_tickers: Vec<ToTicker<M>>,
}
*/
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

        let id = self.tickers.len();
        let name = format!("{}:{}", type_name::<T>(), self.tickers.len());

        let ticker = Box::new(ticker);
        let ticker_inner: TickerBuilderInner<M, T> = TickerBuilderInner {
            ticker: Some(ticker),
            name: Some(name),
            id: id,
            on_tick: None,
            on_build: None,

            sources: Vec::new(),
            set_fibers: Vec::new(),

            on_fibers: Vec::new(),

            system: Rc::clone(system_ref),
        };

        let ticker_ref = Rc::new(RefCell::new(ticker_inner));

        self.tickers.push(Box::new(BuilderHolder {
            ptr: Rc::clone(&ticker_ref)
        }));

        ticker_ref
    }

    /*
    fn fiber(&mut self, source: &Rc<RefCell<Box<SourceInner>>>) {
        self.fibers.push(Rc::clone(source));
    }
     */

    fn external_source(&mut self) -> ExternalSource<M> {
        match &mut self.external_ticker {
            Some(ticker) => {
                let fiber_ref: Rc<RefCell<Option<Fiber<M>>>> = Rc::new(RefCell::new(None));
                let ptr = Rc::clone(&fiber_ref);

                let source = ticker.source(move |t, fiber| {
                    ptr.borrow_mut().replace(fiber);
                });

                ExternalSource {
                    ptr: Rc::clone(&source.ptr),
                    fiber: fiber_ref,
                }

            },
            None => panic!("external source isn't available")
        }
    }

    fn build(&mut self) -> TickerSystem<M> {
        assert!(! self.is_built);
        self.is_built = true;

        /*
        for fiber in self.fibers.drain(..) {
            fiber.borrow_mut().build::<M>();
        }
         */

        let mut tickers: Vec<Box<dyn TickerCall<M>>> = Vec::new();
        for build in self.tickers.drain(..) {
            tickers.push(build.build());
        }

        let spawn_threads = 0;

        let system = TickerSystem::new(tickers, spawn_threads);

        system
    }
}

impl<M:Clone+'static,T:'static> TickerOnBuild<M> for BuilderHolder<M, T> {
    fn build(&self) -> Box<dyn TickerCall<M>> {
        self.ptr.borrow_mut().build()
    }
}

impl<M:Clone, T:Ticker> fmt::Debug for TickerBuilder<M, T> {
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


impl<M:Clone +'static,T:'static> TickerBuilderInner<M,T> {
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
    }

    pub fn source(
        &mut self,
        set_fiber: Box<SetFiber<M,T>>
    ) -> Source<M> {
        assert!(! self.is_built());

        let set_fiber_id = self.set_fiber(Box::new(set_fiber));

        let name = match &self.name {
            Some(name) => Some(String::from(name)),
            None => None,
        };

        let inner = SourceInner {
            name: name,
            set_fiber: set_fiber_id,
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

        let name = match &self.name {
            Some(name) => Some(String::from(name)),
            None => None,
        };

        let inner = SinkInner {
            name: name,
            on_fiber: on_fiber_id,
            ticker: self.id,
        };

        let sink = Rc::new(RefCell::new(Box::new(inner)));

        Sink {
            ptr: sink,
            _marker: Default::default(),
        }
    }

    fn set_fiber(&mut self, set_fiber: Box<dyn Fn(&mut T, Fiber<M>)>) -> usize {
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

        let mut fibers = TickerFibers::<M,T>::new();

        for (source, set_fiber) in self.sources.drain(..).zip(&self.set_fibers) {
            let fiber = source.borrow_mut().build(&mut fibers);

            set_fiber(&mut ticker, fiber);
        }

        let on_fibers = self.on_fibers.drain(..).collect();
        // let to_tickers = self.to_tickers.drain(..).collect();
        // let from_tickers = self.from_tickers.drain(..).collect();

        Box::new(TickerInner {
            id: self.id,
            name: name,
            ticker: ticker,
            //to_tickers: to_tickers,
            //from_tickers: from_tickers,
            on_tick: self.on_tick.take(),
            on_build: self.on_build.take(),
            on_fiber: on_fibers,
        })
    }
}
/*
impl<M:'static> ToTickerInnerBuilder<M> {
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
                );
                self.to_tickers.push(to_ticker.clone());

                to_ticker
            }
        }
    }
}
 */
impl SourceInner {
    fn to(&mut self, sink: &Rc<RefCell<Box<SinkInner>>>) {
        self.to.push(Rc::clone(&sink));
    }

    fn build<M:Clone + 'static,T>(&mut self, fibers: &mut TickerFibers<M,T>) -> Fiber<M> {
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
    fn tick(&mut self, _ticks: u64) {
    }

    fn build(&mut self) {
    }
}
