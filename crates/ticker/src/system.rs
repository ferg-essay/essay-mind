/// Thread grouping of tickers
/// 
/// 

use std::{rc::Rc, sync::{Arc, RwLock, Mutex}, fmt, cell::RefCell};
use std::sync::mpsc;
use std::thread;

use crate::ticker::{TickerInner, ToTicker};
//extern crate env_logger;
//use env_logger::Env;
//use log::{info};

type ThreadRef<T> = Arc<RwLock<ThreadInner<T>>>;

pub type ToThreadRef<T> = Rc<Box<dyn ToThread<T>>>;

thread_local!(static TICKS: RefCell<u64> = RefCell::new(0));

pub struct TickerSystem<T> {
    ticks: u64,
    threads: Vec<TickerThread<T>>,

    ticker_assignment: TickerAssignment,
}

struct TickerThread<T> {
    id: usize,

    ptr: ThreadRef<T>,
}

pub struct ThreadInner<T> {
    id: usize,
    name: String,

    receiver: mpsc::Receiver<Message<T>>,
    sender: mpsc::Sender<Message<T>>,

    to: Vec<ToThreadRef<T>>,

    tickers: Vec<Option<TickerInner<T>>>,

    ticker_assignment: TickerAssignment,

    on_ticks: Vec<usize>,
}

struct TickerAssignment {
    ticker_to_thread: Arc<Mutex<Vec<usize>>>,
}

pub trait ToThread<T> {
    fn send(&self, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T);
}

struct ChannelToThread<T> {
    name: String,
    to: mpsc::Sender<Message<T>>,
}

struct OwnToThread<T> {
    name: String,

    to: Vec<Option<TickerInner<T>>>,
}

pub struct PanicToThread {
    msg: String,
}

struct Message<T> {
    to_ticker: usize,
    on_fiber: usize,

    from_ticker: usize,
    args: T,
}

// 
// Implementation
// 

impl<T:'static> TickerSystem<T> {
    pub fn new(
        mut tickers: Vec<TickerInner<T>> ,
        spawn_threads: u32
    ) -> Self {
        assert!(spawn_threads <= 1);

        let mut system = Self {
            ticks: 0,
            threads: Vec::new(),
            ticker_assignment: TickerAssignment::new(&tickers),
        };

        let n_tickers = tickers.len();

        let external = ThreadInner::new(&mut system, "external", n_tickers);
        let main = ThreadInner::new(&mut system, "main", n_tickers);

        for i in 0..spawn_threads {
            ThreadInner::new(&mut system, &format!("thread-{}", i), n_tickers);
        }

        system.assign_ticker(external.id, tickers.remove(0));

        for ticker in tickers.drain(..) {
            system.assign_ticker(main.id, ticker);
        }

        system.on_build();

        system
    }

    fn assign_ticker(&mut self, thread_id: usize, ticker: TickerInner<T>) {
        let ticker_id = ticker.id;
        self.ticker_assignment.set(ticker_id, thread_id);

        self.threads[thread_id].assign_ticker(ticker);

        for from_ticker_id in self.threads[thread_id].update_ticker(ticker_id) {
            let from_thread_id = self.ticker_assignment.get(from_ticker_id);

            //let thread = self.threads[from_thread_id].clone();

            self.threads[from_thread_id].update_ticker(from_ticker_id);
        }
    }

    pub fn to_thread(&self, to_ticker: &ToTicker<T>)->ToThreadRef<T> {
        let thread_id = self.ticker_assignment.get(to_ticker.from_ticker);

        // let thread = self.threads[thread_id];

        self.threads[thread_id].to_thread(to_ticker.to_ticker)
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

impl<T:'static> TickerThread<T> {
    fn assign_ticker(
        &mut self, 
        ticker: TickerInner<T>
    ) {
        self.ptr.write().unwrap().assign_ticker(ticker);
    }

    fn update_ticker(&self, ticker_id: usize) -> Vec<usize> {
        self.ptr.write().unwrap().update_ticker(ticker_id)
    }

    fn to_thread(&self, to_ticker: usize) -> ToThreadRef<T> {
        self.ptr.read().unwrap().to_thread(to_ticker)
    }

    fn tick(&mut self, ticks: u64) {
        self.ptr.write().unwrap().tick(ticks);
    }

    fn on_build(&mut self) {
        self.ptr.write().unwrap().on_build();
    }
}

pub fn ticks() -> u64 {
    let mut ticks: u64 = 0;

    TICKS.with(|f| {
        ticks = *f.borrow();
    });

    ticks
}

impl<T:'static> ThreadInner<T> {
    fn new(
        system: &mut TickerSystem<T>, 
        name: &str,
        n_tickers: usize
    ) -> TickerThread<T> {
        let id = system.threads.len();

        let (sender, receiver) = mpsc::channel::<Message<T>>();

        let mut to: Vec<ToThreadRef<T>> = Vec::new();

        let mut tickers: Vec<Option<TickerInner<T>>> = Vec::new();

        for _ in 0..n_tickers {
            tickers.push(None);
        }

        to.push(PanicToThread::new(&format!("{} attempted send to external", &name)));

        let mut thread = Self {
            id: id,
            name: String::from(name),
            sender: sender.clone(),
            receiver,
            to,
            tickers,
            ticker_assignment: system.ticker_assignment.clone(),

            on_ticks: Vec::new(),
        };

        if id != 0 {
            for peer_thread in &system.threads {
                let mut peer = peer_thread.ptr.write().unwrap();
                //let to = write;

                let to_thread = if peer.id != 0 {
                    ChannelToThread::new(
                        thread.id, 
                        &thread.name, 
                        peer.id, 
                        &peer.name, 
                        peer.sender.clone()
                    )
                } else {
                    PanicToThread::new(
                        &format!("Attempted send to Thread #0")
                    )
                };

                thread.push_to(to_thread);

                let from_thread = ChannelToThread::new(
                    peer.id, 
                    &peer.name, 
                    thread.id, 
                    &thread.name, 
                    sender.clone()
                );

                peer.push_to(from_thread);
            }
        
            thread.push_to(OwnToThread::new(id, &name));
        }

        let thread_ref = Arc::new(RwLock::new(thread));

        let ticker = TickerThread { id: id, ptr: thread_ref.clone() };

        system.threads.push(ticker);

        TickerThread { id: id, ptr: thread_ref.clone() }
    }

    fn push_to(&mut self, to_thread: ToThreadRef<T>) {
        self.to.push(to_thread);
    }

    fn assign_ticker(&mut self, ticker: TickerInner<T>) {
        let ticker_id = ticker.id;

        if ticker.on_tick.is_some() {
            self.on_ticks.push(ticker_id);
        }

        self.tickers[ticker_id] = Some(ticker);

    }

    fn update_ticker(&self, ticker_id: usize) -> Vec<usize> {
        match &self.tickers[ticker_id] {
            Some(ticker) => { 
                ticker.update_to_tickers(&self);
                ticker.from_ticker_ids()
             },
            None => panic!("Thread.update_ticker called with invalid ticker")
        }
    }

    pub fn to_thread(&self, to_ticker: usize) -> ToThreadRef<T> {
        let thread_id = self.ticker_assignment.get(to_ticker);

        Rc::clone(&self.to[thread_id])
    }

    fn send(&self, thread_id: usize, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T) {
        self.to[thread_id].send(to_ticker, on_fiber, from_ticker, args);
    }

    fn tick(&self, ticks: u64) {
        self.set_ticks(ticks);

        self.receive();

        self.on_ticks(ticks);
    }

    fn on_ticks(&self, ticks: u64) {
        for ticker_id in self.on_ticks.iter() {
            match &self.tickers[*ticker_id] {
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

    fn on_build(&self) {
        for ticker_opt in &self.tickers {
            if let Some(ticker) = ticker_opt {
                ticker.on_build();
            }
        }
    }

    fn receive(&self) {
        let receiver = &self.receiver;

        for msg in receiver.try_iter() {
            match &self.tickers[msg.to_ticker] {
                Some(ticker) => {
                    ticker.send(msg.on_fiber, msg.from_ticker, msg.args);
                },
                None => {
                    panic!("In thread #{} Attempt to call ticker {}",
                        self.id, msg.to_ticker);
                }
            }
        }
    }
}

impl<T> fmt::Display for ThreadInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ThreadInner:{}[{}]", self.id, self.name)
    }
}

impl TickerAssignment {
    fn new<T>(tickers: &Vec<TickerInner<T>>) ->Self {
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

impl<T> Message<T> {
    fn new(to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T) -> Self {
        Self {
            to_ticker,
            on_fiber,

            from_ticker,
            args,
        }
    }
}

impl<T:'static> ChannelToThread<T> {
    fn new(id: usize, name: &str, id_to: usize, name_to: &str, to: mpsc::Sender<Message<T>>) -> ToThreadRef<T> {
        assert!(id_to != 0);

        Rc::new(Box::new(Self {
            name: format!("{}:{}->{}:{}", id, name, id_to, name_to),
            to,
        }))
    }
}

impl<T> ToThread<T> for ChannelToThread<T> {
    fn send(&self, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T)
    {
        self.to.send(Message::new(to_ticker, on_fiber, from_ticker, args)).unwrap();
    }
}

impl<T:'static> OwnToThread<T> {
    fn new(id: usize, name: &str) -> ToThreadRef<T> {
        Rc::new(Box::new(Self {
            name: format!("{}:{}", id, name),
            to: Vec::new(),
        }))
    }
}

impl<T:'static> ToThread<T> for OwnToThread<T> {
    fn send(&self, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T)
    {
        match &self.to[to_ticker] {
            Some(ticker) => ticker.send(from_ticker, on_fiber, args),
            _ => {
                panic!(
                    "Ticker #{} called from Ticker #{} on Thread {}, which doesn't control the ticker.", 
                    to_ticker,
                    from_ticker, 
                    self.name
                )
            }
        }
    }
}

impl PanicToThread {
    pub fn new<T>(msg: &str) -> ToThreadRef<T> {
        Rc::new(Box::new(Self {
            msg: String::from(msg),
        }))
    }
}

impl<T> ToThread<T> for PanicToThread {
    fn send(&self, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T) {
        panic!(
            "{} (Ticker #{} to Ticker #{})", 
            self.msg,
            from_ticker,
            to_ticker
        );
    }
}

pub fn test_thread() {
}
