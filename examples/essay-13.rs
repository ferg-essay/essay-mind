use essay_ecs::prelude::App;
use ui_graphics::UiCanvasPlugin;
use world_plankton::{PlanktonWorldPlugin, PlanktonBodyPlugin};

pub fn main() {
    let mut app = App::new();

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin);
    app.plugin(PlanktonWorldPlugin);
    app.plugin(PlanktonBodyPlugin);

    app.run();
}
