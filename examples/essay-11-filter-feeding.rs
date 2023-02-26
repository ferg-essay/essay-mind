use mind::action::{SharedReader, Action};
use mind::{action, SharedWriter, Gram, Context, MindBuilder, Topos, gram, Fiber};
use mind::Ticker;

fn main() {
    let mut system = MindBuilder::new();

    let world = World::new();
    let filter = FilterIn::new(world.feeding_reader());

    let world = system.ticker(world);
    let sink = world.sink(|w, msg|
        w.request_filter(msg)
    );

    let mut group = action::ActionGroup::new(&mut system);
    let mut filter = group.action(filter);
    let mut source = filter.source(|a, fiber|
         a.fiber = fiber
    );

    source.to(&sink);

    let ext_source = system.external_source();
    ext_source.source().to(group.request());

    let mut system = system.build();

    let fiber = ext_source.fiber();

    system.tick();
    system.tick();
    system.tick();

    fiber.send((gram("filter"), Topos::Nil));

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
            if msg == gram("filter") {
                self.feeding.write(ctx.ticks()).unwrap().update(0.5);

            } else {
                panic!("world-request unknown gram {}", msg);
            }
        }
    }
}

//
// # FilterAction
//

struct FilterIn {
    id: Gram,
    feeding: SharedReader<FeedingPosition>,
    fiber: Fiber,
}

impl FilterIn {
    fn new(feeding: SharedReader<FeedingPosition>) -> Self {
        Self {
            id: gram("filter"),
            feeding,
            fiber: Default::default(),
        }
    }
}

impl Action for FilterIn {
    fn id(&self) -> &Gram {
        &self.id
    }

    fn action(&mut self, ctx: &mut Context) -> bool {
        let position = self.feeding.read(ctx.ticks()).unwrap().position;

        if position < 1. {
            self.fiber.send((gram("filter"), Topos::Nil));

            print!("{}:action {}\n", self.id, position);
            true
        } else {
            print!("{}:complete {}\n", self.id, position);

           false
        }
    }
}