use essay_ecs::prelude::App;
use mind::{world::ApicalWorldPlugin, body::ApicalBodyPlugin};

#[test]
fn test_main() {
    let mut app = App::new();

    app.plugin(ApicalWorldPlugin);
    app.plugin(ApicalBodyPlugin);

    app.tick();
    println!("Tick");
}