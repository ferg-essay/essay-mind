use std::time::Duration;

use essay_plot::api::Colors;
use vertebrate::{
    body::{Body, BodyEatPlugin, BodyPlugin}, hind_motor::{HindEatPlugin, HindLocomotorPlugin}, core_motive::{
        core_eat::CoreEatingPlugin, mid_peptides::CorePeptidesPlugin, motive::{Motive, Seek}, Dwell, CoreExplorePlugin, Roam, Wake, CoreWakePlugin
    }, olfactory_bulb::OlfactoryBulb, olfactory_bulb::{ObEvent, OlfactoryPlugin}, 
    mid_seek::phototaxis::Phototaxis,
    mid_seek::{
        chemotaxis::{ChemotaxisPlugin, Chemotaxis}, 
        habenula_seek::HabenulaSeekPlugin
    }, 
    mid_motor::{tectum::TectumPlugin, MidSustainPlugin}, 
    ui::{
        ui_attention::UiAttentionPlugin, ui_homunculus::UiHomunculusPlugin, ui_motive::Emoji,
        ui_body::{UiBodyPlugin, UiBodyTrailPlugin},
        ui_body_heatmap::UiLocationHeatmapPlugin,
        ui_motive::UiMotivePlugin,
        ui_graph::UiGraphPlugin,
        ui_peptide::UiPeptidePlugin,
        ui_table::UiTablePlugin,
        ui_world::UiWorldPlugin,
    }, world::{
        World, WorldPlugin, OdorType
    }
};
use essay_ecs::{app::event::InEvent, core::ResMut, prelude::App};
use mind_ecs::{Tick, TickSchedulePlugin};
use ui_graphics::UiCanvasPlugin;

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));

    world_roam(&mut app);
    app.plugin(BodyPlugin::new());
    app.plugin(BodyEatPlugin);

    app.plugin(HindLocomotorPlugin);
    app.plugin(HindEatPlugin);

    app.plugin(OlfactoryPlugin::new()
        .odor(OdorType::FoodA)
        .odor(OdorType::FoodB)
    );

    app.plugin(TectumPlugin::new().striatum());
    app.plugin(HabenulaSeekPlugin);
    app.plugin(ChemotaxisPlugin);
    app.plugin(MidSustainPlugin);

    app.plugin(CoreWakePlugin);
    app.plugin(CoreExplorePlugin);
    app.plugin(CorePeptidesPlugin);
    app.plugin(CoreEatingPlugin);

    // app.system(Tick, dwell_olfactory);
    //ui_chemotaxis(&mut app);
    ui_eat(&mut app);

    app.run();
}

fn dwell_olfactory(
    mut dwell: ResMut<Motive<Dwell>>,
    mut ob: InEvent<ObEvent>,
) {
    for event in ob.iter() {
        match event {
            ObEvent::Odor(_odor, _vector) => {
                dwell.set_max(1.);
            },
        }
    }

    //dwell.add(1.);
    //dwell.set_max(1.);
}

pub fn world_roam(app: &mut App) {
    let w = 15;
    let h = 11;

    let h1 = h / 2 - 1;

    let w1 = w / 2;
    let w2 = w1;

    app.plugin(
        WorldPlugin::new(w, h)
        //.wall(((w - 1) / 2, 0), (2, h1))
        //.wall(((w - 1) / 2, h - h1), (2, h1))
        //.floor((0, 0), (w1, h), FloorType::Light)
        //.floor((w2, 0), (w - w2, h), FloorType::Dark)
        .food_odor_r(5, 5, 4, OdorType::FoodA)
    );
}

pub fn world_odor(app: &mut App) {
    let w = 15;
    let h = 11;

    let h1 = h / 2 - 1;

    let w1 = w / 2;
    let w2 = w1;

    app.plugin(
        WorldPlugin::new(w, h)
        //.wall(((w - 1) / 2, 0), (2, h1))
        //.wall(((w - 1) / 2, h - h1), (2, h1))
        //.floor((0, 0), (w1, h), FloorType::Light)
        //.floor((w2, 0), (w - w2, h), FloorType::Dark)
        .food_odor_r(5, 5, 4, OdorType::FoodA)
        .odor_r(9, 5, 4, OdorType::FoodB)
    );
}

fn ui_eat(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    app.plugin(UiLocationHeatmapPlugin::new((2., 0.), (1., 0.7)));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    app.plugin(UiGraphPlugin::new((0.0, 1.0), (2., 1.))
        .colors(colors.clone())
        //.item("v", |tx: &Chemotaxis| tx.value().clamp(0., 1.))
        //.item("gr", |tx: &Chemotaxis| 0.5 * (tx.gradient() + 1.))
    );

    //food_peptides(app, (2.0, 1.0), (0.5, 1.));

    let odor_colors = Colors::from(["green", "azure"]);
    app.plugin(UiAttentionPlugin::new((2.0, 1.0), (0.5, 0.5))
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodB))
    );

    app.plugin(UiMotivePlugin::new((2.0, 1.5), (0.5, 0.5))
        .size(12.)
        .item(Emoji::FaceGrinning, |m: &Motive<Wake>| m.value())
        .item(Emoji::Footprints, |m: &Motive<Roam>| m.value())
        .item(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.value())
        .item(Emoji::DirectHit, |m: &Motive<Seek>| m.value())
        .item(Emoji::FaceDisappointed, |m: &Motive<Seek>| m.value())
        .item(Emoji::FaceSleeping, |m: &Motive<Seek>| m.value())
        .row()
        .item(Emoji::ForkAndKnife, |m: &Motive<Seek>| m.value())
        .item(Emoji::Candy, |m: &Motive<Seek>| m.value())
        .item(Emoji::Cheese, |m: &Motive<Seek>| m.value())
        .item(Emoji::Lemon, |m: &Motive<Seek>| m.value())
        .item(Emoji::Salt, |m: &Motive<Seek>| m.value())
        // .item(Emoji::FaceAstonished, |m: &Motive<Hunger>| m.value())
);
    app.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));
}


fn ui_chemotaxis(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    app.plugin(UiLocationHeatmapPlugin::new((2., 0.), (1., 0.7)));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    app.plugin(UiGraphPlugin::new((0.0, 1.0), (2., 1.))
        .colors(colors.clone())
        .item("v", |tx: &Chemotaxis| tx.value().clamp(0., 1.))
        .item("gr", |tx: &Chemotaxis| 0.5 * (tx.gradient() + 1.))
    );

    //food_peptides(app, (2.0, 1.0), (0.5, 1.));

    let odor_colors = Colors::from(["green", "azure"]);
    app.plugin(UiAttentionPlugin::new((2.0, 1.0), (0.5, 1.))
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodB))
    );

    app.plugin(UiMotivePlugin::new((2.5, 1.), (0.5, 1.)));
}


fn ui_phototaxis(app: &mut App) {
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

    app.plugin(UiMotivePlugin::new((2.5, 1.), (0.5, 1.)));
}
