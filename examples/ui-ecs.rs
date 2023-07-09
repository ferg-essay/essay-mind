//use essaymind::my_test;

use essay_ecs_core::base_app::BaseApp;
use essay_mind::{ecs_main, ui_panel::ui_panel};
use log::{info, LevelFilter};
use ui_graphics::{ui_canvas::ui_main};

pub fn main() {
    let mut app = BaseApp::new();

    // app.add_system(|| { println!("ui-ecs"); });
    app.add_system(ui_panel);

    ui_main(app);
    // app.tick();
}
