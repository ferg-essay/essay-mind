use essay_ecs::prelude::*;
use world::WorldPlugin;

pub fn ecs_main() {
    let mut app = App::new();

    app.system(Update, || println!("tick2"));
    app.plugin(WorldPlugin);

    app.run();
}
