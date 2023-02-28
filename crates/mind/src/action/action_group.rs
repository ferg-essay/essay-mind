use std::collections::HashMap;

use ticker::{Ticker, Context};

use crate::{TickerBuilder, Source, Sink, MindBuilder, MindMessage, Gram, Topos, TickerPtr, Fiber};

pub trait Action {
    // fn id(&self) -> &Gram;
    fn action(&mut self, ctx: &mut ticker::Context) -> bool;
}

type OnAction<A> = dyn Fn(&mut A, &mut ticker::Context)->bool;

trait ActionTrait {
    fn on_action(&mut self, ctx: &mut ticker::Context) -> bool;
}

pub struct ActionGroup {
    mind: MindBuilder,

    //_actions: Vec<Box<dyn ActionTrait>>,

    ticker: TickerBuilder<ActionGroupInner>,

    request_sink: Sink,
    complete_sink: Sink,
}

impl ActionGroup {
    pub fn new(mind: &mut MindBuilder) -> Self {
        // let inner = ActionTicker {};
        // let mut ticker = mind.ticker(inner);
        // todo() add theta

        let ticker = mind.ticker(ActionGroupInner::new());

        let request_sink = ticker.sink(move |t, msg| {
            t.request(msg.0, msg.1);
        });

        let complete_sink = ticker.sink(move |t, msg| {
            t.complete(msg.0, msg.1);
        });

        ActionGroup {
            mind: mind.clone(),
            //_actions: Vec::new(),
            ticker: ticker,
            request_sink: request_sink,
            complete_sink: complete_sink,
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
            self, 
            id, 
            action, 
            Box::new(on_action)
        );

        let sink = item.activate_sink();
        let mut source = self.ticker.source(
            move |g, fiber| {
            g.add_action(name, fiber);
        });

        source.to(&sink);

        item
    }

    pub fn action<A:Action + 'static>(&mut self, id: Gram, action: A) -> ActionBuilder<A> {
        let id = id.clone();

        let mut item = ActionBuilder::<A>::new(
            self,
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
        &self.request_sink
    }
}

struct ActionGroupInner {
    action_map: HashMap<Gram,Fiber>,
    requests: Vec<Gram>,
    is_active: bool,
}

impl ActionGroupInner {
    fn new() -> Self {
        Self {
            action_map: HashMap::new(),
            requests: Vec::new(),
            is_active: false,
        }
    }

    fn add_action(&mut self, gram: Gram, fiber: Fiber) {
        self.action_map.insert(gram, fiber);
    }

    fn request(&mut self, gram: Gram, _topos: Topos) {
        self.requests.push(gram);
        //self.action_map.insert(gram, fiber);
    }

    fn complete(&mut self, _gram: Gram, _topos: Topos) {
        self.is_active = false;
        //self.requests.push(gram);
        //self.action_map.insert(gram, fiber);
    }
}

impl Ticker for ActionGroupInner {
    fn tick(&mut self, _: &mut ticker::Context) {
        if ! self.is_active && self.requests.len() > 0 {
            self.is_active = true;

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

//
// # ActionItem
//

struct ActionItem<A> {
    id: Gram,
    action: A,
    on_action: Box<OnAction<A>>,
    on_complete: Fiber,
    is_active: bool,

    on_activation: Box<OnAction<A>>,
    fiber_request: Fiber,
}

impl<A> ActionItem<A> {
    fn new(id: Gram, action: A, on_action: Box<OnAction<A>>) -> Self {
        ActionItem {
            id,
            action: action,
            on_action: on_action,
            on_complete: Default::default(),
            is_active: false,

            on_activation: Box::new(|a, ctx| false),
            fiber_request: Default::default(),
        }
    }

    pub fn activate(&mut self, _gram: Gram, _topos: Topos) {
        self.is_active = true;
    }
}


impl<A> Ticker for ActionItem<A> {
    fn tick(&mut self, ctx: &mut ticker::Context) {
        if self.is_active {
            if ! (self.on_action)(&mut self.action, ctx) {
                self.is_active = false;
                self.on_complete.send((self.id.clone(), Topos::Nil));
            }
        }

        if (self.on_activation)(&mut self.action, ctx) {
            self.fiber_request.send((self.id.clone(), Topos::Nil));
        }
    }
}

//
// # ActionBuilder
//

pub struct ActionBuilder<A> {
    id: Gram,
    mind: MindBuilder,
    ticker: TickerBuilder<ActionItem<A>>,
    request_sink: Sink,
    activate_sink: Sink,
}

impl<A:'static> ActionBuilder<A> {
    fn new(
        group: &mut ActionGroup,
        id: Gram, 
        action: A,
        on_action: Box<dyn Fn(&mut A, &mut Context) -> bool>
    ) -> Self {
        let mind = &mut group.mind;

        let ticker = mind.ticker(
            ActionItem::new(id.clone(), action, on_action)
        );
        
        let sink = ticker.sink(move |a, msg| {
            a.activate(msg.0, msg.1)
        });

        let mut builder = ActionBuilder {
            id: id,
            mind: mind.clone(),
            ticker: ticker,
            request_sink: group.request_sink.clone(),
            activate_sink: sink,
        };

        let mut source = builder.ticker.source(|item, fiber|
            item.on_complete = fiber,
        );

        source.to(&group.complete_sink);

        builder
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

    pub fn activator(
        &mut self, 
        activator: impl Fn(&mut A, &mut Context)->bool + 'static
    ) {
        let activator = Box::new(activator);
        self.ticker.write(|item| item.on_activation = activator);

        let mut source = self.ticker.source(
            |t, fiber| {
            t.fiber_request = fiber
        });

        source.to(&self.request_sink);
    }

    /*
    pub fn activator_item<A1:Action + 'static>(
            &mut self, 
            activator: A1
        ) -> ActivatorBuilder<A1> {
        let item = ActivatorBuilder::<A1>::new(
            self, 
            self.id.clone(),
            activator,
            Box::new(|a, ctx| a.action(ctx)),
        );

        item
    }
     */

    fn activate_sink(&mut self) -> Sink {
        self.activate_sink.clone()
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
