use crate::{SystemBuilder, tests::AddItem, Ticker, Fiber, system::Context};

#[test]
fn ticker_set_fiber() {
    let mut builder = SystemBuilder::<String>::new();

    let mut adder = AddItem::new();

    let mut test_source = builder.ticker(TestSender::new(&adder));

    let mut source = test_source.source(move |t, fiber| {
        t.fiber = Some(fiber);
    });
    
    let test_sink = builder.ticker(TestSink::new(&adder));

    let sink = test_sink.sink(move |t, msg| {
        t.values.add(format!("msg({})", msg))
    });

    source.to(&sink);
    
    assert_eq!(adder.take(), "");

    let mut system = builder.build();

    assert_eq!(adder.take(), "");
    system.tick();

    assert_eq!(adder.take(), "source-tick(1), msg(hello), sink-tick(1)");
    system.tick();
    system.tick();

    assert_eq!(adder.take(), "source-tick(2), sink-tick(2), source-tick(3), sink-tick(3)");
}

struct TestSender {
    fiber: Option<Fiber<String>>,
    values: AddItem,
}

impl TestSender {
    fn new(values: &AddItem) -> Self {
        Self {
            fiber: None,
            values: values.clone(),
        }
    }

    fn add(&mut self, value: String) {
        self.values.add(value);
    }
}

impl Ticker for TestSender {
    fn tick(&mut self, ctx: &mut Context) {
        self.add(format!("source-tick({})", ctx.ticks()));

        if let Some(fiber) = &self.fiber {
            if ctx.ticks() == 1 {
                fiber.send("hello".to_string());
            }
        }
    }
}

struct TestSink {
    values: AddItem,
}

impl TestSink {
    fn new(values: &AddItem) -> Self {
        Self {
            values: values.clone(),
        }
    }

    fn add(&mut self, value: String) {
        self.values.add(value);
    }
}

impl Ticker for TestSink {
    fn tick(&mut self, ctx: &mut Context) {
        self.add(format!("sink-tick({})", ctx.ticks()));
    }
}
