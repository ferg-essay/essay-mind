mod essay_13_plankton;

use essay_ecs::prelude::App;
use ui_graphics::{UiCanvasPlugin, ui_plot::UiPlotPlugin};
use essay_13_plankton::{PlanktonWorldPlugin, PlanktonBodyPlugin};

pub fn main() {
    let mut app = App::new();

    // UiCanvasPlugin enables graphics
    app.plugin(UiCanvasPlugin);
    app.plugin(UiPlotPlugin);
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