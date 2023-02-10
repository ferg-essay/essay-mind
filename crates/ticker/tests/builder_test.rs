use ticker::{SystemBuilder, Ticker};

#[test]
fn empty_build() {
    let mut system = SystemBuilder::<i32>::new().build();
    system.tick();
}

#[test]
fn build_on_build_on_tick() {
    let mut builder = SystemBuilder::<i32>::new();

    let ticker = builder.ticker(TestAdder::new());
    let counter_ptr = ticker.ptr();
    
    assert!(counter_ptr.borrow_mut().take() == "[]");

    let mut system = builder.build();

    assert!(counter_ptr.borrow_mut().take() == "[\"build\"]");
    system.tick();

    assert!(counter_ptr.borrow_mut().take() == "[\"tick(1)\"]");
    system.tick();
    system.tick();

    assert!(counter_ptr.borrow_mut().take() == "[\"tick(2)\", \"tick(3)\"]");
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
        self.values.push(format!("tick({})", ticks));
    }

    fn build(&mut self) {
        self.values.push(format!("build"));
    }
}
