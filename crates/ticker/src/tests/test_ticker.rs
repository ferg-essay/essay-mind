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
fn node_on_tick_and_write() {
    let mut system = SystemBuilder::<String>::new();

    let mut ticker = system.node(Node::new("my-tick"));
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

#[test]
fn node_step_2() {
    let mut system = SystemBuilder::<String>::new();

    let mut ticker = system.node(Node::new("a"));
    ticker.on_tick(move |t, ctx| t.my_tick(ctx.ticks()));
    ticker.step(2);

    let mut system = system.build();
    let ticker = ticker.unwrap();

    system.tick();

    assert_eq!(ticker.write(|t| t.take()), "");
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "a(2)");
    system.tick();
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "a(4)");

    for _ in 5..16 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()), "a(6), a(8), a(10), a(12), a(14)");

    for _ in 16..32 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(16), a(18), a(20), a(22), a(24), a(26), a(28), a(30)"
    );

    for _ in 32..48 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(32), a(34), a(36), a(38), a(40), a(42), a(44), a(46)"
    );

    for _ in 48..64 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(48), a(50), a(52), a(54), a(56), a(58), a(60), a(62)"
    );

    for _ in 64..80 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(64), a(66), a(68), a(70), a(72), a(74), a(76), a(78)"
    );
}

#[test]
fn node_step_4_offset_1() {
    let mut system = SystemBuilder::<String>::new();

    let mut ticker = system.node(Node::new("a"));
    ticker.on_tick(move |t, ctx| t.my_tick(ctx.ticks()));
    ticker.step(4);
    ticker.offset(1);

    let mut system = system.build();
    let ticker = ticker.unwrap();

    system.tick();

    assert_eq!(ticker.write(|t| t.take()), "a(1)");
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "");
    system.tick();
    system.tick();
    assert_eq!(ticker.write(|t| t.take()), "");

    for _ in 5..16 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()), "a(5), a(9), a(13)");

    for _ in 16..32 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(17), a(21), a(25), a(29)"
    );

    for _ in 32..48 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(33), a(37), a(41), a(45)"
    );

    for _ in 48..64 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(49), a(53), a(57), a(61)"
    );

    for _ in 64..80 {
        system.tick();
    }
    assert_eq!(ticker.write(|t| t.take()),
        "a(65), a(69), a(73), a(77)"
    );
}

struct Node {
    name: String,
    value: Vec<String>,
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: String::from(name),
            value: Vec::new(),
        }
    }

    fn my_tick(&mut self, ticks: u64) {
        self.value.push(format!("{}({})", self.name, ticks));
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