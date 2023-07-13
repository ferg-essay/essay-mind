use essay_ecs::prelude::*;

pub struct TestLog {

}

impl TestLog {
    pub fn new() -> TestLog {
        Self {
        }
    }

    pub fn clear(mut log: ResMut<TestLog>) {
    }

    pub fn log(&mut self, msg: &str) -> &mut Self {
        println!("Msg {:?}", msg);

        self
    }

    pub fn take(&mut self) -> Vec<String> {
        Vec::new()
    }
}

pub struct TestLogPlugin;

impl Plugin for TestLogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestLog::new());

        app.system(First, TestLog::clear);
    }
}