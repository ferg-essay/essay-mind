use std::time::Duration;

use essay::{world_place_preference, food_graph};
use essay_plot::api::Colors;
use vertebrate::body::{BodyPlugin, Body};
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use vertebrate::habenula_med::HabenulaMedPlugin;
use vertebrate::mid_feeding::MidFeedingPlugin;
use vertebrate::mid_peptides::{MidPeptidesPlugin, MidPeptides};
use vertebrate::phototaxis::{PhototaxisPlugin, Phototaxis};
use vertebrate::tuberculum::TuberculumPlugin;
use vertebrate::mid_locomotor::MidLocomotorPlugin;
use vertebrate::olfactory::OlfactoryPlugin;
use vertebrate::tectum::TectumPlugin;
use vertebrate::ui::ui_body::{UiBodyPlugin, UiBodyTrailPlugin};
use vertebrate::ui::ui_body_heatmap::UiLocationHeatmapPlugin;
use ui_graphics::UiCanvasPlugin;
use vertebrate::ui::ui_body_homunculus::UiHomunculusPlugin;
use vertebrate::ui::ui_graph::UiGraphPlugin;
use vertebrate::ui::ui_peptide::UiPeptidePlugin;
use vertebrate::ui::ui_table::UiTablePlugin;
use vertebrate::ui::ui_world::UiWorldPlugin;
use vertebrate::world::World;

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));

    world_place_preference(&mut app);
    app.plugin(BodyPlugin::new());
    app.plugin(OlfactoryPlugin);
    app.plugin(TectumPlugin::new().striatum());
    app.plugin(MidLocomotorPlugin);
    app.plugin(TuberculumPlugin);
    app.plugin(HabenulaMedPlugin);
    app.plugin(MidPeptidesPlugin);
    app.plugin(MidFeedingPlugin);
    app.plugin(PhototaxisPlugin);

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(light)", |w: &World, b: &Body| w.light(b.pos()))
    );

    app.plugin(UiLocationHeatmapPlugin::new((2., 0.), (1., 0.7)));

    // food_graph(&mut app, (0.0, 1.0), (2., 1.));
    let colors = Colors::from(["amber", "sky", "olive", "red", "purple", "blue"]);

    app.plugin(UiGraphPlugin::new((0.0, 1.0), (2., 1.))
        .colors(colors.clone())
        .item("v", |p: &Phototaxis| p.value())
        .item("avg", |p: &Phototaxis| p.average())
        .item("grad", |p: &Phototaxis| p.gradient() / 2. + 0.5)
        .item("s-av", |p: &Phototaxis| p.short_average())
        .item("s-gr", |p: &Phototaxis| p.short_gradient() / 2. + 0.5)
    );

    // food_peptides(&mut app, (2.0, 1.0), (0.5, 1.));

    app.plugin(UiPeptidePlugin::new((2.0, 1.0), (0.5, 1.))
        .colors(colors)
        .item("v", |p: &Phototaxis| p.value())
        .item("av", |p: &Phototaxis| p.average())
        .item("gr", |p: &Phototaxis| p.gradient() / 2. + 0.5)
        .item("sa", |p: &Phototaxis| p.short_average())
        .item("sg", |p: &Phototaxis| p.short_gradient() / 2. + 0.5)
    );

    app.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));

    app.run();
}
