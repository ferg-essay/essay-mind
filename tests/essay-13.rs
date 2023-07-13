use essay_ecs::prelude::App;
use mind::{world::ApicalWorldPlugin, body::ApicalBodyPlugin};
use test_log::{TestLogPlugin, log_take};

#[test]
fn test_main() {
    let mut app = App::new();

    app.plugin(TestLogPlugin);

    app.plugin(ApicalWorldPlugin);
    app.plugin(ApicalBodyPlugin);

    app.tick();
    assert_eq!(log_take(&mut app), vec!["body: (2.5, 2.5)"]);
}