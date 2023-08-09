mod sense_peptide;
mod body;
mod ui_body;
mod world;
mod ui_world;
mod cilia;

pub use body::{
    Body, PlanktonBodyPlugin,
};

pub use self::world::{
    World, PlanktonWorldPlugin,
};

pub use self::ui_world::{
    UiWorld, UiApicalWorldPlugin,
    DrawWorld, DrawItem,
    draw_world,
    spawn_ui_world, world_resize,
};

use essay_ecs::prelude::App;
use ui_graphics::UiCanvasPlugin;

pub fn main() {
    let mut app = App::new();

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin);
    app.plugin(PlanktonWorldPlugin);
    app.plugin(PlanktonBodyPlugin);

    app.run();
}

#[cfg(test)]
mod test {

///
/// basic validation of the physical system
/// 
#[test]
fn body_basic() {
    let mut app = App::new();

    app.plugin(TestLogPlugin);

    app.plugin(PlanktonWorldPlugin);
    app.plugin(PlanktonBodyPlugin);

    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.5) dy=0.0", "cilia: swim=1.0 arrest=0.0"]);

    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.5) dy=0.0", "cilia: swim=1.0 arrest=0.0"]);
}

///
/// if the cilia is arrested, the plankton sinks
///
#[test]
fn arrest_sink() {
    let mut app = App::new();

    app.plugin(TestLogPlugin);
    app.plugin(PlanktonWorldPlugin);
    app.plugin(PlanktonBodyPlugin);

    app.system(PreUpdate, |body: &mut Body| { body.arrest(2.); });

    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.5) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.4) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.4) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.3) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.3) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.2) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
    app.tick();
    assert_eq!(log_take(&mut app), vec![
        "body: (2.5, 2.2) dy=0.0", "cilia: swim=1.0 arrest=1.0"]);
}
}