use mind::{MindBuilder, action::{ActionGroup, Action}, gram, Topos, Gram, Context};

fn main() {
    let mut builder = MindBuilder::new();
    let mut action = TestAction::new("action");
    action.max(4);
    let mut group = ActionGroup::new(&mut builder);
    let action = group.action(
        gram("a"), 
        action
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

    fiber.send((gram("?a"), Topos::Nil));
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
    fn action(&mut self, _topos: Topos, _: &mut Context) -> bool {
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
