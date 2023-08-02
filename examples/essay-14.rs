mod essay_14_slug;

use essay_ecs::prelude::App;
use ui_graphics::{UiCanvasPlugin, ui_plot::UiPlotPlugin};
use essay_14_slug::{SlugBodyPlugin, SlugWorldPlugin};

pub fn main() {
    let mut app = App::new();

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin);
    app.plugin(UiPlotPlugin);
    app.plugin(SlugWorldPlugin);
    app.plugin(SlugBodyPlugin);

    app.run();
}