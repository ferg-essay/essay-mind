use crate::{SystemBuilder, Ticker, system::Context};

#[test]
fn hello() {
    let mut system = SystemBuilder::<String>::new();

    let ticker = system.ticker(Test {});

    let mut system = system.build();

    system.tick();
}

struct Test {

}

impl Ticker for Test {
    fn tick(&mut self, tick: &mut Context) {
        print!("tick\n");
    }
}