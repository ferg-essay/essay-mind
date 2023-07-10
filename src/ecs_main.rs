use essay_ecs::prelude::*;
use world::WorldPlugin;

pub fn ecs_main() {
    let mut app = App::new();

    app.add_system(Update, || println!("tick2"));
    app.add_plugin(WorldPlugin);

    app.run();
}
