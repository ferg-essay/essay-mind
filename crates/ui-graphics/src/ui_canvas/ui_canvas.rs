use essay_ecs::prelude::*;
use essay_plot::artist::PathStyle;
use essay_plot_base::{TextStyle, Bounds};
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path, Clip, driver::Renderer};
use essay_plot_wgpu::{PlotCanvas, PlotRenderer};
use essay_tensor::Tensor;
use winit::event_loop::EventLoop;

use super::{ScreenApi, RendererApi, CanvasState, WgpuCanvas, CanvasView};
use super::winit_loop::{main_loop, WinitEvents};

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

    pub(crate) fn init_view(&mut self) {
        assert!(self.view.is_none());

        if self.is_stale || true {
            self.is_stale = false;

            self.plot_canvas.clear();
            let view = self.wgpu.create_view();
            self.wgpu.clear_screen(&view.view);

            self.view = Some(view);
        }
    }

    pub(crate) fn close_view(&mut self) {
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
    
            plot_renderer.draw_path(path, style, &Clip::None).unwrap();

            plot_renderer.flush();
        }
    }

    pub fn draw_text(&mut self, xy: Point, text: &str, text_style: &TextStyle) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            let style = PathStyle::new();
    
            plot_renderer.draw_text(xy, text, 0., &style, text_style, &Clip::None).unwrap();

            plot_renderer.flush();
        }
    }

    pub fn draw_image(&mut self, pos: &Bounds<Canvas>, colors: Tensor<u8>) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            //let style = PathStyle::new();
    
            plot_renderer.draw_image(pos, &colors, &Clip::None).unwrap();
            plot_renderer.flush();
        }
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.plot_canvas.set_canvas_bounds(width, height);
        self.plot_canvas.set_scale_factor(2.);
        self.set_stale();
    }

    pub(crate) fn set_stale(&mut self) {
        self.is_stale = true;
    }
}

pub struct UiCanvasPlugin;

impl Plugin for UiCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinitEvents>();

        let event_loop = EventLoop::new();

        let wgpu = WgpuCanvas::new(&event_loop);
        let ui_canvas = UiCanvas::new(wgpu);

        app.event::<UiWindowEvent>();
        app.system(First, ui_canvas_window);

        app.insert_resource(ui_canvas);
        app.insert_resource_non_send(event_loop);

        app.system(PreUpdate, ui_canvas_first);
        app.system(PostUpdate, ui_canvas_last);

        app.runner(|app| {
            main_loop(app);
        });
    }
}

fn ui_canvas_first(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.init_view();
}

fn ui_canvas_last(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.close_view();
}

fn ui_canvas_window(
    mut ui_canvas: ResMut<UiCanvas>, 
    mut events: InEvent<UiWindowEvent>
) {
    for event in events.iter() {
        match event {
            UiWindowEvent::Resized(width, height) => {
                ui_canvas.window_bounds(*width, *height);
            }
            _ => {}
        }
    }

}

pub enum UiWindowEvent {
    Resized(u32, u32),
}

impl Event for UiWindowEvent {}

pub enum UiMouseEvent {
}

impl Event for UiMouseEvent {}
