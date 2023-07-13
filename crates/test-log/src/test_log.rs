use essay_ecs::prelude::*;

pub struct TestLog {
    log: Vec<String>,
}

impl TestLog {
    pub fn new() -> TestLog {
        Self {
            log: Default::default(),
        }
    }

    pub fn clear(mut log: ResMut<TestLog>) {
        log.log.drain(..);
    }

    pub fn log(&mut self, msg: &str) -> &mut Self {
        self.log.push(msg.into());

        self
    }

    pub fn take(&mut self) -> Vec<String> {
        let mut log = self.log.drain(..).collect::<Vec<String>>();

        log.sort();

        log
    }
}

pub fn log_take(app: &mut App) -> Vec<String> {
    app.resource_mut::<TestLog>().take()
}

pub struct TestLogPlugin;

impl Plugin for TestLogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestLog::new());

        app.system(First, TestLog::clear);
    }
}