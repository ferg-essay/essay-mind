use essay_ecs::prelude::{Plugin, App};
use winit::event_loop::EventLoop;

use crate::{winit::{WinitEvents, main_loop}, backend::WgpuCanvas, ui_canvas::UiCanvas};

pub struct WinitPlugin;

impl Plugin for WinitPlugin {
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