
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
                    Key::N1 => { ticks.set_n_ticks(1); }
                    Key::N2 => { ticks.set_n_ticks(2); }
                    Key::N3 => { ticks.set_n_ticks(3); }
                    Key::N4 => { ticks.set_n_ticks(4); }
                    Key::N5 => { ticks.set_n_ticks(5); }
                    Key::N6 => { ticks.set_n_ticks(6); }
                    Key::N7 => { ticks.set_n_ticks(7); }
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
