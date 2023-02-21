/// Thread grouping of tickers
/// 
/// 

use std::{sync::{Arc, RwLock, Mutex, RwLockWriteGuard}, fmt, cell::{RefCell, RefMut, Ref}, ops::Deref, rc::Rc, borrow::BorrowMut};

use crate::{ticker::{TickerCall, TickerRef}, fiber::{SystemChannels, ThreadChannels}};

type ThreadRef<T> = Arc<RwLock<ThreadInner<T>>>;

thread_local!(static TICKS: RefCell<u64> = RefCell::new(0));

pub struct TickerSystem<M> {
    ticks: u64,
    threads: Vec<TickerThread<M>>,

    channels: SystemChannels<M>,

    ticker_assignment: TickerAssignment,
}

pub struct Context {
    ticks: u64,
}

struct TickerThread<T> {
    ptr: ThreadRef<T>,
}

pub struct ThreadInner<M> {
    id: usize,
    name: String,

    channels: ThreadChannels<M>,

    tickers: Vec<Option<TickerRef<M>>>,

    ticker_assignment: TickerAssignment,

    context: Context,

    on_ticks: Vec<usize>,
}

pub(crate) struct TickerAssignment {
    ticker_to_thread: Arc<Mutex<Vec<usize>>>,
}

// 
// Implementation
// 
const EXTERNAL_ID: usize = 0;
const MAIN_ID: usize = 1;

impl<M:'static> TickerSystem<M> {
    pub(crate) fn new(
        mut tickers: Vec<TickerRef<M>>,
        spawn_threads: u32
    ) -> Self {
        assert!(spawn_threads <= 1);

        let mut system = Self {
            ticks: 0,
            channels: SystemChannels::new(),
            threads: Vec::new(),
            ticker_assignment: TickerAssignment::new(&tickers),
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
            ThreadInner::new(self, &format!("thread-{}", self.threads.len()), n_tickers);
        }

        for thread in &mut self.threads {
            thread.fill_channels(&self.channels)
        }
    }

    fn update_tickers(&mut self) {
        for thread in &mut self.threads {
            thread.update_tickers();
        }
    }

    fn assign_ticker(&mut self, thread_id: usize, ticker: TickerRef<M>) {
        let ticker_id = ticker.id();
        self.ticker_assignment.set(ticker_id, thread_id);

        self.threads[thread_id].assign_ticker(ticker);
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        self.threads[1].tick(self.ticks);
    }

    fn on_build(&mut self) {
        self.threads[1].on_build();
    }
}

impl<M:'static> TickerThread<M> {
    fn assign_ticker(
        &mut self, 
        ticker: TickerRef<M>
    ) {
        self.ptr.write().unwrap().assign_ticker(ticker);
    }

    fn fill_channels(
        &mut self, 
        system: &SystemChannels<M>,
    ) {
        self.ptr.write().unwrap().channels.fill_thread(system);
    }

    fn update_tickers(
        &mut self, 
    ) {
        self.ptr.write().unwrap().update_tickers();
    }

    fn tick(&mut self, ticks: u64) {
        self.ptr.write().unwrap().tick(ticks);
    }

    fn on_build(&mut self) {
        self.ptr.write().unwrap().on_build();
    }

    fn borrow_ticker(&mut self, index: usize) {
        let guard = self.ptr.write().unwrap();
    }
}

impl<M:'static> ThreadInner<M> {
    fn new(
        system: &mut TickerSystem<M>, 
        name: &str,
        n_tickers: usize
    ) -> TickerThread<M> {
        let id = system.threads.len();

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
            ticker_assignment: system.ticker_assignment.clone(),
            on_ticks: Vec::new(),

            channels: channels,

            context: Context { ticks: 0 },
        };

        let thread_ref = Arc::new(RwLock::new(thread));

        let ticker = TickerThread { ptr: thread_ref.clone() };

        system.threads.push(ticker);

        TickerThread { ptr: thread_ref.clone() }
    }

    
    fn is_ticker_owned(&self, id: usize) -> bool {
        self.tickers[id].is_some()
    }
    
    fn assign_ticker(&mut self, ticker: TickerRef<M>) {
        let ticker_id = ticker.id();

        self.on_ticks.push(ticker_id);

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
 
    fn tick(&mut self, ticks: u64) {
        self.set_ticks(ticks);

        self.receive();

        self.on_ticks(ticks);
    }

    fn on_ticks(&mut self, ticks: u64) {
        self.context.ticks += 1;
        for ticker_id in &self.on_ticks {
            match &mut self.tickers[*ticker_id] {
                Some(ticker) => {
                    (*ticker).tick(&mut self.context);
                }
                None => panic!(
                    "{} on_tick to ticker #{} but not assigned",
                    self,
                    ticker_id
                )
            }
        }
    }

    fn set_ticks(&self, ticks: u64) {
        TICKS.with(|f| {
            *f.borrow_mut() = ticks;
        });
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

impl Context {
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}

struct TickerWriteGuard<'a,T:'static> {
    ptr: &'a mut T,
    guard: RwLockWriteGuard<'a,ThreadInner<T>>,
}

impl<'a,T> Deref for TickerWriteGuard<'a,T> {
    type Target = &'a mut T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}