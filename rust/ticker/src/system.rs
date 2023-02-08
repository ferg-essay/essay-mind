/// Thread grouping of tickers
/// 
/// 

use std::{rc::Rc, sync::{Arc, RwLock}};
use std::sync::mpsc;

use crate::ticker::{TickerInner, TickerRef};

type ThreadRef<T> = Arc<RwLock<ThreadInner<T>>>;

pub type ToThreadRef<T> = Rc<Box<dyn ToThread<T>>>;

pub struct TickerSystem<T> {
    ticks: u32,
    threads: Vec<ThreadRef<T>>,
}

struct ThreadInner<T> {
    id: usize,
    name: String,

    receiver: mpsc::Receiver<Message<T>>,
    sender: mpsc::Sender<Message<T>>,

    to: Vec<ToThreadRef<T>>,

    tickers: Vec<Option<TickerInner<T>>>,
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
    name: String,
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
        tickers: Vec<TickerRef<T>> ,
        spawn_threads: u32
    ) -> Self {
        assert!(spawn_threads <= 1);

        let mut system = Self {
            ticks: 0,
            threads: Vec::new(),
        };

        let n_tickers = tickers.len();

        ThreadInner::new(&mut system, "external", n_tickers);
        ThreadInner::new(&mut system, "main", n_tickers);

        for i in 0..spawn_threads {
            ThreadInner::new(&mut system, &format!("thread-{}", i), n_tickers);
        }

        system
    }

    pub fn ticks(&self) -> u32 {
        self.ticks
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        self.threads[0].write().unwrap().tick(self.ticks);
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

impl<T> ToThread<T> for OwnToThread<T> {
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
    pub fn new<T>(name: &str) -> ToThreadRef<T> {
        Rc::new(Box::new(Self {
            name: String::from(name),
        }))
    }
}

impl<T> ToThread<T> for PanicToThread {
    fn send(&self, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T) {
        panic!(
            "Ticker #{} called from Ticker #{} on external Thread #0:{}, which can't receive messages.", 
            to_ticker,
            from_ticker,
            self.name
        );
    }
}

impl<T:'static> ThreadInner<T> {
    fn new(
        system: &mut TickerSystem<T>, 
        name: &str,
        n_tickers: usize
    ) -> ThreadRef<T> {
        let id = system.threads.len();

        let (sender, receiver) = mpsc::channel::<Message<T>>();

        let mut to: Vec<ToThreadRef<T>> = Vec::new();

        let mut tickers: Vec<Option<TickerInner<T>>> = Vec::new();

        for _ in [0..n_tickers] {
            tickers.push(None);
        }

        to.push(PanicToThread::new(&name));

        let mut thread = Self {
            id: id,
            name: String::from(name),
            sender: sender.clone(),
            receiver,
            to,
            tickers,
        };

        for thread_ref in &system.threads {
            let mut to = thread_ref.write().unwrap();
            //let to = write;

            let to_thread = ChannelToThread::new(
                id, 
                &name, 
                thread.id, 
                &thread.name, 
                thread.sender.clone()
            );

            thread.push_to(to_thread);

            let from_thread = ChannelToThread::new(
                thread.id, 
                &thread.name, 
                id, 
                &name, 
                sender.clone()
            );

            to.push_to(from_thread);
        }

        thread.push_to(OwnToThread::new(id, &name));

        let thread_ref = Arc::new(RwLock::new(thread));

        system.threads.push(Arc::clone(&thread_ref));

        thread_ref
    }

    fn push_to(&mut self, to_thread: ToThreadRef<T>) {
        self.to.push(to_thread);
    }

    fn send(&self, thread_id: usize, to_ticker: usize, on_fiber: usize, from_ticker: usize, args: T) {
        self.to[thread_id].send(to_ticker, on_fiber, from_ticker, args);
    }

    fn tick(&self, tick: u32) {
        println!("tick {}", tick);
    }
}

pub fn test_thread() {
}
