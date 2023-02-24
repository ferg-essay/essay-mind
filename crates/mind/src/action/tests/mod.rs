use crate::{gram, Topos};
use crate::{MindBuilder, action::action::ActionBuilder};
use crate::action::action_group::ActionGroup;

#[test]
fn test() {
    let mut builder = MindBuilder::new();
    let mut group = ActionGroup::new(&mut builder);
    let mut action = group.action(gram("a"), TestAction::new());
    action.on_action(move |a, ctx| {
        a.action(ctx.ticks())
    });
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
    assert_eq!(ptr.write(|a| a.take()), "action(1)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(2)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(3)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "action(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "finish(4)");
    system.tick();
    assert_eq!(ptr.write(|a| a.take()), "");
}

struct TestAction {
    values: Vec<String>,
    count: u64,
}

impl TestAction {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            count: 0,
        }
    }

    fn add(&mut self, msg: String) {
        self.values.push(msg);
    }

    fn action(&mut self, ticks: u64) -> bool {
        if self.count == 0 {
            self.count = 1;
            self.add(format!("action({})", self.count));
            true
        } else if self.count >= 4 {
            self.add(format!("finish({})", self.count));
            self.count = 0;
            false
        } else {
            self.count += 1;
            self.add(format!("action({})", self.count));
            true
        }
    }

    fn take(&mut self) -> String {
        let value = self.values.join(", ");

        self.values.drain(..);

        value
    }
}