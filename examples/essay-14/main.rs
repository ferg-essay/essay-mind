mod body;
mod ui_body;
mod world;
mod ui_world;
mod control;

use body::SlugBodyPlugin;
use essay_ecs::prelude::App;
use ui_graphics::UiCanvasPlugin;
use world::SlugWorldPlugin;

pub fn main() {
    let mut app = App::new();

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new());
    app.plugin(SlugWorldPlugin);
    app.plugin(SlugBodyPlugin);

    app.run();
}
