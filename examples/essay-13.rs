//use essaymind::my_test;

use essay_ecs::prelude::App;
use mind::{world::ApicalWorldPlugin, body::ApicalBodyPlugin};
use ui_graphics::UiCanvasPlugin;

pub fn main() {
    let mut app = App::new();

    //app.add_plugin(WinitPlugin);
    // app.add_system(|| { println!("ui-ecs"); });
    //app.add_system(ui_panel);

    app.plugin(UiCanvasPlugin);
    app.plugin(ApicalWorldPlugin);
    app.plugin(ApicalBodyPlugin);

    app.run();
    //ui_main(app);
    // app.tick();
    println!("Exit");
}
