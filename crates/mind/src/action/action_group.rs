use std::collections::HashMap;

use ticker::Ticker;

use crate::{TickerBuilder, Source, Sink, MindBuilder, MindMessage, Gram, gram, Topos, TickerPtr, Fiber};


pub struct ActionGroup {
    mind: MindBuilder,

    actions: Vec<Box<dyn ActionTrait>>,

    ticker: TickerBuilder<ActionGroupInner>,

    sink: Sink,
}

struct ActionTicker {
    
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
            actions: Vec::new(),
            ticker: ticker,
            sink: sink,
        }
    }

    pub fn action<A:'static>(&mut self, id: Gram, action: A) -> ActionBuilder<A> {
        let name = id.clone();

        let mut item = ActionBuilder::new(&mut self.mind, id, action);

        let sink = item.activate_sink();
        let mut source = self.ticker.source(move |g, fiber| {
            g.add_action(name, fiber);
        });

        source.to(&sink);

        item
    }

    pub fn request(&mut self) -> &Sink {
        &self.sink
    }

    /*
    fn push<A:Action>(&mut self, action: A) {
        //let mut ptr = self.ticker.ptr();
        action.on_complete().to(&self.ticker.sink(move |t: &mut ActionTicker, msg| {
            t.complete(msg.0, msg.1);
        }));
        let item = ActionItem {};


    }
     */
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

    fn request(&mut self, gram: Gram, topos: Topos) {
        self.requests.push(gram);
        //self.action_map.insert(gram, fiber);
    }
}

impl Ticker for ActionGroupInner {
    fn tick(&mut self, ctx: &mut ticker::Context) {
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

pub struct ActionBuilder<A> {
    ticker: TickerBuilder<ActionItem<A>>,
}

impl<A:'static> ActionBuilder<A> {
    fn new(mind: &mut MindBuilder, id: Gram, action: A) -> Self {
        ActionBuilder {
            ticker: mind.ticker(ActionItem::new(id, action))
        }
    }

    pub fn on_action<CB:'static>(&mut self, fun: CB)
        where CB: Fn(&mut A, &mut ticker::Context) -> bool
    {
        let fun2: Box<OnAction<A>> = Box::new(fun);
        //let holder = ActionHolder { on_action: fun };
        //self.ticker.write(move |t| t.on_action = fun2);
        //let holder: Holder<A> = Box::new(Holder { write: |a| a.on_action = fun2});

        self.ticker.write(|t: &mut ActionItem<A>| t.on_action = fun2);

    }

    fn activate_sink(&mut self) -> Sink {
        self.ticker.sink(move |a, msg| {
            a.activate(msg.0, msg.1)
        })
    }

    pub fn unwrap(mut self) -> ActionReader<A> {
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

struct ActionHolder<A> {
    on_action: Box<OnAction<A>>,
}

struct ActionItem<A> {
    id: Gram,
    action: A,
    on_action: Box<OnAction<A>>,
    is_active: bool,
}

impl<A> ActionItem<A> {
    fn new(id: Gram, action: A) -> Self {
        ActionItem {
            id: id,
            action: action,
            on_action: Box::new(|x, c| false),
            is_active: false,
        }
    }

    pub fn activate(&mut self, gram: Gram, topos: Topos) {
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

impl<A> ActionTrait for ActionItem<A> {
    fn on_action(&mut self, ctx: &mut ticker::Context) -> bool {
        self.on_action(ctx)
    }
}

struct ActionFun<A> {
    on_action: Box<OnAction<A>>,
}

impl ActionTicker {
    fn enhance(&self, index: usize, msg: MindMessage) {
        
    }

    fn complete(&self, gram: Gram, topos: Topos) {
        
    }
}


impl Ticker for ActionTicker {
    fn tick(&mut self, ctx: &mut ticker::Context) {

    }
}