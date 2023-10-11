use std::time::Duration;

use vertebrate::body::BodyPlugin;
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::mid_dopamine::MidDopaminePlugin;
use vertebrate::mid_locomotor::MidLocomotorPlugin;
use vertebrate::olfactory::OlfactoryPlugin;
use vertebrate::tectum_action::TectumPlugin;
use vertebrate::ui_body::UiSlugBodyPlugin;
use vertebrate::ui_body_heatmap::UiSlugLocationPlugin;
use ui_graphics::UiCanvasPlugin;
use vertebrate::ui_world::UiSlugWorldPlugin;
use vertebrate::world::WorldPlugin;

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));
    app.plugin(WorldPlugin::new(30, 20));
    app.plugin(BodyPlugin);
    app.plugin(OlfactoryPlugin);
    app.plugin(TectumPlugin::new().ni());
    app.plugin(MidLocomotorPlugin);
    app.plugin(MidDopaminePlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));
    app.plugin(UiSlugWorldPlugin::new((0., 0.), (1., 1.)));
    app.plugin(UiSlugBodyPlugin::new((0., 1.), (2., 1.)));
    app.plugin(UiSlugLocationPlugin::new((1., 0.), (1., 1.)));

    app.run();
}
