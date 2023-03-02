use ticker::Context;

use crate::action::SharedReader;
use crate::{gram, Gram, Topos, SharedWriter};
use crate::{MindBuilder};
use crate::action::action_group::{ActionGroup, Action};

///
/// monoamine: enhance or inhibit the action strength and also the 
/// selection process.
/// 
#[test]
fn test_monoamine_action_power() {
    let mut system = MindBuilder::new();

    let writer = SharedWriter::<TestData>::new();
    let reader = writer.reader();

    let mut action = TestAction::new("action", reader);
    action.max(4);
    let mut group = ActionGroup::new(&mut system);
    let mut action = group.action(gram("action"), action);

    action.activator(|a, _topos, ctx| a.activator(ctx));

    group.decay(0.5);

    let monoamine = system.external_source();
    monoamine.source().to(group.modulate());
    
    let mut system = system.build();

    let monoamine = monoamine.fiber();

    let mut ptr = action.unwrap();

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

    writer.write(system.ticks()).unwrap().value = String::from("sense");
    system.tick();
    writer.write(system.ticks()).unwrap().value = String::from("sense");
    assert_eq!(ptr.write(|a| a.take()), "sense(3)");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1,Unit(0.5)), sense(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2,Unit(0.5)), sense(5)");
    monoamine.send((gram("sense"), Topos::Unit(1.)));
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3,Unit(0.5)), sense(6)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4,Unit(0.5)), sense(7)");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1,Unit(0.625)), sense(8)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2,Unit(0.625)), sense(9)");
    writer.write(system.ticks()).unwrap().value = String::from("");
    system.tick();
    writer.write(system.ticks()).unwrap().value = String::from("");
    assert_eq!(ptr.write(|a| a.take()), "action(3,Unit(0.625))");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4,Unit(0.625))");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1,Unit(0.5078125))");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2,Unit(0.5078125))");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3,Unit(0.5078125))");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4,Unit(0.5078125))");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

}
#[test]
fn test_monoamine_action_selection() {
    let mut system = MindBuilder::new();

    let writer = SharedWriter::<TestData>::new();
    let reader_a = writer.reader();
    let reader_b = writer.reader();

    let mut a = TestAction::new("a", reader_a);
    a.max(2);
    let mut b = TestAction::new("b", reader_b);
    b.max(2);
    let mut group = ActionGroup::new(&mut system);
    let mut a = group.action(gram("a"), a);
    let mut b = group.action(gram("b"), b);

    a.activator(|a, _, ctx| a.activator(ctx));
    b.activator(|a, _, ctx| a.activator(ctx));

    b.set_modulated(true);

    group.decay(0.5);

    let monoamine = system.external_source();
    monoamine.source().to(group.modulate());
    
    let mut system = system.build();

    let monoamine = monoamine.fiber();

    let mut a = a.unwrap();
    let mut b = b.unwrap();

    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "");
    assert_eq!(b.write(|a| a.take()), "");

    writer.write(system.ticks()).unwrap().value = String::from("sense");
    system.tick();
    writer.write(system.ticks()).unwrap().value = String::from("sense");
    assert_eq!(a.write(|a| a.take()), "sense-a(3)");
    assert_eq!(b.write(|a| a.take()), "sense-b(3)");

    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1,Unit(0.5)), sense-a(4)");
    assert_eq!(b.write(|a| a.take()), "sense-b(4)");
    monoamine.send((gram("b"), Topos::Unit(1.)));
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2,Unit(0.5)), sense-a(5)");
    assert_eq!(b.write(|a| a.take()), "sense-b(5)");

    system.tick();
    assert_eq!(a.write(|a| a.take()), "sense-a(6)");
    assert_eq!(b.write(|a| a.take()), "b-start(1,Unit(0.75)), sense-b(6)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "sense-a(7)");
    assert_eq!(b.write(|a| a.take()), "b-end(2,Unit(0.75)), sense-b(7)");

    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-start(1,Unit(0.5)), sense-a(8)");
    assert_eq!(b.write(|a| a.take()), "sense-b(8)");
    system.tick();
    assert_eq!(a.write(|a| a.take()), "a-end(2,Unit(0.5)), sense-a(9)");
    assert_eq!(b.write(|a| a.take()), "sense-b(9)");
}

struct TestAction {
    name: Gram,
    reader: SharedReader<TestData>,
    values: Vec<String>,
    count: u64,
    max: u64, 
}

impl TestAction {
    fn new(
        str: &str,
        reader: SharedReader<TestData>,
    ) -> Self {
        Self {
            name: Gram::from(str),
            reader: reader,
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

    fn activator(&mut self, ctx: &mut Context) -> bool {
        let value = self.reader.read(ctx.ticks()).unwrap().value();

        if value == "sense" {
            self.add(format!("{}-{}({})", value, self.name, ctx.ticks()));
            true
        } else {
            false
        }
    }    
}

impl Action for TestAction {
    fn action(&mut self, topos: Topos, _: &mut Context) -> bool {
        self.count += 1;
        if self.count == 1 {
            self.add(format!("{}-start({},{:?})", self.name, self.count, topos));
            true
        } else if self.count >= self.max {
            self.add(format!("{}-end({},{:?})", self.name, self.count, topos));
            self.count = 0;
            false
        } else {
            self.add(format!("{}({},{:?})", self.name, self.count, topos));
            true
        }
    }    
}

#[derive(Default,Debug)]
struct TestData {
    value: String,
}

impl TestData {
    fn value(&self) -> String {
        self.value.clone()
    }
}
