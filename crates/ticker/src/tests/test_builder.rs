use crate::{SystemBuilder, Ticker, system::Context};

use super::AddItem;

#[test]
fn empty_build() {
    let mut system = SystemBuilder::<i32>::new().build();
    system.tick();
}

#[test]
fn ticker_on_build_on_tick() {
    let mut builder = SystemBuilder::<i32>::new();

    let mut adder = AddItem::new();
    builder.ticker(TestAdder::new(&adder));
    
    assert_eq!(adder.take(), "");

    let mut system = builder.build();

    assert_eq!(adder.take(), "build");
    system.tick();

    assert_eq!(adder.take(), "tick(1)");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "tick(2), tick(3)");
}

#[test]
fn ticker_set_fiber() {
    let mut builder = SystemBuilder::<i32>::new();

    let mut adder = AddItem::new();
    let mut ticker = builder.ticker(TestAdder::new(&adder));

    ticker.source(move |t, _| {
        t.add(format!("set_fiber"))
    });
    
    assert_eq!(adder.take(), "");

    let mut system = builder.build();

    assert_eq!(adder.take(), "set_fiber, build");
    system.tick();

    assert_eq!(adder.take(), "tick(1)");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "tick(2), tick(3)");
}

#[test]
fn ticker_read() {
    let mut builder = SystemBuilder::<i32>::new();
    let test = builder.node(TestItem::new("test1"));

    assert_eq!(test.read(|t| t.value.clone()), "test1")
}

#[test]
fn ticker_write() {
    let mut builder = SystemBuilder::<i32>::new();
    let mut test = builder.node(TestItem::new("test1"));

    assert_eq!(test.write(|t| { t.value = String::from("inner"); t.value.clone() }), "inner");

    builder.build();
    let test = test.unwrap();
    assert_eq!(test.read(|t| t.value.clone()), "inner");
}

#[test]
fn external_fiber_with_fiber_to() {
    let mut builder = SystemBuilder::<i32>::new();

    let mut adder = AddItem::new();
    let ticker = builder.ticker(TestAdder::new(&adder));

    let source = builder.external_source();

    let sink = ticker.sink(move |t, msg| {
        t.add(format!("on_fiber({})", msg));
    });

    source.source().to(&sink);
    // ticker.lock().unwrap().take()
    assert_eq!(adder.take(), "");

    let mut system = builder.build();
    // ticker = ticker.unwrap();
    // ticker.lock().unwrap().take()
    let fiber = source.fiber();
    fiber.send(27);

    assert_eq!(adder.take(), "build");
    system.tick();

    assert_eq!(adder.take(), "on_fiber(27), tick(1)");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "tick(2), tick(3)");
}


struct TestAdder {
    values: AddItem,
}

impl TestAdder {
    fn new(values: &AddItem) -> Self {
        Self {
            values: values.clone(),
        }
    }

    fn add(&mut self, value: String) {
        self.values.add(value);
    }
}

impl Ticker for TestAdder {
    fn tick(&mut self, ctx: &mut Context) {
        self.add(format!("tick({})", ctx.ticks()));
    }

    fn build(&mut self) {
        self.add(format!("build"));
    }
}

struct TestItem {
    value: String,
}

impl TestItem {
    fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }
}
