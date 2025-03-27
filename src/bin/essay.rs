use std::time::Duration;

use essay_plot::api::{Color, Colors, Point};
use log::LevelFilter;
use vertebrate::{
    body::{Body, BodyEat}, 
    builder::AnimalBuilder, 
    hind_brain::{AvoidHerePlugin, HindAvoid, HindEat, HindMove, HindSearch, MoveKind, Serotonin}, 
    motive::{
        Dwell, Forage, Motive, MotiveEat, MotiveTrait, Sleep, Wake
    }, 
    olfactory::{odor_place::OdorPlacePlugin, olfactory_bulb::OlfactoryBulb}, 
    taxis::{
        chemotaxis::Chemotaxis, 
        phototaxis::Phototaxis
    }, 
    ui::{
        ui_attention::UiAttentionPlugin, ui_body::{UiBodyPlugin, UiBodyTrailPlugin}, ui_camera::UiCameraPlugin, ui_emoji::Emoji, ui_graph::UiGraphPlugin, ui_heatmap::UiHeatmapPlugin, ui_homunculus::UiHomunculusPlugin, ui_motive::UiMotivePlugin, ui_peptide::UiPeptidePlugin, ui_retina::UiRetinaPlugin, ui_table::UiTablePlugin, ui_world_hex::{Pattern, UiWorldHexPlugin}, ui_world_map::UiWorldPlugin
    }, 
    util::{self, Seconds}, 
    world::{
        FoodKind, FoodPlugin, OdorKind, OdorPlugin, World, WorldHexPlugin, WorldHexTrait, WorldPlugin
    }
};
use essay_ecs::prelude::App;
use mind_ecs::TickSchedulePlugin;
use ui_graphics::ui_canvas::{UiBuilder, UiSubBuilder};

// 

pub fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).default_format().init();

    let mut app = App::new();

    app.plugin(TickSchedulePlugin::new().ticks(2));

    let (w, h) = (15, 11);
    // let odor_r = 2;
    app.plugin(world_roam(w, h)
        //.loc_odor(2, 4, 3, OdorKind::FoodA)

       //.loc_odor(2, 10, 3, OdorKind::FoodB)

        //.loc_odor(8, 4, 3, OdorKind::FoodA)

        //.loc_odor(8, 10, 3, OdorKind::FoodB)

        //.loc_odor(14, 10, 3, OdorKind::FoodA)
        //.odor_r(15, 5, 4, OdorType::FoodA)

        //.food_odor_r(14, 4, FoodKind::Plain, odor_r, OdorType::FoodA)
    );

    let mut place = WorldHexPlugin::<PlaceKind>::new(w, h);
    place.circle((2., 4.), 3., PlaceKind::AvoidA);
    app.plugin(place);

    app.plugin(OdorPlacePlugin::<PlaceKind>::new()
        .add(PlaceKind::FoodA, "a")
        .add(PlaceKind::FoodB, "b")
        .add(PlaceKind::AvoidA, "c")
        .add(PlaceKind::AvoidB, "d")
        .add(PlaceKind::OtherA, "e")
    );

    app.plugin(AvoidHerePlugin::<PlaceKind>::new()
        .avoid(PlaceKind::AvoidA, true)
        .avoid(PlaceKind::AvoidB, true)
    );

    let mut food = FoodPlugin::new();
    food.gen_count(1).gen_radius(2.).gen_value(Seconds(120.)).gen_kind(FoodKind::Poor);
    app.plugin(food);

    let mut animal = AnimalBuilder::new();

    animal.olfactory()
        .odor(OdorKind::FoodA)
        .odor(OdorKind::FoodB);

    animal.retina()
        .size(8)
        .fov(util::Angle::Deg(120.))
        .eye_angle(util::Angle::Deg(45.));

    animal.seek().seek(false);

    // animal.hind_eat();

    animal.build(&mut app);

    ui_builder(&mut app);
    //app.plugin(UiRetinaPlugin::new()); // ((2.0, 0.0), [0.5, 0.5])));

    app.run().unwrap();
}

pub struct Dummy;
impl MotiveTrait for Dummy {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlaceKind {
    None,
    FoodA,
    FoodB,
    AvoidA,
    AvoidB,
    OtherA,
    OtherB,
}

impl Default for PlaceKind {
    fn default() -> Self {
        Self::None
    }
}

impl WorldHexTrait for PlaceKind {}

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
        //.food_odor_r(5, 5, FoodKind::Sweet, 4, OdorKind::FoodA)
    );

    let mut food = FoodPlugin::new();
    food.food(5, 5).kind(FoodKind::Sweet).odor_r(4, OdorKind::FoodA);
    app.plugin(food);

    let mut odor = OdorPlugin::new();
    odor.odor_r(15, 5, 4, OdorKind::FoodA);
    app.plugin(odor);
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
        // .food_odor_r(5, 5, FoodKind::Sweet, 4, OdorKind::FoodA)
        //.odor_r(9, 5, 4, OdorKind::FoodB)
    );

    let mut food = FoodPlugin::new();
    food.food(5, 5).kind(FoodKind::Sweet).odor_r(4, OdorKind::FoodA);
    app.plugin(food);

    let mut odor = OdorPlugin::new();
    odor.odor_r(15, 5, 4, OdorKind::FoodA);
    app.plugin(odor);
}


#[allow(unused)]
fn ui_eat(ui: &mut UiBuilder) {
    /*
    // UiCanvasPlugin enables graphics
    ui.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    ui.plugin(UiWorldPlugin::new((0., 0.), (2., 1.0))
        );
    ui.plugin(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    ui.plugin(UiBodyTrailPlugin);

    ui.plugin(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    ui.plugin(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    ui.plugin(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
        .colors(colors.clone())
        //.item("v", |tx: &Chemotaxis| tx.value().clamp(0., 1.))
        //.item("gr", |tx: &Chemotaxis| 0.5 * (tx.gradient() + 1.))
    );

    //food_peptides(app, (2.0, 1.0), (0.5, 1.));

    let odor_colors = Colors::from(["green", "azure"]);
    ui.plugin(UiAttentionPlugin::new((2.0, 1.0), (0.5, 0.5))
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodB))
    );

    ui_motive(app, (2.0, 1.5), (0.5, 0.5));
    ui.plugin(UiHomunculusPlugin::new((2.5, 1.), (0.5, 1.)));
    */
}

fn ui_motive(ui: &mut UiSubBuilder) {
    ui.view(UiMotivePlugin::new()
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
        .row()
        .item(Emoji::FaceCowboy, |m: &Serotonin<HindSearch>| m.active_value())
        .item(Emoji::ForkAndKnife, |m: &Serotonin<HindEat>| m.active_value())
        .item(Emoji::Warning, |m: &Serotonin<HindAvoid>| m.active_value())
    );
}

fn ui_homunculus(ui: &mut UiSubBuilder) {
    ui.view(UiHomunculusPlugin::new()
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

fn ui_builder(app: &mut App) {
    // UiCanvasPlugin enables graphics
    // app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));
    let odor_colors = Colors::from(["green", "azure"]);

    UiBuilder::build(app, |ui| {
        ui.horizontal_size(0.5, |ui| {
            ui.horizontal(|ui| {
                ui.view(UiGraphPlugin::new()
                    .item("v", |b: &Body| b.speed())
                );
                ui_motive(ui);
            });

            ui.horizontal_size(0.5, |ui| {
                ui_homunculus(ui);
            });
        });

        ui.horizontal(|ui| {
            // Main

            let alpha = 0.4;
            let mut hex = UiWorldHexPlugin::new();
            hex.tile(PlaceKind::None);
            hex.tile(PlaceKind::FoodA).pattern(Pattern::CheckerBoard(8), Color::from("red").set_alpha(alpha));
            hex.tile(PlaceKind::FoodB).pattern(Pattern::CheckerBoard(8), Color::from("teal").set_alpha(alpha));
            hex.tile(PlaceKind::OtherA).pattern(Pattern::CheckerBoard(8), Color::from("orange").set_alpha(alpha));
            hex.tile(PlaceKind::AvoidA).pattern(Pattern::CheckerBoard(4), Color::from("purple").set_alpha(alpha));

            ui.view((
                hex,
                UiWorldPlugin::new(),
                UiBodyPlugin::new()
            ));

        
            ui.vertical_size(0.5, |ui| {
                ui.view(UiAttentionPlugin::new()
                    .colors(odor_colors)
                    // .item("v", |p: &Phototaxis| p.value())
                    .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodA))
                    .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodB))
                );

                ui.view(UiRetinaPlugin::new());
                // ui.view(UiCameraPlugin::new());

                ui.view(UiHeatmapPlugin::new());

            });
        });
    });
}


#[allow(unused)]
fn ui_chemotaxis(ui: &mut UiBuilder) {
    /*
    // UiCanvasPlugin enables graphics
    // app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    ui.view(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    ui.view(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    ui.view(UiBodyTrailPlugin);

    ui.view(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(food)", |w: &World, b: &Body| 0.) // if b.eat().is_sensor_food() { 1. } else { 0. })
    );

    ui.view(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    let colors = Colors::from(["amber", "azure", "red", "purple", "blue", "green", "olive"]);
    // food_graph(app, (0.0, 1.0), (2., 1.));

    ui.view(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
        .colors(colors.clone())
        .item("v", |tx: &Chemotaxis| tx.value().clamp(0., 1.))
        .item("gr", |tx: &Chemotaxis| 0.5 * (tx.gradient() + 1.))
    );

    //food_peptides(app, (2.0, 1.0), (0.5, 1.));

    let odor_colors = Colors::from(["green", "azure"]);
    ui.view(UiAttentionPlugin::new((2.0, 1.0), (0.5, 1.))
        .colors(odor_colors)
        // .item("v", |p: &Phototaxis| p.value())
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodA))
        .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodB))
    );

    ui.view(UiMotivePlugin::new((2.5, 1.), (0.5, 1.)));
    */
}

#[allow(unused)]
fn ui_phototaxis(ui: &mut UiBuilder) {
    /*
    // UiCanvasPlugin enables graphics
    // app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));

    ui.view(UiWorldPlugin::new((0., 0.), (2., 1.0)));
    ui.view(UiBodyPlugin); // ::new((0., 0.5), (0.25, 0.5)));
    ui.view(UiBodyTrailPlugin);

    ui.view(UiTablePlugin::new((2., 0.7), (1., 0.3))
        .p_item("p(light)", |w: &World, b: &Body| w.light(b.pos()))
    );

    ui.view(UiHeatmapPlugin::new(((2., 0.), [1., 0.7])));

    // food_graph(&mut app, (0.0, 1.0), (2., 1.));
    let colors = Colors::from(["amber", "sky", "olive", "red", "purple", "blue"]);

    ui.view(UiGraphPlugin::new(((0.0, 1.0), [2., 1.]))
        .colors(colors.clone())
        .item("v", |p: &Phototaxis| p.value())
        .item("avg", |p: &Phototaxis| p.average())
        .item("grad", |p: &Phototaxis| p.gradient() / 2. + 0.5)
        .item("s-av", |p: &Phototaxis| p.short_average())
        .item("s-gr", |p: &Phototaxis| p.short_gradient() / 2. + 0.5)
    );

    // food_peptides(&mut app, (2.0, 1.0), (0.5, 1.));

    ui.view(UiPeptidePlugin::new((2.0, 1.0), (0.5, 1.))
        .colors(colors)
        .item("v", |p: &Phototaxis| p.value())
        .item("av", |p: &Phototaxis| p.average())
        .item("gr", |p: &Phototaxis| p.gradient() / 2. + 0.5)
        .item("sa", |p: &Phototaxis| p.short_average())
        .item("sg", |p: &Phototaxis| p.short_gradient() / 2. + 0.5)
    );

    ui.view(UiMotivePlugin::new((2.5, 1.), (0.5, 1.)));
    */
}
