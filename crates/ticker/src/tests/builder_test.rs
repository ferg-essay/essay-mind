use crate::{SystemBuilder, Ticker};

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

    ticker.source(move |t, fiber| {
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
fn external_fiber_with_fiber_to() {
    let mut builder = SystemBuilder::<i32>::new();

    let mut adder = AddItem::new();
    let ticker = builder.ticker(TestAdder::new(&adder));

    let mut source = builder.external_source();

    let sink = ticker.sink(move |t, msg| {
        t.add(format!("on_fiber({})", msg));
    });

    source.source().to(&sink);
    
    assert_eq!(adder.take(), "");

    let mut system = builder.build();

    let fiber = source.fiber();
    fiber.send(27);

    assert_eq!(adder.take(), "build");
    system.tick();

    assert_eq!(adder.take(), "on_fiber(27), tick(1)");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "tick(2), tick(3)");
}

#[test]
fn external_fiber_with_ticker_on_fiber() {
    let mut builder = SystemBuilder::<i32>::new();

    let mut adder = AddItem::new();
    let ticker = builder.ticker(TestAdder::new(&adder));
    //let counter_ptr = ticker.ptr();

    let mut ext_source = builder.external_source();
    //let ptr = ticker.ptr();

    let sink = ticker.sink(move |t, msg| {
            t.add(format!("on_fiber({})", msg));
        }
    );

    ext_source.source().to(&sink);
    
    assert_eq!(adder.take(), "");

    let mut system = builder.build();

    let fiber = ext_source.fiber();
    fiber.send(27);

    assert_eq!(adder.take(), "build");
    system.tick();

    assert_eq!(adder.take(), "[\"on_fiber(0, 27)\", \"tick(1)\"]");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "[\"tick(2)\", \"tick(3)\"]");
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

    fn peek(&self) -> String {
        self.values.peek()
    }

    fn take(&mut self) -> String {
        self.values.take()
    }
}

impl Ticker for TestAdder {
    fn tick(&mut self, ticks: u64) {
        self.add(format!("tick({})", ticks));
    }

    fn build(&mut self) {
        self.add(format!("build"));
    }
}
