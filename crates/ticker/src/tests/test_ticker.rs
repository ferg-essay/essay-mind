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