use std::time::Duration;

use vertebrate::body::{BodyPlugin, Body};
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::habenula_med::HabenulaMedPlugin;
use vertebrate::mid_feeding::MidFeedingPlugin;
use vertebrate::mid_peptides2::{MidPeptides2Plugin, MidPeptides2};
use vertebrate::tuberculum::TuberculumPlugin;
use vertebrate::mid_locomotor::MidLocomotorPlugin;
use vertebrate::olfactory::OlfactoryPlugin;
use vertebrate::tectum::TectumPlugin;
use vertebrate::ui::ui_body::{UiBodyPlugin, UiBodyTrailPlugin};
use vertebrate::ui::ui_body_heatmap::UiLocationHeatmapPlugin;
use ui_graphics::UiCanvasPlugin;
use vertebrate::ui::ui_body_homunculus::UiHomunculusPlugin;
use vertebrate::ui::ui_graph2::UiGraph2Plugin;
use vertebrate::ui::ui_peptide2::UiPeptide2Plugin;
use vertebrate::ui::ui_world::UiWorldPlugin;
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
        app.plugin(MidPeptides2Plugin);
    app.plugin(MidFeedingPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiLocationHeatmapPlugin::new((2., 0.), (1., 1.)));

    app.plugin(UiGraph2Plugin::new((0.0, 1.0), (2., 1.))
        .colors(["amber", "sky", "olive", "red", "green", "blue"])
        .item("ox", |p: &MidPeptides2| p.explore_food())
        .item("DA", |p: &MidPeptides2| p.seek_food())
        .item("Hb", |p: &MidPeptides2| p.give_up_seek_food())
        .item("Df", |p: &MidPeptides2| p.near_food())
        .item("gl", |b: &Body| b.eat().glucose())
    );

    app.plugin(UiPeptide2Plugin::new((2.0, 1.0), (0.5, 1.))
        .colors(["amber", "sky", "olive", "red", "green", "blue"])
        .item("ox", |p: &MidPeptides2| p.explore_food())
        .item("DA", |p: &MidPeptides2| p.seek_food())
        .item("Hb", |p: &MidPeptides2| p.give_up_seek_food())
        .item("Df", |p: &MidPeptides2| p.near_food())
        .item("gl", |b: &Body| b.eat().glucose())
    );

    app.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));

    app.run();
}
