mod body;
mod world;
mod control;
mod ui_body;
mod ui_body_locmap;
mod ui_world;

use body::SlugBodyPlugin;
use essay_ecs::prelude::App;
use ui_body::UiSlugBodyPlugin;
use ui_body_locmap::UiSlugLocationPlugin;
use ui_graphics::UiCanvasPlugin;
use ui_world::UiSlugWorldPlugin;
use world::SlugWorldPlugin;

pub fn main() {
    let mut app = App::new();

    app.plugin(SlugWorldPlugin);
    app.plugin(SlugBodyPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new());
    app.plugin(UiSlugWorldPlugin::new((0., 0.), (1., 1.)));
    app.plugin(UiSlugBodyPlugin::new((0., 1.), (2., 1.)));
    app.plugin(UiSlugLocationPlugin::new((1., 0.), (1., 1.)));

    app.run();
}
