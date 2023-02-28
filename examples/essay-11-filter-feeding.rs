use mind::action::{SharedReader, Action};
use mind::{action, SharedWriter, Gram, Context, MindBuilder, Topos, gram, Fiber};
use mind::Ticker;

fn main() {
    let mut system = MindBuilder::new();

    let world = World::new();

    let filter_in = FilterIn::new(world.feeding_reader());
    let filter_in_sense = FilterInSense::new(world.feeding_reader());

    let filter_out = FilterOut::new(world.feeding_reader());
    let filter_out_sense = FilterOutSense::new(world.feeding_reader());

    let world = system.ticker(world);
    let world_sink = world.sink(|w, msg|
        w.request_filter(msg)
    );

    let mut group = action::ActionGroup::new(&mut system);

    let mut filter_in = group.action(gram("filter-in"), filter_in);
    let mut source = filter_in.source(
        |a, fiber|
        a.world_fiber = fiber
    );

    source.to(&world_sink);

    filter_in.activator(filter_in_sense);

    let mut filter_out = group.action(gram("filter-out"), filter_out);
    let mut source = filter_out.source(
        |a, fiber|
        a.world_fiber = fiber
    );

    source.to(&world_sink);

    filter_out.activator(filter_out_sense);

    //let ext_source = system.external_source();
    //ext_source.source().to(group.request());

    let mut system = system.build();

    //let fiber = ext_source.fiber();

    system.tick();
    system.tick();
    system.tick();
    system.tick();
    system.tick();
    system.tick();
    system.tick();
    system.tick();
}

//
// # FeedingPosition
//

/// Proprioception for the filter feeding position.
#[derive(Default,Debug)]
struct FeedingPosition {
    position: f32,
}

impl FeedingPosition {
    fn update(&mut self, delta: f32) {
        self.position = self.position + delta;

        if self.position < 0. {
            self.position = 0.;
        } else if self.position > 1. {
            self.position = 1.;
        }
    }
}

//
// # World
//

struct World {
    feeding: SharedWriter<FeedingPosition>,
    request: Option<Gram>,
}

impl World {
    fn new() -> Self {
        Self {
            feeding: SharedWriter::new(),
            request: None,
        }
    }

    fn feeding_reader(&self) -> SharedReader<FeedingPosition> {
        self.feeding.reader()
    }

    fn request_filter(&mut self, msg: (Gram, Topos)) {
        self.request = Some(msg.0);
    }
}

impl Ticker for World {
    fn tick(&mut self, ctx: &mut Context) {
        if let Some(msg) = self.request.take() {
            if msg == gram("filter-in") {
                self.feeding.write(ctx.ticks()).unwrap().update(0.25);
                print!("{}:world-filter-in {}\n", ctx.ticks(),
                self.feeding.write(ctx.ticks()).unwrap().position);
            } else if msg == gram("filter-out") {
                self.feeding.write(ctx.ticks()).unwrap().update(-0.25);
                print!("{}:world-filter-out {}\n", ctx.ticks(),
                    self.feeding.write(ctx.ticks()).unwrap().position);
            } else {
                panic!("world-request unknown gram {}", msg);
            }
        }
    }
}

//
// # FilterIn
//

struct FilterIn {
    id: Gram,
    feeding: SharedReader<FeedingPosition>,
    world_fiber: Fiber,
}

impl FilterIn {
    fn new(feeding: SharedReader<FeedingPosition>) -> Self {
        Self {
            id: gram("filter-in"),
            feeding,
            world_fiber: Default::default(),
        }
    }
}

impl Action for FilterIn {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position < 1. {
            self.world_fiber.send((gram("filter-in"), Topos::Nil));

            print!("{}:action {}\n", self.id, position);
            true
        } else {
            print!("{}:complete {}\n", self.id, position);

           false
        }
    }
}


//
// # FilterInSense
//

struct FilterInSense {
    feeding: SharedReader<FeedingPosition>,
    request_fiber: Fiber,
}

impl FilterInSense {
    fn new(feeding: SharedReader<FeedingPosition>) -> Self {
        Self {
            feeding,
            request_fiber: Default::default(),
        }
    }
}

impl Action for FilterInSense {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position < 1. {
            //self.request_fiber.send((gram("filter-in"), Topos::Nil));

            true
        } else {
           false
        }
    }
}


//
// # FilterOut
//

struct FilterOut {
    id: Gram,
    feeding: SharedReader<FeedingPosition>,
    world_fiber: Fiber,
}

impl FilterOut {
    fn new(feeding: SharedReader<FeedingPosition>) -> Self {
        Self {
            id: gram("filter-out"),
            feeding,
            world_fiber: Default::default(),
        }
    }
}

impl Action for FilterOut {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position > 0. {
            self.world_fiber.send((gram("filter-out"), Topos::Nil));

            print!("{}:action {}\n", self.id, position);
            true
        } else {
            print!("{}:complete {}\n", self.id, position);

           false
        }
    }
}


//
// # FilterOutSense
//

struct FilterOutSense {
    feeding: SharedReader<FeedingPosition>,
    request_fiber: Fiber,
}

impl FilterOutSense {
    fn new(feeding: SharedReader<FeedingPosition>) -> Self {
        Self {
            feeding,
            request_fiber: Default::default(),
        }
    }
}

impl Action for FilterOutSense {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position > 0. {
            //self.request_fiber.send((gram("filter-out"), Topos::Nil));

            true
        } else {
           false
        }
    }
}
