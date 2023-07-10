//use essaymind::my_test;

use essay_ecs::prelude::App;
use essay_mind::{ecs_main, ui_panel::ui_panel};
use log::{info, LevelFilter};
use ui_graphics::{ui_canvas::ui_main, plugin::WinitPlugin};

pub fn main() {
    let mut app = App::new();

    //app.add_plugin(WinitPlugin);
    // app.add_system(|| { println!("ui-ecs"); });
    //app.add_system(ui_panel);

    ui_main(app);
    // app.tick();
    println!("Exit");
}
