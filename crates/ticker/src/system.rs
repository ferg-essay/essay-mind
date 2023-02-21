/// Thread grouping of tickers
/// 
/// 

use std::{rc::Rc, sync::{Arc, RwLock, Mutex}, fmt, cell::RefCell};
use std::sync::mpsc;

use crate::{ticker::{TickerInner, TickerCall}, fiber::{ToThread, SystemChannels, ThreadChannels, ToThreadRef}};
//extern crate env_logger;
//use env_logger::Env;
//use log::{info};

type ThreadRef<T> = Arc<RwLock<ThreadInner<T>>>;


thread_local!(static TICKS: RefCell<u64> = RefCell::new(0));

pub struct TickerSystem<M> {
    ticks: u64,
    threads: Vec<TickerThread<M>>,

    channels: SystemChannels<M>,

    ticker_assignment: TickerAssignment,
}

struct TickerThread<T> {
    id: usize,

    ptr: ThreadRef<T>,
}

pub struct ThreadInner<M> {
    id: usize,
    name: String,

    //receiver: mpsc::Receiver<Message<M>>,
    //sender: mpsc::Sender<Message<M>>,

    //to: Vec<ToThreadRef<M>>,

    channels: ThreadChannels<M>,

    tickers: Vec<Option<Box<dyn TickerCall<M>>>>,

    ticker_assignment: TickerAssignment,

    on_ticks: Vec<usize>,
}

struct TickerAssignment {
    ticker_to_thread: Arc<Mutex<Vec<usize>>>,
}

// 
// Implementation
// 
const EXTERNAL_ID: usize = 0;
const MAIN_ID: usize = 1;

impl<M:'static> TickerSystem<M> {
    pub fn new(
        mut tickers: Vec<Box<dyn TickerCall<M>>>,
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

        system.on_build();

        system
    }

    fn init_threads(&mut self, spawn_threads: u32, n_tickers: usize) {
        let external = ThreadInner::new(self, "external", n_tickers);
        let main = ThreadInner::new(self, "main", n_tickers);

        for i in 0..spawn_threads {
            ThreadInner::new(self, &format!("thread-{}", self.threads.len()), n_tickers);
        }

        for thread in &mut self.threads {
            thread.fill_channels(&self.channels)
        }
    }

    fn assign_ticker(&mut self, thread_id: usize, ticker: Box<dyn TickerCall<M>>) {
        let ticker_id = ticker.id();
        self.ticker_assignment.set(ticker_id, thread_id);

        self.threads[thread_id].assign_ticker(ticker);

        /*
        for from_ticker_id in self.threads[thread_id].update_ticker(ticker_id) {
            let from_thread_id = self.ticker_assignment.get(from_ticker_id);

            //let thread = self.threads[from_thread_id].clone();

            self.threads[from_thread_id].update_ticker(from_ticker_id);
        }
         */
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
        ticker: Box<dyn TickerCall<M>>
    ) {
        self.ptr.write().unwrap().assign_ticker(ticker);
    }

    fn fill_channels(
        &mut self, 
        system: &SystemChannels<M>,
    ) {
        self.ptr.write().unwrap().channels.fill_thread(system);
    }

    /*
    fn update_ticker(&self, ticker_id: usize) -> Vec<usize> {
        self.ptr.write().unwrap().update_ticker(ticker_id)
    }
     */

    /*
    fn to_thread(&self, to_ticker: usize) -> ToThreadRef<M> {
        self.ptr.read().unwrap().to_thread(to_ticker)
    }
     */

    fn tick(&mut self, ticks: u64) {
        self.ptr.write().unwrap().tick(ticks);
    }

    fn on_build(&mut self) {
        self.ptr.write().unwrap().on_build();
    }
}

impl<M:'static> ThreadInner<M> {
    fn new(
        system: &mut TickerSystem<M>, 
        name: &str,
        n_tickers: usize
    ) -> TickerThread<M> {
        let id = system.threads.len();

        let mut tickers: Vec<Option<Box<dyn TickerCall<M>>>> = Vec::new();
        for _ in 0..n_tickers {
            tickers.push(None);
        }

        let channels = if id == 0 {
            system.channels.push_external_thread()
        } else {
            system.channels.push_thread()
        };

        let mut thread = Self {
            id: id,
            name: String::from(name),

            tickers: tickers,
            ticker_assignment: system.ticker_assignment.clone(),
            on_ticks: Vec::new(),

            channels: channels,
        };

        let thread_ref = Arc::new(RwLock::new(thread));

        let ticker = TickerThread { id: id, ptr: thread_ref.clone() };

        system.threads.push(ticker);

        TickerThread { id: id, ptr: thread_ref.clone() }
    }

    /*
    fn push_to(&mut self, to_thread: ToThreadRef<M>) {
        self.to.push(to_thread);
    }
     */

    fn assign_ticker(&mut self, ticker: Box<dyn TickerCall<M>>) {
        let ticker_id = ticker.id();

        self.on_ticks.push(ticker_id);

        assert!(self.tickers[ticker_id].is_none());
        self.tickers[ticker_id] = Some(ticker);
    }

    /*
    fn update_ticker(&self, ticker_id: usize) -> Vec<usize> {
        match &self.tickers[ticker_id] {
            Some(ticker) => { 
                ticker.update_to_tickers(&self);
                ticker.from_ticker_ids()
             },
            None => panic!("Thread.update_ticker called with invalid ticker")
        }
    }
     */

    /*
    pub fn to_thread(&self, to_ticker: usize) -> ToThreadRef<M> {
        let thread_id = self.ticker_assignment.get(to_ticker);

        Rc::clone(&self.to[thread_id])
    }
     */

    fn tick(&mut self, ticks: u64) {
        self.set_ticks(ticks);

        self.receive();

        self.on_ticks(ticks);
    }

    fn on_ticks(&mut self, ticks: u64) {
        for ticker_id in &self.on_ticks {
            match &mut self.tickers[*ticker_id] {
                Some(ticker) => ticker.tick(ticks),
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
        /*
        let receiver = &self.receiver;

        for msg in receiver.try_iter() {
            match &mut self.tickers[msg.to_ticker] {
                Some(ticker) => {
                    ticker.send(msg.on_fiber, msg.args);
                },
                None => {
                    panic!("In thread #{} Attempt to call ticker {}",
                        self.id, msg.to_ticker);
                }
            }
        }
        */
    }
}

impl<T> fmt::Display for ThreadInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ThreadInner:{}[{}]", self.id, self.name)
    }
}

impl TickerAssignment {
    fn new<M>(tickers: &Vec<Box<dyn TickerCall<M>>>) ->Self {
        let mut ticker_to_thread: Vec<usize> = Vec::new();
        
        ticker_to_thread.resize(tickers.len(), 0);

        Self {
            ticker_to_thread: Arc::new(Mutex::new(ticker_to_thread)),
        }
    }

    fn get(&self, ticker_id: usize) -> usize {
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

pub fn test_thread() {
}
