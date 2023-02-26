use std::collections::HashMap;

use ticker::{Ticker, Context};

use crate::{TickerBuilder, Source, Sink, MindBuilder, MindMessage, Gram, Topos, TickerPtr, Fiber};

pub trait Action {
    fn id(&self) -> &Gram;
    fn action(&mut self, ctx: &mut ticker::Context) -> bool;
}

pub struct ActionGroup {
    mind: MindBuilder,

    _actions: Vec<Box<dyn ActionTrait>>,

    ticker: TickerBuilder<ActionGroupInner>,

    sink: Sink,
}

impl ActionGroup {
    pub fn new(mind: &mut MindBuilder) -> Self {
        // let inner = ActionTicker {};
        // let mut ticker = mind.ticker(inner);
        // todo() add theta

        let ticker = mind.ticker(ActionGroupInner::new());

        let sink = ticker.sink(move |t, msg| {
            t.request(msg.0, msg.1);
        });

        ActionGroup {
            mind: mind.clone(),
            _actions: Vec::new(),
            ticker: ticker,
            sink: sink,
        }
    }

    pub fn node<A:'static>(
        &mut self, 
        id: Gram, 
        action: A,
        on_action: impl Fn(&mut A, &mut Context) -> bool + 'static
    ) -> ActionBuilder<A> {
        let name = id.clone();

        let mut item = ActionBuilder::new(
            &mut self.mind, 
            id, 
            action, 
            Box::new(on_action)
        );

        let sink = item.activate_sink();
        let mut source = self.ticker.source(move |g, fiber| {
            g.add_action(name, fiber);
        });

        source.to(&sink);

        item
    }

    pub fn action<A:Action + 'static>(&mut self, action: A) -> ActionBuilder<A> {
        let id = action.id().clone();

        let mut item = ActionBuilder::<A>::new(
            &mut self.mind, 
            id.clone(), 
            action,
            Box::new(|a, ctx| a.action(ctx)),
        );

        let sink = item.activate_sink();
        let mut source = self.ticker.source(move |g, fiber| {
            g.add_action(id, fiber);
        });

        source.to(&sink);

        //item.on_action(move |a, ctx| a.action(ctx));

        item
    }

    pub fn request(&mut self) -> &Sink {
        &self.sink
    }
}

struct ActionGroupInner {
    action_map: HashMap<Gram,Fiber>,
    requests: Vec<Gram>,
    _is_active: bool,
}

impl ActionGroupInner {
    fn new() -> Self {
        Self {
            action_map: HashMap::new(),
            requests: Vec::new(),
            _is_active: false,
        }
    }

    fn add_action(&mut self, gram: Gram, fiber: Fiber) {
        self.action_map.insert(gram, fiber);
    }

    fn request(&mut self, gram: Gram, _topos: Topos) {
        self.requests.push(gram);
        //self.action_map.insert(gram, fiber);
    }
}

impl Ticker for ActionGroupInner {
    fn tick(&mut self, _: &mut ticker::Context) {
        if self.requests.len() > 0 {
            let gram = self.requests.remove(0);
            self.requests.drain(..);
            match self.action_map.get(&gram) {
                Some(fiber) => {
                    fiber.send((Gram::from("action"), Topos::Nil))
                }
                None => { panic!("group call to unknown action {}", gram); }
            }
        }
    }
}

struct ActionTicker {
    
}

//
// # ActionBuilder
//

pub struct ActionBuilder<A> {
    ticker: TickerBuilder<ActionItem<A>>,
}

impl<A:'static> ActionBuilder<A> {
    fn new(
        mind: &mut MindBuilder,
        id: Gram, 
        action: A,
        on_action: Box<dyn Fn(&mut A, &mut Context) -> bool>
    ) -> Self {
        ActionBuilder {
            ticker: mind.ticker(ActionItem::new(id, action, on_action))
        }
    }

    pub fn source(&mut self, set: impl FnOnce(&mut A, Fiber) + 'static) -> Source {
        self.ticker.source(|t, fiber| {
            set(&mut t.action, fiber)
        })
    }

    pub fn sink(&mut self, on_msg: impl Fn(&mut A, MindMessage) + 'static) -> Sink {
        self.ticker.sink(move |t, msg| {
            on_msg(&mut t.action, msg)
        })
    }

    fn activate_sink(&mut self) -> Sink {
        self.ticker.sink(move |a, msg| {
            a.activate(msg.0, msg.1)
        })
    }

    pub fn unwrap(self) -> ActionReader<A> {
        ActionReader(self.ticker.unwrap())
    }
}

pub struct ActionReader<A>(TickerPtr<ActionItem<A>>);

impl<A:'static> ActionReader<A> {
    pub fn read<R>(&self, read: impl FnOnce(&A) -> R) -> R {
        self.0.read(|t| read(&t.action))
    }

    pub fn write<R>(&mut self, write: impl FnOnce(&mut A) -> R) -> R {
        self.0.write(|t| write(&mut t.action))
    }
}


type OnAction<A> = dyn Fn(&mut A, &mut ticker::Context)->bool;

//trait Action {
//    fn on_complete(&self)->Source;
//}

trait ActionTrait {
    fn on_action(&mut self, ctx: &mut ticker::Context) -> bool;
}

struct ActionItem<A> {
    _id: Gram,
    action: A,
    on_action: Box<OnAction<A>>,
    is_active: bool,
}

impl<A> ActionItem<A> {
    fn new(id: Gram, action: A, on_action: Box<OnAction<A>>) -> Self {
        ActionItem {
            _id: id,
            action: action,
            on_action: on_action,
            is_active: false,
        }
    }

    pub fn activate(&mut self, _gram: Gram, _topos: Topos) {
        self.is_active = true;
    }
}


impl<A> Ticker for ActionItem<A> {
    fn tick(&mut self, ctx: &mut ticker::Context) {
        if self.is_active && ! (self.on_action)(&mut self.action, ctx) {
            self.is_active = false;
        }
    }
}
/*
impl<A> ActionTrait for ActionItem<A> {
    fn on_action(&mut self, ctx: &mut ticker::Context) -> bool {
        self.on_action(ctx)
    }
}
 */

impl ActionTicker {
}


impl Ticker for ActionTicker {
    fn tick(&mut self, _: &mut ticker::Context) {

    }
}