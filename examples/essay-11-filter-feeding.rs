use mind::action::{SharedReader, Action};
use mind::{action, SharedWriter, Gram, Context, MindBuilder, Topos, gram, Fiber};
use mind::Ticker;

fn main() {
    let mut system = MindBuilder::new();

    let world = World::new();

    let filter_in = FilterIn::new(world.feeding_reader());

    let filter_out = FilterOut::new(world.feeding_reader());

    let world = system.ticker(world);
    let world_sink = world.sink(|w, msg|
        w.request_filter(msg)
    );

    let mut group = action::ActionGroup::new(&mut system);

    let mut filter_in = group.action(gram("filter-in"), filter_in);
    filter_in.source(
        |a, fiber|
        a.world_fiber = fiber
    ).to(&world_sink);

    filter_in.activator(|a, ctx| a.activator(ctx));

    let mut filter_out = group.action(gram("filter-out"), filter_out);
    filter_out.source(
        |a, fiber|
        a.world_fiber = fiber
    ).to(&world_sink);

    filter_out.activator(|a, ctx| a.activator(ctx));

    let mut system = system.build();

    for _ in 0..24 {
        system.tick();
    }
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
    fn update(&mut self, pos: f32) {
        self.position = pos;

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

    pos: f32,
}

impl World {
    fn new() -> Self {
        Self {
            feeding: SharedWriter::new(),
            request: None,
            pos: 0.
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
                self.pos += 0.25;
                if self.pos > 1. {
                    self.pos = 1.
                }
                self.feeding.write(ctx.ticks()).unwrap().update(self.pos);
                print!("{}:world-filter-in {}\n", ctx.ticks(),
                self.feeding.write(ctx.ticks()).unwrap().position);
            } else if msg == gram("filter-out") {
                self.pos -= 0.25;
                if self.pos < 0. {
                    self.pos = 0.
                }
                self.feeding.write(ctx.ticks()).unwrap().update(self.pos);
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

    fn activator(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position < 0.75 {
            true
        } else {
           false
        }
    }
}

impl Action for FilterIn {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position < 1. {
            self.world_fiber.send((self.id.clone(), Topos::Nil));

            print!("  {}:action {}\n", self.id, position);
            true
        } else {
            print!("  {}:complete {}\n", self.id, position);

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

    fn activator(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position > 0.25 {
            true
        } else {
           false
        }
    }
}

impl Action for FilterOut {
    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position > 0. {
            self.world_fiber.send((gram("filter-out"), Topos::Nil));

            print!("  {}:action {}\n", self.id, position);
            true
        } else {
            print!("  {}:complete {}\n", self.id, position);

           false
        }
    }
}
