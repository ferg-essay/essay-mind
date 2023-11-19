use std::time::Duration;

use vertebrate::body::BodyPlugin;
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::mid_feeding::{ExploreFood, SeekFood, MidFeedingPlugin, EatFood, CueSeekFood, CueAvoidFood};
use vertebrate::habenula_med::HabenulaMedPlugin;
use vertebrate::mid_peptides::MidPeptidesPlugin;
use vertebrate::tuberculum::TuberculumPlugin;
use vertebrate::mid_locomotor::MidLocomotorPlugin;
use vertebrate::olfactory::OlfactoryPlugin;
use vertebrate::tectum::TectumPlugin;
use vertebrate::ui_body::{UiBodyPlugin, UiBodyTrailPlugin};
use vertebrate::ui_body_graph::UiGraphPlugin;
use vertebrate::ui_body_heatmap::UiLocationHeatmapPlugin;
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
    app.plugin(MidPeptidesPlugin);
    app.plugin(MidFeedingPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiLocationHeatmapPlugin::new((2., 0.), (1., 1.)));

    app.plugin(UiGraphPlugin::new((0.0, 1.0), (2., 1.))
        .line(ExploreFood, "or")
        .line(CueSeekFood, "gh")
        .line(CueAvoidFood, "nt")
        .line(SeekFood, "DA")
        .line(EatFood, "cc")
    );
    app.plugin(UiPeptidePlugin::new((2.0, 1.0), (0.5, 1.))
        .peptide(ExploreFood, "or") // orexin
        .peptide(CueSeekFood, "gh") // ghrelin
        .peptide(CueAvoidFood, "nt") // neurotensin
        .peptide(SeekFood, "DA") // dopamine
        .peptide(EatFood, "cc") // npy
    );
    app.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));

    app.run();
}
