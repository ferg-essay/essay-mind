use essay_ecs::prelude::{Plugin, App};
use winit::event_loop::EventLoop;

use super::{winit_loop::{WinitEvents, main_loop}, WgpuCanvas, ui_canvas::UiCanvas};

pub struct UiCanvasPlugin;

impl Plugin for UiCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinitEvents>();

        let event_loop = EventLoop::new();

        let wgpu = WgpuCanvas::new(&event_loop);
        let ui_canvas = UiCanvas::new(wgpu);

        app.insert_resource(ui_canvas);
        app.insert_resource_non_send(event_loop);

        app.set_runner(|app| {
            main_loop(app);
        });
    }
}