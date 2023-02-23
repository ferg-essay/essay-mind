//! Fibers communicate between tickers.

use std::{rc::Rc, cell::{RefCell}, sync::mpsc};

//use log::info;

use crate::{ticker::{TickerRef}, system::{TickerAssignment}};

//pub type FiberRef<M> = Rc<RefCell<Option<Box<dyn FiberInner<M>>>>>;
pub(crate) type FiberSourceRef<M> = Rc<RefCell<Box<dyn FiberInner<M>>>>;

pub(crate) type ChannelRef<T> = Rc<RefCell<Box<dyn Channel<T>>>>;

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

struct Message<M> {
    to_ticker: usize,
    on_fiber: usize,

    args: M,
}

pub(crate) struct TickerFibers<M> {
    sources: Vec<FiberSourceRef<M>>,
    
    // pub on_fiber: Vec<Box<OnFiber<M,T>>>,
}

pub(crate) trait FiberInner<M> {
    /// Sends a message to target `Ticker` on_fiber closures
    fn send(&mut self, args: M);

    /// Builds the channel
    fn build_channel(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    );
}

struct FiberZero {
}

struct FiberOne<M> {
    sink: usize,

    to: ChannelRef<M>,
    
    on_fiber: usize,
}

pub struct FiberMany<M> {
    to: Vec<FiberOne<M>>,
    tail: FiberOne<M>,
}

struct FiberPanic {

}
impl<M:Clone + 'static> TickerFibers<M> {
    pub(crate) fn new() -> TickerFibers<M> {
        Self {
            sources: Vec::new(),
            // on_fiber: Vec::new(),
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

    pub fn build_channel(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        for source in &mut self.sources {
            source.borrow_mut().build_channel(tickers, channels)
        }
    }
}

impl<M> FiberOne<M> {
    fn new(to: (usize, usize, usize)) -> Self {
        FiberOne {
            // source: to.0,
            sink: to.1,
            on_fiber: to.2,
            to: PanicChannel::new("unconfigured fiber")
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

    fn build_channel(
        &mut self, 
        _tickers: &TickerAssignment,
        _channels: &ThreadChannels<M>
    ) {
        panic!("building channel for unconfigured fiber")
    }
 }

impl<M> FiberInner<M> for FiberZero {
    /// send a message to the fiber targets.
    fn send(&mut self, _args: M) {
    }

    fn build_channel(
        &mut self, 
        _tickers: &TickerAssignment,
        _channels: &ThreadChannels<M>
    ) {
    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberOne<M> {
    fn send(&mut self, args: M) {
        self.to.borrow_mut().send(self.sink, self.on_fiber, args);
    }

    fn build_channel(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        // let source_thread = tickers.get(self.source);
        let sink_thread = tickers.get(self.sink);

        self.to = channels.get(sink_thread);

    }
}

impl<M:Clone + 'static> FiberInner<M> for FiberMany<M> {
    fn send(&mut self, args: M) {
        for to in &mut self.to {
            to.send(args.clone());
        }

        self.tail.send(args);
    }

    fn build_channel(
        &mut self, 
        tickers: &TickerAssignment,
        channels: &ThreadChannels<M>
    ) {
        for to in &mut self.to {
            to.build_channel(tickers, channels);
        }

        self.tail.build_channel(tickers, channels);
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

pub(crate) trait Channel<M> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: M);

    fn update_tickers(&mut self, _tickers: &Vec<TickerRef<M>>) {}
}

pub(crate) struct SystemChannels<M> {
    senders: Vec<mpsc::Sender<Message<M>>>,
}

pub(crate) struct ThreadChannels<M> {
    id: usize,

    receiver: mpsc::Receiver<Message<M>>,

    channels: Vec<ChannelRef<M>>,
}

impl<M> SystemChannels<M> {
    pub(crate) fn new() -> Self {
        Self {
            senders: Vec::new(),
        }
    }

    pub(crate) fn push_external_thread(&mut self) -> ThreadChannels<M> {
        let (_, receiver) = mpsc::channel::<Message<M>>();

        let mut channels = Vec::<ChannelRef<M>>::new();

        channels.push(PanicChannel::new("invalid send to external ticker"));

        // self.senders.push(sender);

        ThreadChannels {
            id: 0,
            receiver: receiver,
            channels: channels,
        }
    }

    pub(crate) fn push_thread(&mut self) -> ThreadChannels<M> {
        let (sender, receiver) = mpsc::channel::<Message<M>>();

        let mut channels = Vec::<ChannelRef<M>>::new();

        channels.push(PanicChannel::new("invalid send to external ticker"));

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
                OwnChannel::new(self.id)
            } else { // if i > 0 {
                SenderChannel::new(self.id, i + 1, sender.clone())
            }; /* else {
                PanicToThread::new("sending to external thread")
            };*/

            self.channels.push(channel);
        }
    }

    pub(crate) fn update_tickers(&mut self, tickers: &Vec<TickerRef<M>>) {
        for channel in &mut self.channels {
            channel.borrow_mut().update_tickers(tickers);
        }
    }

    pub(crate) fn get(&self, sink: usize) -> ChannelRef<M> {
        assert!(sink != 0);

        Rc::clone(&self.channels[sink])
    }

    pub(crate) fn receive(&self, tickers: &mut Vec<TickerRef<M>>) {
        for msg in self.receiver.try_iter() {
            tickers[msg.to_ticker].send(msg.on_fiber, msg.args);
        }
    }
}

struct SenderChannel<M> {
    //_name: String,
    to: mpsc::Sender<Message<M>>,
}

struct OwnChannel<M> {
    //name: String,
    //id: usize,

    to: Vec<TickerRef<M>>,
}

pub struct PanicChannel {
    msg: String,
}

impl<T:'static> SenderChannel<T> {
    fn new(_source: usize, sink: usize, to: mpsc::Sender<Message<T>>) -> ChannelRef<T> {
        assert!(sink != 0);

        // _name: format!("{}->{}", source, sink),
        Rc::new(RefCell::new(Box::new(Self {
            to,
        })))
    }
}

impl<T> Channel<T> for SenderChannel<T> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: T)
    {
        self.to.send(Message::new(to_ticker, on_fiber, args)).unwrap();
    }
}

impl<M:'static> OwnChannel<M> {
    fn new(_id: usize) -> ChannelRef<M> {
        Rc::new(RefCell::new(Box::new(Self {
            // name: format!("{}:{}", id, name),
            // id: id,
            to: Vec::new(),
        })))
    }
}

impl<M:'static> Channel<M> for OwnChannel<M> {
    fn send(&mut self, to_ticker: usize, on_fiber: usize, args: M) {
        self.to[to_ticker].send(on_fiber, args);
    }

    fn update_tickers(&mut self, tickers: &Vec<TickerRef<M>>) {
        self.to.drain(..);

        for ticker in tickers {
            self.to.push((*ticker).clone());
        }
    }
}

impl PanicChannel {
    pub(crate) fn new<T>(msg: &str) -> ChannelRef<T> {
        Rc::new(RefCell::new(Box::new(Self {
            msg: String::from(msg),
        })))
    }
}

impl<T> Channel<T> for PanicChannel {
    fn send(&mut self, to_ticker: usize, _on_fiber: usize, _args: T) {
        panic!(
            "{} (To Ticker #{})", 
            self.msg,
            to_ticker
        );
    }
}

