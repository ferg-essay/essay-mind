
//
// UiLateralLinePlugin
//

use essay_ecs::{app::{App, Plugin, PreUpdate}, core::{Res, ResMut}};
use essay_plot::api::input::{Event, Key};
use mind_ecs::TickConfig;
use ui_graphics::UiCanvas;

fn key_listen(canvas: Res<UiCanvas>, mut ticks: ResMut<TickConfig>) {
    for event in canvas.input().events() {
        match event {
            Event::KeyPress(key) => {
                match key {
                    Key::Space => { ticks.toggle_run(); }
                    Key::T => { ticks.one_tick(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct UiRunControl;

impl Plugin for UiRunControl {
    fn build(&self, app: &mut App) {
        app.system(PreUpdate, key_listen);
    }
}
