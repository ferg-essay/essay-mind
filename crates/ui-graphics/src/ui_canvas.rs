use std::sync::{Arc, Mutex};

use essay_ecs_core::base_app::BaseApp;
use essay_plot::artist::PathStyle;
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path, Clip, driver::Renderer};
use essay_plot_wgpu::{PlotCanvas, PlotRenderer};
use winit::event_loop::EventLoop;

use crate::backend::{ScreenApi, RendererApi, main_loop, CanvasState, WgpuCanvas};

pub fn ui_main(mut app: BaseApp) {
    let events = EventLoop::new();
    let wgpu = WgpuCanvas::new(&events);
    let ui_canvas = UiCanvas::new(wgpu);

    app.insert_resource(ui_canvas);

    main_loop(app, events);
}

pub struct UiCanvas {
    wgpu: WgpuCanvas,
    plot_canvas: PlotCanvas,
}

impl UiCanvas {
    fn new(wgpu: WgpuCanvas) -> Self {
        let plot_renderer = PlotCanvas::new(
            &wgpu.device,
            wgpu.config.format,
        );

        Self {
            wgpu,
            plot_canvas: plot_renderer,
        }
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        self.wgpu.draw(|canvas, view| {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &canvas.device, 
                Some(&canvas.queue), 
                Some(view)
            );

            //canvas.clear_screen(&view);
    
            plot_renderer.draw_path(path, style, &Clip::None);

            plot_renderer.flush();
        });
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.plot_canvas.set_canvas_bounds(width, height);
    }

    pub(crate) fn set_stale(&mut self) {
        // self.is_stale = true;
    }
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