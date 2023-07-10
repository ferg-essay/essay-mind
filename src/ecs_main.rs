use essay_ecs::prelude::*;
use ui_graphics::ui_canvas::ui_main;

pub fn ecs_main() {
    let mut app = App::new();

    app.add_system(Update, || println!("tick2"));

    ui_main(app);
}
