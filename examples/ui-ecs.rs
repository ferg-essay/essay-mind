//use essaymind::my_test;

use essay_ecs::prelude::App;
use ui_graphics::{ui_canvas::ui_main};

pub fn main() {
    let mut app = App::new();

    //app.add_plugin(WinitPlugin);
    // app.add_system(|| { println!("ui-ecs"); });
    //app.add_system(ui_panel);

    ui_main(app);
    // app.tick();
    println!("Exit");
}
