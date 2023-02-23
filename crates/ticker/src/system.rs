/// Thread grouping of tickers
/// 
/// 

use std::{sync::{Arc, RwLock, Mutex, RwLockWriteGuard}, fmt, cell::{RefCell, RefMut, Ref}, ops::Deref, rc::Rc, borrow::BorrowMut};

use crate::{ticker::{TickerCall, TickerRef, TickerOuter}, fiber::{SystemChannels, ThreadChannels}, SystemBuilder, builder::SystemBuilderInner};

type ThreadRef<T> = Arc<RwLock<ThreadInner<T>>>;

pub const STEP_LIMIT: usize = 64;

thread_local!(static TICKS: RefCell<u64> = RefCell::new(0));

pub struct TickerSystem<M> {
    ticks: u64,
    theta: f64,
    theta_step: f64,

    thread_group: Arc<Mutex<ThreadGroup<M>>>,
    /*
    threads: Vec<TickerThread<M>>,
    ticker_assignment: TickerAssignment,
    */

    channels: SystemChannels<M>,
}

pub struct ThreadGroup<M> {
    threads: Vec<TickerThread<M>>,
    ticker_assignment: TickerAssignment,
}

impl<M:Clone + 'static> ThreadGroup<M> {
    pub fn read<T:'static,R>(&self, ticker: &TickerOuter<M,T>, fun: impl FnOnce(&T)->R) -> R {
        let ticker_id = ticker.id;
        let thread_id = self.ticker_assignment.ticker_to_thread.lock().unwrap()[ticker_id];

        self.threads[thread_id].0.read().unwrap().read(&ticker, fun)
    }

    pub fn write<T:'static,R>(&self, ticker: &TickerOuter<M,T>, fun: impl FnOnce(&mut T)->R) -> R {
        let ticker_id = ticker.id;
        let thread_id = self.ticker_assignment.ticker_to_thread.lock().unwrap()[ticker_id];

        self.threads[thread_id].0.write().unwrap().write(&ticker, fun)
    }
}

pub struct Context {
    ticks: u64,
    theta: f64,
}

#[derive(Clone)]
struct TickerThread<T>(Arc<RwLock<ThreadInner<T>>>);

pub struct ThreadInner<M> {
    id: usize,
    name: String,

    channels: ThreadChannels<M>,

    tickers: Vec<Option<TickerRef<M>>>,

    ticker_assignment: TickerAssignment,

    context: Context,

    on_ticks: Vec<usize>,
    step_ticks: [Vec<usize>; STEP_LIMIT],
    theta_ticks: Vec<ThetaTicker>,
}

pub(crate) struct TickerAssignment {
    ticker_to_thread: Arc<Mutex<Vec<usize>>>,
}

// 
// Implementation
// 
const EXTERNAL_ID: usize = 0;
const MAIN_ID: usize = 1;

impl<M:Clone + 'static> TickerSystem<M> {
    /*
    pub(crate) fn new_thread_group(tickers: &Vec<TickerRef<M>>) -> Arc<Mutex<ThreadGroup<M>>> {

        Arc::new(Mutex::new(group))
    }
     */

    pub(crate) fn new(
        mut tickers: Vec<TickerRef<M>>,
        spawn_threads: u32,
        builder: &SystemBuilderInner<M>,
    ) -> Self {
        assert!(spawn_threads <= 1);

        let group: ThreadGroup<M> = ThreadGroup {
            threads: Vec::new(),
            ticker_assignment: TickerAssignment::new(&tickers),
        };

        let frequency: f64 = builder.frequency;
        let theta_frequency: f64 = builder.theta.1;
        let theta_step = theta_frequency / frequency;

        let thread_group = Arc::new(Mutex::new(group));

        let mut system = Self {
            ticks: 0,
            theta: 0.,
            theta_step: theta_step,
            thread_group: thread_group,
            channels: SystemChannels::new(),
        };

        let n_tickers = tickers.len();

        system.init_threads(spawn_threads, n_tickers);

        system.assign_ticker(EXTERNAL_ID, tickers.remove(0));

        for ticker in tickers.drain(..) {
            system.assign_ticker(MAIN_ID, ticker);
        }

        system.update_tickers();

        system.on_build();

        system
    }

    fn init_threads(&mut self, spawn_threads: u32, n_tickers: usize) {
        ThreadInner::new(self, "external", n_tickers);
        ThreadInner::new(self, "main", n_tickers);

        for _ in 0..spawn_threads {
            let name = &format!("thread-{}", self.thread_group.lock().unwrap().threads.len());
            ThreadInner::new(self, name, n_tickers);
        }

        for thread in &mut self.thread_group.lock().unwrap().threads {
            thread.fill_channels(&self.channels)
        }
    }

    fn update_tickers(&mut self) {
        for thread in &mut self.thread_group.lock().unwrap().threads {
            thread.update_tickers();
        }
    }

    fn assign_ticker(&mut self, thread_id: usize, ticker: TickerRef<M>) {
        let ticker_id = ticker.id();
        self.thread_group.lock().unwrap().ticker_assignment.set(ticker_id, thread_id);

        self.thread_group.lock().unwrap().threads[thread_id].assign_ticker(ticker);
    }

    pub(crate) fn thread_group(&self) -> Arc<Mutex<ThreadGroup<M>>> {
        Arc::clone(&self.thread_group)
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        self.theta += self.theta_step;

        self.thread_group.lock().unwrap().threads[1].tick(self.ticks, self.theta);
    }

    fn on_build(&mut self) {
        self.thread_group.lock().unwrap().threads[1].on_build();
    }
}

//
// # Context
//

impl Context {
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context").field("ticks", &self.ticks).finish()
    }
}

//
// # TickerThread
//

impl<M:Clone + 'static> TickerThread<M> {
    fn assign_ticker(
        &mut self, 
        ticker: TickerRef<M>
    ) {
        self.0.write().unwrap().assign_ticker(ticker);
    }

    fn fill_channels(
        &mut self, 
        system: &SystemChannels<M>,
    ) {
        self.0.write().unwrap().channels.fill_thread(system);
    }

    fn update_tickers(
        &mut self, 
    ) {
        self.0.write().unwrap().update_tickers();
    }

    fn tick(&mut self, ticks: u64, theta: f64) {
        self.0.write().unwrap().tick(ticks, theta);
    }

    fn on_build(&mut self) {
        self.0.write().unwrap().on_build();
    }
}

//
// # ThreadInner
//

impl<M:Clone+'static> ThreadInner<M> {
    fn new(
        system: &mut TickerSystem<M>, 
        name: &str,
        n_tickers: usize
    ) -> TickerThread<M> {
        let id = system.thread_group.lock().unwrap().threads.len();

        let mut tickers: Vec<Option<TickerRef<M>>> = Vec::new();
        for _ in 0..n_tickers {
            tickers.push(None);
        }

        let channels = if id == 0 {
            system.channels.push_external_thread()
        } else {
            system.channels.push_thread()
        };

        let thread = Self {
            id: id,
            name: String::from(name),

            tickers: tickers,
            ticker_assignment: system.thread_group.lock().unwrap().ticker_assignment.clone(),

            on_ticks: Vec::new(),
            step_ticks: (0..STEP_LIMIT).map(|_| Vec::new())
                .collect::<Vec<Vec<usize>>>()
                .try_into()
                .unwrap(),
            theta_ticks: Vec::new(),

            channels: channels,

            context: Context { ticks: 0, theta: 0. },
        };

        let thread_ref = Arc::new(RwLock::new(thread));

        let ticker = TickerThread(thread_ref);

        system.thread_group.lock().unwrap().threads.push(ticker.clone());

        ticker
    }

    fn assign_ticker(&mut self, ticker: TickerRef<M>) {
        let ticker_id = ticker.id();

        let offset = ticker.offset();
        let step = ticker.step();
        let theta: f32 = ticker.theta();

        if step == 1 {
            self.on_ticks.push(ticker_id);
        } else if step > 1 {
            for i in 0..STEP_LIMIT / step {
                self.step_ticks[i * step + offset].push(ticker_id);
            }
        } else if theta >= 0. {
            self.theta_ticks.push(ThetaTicker {
                id: ticker_id,
                phase: theta as f64,
                next_theta: 0.,
            })
        }

        assert!(self.tickers[ticker_id].is_none());
        self.tickers[ticker_id] = Some(ticker);
    }

    fn update_tickers(&mut self) {
        for ticker in &mut self.tickers {
            match ticker {
                Some(ticker) => {
                    ticker.update(&self.ticker_assignment, &self.channels);
                },
                None => {},
            }
        }

        self.channels.update_tickers(&self.tickers);
    }
 
    fn tick(&mut self, ticks: u64, theta: f64) {
        self.context.ticks = ticks;
        self.context.theta = theta;

        self.receive();

        self.on_ticks();

        self.on_step_ticks();

        self.on_theta_ticks();
    }

    fn on_ticks(&mut self) {
        let ctx = &mut self.context;

        for ticker_id in &self.on_ticks {
            Self::eval_on_tick(&mut self.tickers[*ticker_id], ctx);
        }
    }

    fn on_step_ticks(&mut self) {
        let ctx = &mut self.context;

        let offset = (ctx.ticks % STEP_LIMIT as u64) as usize;

        for ticker_id in &self.step_ticks[offset] {
            Self::eval_on_tick(&mut self.tickers[*ticker_id], ctx);
        }
    }

    fn on_theta_ticks(&mut self) {
        let ctx = &mut self.context;
        let theta = ctx.theta;

        for ticker in &mut self.theta_ticks {
            if ticker.next_theta <= theta {
               let id = ticker.id;

                Self::eval_on_tick(&mut self.tickers[id], ctx);

                ticker.next_theta = ticker.next_theta + 1.;
                ticker.next_theta += ticker.phase - ticker.next_theta % 1.;
            }
        }
    }

    fn eval_on_tick(
        // &self, 
        ticker: &mut Option<Box<dyn TickerCall<M>>>,
        ctx: &mut Context,
    ) {
        match ticker {
            Some(ticker) => {
                (*ticker).tick(ctx);
            }
            None => panic!(
                "{:?} on_tick to ticker not assigned to this thread",
                ctx,
            )
        }
    }

    fn on_build(&mut self) {
        for ticker_opt in &mut self.tickers {
            if let Some(ticker) = ticker_opt {
                ticker.on_build();
            }
        }
    }

    fn receive(&mut self) {
        self.channels.receive(&mut self.tickers);
    }

    fn read<T:'static,R>(&self, ticker: &TickerOuter<M,T>, fun: impl FnOnce(&T)->R) -> R {
        assert!(self.tickers[ticker.id].is_some());

        ticker.read(fun)
    }

    fn write<T:'static,R>(&self, ticker: &TickerOuter<M,T>, fun: impl FnOnce(&mut T)->R) -> R {
        assert!(self.tickers[ticker.id].is_some());

        ticker.write(fun)
    }
}

struct ThetaTicker {
    id: usize,
    phase: f64,
    next_theta: f64,
}

struct ThreadTickers<M> {
    ptr: Rc<RefCell<Vec<Option<Box<dyn TickerCall<M>>>>>>,
}

impl<M> ThreadTickers<M> {
    fn new(n_tickers: usize) -> Self {
        let mut tickers = Vec::<Option<Box<dyn TickerCall<M>>>>::new();

        for _ in 0..n_tickers {
            tickers.push(None);
        }

        Self {
            ptr: Rc::new(RefCell::new(tickers))
        }
    }
}

impl<T> fmt::Display for ThreadInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ThreadInner:{}[{}]", self.id, self.name)
    }
}

//
// # TickerAssignment

impl TickerAssignment {
    fn new<M>(tickers: &Vec<TickerRef<M>>) ->Self {
        let mut ticker_to_thread: Vec<usize> = Vec::new();
        
        ticker_to_thread.resize(tickers.len(), 0);

        Self {
            ticker_to_thread: Arc::new(Mutex::new(ticker_to_thread)),
        }
    }

    pub fn get(&self, ticker_id: usize) -> usize {
        self.ticker_to_thread.lock().unwrap()[ticker_id]
    }

    fn set(&self, ticker_id: usize, thread_id: usize) {
        self.ticker_to_thread.lock().unwrap()[ticker_id] = thread_id;
    }
}

impl Clone for TickerAssignment {
    fn clone(&self) -> Self {
        Self {
            ticker_to_thread: Arc::clone(&self.ticker_to_thread),
        }
    }
}
