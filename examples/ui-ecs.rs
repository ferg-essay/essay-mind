//use essaymind::my_test;

use essay_ecs::prelude::App;
use world::WorldPlugin;

pub fn main() {
    let mut app = App::new();

    //app.add_plugin(WinitPlugin);
    // app.add_system(|| { println!("ui-ecs"); });
    //app.add_system(ui_panel);

    app.plugin(WorldPlugin);

    app.run();
    //ui_main(app);
    // app.tick();
    println!("Exit");
}
