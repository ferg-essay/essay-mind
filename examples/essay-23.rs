use std::time::Duration;

use vertebrate::body::BodyPlugin;
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::feeding::{ExploreFood, SeekFood, MidFeedingPlugin, EatFood};
use vertebrate::habenula_med::HabenulaMedPlugin;
use vertebrate::mid_peptide_canal::MidPeptideCanalPlugin;
use vertebrate::tuberculum::TuberculumPlugin;
use vertebrate::mid_locomotor::MidLocomotorPlugin;
use vertebrate::olfactory::OlfactoryPlugin;
use vertebrate::tectum::TectumPlugin;
use vertebrate::ui_body::{UiBodyPlugin, UiBodyTrailPlugin};
use vertebrate::ui_body_heatmap::UiSlugLocationPlugin;
use ui_graphics::UiCanvasPlugin;
use vertebrate::ui_body_homunculus::UiHomunculusPlugin;
use vertebrate::ui_peptide::UiPeptidePlugin;
use vertebrate::ui_world::UiWorldPlugin;
use vertebrate::world::{WorldPlugin, OdorType};

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));
    app.plugin(
        WorldPlugin::new(20, 10)
        .wall((4, 5), (4, 1))
        .wall((4, 0), (1, 5))
        .food_odor(1, 1, OdorType::FoodA)
        .food_odor(8, 2, OdorType::FoodB)
        .odor(0, 9, OdorType::AvoidA)
    );
    app.plugin(BodyPlugin::new());
    app.plugin(OlfactoryPlugin);
    app.plugin(TectumPlugin::new().striatum());
    app.plugin(MidLocomotorPlugin);
    app.plugin(TuberculumPlugin);
    app.plugin(HabenulaMedPlugin);
    app.plugin(MidPeptideCanalPlugin);
    app.plugin(MidFeedingPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));
    app.plugin(UiWorldPlugin::new((0., 0.), (0.5, 0.5)));
    app.plugin(UiSlugLocationPlugin::new((0.5, 0.), (0.5, 0.5)));
    app.plugin(UiBodyPlugin::new((0., 0.5), (0.5, 0.5)));
    app.plugin(UiHomunculusPlugin::new((0.5, 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);
    app.plugin(UiPeptidePlugin::new((0.75, 0.5), (0.25, 0.25))
        .peptide(ExploreFood, "X") // orexin
        .peptide(SeekFood, "S") // mch
        .peptide(EatFood, "C") // npy
);

    app.run();
}
