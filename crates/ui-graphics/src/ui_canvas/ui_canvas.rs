use std::time::Duration;

use essay_ecs::prelude::*;
use essay_graphics::layout::{Layout, View};
use essay_plot::artist::PathStyle;
use essay_plot::api::{Bounds, CanvasEvent, FontStyle, FontTypeId, TextStyle};
use essay_plot::api::{Canvas, Point, Path, Clip, driver::Renderer, driver::Drawable};
use essay_plot::graph::graph::GraphBuilder;
use essay_plot::graph::Graph;
use essay_plot::prelude::{ImageId, Color};
use essay_plot::wgpu::{PlotCanvas, PlotRenderer};
use essay_tensor::Tensor;
use winit::event_loop::EventLoop;

use super::{WgpuCanvas, CanvasView};
use super::winit_loop::{main_loop, WinitEvents};

pub struct UiCanvas {
    wgpu: WgpuCanvas,
    canvas: PlotCanvas,
    layout: GraphBuilder,

    view: Option<CanvasView>,
    is_stale: bool,
}

impl UiCanvas {
    pub(crate) fn new(wgpu: WgpuCanvas) -> Self {
        let canvas = PlotCanvas::new(
            &wgpu.device,
            &wgpu.queue,
            wgpu.config.format,
            wgpu.config.width,
            wgpu.config.height,
        );

        Self {
            wgpu,
            canvas,
            layout: GraphBuilder::new(Layout::new()),
            view: None,
            is_stale: true,
        }
    }

    pub(crate) fn init_view(&mut self) {
        assert!(self.view.is_none());

        if self.is_stale || true {
            self.is_stale = false;

            self.canvas.clear();
            let view = self.wgpu.create_view();
            self.wgpu.clear_screen(&view.view);

            self.view = Some(view);
        }
    }

    pub(crate) fn close_view(&mut self) {
        if let Some(view) = self.view.take() {
            let pos = self.canvas.bounds().clone();

            let mut renderer = PlotRenderer::new(
                &mut self.canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            self.layout.get_layout_mut().draw(&mut renderer, &pos);
            //self.canvas..draw
            //self.layout.

            renderer.flush(&Clip::None);

            view.flush();
        }
    }

    pub fn graph(&mut self, pos: impl Into<Bounds<Layout>>) -> Graph {
        self.layout.graph(pos)
    }

    pub fn view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Layout>>, 
        view: T
    ) -> View<T> {
        self.layout.get_layout_mut().add_view(pos, view)
    }

    pub fn renderer<'a>(&'a mut self, clip: Clip) -> Option<UiRender<'a>> {
        match &self.view {
            Some(view) => {
                Some(UiRender::new(PlotRenderer::new(
                    &mut self.canvas, 
                    &self.wgpu.device, 
                    Some(&self.wgpu.queue), 
                    Some(&view.view)),
                    clip
                ))
            },
            None => None
        }
    }

    pub fn renderer_viewless<'a>(&'a mut self) -> PlotRenderer<'a> {
        PlotRenderer::new(
            &mut self.canvas, 
            &self.wgpu.device, 
            Some(&self.wgpu.queue), 
            None,
        )
    }

    pub fn renderer_draw<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(PlotRenderer::new(
                    &mut self.canvas, 
                    &self.wgpu.device, 
                    Some(&self.wgpu.queue), 
                    Some(&view.view)
                ))
            },
            None => None
        }
    }

    fn plot_renderer<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(PlotRenderer::new(
                    &mut self.canvas, 
                    &self.wgpu.device, 
                    Some(&self.wgpu.queue), 
                    Some(&view.view)
                ))
            },
            None => None
        }
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        if let Some(mut renderer) = self.plot_renderer() {
            renderer.draw_path(path, style, &Clip::None).unwrap();

            renderer.flush(&Clip::None);
        }

        /*
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);
    
            plot_renderer.draw_path(path, style, &Clip::None).unwrap();

            plot_renderer.flush(&Clip::None);
        }
        */
    }

    pub fn draw_text(
        &mut self, 
        xy: impl Into<Point>, 
        text: &str, 
        text_style: &TextStyle
    ) {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.canvas, 
                &self.wgpu.device, 
                Some(&self.wgpu.queue), 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            let style = PathStyle::new();
    
            plot_renderer.draw_text(xy.into(), text, 0., &style, text_style, &Clip::None).unwrap();

            plot_renderer.flush(&Clip::None);
        }
    }

    pub fn create_image(&mut self, colors: Tensor<u8>) -> Option<ImageId> {
        if let Some(view) = &self.view {
            let mut plot_renderer = PlotRenderer::new(
                &mut self.canvas, 
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
                &mut self.canvas, 
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
                &mut self.canvas, 
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

    /*
    pub fn plot_renderer<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(PlotRenderer::new(
                    &mut self.canvas, 
                    &self.wgpu.device, 
                    Some(&self.wgpu.queue), 
                    Some(&view.view)
                ))
            },
            None => None
        }
    }
    */

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.canvas.resize(&self.wgpu.device, width, height);
        self.canvas.set_scale_factor(2.);
        self.set_stale();

        let mut renderer = PlotRenderer::new(
            &mut self.canvas, 
            &self.wgpu.device, 
            Some(&self.wgpu.queue), 
            None, // Some(&view.view)
        );

        self.layout.event(
            &mut renderer, 
            &CanvasEvent::Resize(Bounds::from([width as f32, height as f32]))
        );
    }

    pub(crate) fn set_stale(&mut self) {
        self.is_stale = true;
    }
}

pub struct UiRender<'a> {
    renderer: PlotRenderer<'a>,
    clip: Clip,
}

impl<'a> UiRender<'a> {
    fn new(renderer: PlotRenderer<'a>, clip: Clip) -> Self {
        Self {
            renderer,
            clip
        }
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        self.renderer.draw_path(path, style, &Clip::None).unwrap();

        // self.renderer.flush(&seClip::None);
    }

    pub fn draw_markers(
        &mut self, 
        path: &Path<Canvas>,
        xy: impl Into<Tensor>,
        sizes: impl Into<Tensor>,
        colors: &Vec<Color>,
    ) {
        let color: Tensor<u32> = colors.iter().map(|c| c.to_rgba()).collect();
        let style = PathStyle::new();
    
        self.renderer.draw_markers(
            path, 
            &xy.into(), 
            &sizes.into(), 
            &color,
            &style,
            &Clip::None,
        ).unwrap();

        // plot_renderer.flush(clip);
    }

    pub fn draw_text(
        &mut self, 
        xy: impl Into<Point>, 
        text: &str, 
        path_style: &PathStyle,
        text_style: &TextStyle
    ) {
        //canvas.clear_screen(&view);
    
        self.renderer.draw_text(
            xy.into(), 
            text,
            0., 
            path_style, 
            text_style, 
            &Clip::None
        ).unwrap();
    }

    pub fn draw_image(&mut self, pos: &Bounds<Canvas>, image: ImageId) {
        self.renderer.draw_image_ref(pos, image, &Clip::None).unwrap();
    }

    pub fn flush(&mut self) {
        self.renderer.flush(&self.clip);
    }

    pub fn font(&mut self, family: &str) -> FontTypeId {
        let mut style = FontStyle::new();
        
        style.family(family);
        
        self.renderer.font(&style).unwrap()
    }
}

impl<'a> Drop for UiRender<'a> {
    fn drop(&mut self) {
        self.flush();
    }
}

pub struct UiView {

}

impl UiView {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Drawable for UiView {
    fn draw(&mut self, _renderer: &mut dyn Renderer, _pos: &Bounds<Canvas>) {
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &CanvasEvent) {
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

        let event_loop = EventLoop::new().unwrap();

        let wgpu = WgpuCanvas::new(&event_loop);
        let ui_canvas = UiCanvas::new(wgpu);

        app.event::<UiWindowEvent>();
        app.system(First, ui_canvas_window);

        app.insert_resource(ui_canvas);
        app.insert_resource_non_send(event_loop);

        app.system(PreUpdate, ui_canvas_pre_update);
        app.system(PostUpdate, ui_canvas_post_update);

        let time = self.time.clone();
        app.runner(move |app| {
            main_loop(app, time, 1);
        });
    }
}

fn ui_canvas_pre_update(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.init_view();
}

fn ui_canvas_post_update(mut ui_canvas: ResMut<UiCanvas>) {
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
