use crate::{SystemBuilder, Ticker, system::Context};

#[test]
fn test_ticker_read() {
    let mut system = SystemBuilder::<String>::new();

    let ticker = system.ticker(Test { tick: 0 });

    let mut system = system.build();

    system.tick();

    let ticker = ticker.unwrap();

    assert_eq!(ticker.read(|t| t.read1()), "read1-call(1)");
    assert_eq!(ticker.read(|t| t.read2()), "read2-call(1)");
    system.tick();
    assert_eq!(ticker.read(|t| t.read1()), "read1-call(2)");
    assert_eq!(ticker.read(|t| t.read2()), "read2-call(2)");
}

#[test]
fn test_node_tick_and_write() {
    let mut system = SystemBuilder::<String>::new();

    let ticker = system.node(Node { value: Vec::new() });
    ticker.on_tick(move |t, ctx| t.my_tick(ctx.ticks()));

    let mut system = system.build();

    system.tick();

    let ticker = ticker.unwrap();

    assert_eq!(ticker.write(|t| t.take()), "my-tick(1)");
    assert_eq!(ticker.write(|t| t.take()), "");
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "my-tick(2)");
    system.tick();
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "my-tick(3), my-tick(4)");
}

struct Node {
    value: Vec<String>,
}

impl Node {
    fn my_tick(&mut self, ticks: u64) {
        self.value.push(format!("my-tick({})", ticks));
    }

    fn take(&mut self) -> String {
        let value = self.value.join(", ");

        self.value.drain(..);

        value
    }
}

struct Test {
    tick: u64,
}

impl Test {
    fn read1(&self) -> String {
        format!("read1-call({})", self.tick)
    }

    fn read2(&self) -> String {
        format!("read2-call({})", self.tick)
    }
}

impl Ticker for Test {

    fn tick(&mut self, ctx: &mut Context) {
        self.tick = ctx.ticks();
    }
}