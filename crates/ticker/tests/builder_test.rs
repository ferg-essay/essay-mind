use ticker::{SystemBuilder, Ticker};

#[test]
fn empty_build() {
    let mut system = SystemBuilder::<i32>::new().build();
    system.tick();
}

#[test]
fn ticker_on_build_on_tick() {
    let mut builder = SystemBuilder::<i32>::new();

    let ticker = builder.ticker(TestAdder::new());
    let counter_ptr = ticker.ptr();
    
    assert_eq!(counter_ptr.borrow_mut().take(), "[]");

    let mut system = builder.build();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"build\"]");
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"tick(1)\"]");
    system.tick();
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"tick(2)\", \"tick(3)\"]");
}

struct TestAdder {
    values: Vec<String>,
}

impl TestAdder {
    fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    fn add(&mut self, value: String) {
        self.values.push(value);
    }

    fn peek(&self) -> String {
        format!("{:?}", self.values)
    }

    fn take(&mut self) -> String {
        let msg = self.peek();

        self.values.drain(..);

        msg
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

#[test]
fn external_fiber_with_fiber_to() {
    let mut builder = SystemBuilder::<i32>::new();

    let ticker = builder.ticker(TestAdder::new());
    let counter_ptr = ticker.ptr();

    let mut fiber = builder.external_fiber();
    let ptr = ticker.ptr();

    fiber.to(&ticker, move |id, msg| {
        ptr.borrow_mut().add(format!("on_fiber({}, {})", id, msg));
    });
    
    assert_eq!(counter_ptr.borrow_mut().take(), "[]");

    let mut system = builder.build();

    let fiber = fiber.fiber();
    fiber.send(27);

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"build\"]");
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"on_fiber(0, 27)\", \"tick(1)\"]");
    system.tick();
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"tick(2)\", \"tick(3)\"]");
}

#[test]
fn external_fiber_with_ticker_on_fiber() {
    let mut builder = SystemBuilder::<i32>::new();

    let ticker = builder.ticker(TestAdder::new());
    let counter_ptr = ticker.ptr();

    let mut fiber = builder.external_fiber();
    let ptr = ticker.ptr();

    ticker.on_fiber(
        &mut fiber,
move |id, msg| {
            ptr.borrow_mut().add(format!("on_fiber({}, {})", id, msg));
        }
    );
    
    assert_eq!(counter_ptr.borrow_mut().take(), "[]");

    let mut system = builder.build();

    let fiber = fiber.fiber();
    fiber.send(27);

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"build\"]");
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"on_fiber(0, 27)\", \"tick(1)\"]");
    system.tick();
    system.tick();

    assert_eq!(counter_ptr.borrow_mut().take(), "[\"tick(2)\", \"tick(3)\"]");
}
