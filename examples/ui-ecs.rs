//use essaymind::my_test;

use essay_ecs_core::base_app::BaseApp;
use essay_mind::ecs_main;
use log::{info, LevelFilter};
use ui_graphics::{ui_graph::ui_main};

pub fn main() {
    let mut app = BaseApp::new();

    app.add_system(|| { println!("ui-ecs"); });

    ui_main(app);
    // app.tick();
}
