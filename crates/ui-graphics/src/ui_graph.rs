use essay_ecs_core::base_app::BaseApp;
use essay_plot::artist::PathStyle;
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path};
use winit::event_loop::EventLoop;

use crate::backend::{ScreenApi, RendererApi, main_loop, CanvasState, WgpuCanvas};

pub fn ui_main(mut app: BaseApp) {
    let events = EventLoop::new();
    let wgpu = WgpuCanvas::new(&events);

    app.insert_resource(wgpu);

    main_loop(app, events);
}

struct Screen {
    count: usize,
}

impl ScreenApi for Screen {
    fn tick(&mut self) {
        let count = self.count;
        self.count = count + 1;

        //println!("Tick {:?}", count);
    }

    fn draw(&mut self, canvas: &mut dyn RendererApi) {
        let mut style = PathStyle::new();
        style.color("azure");

        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(20., 20.)),
            PathCode::LineTo(Point(400., 20.)),
            PathCode::LineTo(Point(400., 40.)),
            PathCode::ClosePoly(Point(20., 40.)),
        ]);

        canvas.draw_path(&path, &style);
    }

    fn event(&mut self, canvas: &CanvasState, event: &CanvasEvent) {
        println!("Event {:?}", event);
    }
}