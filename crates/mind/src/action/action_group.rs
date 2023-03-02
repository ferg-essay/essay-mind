use std::collections::HashMap;

use ticker::{Ticker, Context};

use crate::{TickerBuilder, Source, Sink, MindBuilder, MindMessage, Gram, Topos, TickerPtr, Fiber};

pub trait Action {
    // fn id(&self) -> &Gram;
    fn action(&mut self, topos: Topos, ctx: &mut ticker::Context) -> bool;
}

type OnAction<A> = dyn Fn(&mut A, Topos, &mut ticker::Context)->bool;

trait ActionTrait {
    fn on_action(&mut self, topos: Topos, ctx: &mut ticker::Context) -> bool;
}

pub struct ActionGroup {
    mind: MindBuilder,

    ticker: TickerBuilder<ActionGroupInner>,

    request_sink: Sink,

    complete_sink: Sink,

    modulate_sink: Sink,
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

        let modulate_sink = ticker.sink(move |t, msg| {
            t.modulate(msg.0, msg.1);
        });

        ActionGroup {
            mind: mind.clone(),

            ticker,
            request_sink,
            complete_sink,

            modulate_sink,
        }
    }

    pub fn node<A:'static>(
        &mut self, 
        id: Gram, 
        action: A,
        on_action: impl Fn(&mut A, Topos, &mut Context) -> bool + 'static
    ) -> ActionBuilder<A> {
        let name = id.clone();

        self.ticker.write(|g| g.add_action(id.clone()));

        let mut item = ActionBuilder::new(
            self, 
            id, 
            action, 
            Box::new(on_action)
        );

        let sink = item.activate_sink();
        let mut source = self.ticker.source(
            move |g, fiber| {
            g.set_fiber(name, fiber);
        });

        source.to(&sink);

        item
    }

    pub fn action<A:Action + 'static>(&mut self, id: Gram, action: A) -> ActionBuilder<A> {
        self.node(
            id,
            action, 
            |a, topos, ctx| 
            a.action(topos, ctx)
        )
    }

    pub fn request(&mut self) -> &Sink {
        &self.request_sink
    }

    pub fn modulate(&mut self) -> &Sink {
        &self.modulate_sink
    }

    pub fn decay(&mut self, decay: f32) -> &Self {
        assert!(0. <= decay && decay <= 1.);

        self.ticker.write(|g| g.decay = decay);

        self
    }
}

struct ActionGroupInner {
    action_map: HashMap<Gram,ActionItemInner>,
    requests: Vec<Gram>,
    is_active: bool,

    decay: f32,
    plateau: f32,
}

struct ActionItemInner {
    name: Gram,
    fiber: Fiber,
    is_modulated: bool,
}

impl ActionGroupInner {
    fn new() -> Self {
        Self {
            action_map: HashMap::new(),
            requests: Vec::new(),
            is_active: false,
            decay: 0.5,
            plateau: 0.5,
        }
    }

    fn add_action(
        &mut self, 
        gram: Gram, 
    ) {
        let item = ActionItemInner {
            name: gram.clone(),
            fiber: Default::default(),
            is_modulated: false,
        };

        self.action_map.insert(gram, item);
    }

    fn set_fiber(
        &mut self, 
        gram: Gram, 
        fiber: Fiber,
    ) {
        if let Some(item) = self.action_map.get_mut(&gram) {
            item.fiber = fiber
        } else {
            panic!("Unassigned action {:?}", gram)
        }
    }

    fn set_modulated(
        &mut self,
        gram: Gram,
        is_modulated: bool,
    ) {
        if let Some(item) = self.action_map.get_mut(&gram) {
            item.is_modulated = is_modulated
        } else {
            panic!("Unassigned action {:?}", gram)
        }
    }

    fn request(&mut self, gram: Gram, _topos: Topos) {
        self.requests.push(gram);
        //self.action_map.insert(gram, fiber);
    }

    fn complete(&mut self, _gram: Gram, _topos: Topos) {
        self.is_active = false;
    }

    fn modulate(&mut self, _gram: Gram, topos: Topos) {
        match topos {
            Topos::Unit(value) => {
                assert!(0. <= value && value <= 1.);
                self.plateau = value;
            }
            _ => {
                panic!("Unexpected topos type {:?} to the modulation", topos);
            }
        }
    }

    fn select_action(&mut self) -> Option<&ActionItemInner> {
        let mut best_item = None;
        let mut best_power = 0.0f32;

        for gram in self.requests.drain(..) {
            match self.action_map.get(&gram) {
                Some(item) => {
                    let mut power = if item.is_modulated { self.plateau } else { 0.5 };

                    // cutoff for monoamine effect on selection
                    let cutoff = 0.05;
                    if 0.5 - cutoff < power && power < 0.5 + cutoff {
                        power = 0.5;
                    }

                    if best_power < power {
                        best_power = power;
                        best_item = Some(item);
                    }
                }
                None => {}
            }
        }

        best_item
    }
}

impl Ticker for ActionGroupInner {
    fn tick(&mut self, _: &mut ticker::Context) {
        let plateau = self.plateau;

        self.plateau = (plateau - 0.5) * self.decay + 0.5;

        if self.is_active {
            return
        }

        let is_active = match self.select_action() {
            Some(item) => {
                let plateau = if item.is_modulated { plateau } else { 0.5 };
                item.fiber.send((item.name.clone(), Topos::Unit(plateau)));
                true
            }
            None => false
        };

        self.is_active = is_active;
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

    on_activation: Box<OnAction<A>>,
    fiber_request: Fiber,

    is_active: bool,
    topos: Topos,
}

impl<A> ActionItem<A> {
    fn new(id: Gram, action: A, on_action: Box<OnAction<A>>) -> Self {
        ActionItem {
            id,
            action: action,
            on_action: on_action,
            on_complete: Default::default(),

            on_activation: Box::new(|_, _, _| false),
            fiber_request: Default::default(),

            is_active: false,
            topos: Topos::Nil,
        }
    }

    pub fn activate(&mut self, _gram: Gram, topos: Topos) {
        self.is_active = true;
        self.topos = topos;
    }
}


impl<A> Ticker for ActionItem<A> {
    fn tick(&mut self, ctx: &mut ticker::Context) {
        if self.is_active {
            if ! (self.on_action)(&mut self.action, self.topos.clone(), ctx) {
                self.is_active = false;
                self.topos = Topos::Nil;
                self.on_complete.send((self.id.clone(), Topos::Nil));
            }
        }

        if (self.on_activation)(&mut self.action, Topos::Nil, ctx) {
            self.fiber_request.send((self.id.clone(), Topos::Nil));
        }
    }
}

//
// # ActionBuilder
//

pub struct ActionBuilder<A> {
    id: Gram,
    _mind: MindBuilder,
    group: TickerBuilder<ActionGroupInner>,
    ticker: TickerBuilder<ActionItem<A>>,
    request_sink: Sink,
    activate_sink: Sink,
}

impl<A:'static> ActionBuilder<A> {
    fn new(
        group: &mut ActionGroup,
        id: Gram, 
        action: A,
        on_action: Box<dyn Fn(&mut A, Topos, &mut Context) -> bool>
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
            _mind: mind.clone(),
            group: group.ticker.clone(),
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
        activator: impl Fn(&mut A, Topos, &mut Context)->bool + 'static
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

    pub fn set_modulated(&mut self, is_modulated: bool) {
        self.group.write(|g| 
            g.set_modulated(self.id.clone(), is_modulated)
        );
    }

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
