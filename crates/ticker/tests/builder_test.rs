use std::{rc::Rc, cell::RefCell};

use ticker::SystemBuilder;

#[test]
fn empty_build() {
    let mut system = SystemBuilder::<i32>::new().build();
    system.tick();
}

#[test]
fn build_on_tick() {
    let mut builder = SystemBuilder::<i32>::new();

    let counter = Rc::new(RefCell::new(TestCounter { value: 0 }));
    let counter2 = Rc::clone(&counter);

    let ticker = builder.ticker();
    ticker.on_tick(move |_ticks| counter.borrow_mut().update());
    let mut system = builder.build();

    assert!(counter2.borrow().value == 0);
    system.tick();
    assert!(counter2.borrow().value == 1);
    system.tick();
    system.tick();
    assert!(counter2.borrow().value == 3);
}

struct TestCounter {
    value: u64,
}

impl TestCounter {
    fn update(&mut self) {
        self.value += 1;
    }
}

#[test]
fn item_builder() {
    let mut system = SystemBuilder::<i32>::new();

    let item = TestItem { value: 0};
    let test_builder = TestBuilder::new(&mut system, item);

    let mut system = system.build();

    system.tick();
    print!("help\n");
}

struct TestBuilder<T> {
    ptr: Rc<RefCell<TestBuilderInner<T>>>,
}

impl<T:Builder + 'static> TestBuilder<T> {
    fn new(system: &mut SystemBuilder<i32>, item: T) -> Self {
        let builder_ref = Rc::new(RefCell::new(TestBuilderInner { item }));

        let builder_ref2 = builder_ref.clone();

        let ticker = system.ticker();
        ticker.on_build(move || builder_ref2.borrow_mut().build());

        Self {
            ptr: builder_ref
        }
    }
}

struct TestBuilderInner<T> {
    item: T,
}

impl<T:Builder> TestBuilderInner<T> {
    fn build(&mut self) {
        self.item.build();
    }
}

trait Builder {
    fn build(&mut self);
}


struct TestItem {
    value: i32,
}

impl Builder for TestItem {
    fn build(&mut self) {
        self.value = 1;
    }
}
