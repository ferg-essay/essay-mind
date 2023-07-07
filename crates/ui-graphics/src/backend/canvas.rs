use super::{wgpu_canvas::WgpuCanvas, screen::ScreenApi};
use essay_plot_base::{Path, Canvas, PathOpt, Clip, driver::Renderer};
use essay_plot_wgpu::{PlotCanvas, PlotRenderer};

pub struct CanvasState {
    wgpu_canvas: WgpuCanvas,

    plot_renderer: PlotCanvas,

    is_stale: bool, 
}

impl CanvasState {
    pub(crate) fn new(
        wgpu_canvas: WgpuCanvas,
    ) -> Self {
        let plot_renderer = PlotCanvas::new(
            &wgpu_canvas.device,
            wgpu_canvas.config.format,
        );

        Self {
            wgpu_canvas,
            plot_renderer,
            is_stale: true,
        }
    }

    /*
    pub(crate) fn renderer<'a>(&'a mut self) -> Renderer<'a> {
        Renderer {
            state: self
        }
    }
    */

    pub(crate) fn set_canvas_bounds(&mut self, width: u32, height: u32) {
        self.wgpu_canvas.window_bounds(width, height);
        self.plot_renderer.set_canvas_bounds(width, height);
    }

    pub(crate) fn clear(&self) {
        // todo!()
    }

    pub(crate) fn is_stale(&self) -> bool {
        self.is_stale
    }
    pub(crate) fn draw(
        &mut self, 
        figure: &mut Box<dyn ScreenApi>,
    ) {
        self.is_stale = false;

        let frame = self.wgpu_canvas.surface
            .get_current_texture()
            .expect("Failed to get next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.wgpu_canvas.clear_screen(&view);

        let mut renderer = self.renderer(&view);

        figure.draw(&mut renderer);

        //let mut canvas = state.renderer(&self.device);

        //figure.draw(&canvas);

        //self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    fn renderer<'a>(&'a mut self, view: &'a wgpu::TextureView) -> EssayRenderer<'a> {
        EssayRenderer::new(self, view)
    }

    pub(crate) fn tick(&self, figure: &Box<dyn ScreenApi>) {
        //todo!()
    }
}

pub trait RendererApi {
    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    );
}

pub struct EssayRenderer<'a> {
    state: &'a mut CanvasState,
    view: &'a wgpu::TextureView,
}

impl<'a> EssayRenderer<'a> {
    pub fn new(
        state: &'a mut CanvasState,
        view: &'a wgpu::TextureView,
    ) -> Self {
        /*
        let renderer = state.plot_renderer.renderer(
            &state.wgpu_canvas.device,
            &state.wgpu_canvas.queue,
            view
        );
        */

        Self {
            state,
            view,
            // plot_renderer: renderer,
        }
    }

    pub(crate) fn set_canvas_bounds(&mut self, width: u32, height: u32) {
        self.state.set_canvas_bounds(width, height);
    }

    pub(crate) fn clear(&mut self) {
    }

    pub(crate) fn is_request_redraw(&self) -> bool {
        false
    }
}

impl RendererApi for EssayRenderer<'_> {
    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    ) {
        let mut renderer = self.state.plot_renderer.renderer(
            &mut self.state.wgpu_canvas.device,
            &mut self.state.wgpu_canvas.queue,
            &self.view,
        );

        renderer.draw_path(path, style, &Clip::None).unwrap();
        renderer.flush();
    }
}