use vertebrate::body::SlugBodyPlugin;
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::ui_body::UiSlugBodyPlugin;
use vertebrate::ui_body_heatmap::UiSlugLocationPlugin;
use ui_graphics::UiCanvasPlugin;
use vertebrate::ui_world::UiSlugWorldPlugin;
use vertebrate::world::SlugWorldPlugin;

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(8));
    app.plugin(SlugWorldPlugin);
    app.plugin(SlugBodyPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new());
    app.plugin(UiSlugWorldPlugin::new((0., 0.), (1., 1.)));
    app.plugin(UiSlugBodyPlugin::new((0., 1.), (2., 1.)));
    app.plugin(UiSlugLocationPlugin::new((1., 0.), (1., 1.)));

    app.run();
}
