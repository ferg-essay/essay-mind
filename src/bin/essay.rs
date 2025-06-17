use essay_plot::api::{Color, Colors};
use log::LevelFilter;
use vertebrate::{
    body::BodyEat,
    builder::AnimalBuilder, 
    hind_brain::{
        r1_thigmotaxis::{Thigmotaxis, ThigmotaxisStrategy}, 
        ArtrR2, AvoidHerePlugin, HindAvoid, HindEat, HindMove, MoveKind, Serotonin
    }, mid_brain::tectum::OrientTectum, 
    hypothalamus::{
        Dwell, Forage, Motive, MotiveEat, MotiveTrait, Sleep, Wake
    }, 
    olfactory::{odor_place::OdorPlacePlugin, olfactory_bulb::OlfactoryBulb}, 
    retina::Retina, 
    ui::{
        ui_attention::UiAttentionPlugin, ui_body::UiBodyPlugin, ui_emoji::Emoji, ui_heatmap::UiHeatmapPlugin, 
        ui_homunculus::{Orient, UiHomunculusPlugin}, 
        ui_lateral_line::UiLateralLinePlugin, ui_motive::UiMotivePlugin, ui_radar::UiRadarPlugin, ui_retina::UiRetinaPlugin, ui_run_control::UiRunControl, ui_trail::UiTrailPlugin, 
        ui_world_hex::{Pattern, UiWorldHexPlugin}, 
        ui_world_map::UiWorldPlugin
    }, 
    util::{self, Heading, Seconds, Turn}, 
    world::{
        FoodKind, FoodPlugin, OdorKind, OdorPlugin, WorldHexPlugin, WorldHexTrait, WorldPlugin
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

    let (w, h) = (21, 13);
    
    app.plugin(world_thigmotaxis(w, h)
    );

    let mut place = WorldHexPlugin::<PlaceKind>::new(w, h);
    place.circle((w as f32 - 7., 4.), 3., PlaceKind::AvoidA);
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

    let food = FoodPlugin::new();
    //food.gen_count(1).gen_radius(2.).gen_value(Seconds(120.)).gen_kind(FoodKind::Poor);
    app.plugin(food);

    let mut animal = AnimalBuilder::new();

    animal.lateral_line();

    animal.olfactory()
        .odor(OdorKind::FoodA)
        .odor(OdorKind::FoodB);

    animal.retina()
        .size(Retina::SIZE as u32)
        .fov(util::Angle::Deg(150.))// fov
        .eye_angle(util::Angle::Deg(45.));

    animal.seek().seek(false);

    animal.pretectum_obstacle().enable(true);
    animal.pretectum_touch().enable(true);
    animal.pretectum_lateral_line().enable(true);

    animal.tectum_looming().enable(false);
    
    animal.tectum_orient()
        .enable(true)
        .turn(Turn::Unit(0.15))
        .inhibited_value(0.5)
        .memory_time(Seconds(2.0))
        .timeout(Seconds(30.))
        .timeout_recover(Seconds(15.));

    animal.hind_thigmotaxis()
        .enable(true)
        .strategy(ThigmotaxisStrategy::Direct)
        .turn(Turn::Unit(0.15))
        .inhibited_value(0.5)
        .memory_time(Seconds(1.0))
        .timeout(Seconds(120.))
        .timeout_recover(Seconds(15.));

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

pub fn world_lateral_line(w: usize, h: usize) -> WorldPlugin {
    let h1 = h / 2 - 1;
    let w1 = w / 3 - 1;
    WorldPlugin::new(w, h)
        .wall((w1, 0), (1, h1 + 1))
        .wall((2 * w1, h1), (1, h - h1))
}

pub fn world_thigmotaxis(w: usize, h: usize) -> WorldPlugin {
    let h1 = h / 2 - 1;
    let w1 = w / 3 - 1;
    WorldPlugin::new(w, h)
        .wall((w1, 0), (1, h1 + 1))
        .wall((1, h1 + 3), (2, 2))
        .wall((5, h1 + 3), (2, 2))
        //.wall((9, h1 + 3), (2, 2))
        .wall((2 * w1, h1), (1, h - h1))
}

pub fn world_empty(w: usize, h: usize) -> WorldPlugin {
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

fn ui_homunculus(ui: &mut UiSubBuilder) {
    ui.plugin(UiHomunculusPlugin::new()
        .emoji(Emoji::FaceVomiting, |m: &HindEat| m.is_vomiting())
        .emoji(Emoji::FaceGrimacing, |m: &HindEat| m.is_gaping())
        .emoji(Emoji::ForkAndKnife, |m: &HindEat| m.is_eating())
        .emoji(Emoji::DirectHit, |m: &HindMove| m.action_kind() == MoveKind::Seek)
        .emoji(Emoji::Warning, |m: &HindMove| m.action_kind() == MoveKind::Avoid)
        .emoji(Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.is_active())
        .emoji(Emoji::Shark, |m: &Thigmotaxis| m.is_active())
        .emoji(Emoji::Footprints, |m: &HindMove| m.action_kind() == MoveKind::Roam)
        .emoji(Emoji::FaceSleeping, |m: &Motive<Wake>| ! m.is_active())
        .orient(|taxis: &Thigmotaxis| {
            if taxis.left_active() {
                Some(Orient(Heading::Unit(-0.20), 1.))
            } else {
                None
            }
        })
        .orient(|taxis: &Thigmotaxis| {
            if taxis.right_active() {
                Some(Orient(Heading::Unit(0.2), 1.))
            } else {
                None
            }
        })
        .orient(|taxis: &OrientTectum| {
            if taxis.active_left() {
                Some(Orient(Heading::Unit(-0.20), 1.))
            } else {
                None
            }
        })
        .orient(|taxis: &OrientTectum| {
            if taxis.active_right() {
                Some(Orient(Heading::Unit(0.2), 1.))
            } else {
                None
            }
        })
    );
}

fn ui_builder(app: &mut App) {
    // UiCanvasPlugin enables graphics
    // app.plugin(UiCanvasPlugin::new().frame_ms(Duration::from_millis(50)));
    // <div style="background-color:rgb(236, 254, 255); width: 10px; padding: 10px; border: 1px solid;"></div>
    let odor_colors = Colors::from(["green", "azure"]);

    UiBuilder::build(app, |ui| {
        ui.horizontal_size(0.5, |ui| {
            ui.horizontal(|ui| {
                // ui.view(UiGraphPlugin::new()
                //    .item("v", |b: &Body| b.speed())
                // );
                //ui.plugin(UiTablePlugin::new()
                //    .item("v", |b: &Body| b.speed())
                //    .item("hd", |b: &Body| b.head_dir().to_unit())
                //);

                ui.canvas::<Dummy>();

                /*
                let mut button = false;
                ui.app().system(Update, move |mut ui: UiPos<Dummy>, body: Res<Body>| {
                    ui.draw(|ui| {
                        ui.label(&format!("head {}", body.head_dir().to_unit()));
                        ui.button("press", button).onclick(|| button=!button);
                    });
                });
                */

                ui.plugin(UiLateralLinePlugin::new());

                ui_radar(ui);
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
            hex.tile(PlaceKind::FoodA).pattern(Pattern::CheckerBoard(8), Color::from("aquamarine").with_alpha(alpha));
            hex.tile(PlaceKind::FoodB).pattern(Pattern::CheckerBoard(8), Color::from("teal").with_alpha(alpha));
            hex.tile(PlaceKind::OtherA).pattern(Pattern::CheckerBoard(8), Color::from("orange").with_alpha(alpha));
            hex.tile(PlaceKind::AvoidA).pattern(Pattern::CheckerBoard(4), Color::from("red").with_alpha(alpha));

            let mut trail = UiTrailPlugin::new();
            trail.len(512);

            ui.plugin((
                hex,
                UiWorldPlugin::new(),
                UiBodyPlugin::new(),
                trail,
            ));

        
            ui.vertical_size(0.5, |ui| {
                ui.plugin(UiAttentionPlugin::new()
                    .colors(odor_colors)
                    // .item("v", |p: &Phototaxis| p.value())
                    .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodA))
                    .item(|ob: &OlfactoryBulb| ob.value_pair(OdorKind::FoodB))
                );

                ui.plugin(UiRetinaPlugin::new());
                //ui.plugin(UiCameraPlugin::new());

                ui.plugin(UiHeatmapPlugin::new());

            });
        });
    });

    app.plugin(UiRunControl);
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


fn ui_motive(ui: &mut UiSubBuilder) {
    ui.plugin(UiMotivePlugin::new()
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
        .item(Emoji::FaceCowboy, |m: &Serotonin<ArtrR2>| m.active_value())
        .item(Emoji::ForkAndKnife, |m: &Serotonin<HindEat>| m.active_value())
        .item(Emoji::Warning, |m: &Serotonin<HindAvoid>| m.active_value())
    );
}

fn ui_radar(ui: &mut UiSubBuilder) {
    ui.plugin(UiRadarPlugin::new()
        .item(0., Emoji::Coffee, |m: &Sleep| {
            if m.is_forage() { 
                1. 
            } else if m.is_wake() { 
                0.5 
            } else {
                0.
            }
        })
        .item(30., Emoji::DirectHit, |m: &HindMove| if m.action_kind() == MoveKind::Seek { 1. } else { 0. })
        .item(60., Emoji::MagnifyingGlassLeft, |m: &Motive<Dwell>| m.value())
        .item(90., Emoji::Footprints, |m: &HindMove| if m.action_kind() == MoveKind::Roam { 1. } else { 0. })
        //.item(135., Emoji::Shark, |m: &Thigmotaxis| m.active_value())
        .item(135., Emoji::Shark, |m: &OrientTectum| m.active_value())
        .item(180., Emoji::FaceSleeping, |m: &Sleep| if m.is_wake() { 0. } else { 1. })
        .item(240., Emoji::NoEntry, |m: &HindMove| {
            if m.is_obstacle() || m.is_avoid() { 1. } else { 0. }
        })
        .item(300., Emoji::Warning, |m: &Serotonin<HindAvoid>| m.active_value())
    );
}
