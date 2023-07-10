use essay_ecs::prelude::*;
use essay_plot::artist::PathStyle;
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path, Clip, driver::Renderer};
use essay_plot_wgpu::{PlotCanvas, PlotRenderer};
use winit::event_loop::EventLoop;

use super::{ScreenApi, RendererApi, CanvasState, WgpuCanvas, CanvasView};
use super::plugin::UiCanvasPlugin;
use super::winit_loop::main_loop;

pub fn ui_main(mut app: App) {
    app.add_plugin(UiCanvasPlugin);

    //let events = EventLoop::new();
    //let wgpu = WgpuCanvas::new(&events);
    //let ui_canvas = UiCanvas::new(wgpu);

    //app.insert_resource(EventLoop::new());
    app.add_system(First, ui_canvas_first);
    app.add_system(Last, ui_canvas_last);

    app.add_system(Update, draw_box);
    app.add_system(Update, draw_box2);

    //main_loop(app);

    app.run();
}

fn draw_box(mut ui_canvas: ResMut<UiCanvas>) {
    let path = Path::<Canvas>::new(vec![
        PathCode::MoveTo(Point(100., 100.)),
        PathCode::LineTo(Point(400., 100.)),
        PathCode::ClosePoly(Point(100., 200.))
    ]);

    let mut style = PathStyle::new();
    style.color("beige");

    ui_canvas.draw_path(&path, &style);
}

fn draw_box2(mut ui_canvas: ResMut<UiCanvas>) {
    let path = Path::<Canvas>::new(vec![
        PathCode::MoveTo(Point(1000., 100.)),
        PathCode::LineTo(Point(1400., 100.)),
        PathCode::ClosePoly(Point(1500., 200.))
    ]);

    let mut style = PathStyle::new();
    style.color("azure");

    ui_canvas.draw_path(&path, &style);
}

pub struct UiCanvas {
    wgpu: WgpuCanvas,
    plot_canvas: PlotCanvas,

    view: Option<CanvasView>,
    is_stale: bool,
}

impl UiCanvas {
    pub(crate) fn new(wgpu: WgpuCanvas) -> Self {
        let plot_renderer = PlotCanvas::new(
            &wgpu.device,
            wgpu.config.format,
        );

        Self {
            wgpu,
            plot_canvas: plot_renderer,
            view: None,
            is_stale: true,
        }
    }

    pub fn init_view(&mut self) {
        assert!(self.view.is_none());

        if self.is_stale {
            self.is_stale = false;

            let view = self.wgpu.create_view();
            self.wgpu.clear_screen(&view.view);

            self.view = Some(view);
        }
    }

    pub fn close_view(&mut self) {
        if let Some(view) = self.view.take() {
            view.flush();
        }
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);
    
            plot_renderer.draw_path(path, style, &Clip::None);

            plot_renderer.flush();
        }
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.plot_canvas.set_canvas_bounds(width, height);
        self.set_stale();
    }

    pub(crate) fn set_stale(&mut self) {
        self.is_stale = true;
    }
}

fn ui_canvas_first(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.init_view();
}

fn ui_canvas_last(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.close_view();
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