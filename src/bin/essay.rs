use std::time::Duration;

use essay_plot::api::{Colors, Point};
use log::LevelFilter;
use vertebrate::{
    body::{Body, BodyEat}, 
    builder::AnimalBuilder, 
    hind_brain::{HindEat, HindMove, MoveKind}, 
    motive::{
        Dwell, Forage, Motive, MotiveEat, MotiveTrait, Sleep, Wake
    }, 
    olfactory::olfactory_bulb::OlfactoryBulb, 
    taxis::{
        chemotaxis::Chemotaxis, 
        phototaxis::Phototaxis
    }, 
    ui::{
        ui_attention::UiAttentionPlugin, ui_body::{UiBodyPlugin, UiBodyTrailPlugin}, ui_emoji::Emoji, ui_graph::UiGraphPlugin, ui_heatmap::UiHeatmapPlugin, ui_homunculus::UiHomunculusPlugin, ui_motive::UiMotivePlugin, ui_peptide::UiPeptidePlugin, ui_retina::UiRetinaPlugin, ui_table::UiTablePlugin, ui_world_hex::{Pattern, UiWorldHexPlugin}, ui_world_map::UiWorldPlugin
    }, 
    util::{self}, 
    world::{
        FoodKind, OdorKind, OdorType, World, WorldPlugin
    }
};
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use ui_graphics::UiCanvasPlugin;

// 

pub fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).default_format().init();

    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));

    app.plugin(world_roam(21, 15)
        // .odor_r(5, 5, 4, OdorType::FoodA)
        // .odor_r(15, 5, 4, OdorType::FoodA)
        .food_odor_r(5, 5, FoodKind::Plain, 3, OdorType::FoodA)
        .food_odor_r(10, 10, FoodKind::Sweet, 3, OdorType::FoodA)
        .food_odor_r(15, 5, FoodKind::Bitter, 3, OdorType::FoodA)
        .loc_odor(5, 5, 3, OdorKind::B)
    );

    //app.plugin(world_roam(21, 15)
        // .food_odor_r(5, 5, 4, OdorType::FoodA)
    //);

    let mut animal = AnimalBuilder::new();

    animal.olfactory()
        .odor(OdorType::FoodA)
        .odor(OdorType::FoodB);

    animal.retina()
        .size(8)
        .fov(util::Angle::Deg(120.))
        .eye_angle(util::Angle::Deg(45.));

    animal.seek().seek(true);

    animal.build(&mut app);

    ui_eat_flat(&mut app);
    app.plugin(UiRetinaPlugin::new(((2.0, 0.0), [0.5, 0.5])));

    app.run().unwrap();
}

pub struct Dummy;
impl MotiveTrait for Dummy {}

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

pub fn world_roam(w: usize, h: usize) -> WorldPlugin {
    WorldPlugin::new(w, h)
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
        .food_odor_r(5, 5, FoodKind::Sweet, 4, OdorType::FoodA)
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
        .food_odor_r(5, 5, FoodKind::Sweet, 4, OdorType::FoodA)
        .odor_r(9, 5, 4, OdorType::FoodB)
    );
}


#[allow(unused)]
fn ui_eat(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0))
        );
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
        .item(Emoji::Footprints, |m: &HindMove| if m.action_kind() == MoveKind::Roam { 1. } else { 0. })
        .item(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.value())
        .item(Emoji::DirectHit, |m: &HindMove| if m.action_kind() == MoveKind::Seek { 1. } else { 0. })
        .item(Emoji::NoEntry, |m: &HindMove| if m.action_kind() == MoveKind::Avoid { 1. } else { 0. })
        .item(Emoji::FaceDisappointed, |m: &Motive<Dummy>| m.value())
        //.item(Emoji::FaceGrinning, |m: &Motive<Wake>| m.value())
        .item(Emoji::Coffee, |m: &Sleep| {
            if m.is_forage() { 
                1. 
            } else if m.is_wake() { 
                0.5 
            } else {
                0.
            }
        })
        .item(Emoji::FaceSleeping, |m: &Sleep| if m.is_wake() { 0. } else { 1. })
        .row()
        .item(Emoji::FaceCowboy, |m: &Motive<Forage>| m.value())
        .item(Emoji::ForkAndKnife, |m: &HindEat| if m.is_eating() { 1. } else { 0. })
        .item(Emoji::Pig, |m: &MotiveEat| m.sated())
        .item(Emoji::FaceGrimacing, |m: &HindEat| if m.is_gaping() { 1. } else { 0. })
        .item(Emoji::FaceVomiting, |m: &HindEat| if m.is_vomiting() { 1. } else { 0. })
        .item(Emoji::Warning, |m: &MotiveEat| if m.is_alarm() { 1. } else { 0. })
        .row()
        .item(Emoji::Candy, |m: &BodyEat| m.sweet())
        .item(Emoji::Cheese, |m: &BodyEat| m.umami())
        .item(Emoji::Lemon, |m: &BodyEat| m.bitter())
        .item(Emoji::FaceVomiting, |m: &BodyEat| m.sickness())
        // .item(Emoji::FaceAstonished, |m: &Motive<Hunger>| m.value())
    );
}

fn ui_homunculus(app: &mut App, xy: (f32, f32), wh: (f32, f32)) {
    app.plugin(UiHomunculusPlugin::new(xy, wh)
        .item(Emoji::FaceVomiting, |m: &HindEat| m.is_vomiting())
        .item(Emoji::FaceGrimacing, |m: &HindEat| m.is_gaping())
        .item(Emoji::ForkAndKnife, |m: &HindEat| m.is_eating())
        .item(Emoji::DirectHit, |m: &HindMove| m.action_kind() == MoveKind::Seek)
        .item(Emoji::Warning, |m: &HindMove| m.action_kind() == MoveKind::Avoid)
        .item(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.is_active())
        .item(Emoji::Footprints, |m: &HindMove| m.action_kind() == MoveKind::Roam)
        .item(Emoji::FaceSleeping, |m: &Motive<Wake>| ! m.is_active())
    );
}

fn ui_eat_flat(app: &mut App) {
    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    app.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    app.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    app.plugin(UiBodyTrailPlugin);

    let mut hex = UiWorldHexPlugin::new();
    hex.tile(OdorKind::None);
    hex.tile(OdorKind::A).pattern(Pattern::CheckerBoard(8), "red");
    hex.tile(OdorKind::B).pattern(Pattern::CheckerBoard(8), "teal");
    hex.tile(OdorKind::C).pattern(Pattern::CheckerBoard(8), "orange");

    app.plugin(hex);

    // let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);

    let odor_colors = Colors::from(["green", "azure"]);

    ui_motive(app, (2.0, 0.5), (0.5, 0.5));

    ui_homunculus(app, (2.5, 0.5), (0.5, 0.5));
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
