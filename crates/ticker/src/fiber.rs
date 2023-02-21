//! Fibers communicate between tickers.

use std::{fmt, rc::Rc, cell::{RefCell, Ref}, sync::mpsc};

//use log::info;

use crate::{ticker::{TickerCall}, system::{ThreadInner}, OnFiber};

//pub type FiberRef<M> = Rc<RefCell<Option<Box<dyn FiberInner<M>>>>>;
pub(crate) type FiberSourceRef<M> = Rc<RefCell<Box<dyn FiberInner<M>>>>;

pub type ToThreadRef<T> = Rc<RefCell<Box<dyn ToThread<T>>>>;

/// Message channel to `Ticker` targets, where each target is
/// a callback in a Ticker's context.
pub struct Fiber<M:Clone>
{
    //pub id: usize,
    //pub name: String,

    ptr: FiberSourceRef<M>,
}

//
// Implementation
//
/*
struct FiberHolder<M> {
    opt: Option<Box<dyn FiberInner<M>>>,
}

impl<M:Clone> FiberHolder<M> {
    fn send(&self, args: M) {
        match &self.opt {
            Some(fiber) => fiber.send(args),
            None => panic!("mismatched holder")
        }
    }

    fn replace(&mut self, fiber: Box<dyn FiberInner<M>>) {
        self.opt.replace(fiber);
    }
}
 */

impl<M:'static + Clone> Fiber<M> {
    /// send a message to fiber targets, on_fiber closures of target tickers.
    pub fn send(&self, args: M) {
        self.ptr.borrow_mut().send(args);

        //self.fiber_ref.borrow().expect("unconfigured fiber").send(args);
    }
}

impl<M:'static + Clone> Clone for Fiber<M> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

//
// # inner

pub(crate) struct SystemChannels<M> {
    senders: Vec<mpsc::Sender<Message<M>>>,
}

pub(crate) struct ThreadChannels<M> {
    id: usize,

    receiver: mpsc::Receiver<Message<M>>,

    channels: Vec<ToThreadRef<M>>,
}

impl<M> SystemChannels<M> {
    pub(crate) fn new() -> Self {
        Self {
            senders: Vec::new(),
        }
    }

    pub(crate) fn push_external_thread(&mut self) -> ThreadChannels<M> {
        let (sender, receiver) = mpsc::channel::<Message<M>>();

        let mut channels = Vec::<ToThreadRef<M>>::new();

        channels.push(PanicToThread::new("invalid send to external ticker"));

        // self.senders.push(sender);

        ThreadChannels {
            id: 0,
            receiver: receiver,
            channels: channels,
        }
    }

    pub(crate) fn push_thread(&mut self) -> ThreadChannels<M> {
        let (sender, receiver) = mpsc::channel::<Message<M>>();

        let mut channels = Vec::<ToThreadRef<M>>::new();

        channels.push(PanicToThread::new("invalid send to external ticker"));

        self.senders.push(sender);

        ThreadChannels {
            id: self.senders.len(),
            receiver: receiver,
            channels: channels,
        }
    }
}

impl<M:'static> ThreadChannels<M> {
    pub(crate) fn fill_thread(&mut self, system: &SystemChannels<M>) {
        assert!(self.channels.len() == 1);

        for (i, sender) in system.senders.iter().enumerate() {
            let channel = if i + 1 == self.id {
                OwnToThread::new(self.id)
            } else if i > 0 {
                ChannelToThread::new(i + 1, self.id, sender.clone())
            } else {
                PanicToThread::new("sending to external thread")
            };

            self.channels.push(channel);
        }


    }
}

pub trait ToThread<M> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: M);
}

struct ChannelToThread<M> {
    //_name: String,
    to: mpsc::Sender<Message<M>>,
}

struct OwnToThread<M> {
    //name: String,
    id: usize,

    to: Vec<Option<Box<dyn TickerCall<M>>>>,
}

pub struct PanicToThread {
    msg: String,
}

struct Message<M> {
    to_ticker: usize,
    on_fiber: usize,

    args: M,
}

pub(crate) struct TickerFibers<M, T> {
    sources: Vec<FiberSourceRef<M>>,
    
    pub on_fiber: Vec<Box<OnFiber<M,T>>>,
}

pub(crate) trait FiberInner<M> {
    /// Sends a message to target `Ticker` on_fiber closures
    fn send(&mut self, args: M);

    /// Builds the channel
    fn build_channel(&mut self, thread: &ThreadInner<M>);
}

struct FiberZero {
}

struct FiberOne<M> {
    source: usize,
    sink: usize,

    to: ToThreadRef<M>,
    
    on_fiber: usize,
}

pub struct FiberMany<M> {
    to: Vec<FiberOne<M>>,
    tail: FiberOne<M>,
}

struct FiberPanic {

}
impl<M:Clone + 'static,T> TickerFibers<M, T> {
    pub(crate) fn new() -> TickerFibers<M,T> {
        Self {
            sources: Vec::new(),
            on_fiber: Vec::new(),
        }
    }

    pub(crate) fn new_fiber(
        &mut self, 
        to: &mut Vec<(usize, usize, usize)>
    ) -> Fiber<M> {
        let ptr = Rc::new(RefCell::new(self.new_inner(to)));

        self.sources.push(Rc::clone(&ptr));

        let fiber = Fiber {
            ptr: ptr,
        };

        fiber
    }

    fn new_inner(&self, to: &mut Vec<(usize, usize,usize)>) -> Box<dyn FiberInner<M>> {
        match to.len() {
            0 => Box::new(FiberZero {}),
            1 => Box::new(FiberOne::new(to[0])),
            _ => Box::new(FiberMany::new(to)),
        }
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        for source in &mut self.sources {
            source.borrow_mut().build_channel(thread)
        }
    }
    
    fn update_sources(&self, thread: &ThreadInner<M>) {
        for source in &self.sources {
            //let mut to_ticker_inner: &ToTickerInner<T> = &
            source.borrow_mut().build_channel(thread);
        }

        //self.from_ticker_ids()
    }

    /*
    fn from_ticker_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        for from in &self.sources {
            let from_id = from.from_ticker;

            if from_id != from.to_ticker && ! ids.contains(&from_id) {
                ids.push(from_id);
            }
        }

        ids
    }
     */
}

/*
impl<M:Clone> Clone for Fiber<M> {
    fn clone(&self) -> Self {
        Self {
            //id: self.id,
            //name: self.name.clone(),
            ptr: Rc::clone(&self.ptr),
        }
    }
}
 */

impl<M> FiberOne<M> {
    fn new(to: (usize, usize, usize)) -> Self {
        FiberOne {
            source: to.0,
            sink: to.1,
            on_fiber: to.2,
            to: PanicToThread::new("unconfigured fiber")
        }
    }
}

impl<M> FiberMany<M> {
    fn new(to: &mut Vec<(usize, usize, usize)>) -> Self {
        let tail = FiberOne::new(to.pop().unwrap());

        let mut head: Vec<FiberOne<M>> = Vec::new();

        for item in to {
            head.push(FiberOne::new(*item));
        }

        Self {
            to: head,
            tail: tail,
        }
    }
}

impl<M> FiberInner<M> for FiberPanic {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
        panic!("sending message to unconfigured fiber")
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        panic!("building channel for unconfigured fiber")
    }
 }

impl<M> FiberInner<M> for FiberZero {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberOne<M> {
    fn send(&mut self, args: M) {
        self.to.borrow_mut().send(self.sink, self.on_fiber, args);
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        todo!()
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberMany<M> {
    fn send(&mut self, args: M) {
        for to in &mut self.to {
            to.send(args.clone());
        }

        self.tail.send(args);
    }

    fn build_channel(&mut self, thread: &ThreadInner<M>) {
        for to in &mut self.to {
            to.build_channel(thread);
        }

        self.tail.build_channel(thread);
    }
}


impl<T> Message<T> {
    fn new(to_ticker: usize, on_fiber: usize, args: T) -> Self {
        Self {
            to_ticker,
            on_fiber,

            args,
        }
    }
}

impl<T:'static> ChannelToThread<T> {
    fn new(source: usize, sink: usize, to: mpsc::Sender<Message<T>>) -> ToThreadRef<T> {
        assert!(sink != 0);

        // _name: format!("{}->{}", source, sink),
        Rc::new(RefCell::new(Box::new(Self {
            to,
        })))
    }
}

impl<T> ToThread<T> for ChannelToThread<T> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: T)
    {
        self.to.send(Message::new(to_ticker, on_fiber, args)).unwrap();
    }
}

impl<T:'static> OwnToThread<T> {
    fn new(id: usize) -> ToThreadRef<T> {
        Rc::new(RefCell::new(Box::new(Self {
            // name: format!("{}:{}", id, name),
            id: id,
            to: Vec::new(),
        })))
    }
}

impl<T:'static> ToThread<T> for OwnToThread<T> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: T)
    {
        match &mut self.to[to_ticker] {
            Some(ticker) => ticker.send(on_fiber, args),
            _ => {
                panic!(
                    "Ticker #{} called on Thread {}, which doesn't control the ticker.", 
                    to_ticker,
                    self.id
                )
            }
        }
    }
}

impl PanicToThread {
    pub fn new<T>(msg: &str) -> ToThreadRef<T> {
        Rc::new(RefCell::new(Box::new(Self {
            msg: String::from(msg),
        })))
    }
}

impl<T> ToThread<T> for PanicToThread {
    fn send(&mut self, to_ticker: usize, _on_fiber: usize, args: T) {
        panic!(
            "{} (To Ticker #{})", 
            self.msg,
            to_ticker
        );
    }
}

