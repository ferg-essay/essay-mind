use std::time::Duration;

use essay_ecs::prelude::*;
use essay_plot::artist::PathStyle;
use essay_plot::api::{TextStyle, Bounds};
use essay_plot::api::{CanvasEvent, PathCode, Canvas, Point, Path, Clip, driver::Renderer};
use essay_plot::frame::Data;
use essay_plot::prelude::{ImageId, Color};
use essay_plot::wgpu::{PlotCanvas, PlotRenderer};
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

            plot_renderer.flush(&Clip::None);
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

            plot_renderer.flush(&Clip::None);
        }
    }

    pub fn create_image(&mut self, colors: Tensor<u8>) -> Option<ImageId> {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            //let style = PathStyle::new();
    
            Some(plot_renderer.create_image(&colors))
        } else {
            None
        }
    }

    pub fn draw_image(&mut self, pos: &Bounds<Canvas>, image: ImageId) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            //let style = PathStyle::new();
    
            plot_renderer.draw_image_ref(pos, image, &Clip::None).unwrap();
            plot_renderer.flush(&Clip::None);
        }
    }

    pub fn draw_markers(
        &mut self, 
        path: &Path<Canvas>,
        xy: impl Into<Tensor>,
        sizes: impl Into<Tensor>,
        colors: &Vec<Color>,
        clip: &Clip,
    ) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.plot_canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            let color: Tensor<u32> = colors.iter().map(|c| c.to_rgba()).collect();
            let style = PathStyle::new();
    
            plot_renderer.draw_markers(
                path, 
                &xy.into(), 
                &sizes.into(), 
                &color,
                &style,
                &Clip::None,
            ).unwrap();

            plot_renderer.flush(clip);
        }
    }

    pub fn plot_renderer<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(PlotRenderer::new(
                    &mut self.plot_canvas, 
                    &self.wgpu.device, 
                    Some(&self.wgpu.queue), 
                    Some(&view.view)
                ))
            },
            None => None
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

pub struct UiCanvasPlugin {
    time: Duration,
}

impl UiCanvasPlugin {
    pub fn new() -> Self {
        Self {
            time: Duration::from_millis(30),
        }
    }

    pub fn frame_ms(self, time: impl Into<Duration>) -> Self {
        Self {
            time: time.into(),
            .. self
        }
    }
}

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

        let time = self.time.clone();
        app.runner(move |app| {
            main_loop(app, time, 1);
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

#[derive(Event)]
pub enum UiWindowEvent {
    Resized(u32, u32),
}

#[derive(Event)]
pub enum UiMouseEvent {
}
