use std::time::Duration;

use essay_plot::api::{Colors, Point};
use vertebrate::{
    body::{Body, BodyEatPlugin, BodyPlugin}, core_motive::{
        eat::{Eat, FoodSearch, Sated}, 
        wake::Sleep, CoreWakePlugin, Dwell, Motive, MotiveTrait, Roam, Wake
    }, 
    hab_taxis::{
        chemotaxis::{Avoid, Chemotaxis, Seek}, phototaxis::Phototaxis
    }, 
    hind_motor::{HindEat, HindEatPlugin, HindMovePlugin}, 
    olfactory_bulb::{ObEvent, OlfactoryBulb, OlfactoryPlugin}, retina::RetinaPlugin, 
    tectum::{TectumLoomingPlugin, TectumPlugin}, 
    ui::{
        ui_attention::UiAttentionPlugin, 
        ui_body::{UiBodyPlugin, UiBodyTrailPlugin}, 
        ui_emoji::Emoji, ui_graph::UiGraphPlugin, ui_heatmap::UiHeatmapPlugin, 
        ui_homunculus::UiHomunculusPlugin, ui_motive::UiMotivePlugin, ui_peptide::UiPeptidePlugin, ui_retina::UiRetinaPlugin, 
        ui_table::UiTablePlugin, ui_world_map::UiWorldPlugin
    }, 
    util::{self}, world::{
        OdorType, World, WorldPlugin
    }
};
use essay_ecs::{app::event::InEvent, core::{Res, ResMut}, prelude::App};
use mind_ecs::TickSchedulePlugin;
use ui_graphics::UiCanvasPlugin;

pub fn main() {
    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));

    app.plugin(world_lateral_line()
        .food(3, 7)
    );
    //world_food_and_non_food(&mut app);

    // app.plugin(BodyPlugin::new().cast_period(Seconds(0.2).max(Ticks(7))));
    app.plugin(BodyPlugin::new());
    app.plugin(BodyEatPlugin);

    //app.plugin(HindLevyPlugin);
    //app.plugin(_HindMovePlugin);
    app.plugin(HindMovePlugin);
    app.plugin(HindEatPlugin);

    app.plugin(OlfactoryPlugin::new()
        .odor(OdorType::FoodA)
        .odor(OdorType::FoodB)
    );

    app.plugin(RetinaPlugin::new()
        .size(4)
        .fov(util::Angle::Deg(120.))
        .eye_angle(util::Angle::Deg(45.))
    );

    app.plugin(TectumPlugin::new().striatum());
    app.plugin(TectumLoomingPlugin::new());
    // app.plugin(ChemotaxisPlugin);
    // app.plugin(TegSeekPlugin::<OlfactoryBulb, FoodSearch>::new());
    //app.plugin(KlinotaxisPlugin::<OlfactoryBulb, FoodSearch>::new());
    // app.plugin(LateralLinePlugin);

    //app.plugin(MidMotorPlugin);

    app.plugin(CoreWakePlugin::new());
    // app.plugin(CoreExplorePlugin);
    // app.plugin(CorePeptidesPlugin);
    // app.plugin(CoreEatingPlugin);

    // app.system(Tick, dwell_olfactory);
    //app.system(Tick, dwell_eat);
    //ui_chemotaxis(&mut app);
    ui_eat_flat(&mut app);
    // app.plugin(UiCameraPlugin::new((2., -1.), (0.5, 0.5)).fov(Angle::Deg(90.)));
    app.plugin(UiRetinaPlugin::new(((2.0, 0.0), [0.5, 0.5])));
    //app.plugin(UiRetinaPlugin::new(((1.5, -1.0), [1., 1.])));

    app.run();
}

pub struct Dummy;
impl MotiveTrait for Dummy {}


#[allow(unused)]
fn dwell_eat(
    mut dwell: ResMut<Motive<Dwell>>,
    eat: Res<HindEat>,
) {
    if eat.is_eat() {
        dwell.set_max(1.);
    }
}


#[allow(unused)]
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

pub fn world_lateral_line() -> WorldPlugin {
    let w = 15;
    let h = 11;

    let h1 = h / 2 - 1;

    // let w1 = w / 2;
    // let w2 = w1;

    WorldPlugin::new(w, h)
        .wall((2, 0), (1, h1 + 1))
        .wall((5, h1), (1, h - h1))
        .wall((8, 0), (1, h1 + 2))
        //.wall(((w - 1) / 2, h - h1), (2, h1))
        //.floor((0, 0), (w1, h), FloorType::Light)
        //.floor((w2, 0), (w - w2, h), FloorType::Dark)
}

pub fn world_roam(app: &mut App) {
    let w = 15;
    let h = 11;

    // let h1 = h / 2 - 1;

    // let w1 = w / 2;
    // let w2 = w1;

    app.plugin(
        WorldPlugin::new(w, h)
        //.wall(((w - 1) / 2, 0), (2, h1))
        //.wall(((w - 1) / 2, h - h1), (2, h1))
        //.floor((0, 0), (w1, h), FloorType::Light)
        //.floor((w2, 0), (w - w2, h), FloorType::Dark)
        .food_odor_r(5, 5, 4, OdorType::FoodA)
    );
}

pub fn world_food_and_non_food(app: &mut App) {
    let w = 21;
    let h = 15;

    // let h1 = h / 2 - 1;

    // let w1 = w / 2;
    // let w2 = w1;

    app.plugin(
        WorldPlugin::new(w, h)
        //.wall(((w - 1) / 2, 0), (2, h1))
        //.wall(((w - 1) / 2, h - h1), (2, h1))
        //.floor((0, 0), (w1, h), FloorType::Light)
        //.floor((w2, 0), (w - w2, h), FloorType::Dark)
        .food_odor_r(5, 5, 4, OdorType::FoodA)
        .odor_r(15, 5, 4, OdorType::FoodA)
    );
}

pub fn world_odor(app: &mut App) {
    let w = 15;
    let h = 11;

    // let h1 = h / 2 - 1;

    // let w1 = w / 2;
    // let w2 = w1;

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


#[allow(unused)]
fn ui_eat(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    app.plugin(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    app.plugin(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
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

    ui_motive(app, (2.0, 1.5), (0.5, 0.5));
    app.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));
}

fn ui_motive(app: &mut App, xy: impl Into<Point>, wh: impl Into<Point>) {
    app.plugin(UiMotivePlugin::new(xy, wh)
        .size(12.)
        .item(Emoji::Footprints, |m: &Motive<Roam>| m.value())
        .item(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.value())
        .item(Emoji::DirectHit, |m: &Motive<Seek>| m.value())
        .item(Emoji::NoEntry, |m: &Motive<Avoid>| m.value())
        .item(Emoji::FaceDisappointed, |m: &Motive<Dummy>| m.value())
        //.item(Emoji::FaceGrinning, |m: &Motive<Wake>| m.value())
        .item(Emoji::Coffee, |m: &Motive<Wake>| m.value())
        .item(Emoji::FaceSleeping, |m: &Motive<Sleep>| m.value())
        .row()
        .item(Emoji::ForkAndKnife, |m: &Motive<Eat>| m.value())
        .item(Emoji::Pig, |m: &Motive<Sated>| m.value())
        .item(Emoji::Candy, |m: &Motive<Dummy>| m.value())
        .item(Emoji::Cheese, |m: &Motive<FoodSearch>| m.value())
        .item(Emoji::Lemon, |m: &Motive<Dummy>| m.value())
        .item(Emoji::Salt, |m: &Motive<Dummy>| m.value())
        // .item(Emoji::FaceAstonished, |m: &Motive<Hunger>| m.value())
    );
}

fn ui_eat_flat(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    // let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);

    let odor_colors = Colors::from(["green", "azure"]);

    ui_motive(app, (2.0, 0.5), (0.5, 0.5));

    app.plugin(UiHomunculusPlugin::new((2.5, 0.5), (0.5, 0.5))
        .item(Emoji::ForkAndKnife, |m: &Motive<Eat>| m.is_active())
        .item(Emoji::DirectHit, |m: &Motive<Seek>| m.is_active())
        .item(Emoji::NoEntry, |m: &Motive<Avoid>| m.is_active())
        .item(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.is_active())
        .item(Emoji::Footprints, |m: &Motive<Roam>| m.is_active())
        .item(Emoji::FaceSleeping, |m: &Motive<Sleep>| m.is_active())
    );
    //app.plugin(UiCameraPlugin::new((2., -1.), (0.5, 0.5)).fov(Angle::Deg(120.)));

    app.plugin(UiAttentionPlugin::new((2.5, 0.), (0.5, 0.5))
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodB))
    );

    // app.plugin(UiHeatmapPlugin::new(((2.0, -0.5), [1.0, 0.5])));
    /*
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorType::FoodB))
    );
    */
}


#[allow(unused)]
fn ui_chemotaxis(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    app.plugin(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    app.plugin(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
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

#[allow(unused)]
fn ui_phototaxis(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    app.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(light)", |w: &World, b: &Body| w.light(b.pos()))
    );

    app.plugin(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    // food_graph(&mut app, (0.0, 1.0), (2., 1.));
    let colors = Colors::from(["amber", "sky", "olive", "red", "purple", "blue"]);

    app.plugin(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
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
