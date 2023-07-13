use essay_ecs::prelude::*;
use mind::world::ApicalWorldPlugin;

pub fn ecs_main() {
    let mut app = App::new();

    app.system(Update, || println!("tick2"));
    app.plugin(ApicalWorldPlugin);

    app.run();
}
