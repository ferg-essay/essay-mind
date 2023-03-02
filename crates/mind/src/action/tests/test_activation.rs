use ticker::Context;

use crate::action::SharedReader;
use crate::{gram, Gram, SharedWriter, Topos};
use crate::{MindBuilder};
use crate::action::action_group::{ActionGroup, Action};

///
/// activation: connected sensor-processing node to the action that
/// looks at senses to activate the node, as opposed to an external
/// activation.
/// 
#[test]
fn test_activation() {
    let mut builder = MindBuilder::new();

    let writer = SharedWriter::<TestData>::new();
    let reader = writer.reader();

    let mut action = TestAction::new("action", reader);
    action.max(4);
    let mut group = ActionGroup::new(&mut builder);
    let mut action = group.action(gram("action"), action);

    //action.activator_item(TestActivator::new("sensor", reader));
    action.activator(|a, _, ctx| a.activator(ctx));
    
    let mut system = builder.build();

    //let fiber = ext_source.fiber();

    let mut ptr = action.unwrap();

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

    //fiber.send((gram("a"), Topos::Nil));
    writer.write(system.ticks()).unwrap().value = String::from("sense");
    system.tick();
    writer.write(system.ticks()).unwrap().value = String::from("sense");
    assert_eq!(ptr.write(|a| a.take()), "sense-activate(3)");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1), sense-activate(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2), sense-activate(5)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3), sense-activate(6)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4), sense-activate(7)");

    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-start(1), sense-activate(8)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2), sense-activate(9)");
    writer.write(system.ticks()).unwrap().value = String::from("");
    system.tick();
    writer.write(system.ticks()).unwrap().value = String::from("");
    assert_eq!(ptr.write(|a| a.take()), "action(3)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action-end(4)");

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
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");

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
            self.add(format!("sense-activate({})", ctx.ticks()));
            true
        } else {
            false
        }
    }    
}

impl Action for TestAction {
    fn action(&mut self, _: Topos, _: &mut Context) -> bool {
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

#[derive(Default,Debug)]
struct TestData {
    value: String,
}

impl TestData {
    fn value(&self) -> String {
        self.value.clone()
    }
}
