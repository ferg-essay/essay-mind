//! Single ticking node

use crate::builder::*;
use crate::fiber::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

pub type TickFn = dyn Fn(u32)->() + Send;
pub type TickerShared<T> = Arc<Mutex<TickerImpl<T>>>;

type ThreadGroupShared<T> = Arc<RwLock<Option<ThreadGroup<T>>>>;

pub struct Ticker<T> {
    pub id: usize,
    pub name: String,

    group: ThreadGroupShared<T>,

    ticker: TickerShared<T>,
}

type ChannelItem<T> = (usize, usize, Box<T>);

pub struct ThreadGroup<T> {
    sender: Sender<ChannelItem<T>>,

    receiver: Receiver<ChannelItem<T>>,

    tickers: Vec<TickerShared<T>>,
}

impl<T> ThreadGroup<T> {
    fn send(&self, ticker: usize, on_fiber_i: usize, args: Box<T>) {
        self.sender.send((ticker, on_fiber_i, args)).unwrap();
    }
}

impl<T> Ticker<T> {
    fn tick(&self, ticks: u32) {
        self.ticker.lock().unwrap().tick(ticks);
    }

    pub(crate) fn send_fiber(&self, on_fiber_i: usize, args:Box<T>) {
        //let group : &TickerThreadGroup<T> = &
        match self.group.read().unwrap().as_ref() {
            Some(group) => { group.send(self.id, on_fiber_i, args); },
            None => { panic!("Thread group is not assigned"); }
        };
        //        let group = self.group.read().unwrap().expect("Thread group is not assigned");
        //group.send(self.id, on_fiber_i, args);
        //let ptr = Box::into_raw(Box::new(args));
        //group.sender.send((self.id, ptr)).unwrap();
        //group.sender.send((self.id, on_fiber_i, args)).unwrap();
    }

    fn on_fiber(&self, on_fiber_i: usize, args: Box<T>) {
        self.ticker.lock().unwrap().on_fiber(on_fiber_i, args);
    }
}

impl<T> Clone for Ticker<T> {
    fn clone(&self) -> Self {
        Ticker {
            id: self.id,
            name: self.name.clone(),
            group: self.group.clone(),
            ticker: self.ticker.clone(),
        }
    }
}

impl<T> fmt::Display for Ticker<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker[{}]", self.name)
    }
}

trait FiberArgs {
}

pub struct TickerImpl<T> {
    id: usize,

    name: String,

    group: ThreadGroupShared<T>,

    on_tick: Option<Box<TickFn>>,

    pub on_fiber: Vec<(FiberId,Box<FiberFn<T>>)>,
}

impl<T> TickerImpl<T> {
    pub fn new(id: usize, name: String, on_tick: Option<Box<TickFn>>, on_fibers: Vec<(FiberId, Box<FiberFn<T>>)>) -> Ticker<T> {
        let group = Arc::new(RwLock::new(None));
 
        let ticker = TickerImpl {
            id,
            name: name.clone(),
            group: group.clone(),
            on_tick : on_tick,
            on_fiber: on_fibers,
        };

        let ticker_ref = Arc::new(Mutex::new(ticker));

        Ticker {
            id: id,
            name: name,
            group: group,
            ticker: ticker_ref,
        }
    }

    fn tick(&self, ticks: u32) {
        let on_tick = self.on_tick.as_ref().expect("TickerImpl.tick called but no on_tick was defined.");

        on_tick(ticks);
    }

    fn on_fiber(&self, on_fiber_i: usize, args: Box<T>)
    {
        let (fiber_id, on_fiber) = &self.on_fiber[on_fiber_i];

        on_fiber(fiber_id, args.as_ref());
    }
}

pub struct TickerSystem<T> {
    pub(crate) system: Arc<RwLock<TickerSystemImpl<T>>>
}

impl<T> TickerSystem<T> {
    pub fn ticks(&self) -> u32 {
        self.system.read().unwrap().ticks
    }

    pub fn tick(&self) {
        self.system.write().unwrap().tick();
    }
}

pub struct TickerSystemImpl<T> {
    pub ticks: u32,

    pub(crate) tickers: Vec<Ticker<T>>,

    pub(crate) on_tickers: Vec<TickerShared<T>>,

    group: ThreadGroupShared<T>,
}

impl<T> TickerSystemImpl<T> {
    pub fn new(mut tickers: Vec<Ticker<T>>) -> TickerSystem<T>
    {
        let mut ticker_refs : Vec<TickerShared<T>> = Vec::new();
        let mut on_tickers : Vec<TickerShared<T>> = Vec::new();

        for ticker in &tickers {
            let ticker_ref = &ticker.ticker;

            ticker_refs.push(Arc::clone(ticker_ref));

            if ticker_ref.lock().unwrap().on_tick.is_some() {
                on_tickers.push(Arc::clone(ticker_ref));
            }
        }

        let (sender, receiver) = mpsc::channel::<ChannelItem<T>>();

        let mut group = ThreadGroup {
            sender: sender,
            receiver: receiver,
            tickers: ticker_refs,
        };

        let group_ref = Arc::new(RwLock::new(Some(group)));

        for mut ticker in &mut tickers {
            ticker.group = group_ref.clone();
        }


        let system = TickerSystemImpl {
            ticks: 0,
            tickers: tickers,
            on_tickers: on_tickers,
            group: group_ref,
        };

        TickerSystem {
            system: Arc::new(RwLock::new(system))
        }
    }
    pub fn tick(&mut self) {
        let ticks = self.ticks + 1;
        self.ticks = ticks;

        for ticker in &self.on_tickers {
            ticker.lock().unwrap().tick(ticks);
        }
    }
}
