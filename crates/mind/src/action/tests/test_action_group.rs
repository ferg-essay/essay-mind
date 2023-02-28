use ticker::Context;

use crate::{gram, Topos, Gram};
use crate::{MindBuilder};
use crate::action::action_group::{ActionGroup, Action};

#[test]
fn action_node() {
    let mut builder = MindBuilder::new();
    let mut action = TestAction::new("action");
    action.max(4);
    let mut group = ActionGroup::new(&mut builder);
    let action = group.node(
        gram("a"), 
        action,
        |a, ctx| { a.action(ctx) }
    );
    let ext_source = builder.external_source();
    ext_source.source().to(group.request());
    
    let mut system = builder.build();

    let fiber = ext_source.fiber();

    let mut ptr = action.unwrap();

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
}

#[test]
fn action_trait() {
    let mut builder = MindBuilder::new();
    let mut action = TestAction::new("action");
    action.max(4);
    let mut group = ActionGroup::new(&mut builder);
    let action = group.action(gram("action"), action);

    let ext_source = builder.external_source();
    ext_source.source().to(group.request());
    
    let mut system = builder.build();

    let fiber = ext_source.fiber();

    let mut ptr = action.unwrap();

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

    fiber.send((gram("action"), Topos::Nil));
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
}

#[test]
fn action_choice() {
    let mut builder = MindBuilder::new();
    let mut group = ActionGroup::new(&mut builder);

    let mut a = TestAction::new("a");
    a.max(2);
    let a = group.action(gram("a"), a);

    let mut b = TestAction::new("b");
    b.max(2);
    let b = group.action(gram("b"), b);

    let ext_source = builder.external_source();
    ext_source.source().to(group.request());
    
    let mut system = builder.build();

    let fiber = ext_source.fiber();

    let mut a = a.unwrap();
    let mut b = b.unwrap();

    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|b| b.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");

    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-start(1)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-end(2)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();

    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();

    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();

    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-start(1)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-end(2)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
}

#[test]
fn action_competition() {
    let mut builder = MindBuilder::new();

    let mut group = ActionGroup::new(&mut builder);

    let mut a = TestAction::new("a");
    a.max(2);
    let a = group.action(gram("a"), a);

    let mut b = TestAction::new("b");
    b.max(2);
    let b = group.action(gram("b"), b);
    
    let ext_source = builder.external_source();
    ext_source.source().to(group.request());
    
    let mut system = builder.build();

    let fiber = ext_source.fiber();

    let mut a = a.unwrap();
    let mut b = b.unwrap();

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|b| b.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();

    fiber.send((gram("b"), Topos::Nil));
    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-start(1)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-end(2)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
}

//
// Test that simultaneous, timed requests are only evaluated when the
// actions complete.
//
#[test]
fn action_competition_async() {
    let mut builder = MindBuilder::new();

    let mut group = ActionGroup::new(&mut builder);

    let mut a = TestAction::new("a");
    a.max(2);
    let a = group.action(gram("a"), a);

    let mut b = TestAction::new("b");
    b.max(2);
    let b = group.action(gram("b"), b);
    
    let ext_source = builder.external_source();
    ext_source.source().to(group.request());
    
    let mut system = builder.build();

    let fiber = ext_source.fiber();

    let mut a = a.unwrap();
    let mut b = b.unwrap();

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|b| b.take()), "");
    
    fiber.send((gram("b"), Topos::Nil));
    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-start(1)");

    fiber.send((gram("b"), Topos::Nil));
    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-end(2)");

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-start(1)");

    fiber.send((gram("a"), Topos::Nil));
    fiber.send((gram("b"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "b-end(2)");

    fiber.send((gram("b"), Topos::Nil));
    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1)");
    assert_eq!(b.write(|a| a.take()), "");

    fiber.send((gram("b"), Topos::Nil));
    fiber.send((gram("a"), Topos::Nil));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2)");
    assert_eq!(b.write(|a| a.take()), "");
}

struct TestAction {
    name: Gram,
    values: Vec<String>,
    count: u64,
    max: u64, 
}

impl TestAction {
    fn new(str: &str) -> Self {
        Self {
            name: Gram::from(str),
            values: Vec::new(),
            count: 0,
            max: 2,
        }
    }

    fn max(&mut self, time: u64) {
        self.max = time;
    }

    fn add(&mut self, msg: String) {
        self.values.push(msg);
    }

    fn take(&mut self) -> String {
        let value = self.values.join(", ");

        self.values.drain(..);

        value
    }
}

impl Action for TestAction {
    fn action(&mut self, _: &mut Context) -> bool {
        self.count += 1;
        if self.count == 1 {
            self.add(format!("{}-start({})", self.name, self.count));
            true
        } else if self.count >= self.max {
            self.add(format!("{}-end({})", self.name, self.count));
            self.count = 0;
            false
        } else {
            self.add(format!("{}({})", self.name, self.count));
            true
        }
    }    
}
