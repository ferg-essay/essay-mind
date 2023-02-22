use mind::gram::Gram;
use mind::{Topos};
use ticker::{SystemBuilder, Context};
use ticker::Ticker;

type TestArgs = (Gram, Topos);

#[test]
fn basic() {
    let mut system = SystemBuilder::<TestArgs>::new();
    let ticker = system.ticker(
        TestTicker { val: String::new()}
    );
    /*
    let test = ticker.ptr();

    let on_call = ticker.ptr();

    let mut fiber = system.external_fiber();
    
    ticker.on_fiber(&mut fiber, move |id, args| {
        on_call.borrow_mut().call(id, args);
    });

    let mut system = system.build();
    let fiber = fiber.fiber();
    fiber.send((Gram::from("msg"), Topos::Nil, ));
    system.tick();

    assert_eq!(test.borrow_mut().take(), "id:0 call((g\"msg\", Nil))");
    */
}

struct TestTicker {
    val: String,
}

impl TestTicker {
    fn call(&mut self, id: usize, args: TestArgs) {
        self.val = format!("id:{} call({:?})", id, args);
    }

    fn take(&mut self) -> String {
        let v = self.val.clone();
        self.val = String::new();
        v
    }
}

impl Ticker for TestTicker {
    fn tick(&mut self, _ticks: &mut Context) {
    }

    fn build(&mut self) {
    }
}
