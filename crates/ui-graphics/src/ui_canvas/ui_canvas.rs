use std::ops::{Deref, DerefMut};
use std::time::Duration;

use essay_ecs::prelude::*;
use essay_graphics::{
    api::{
        renderer::{Canvas, Drawable, Event, Renderer, Result},
        Point, Path, Bounds, Color, ImageId,
    },
    layout::{View, ViewArc, PageBuilder, Page},
    wgpu::{PlotCanvas, PlotRenderer},
};
use essay_plot::artist::PathStyle;
use essay_plot::api::{FontStyle, FontTypeId, TextStyle};
use essay_plot::chart::Chart;
use essay_tensor::Tensor;
use winit::event_loop::EventLoop;

use super::{WgpuCanvas, CanvasView};
use super::winit_loop::{main_loop, WinitEvents};

pub struct UiCanvas {
    wgpu: WgpuCanvas,
    canvas: PlotCanvas,
    // layout: ChartBuilder,
    page: Page,

    view: Option<CanvasView>,
    is_stale: bool,
}

impl UiCanvas {
    pub(crate) fn new(wgpu: WgpuCanvas, page: Page) -> Self {
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
            // layout: ChartBuilder::new(Layout::new()),
            page,
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

    pub(crate) fn draw(&mut self) {
        if let Some(view) = &self.view {
            let mut renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
                Some(&view.view)
            );

            self.page.draw(&mut renderer).unwrap();
        }
    }

    pub(crate) fn close_view(&mut self) {
        self.view.take();
    }

    /*
    pub fn chart(&mut self, pos: impl Into<Bounds<Layout>>) -> Chart {
        self.layout.chart(pos)
    }

    pub fn view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Layout>>, 
        view: T
    ) -> View<T> {
        self.layout.get_layout_mut().view(pos, view)
    }

    pub fn subview<T: Drawable + Send + 'static>(
        &mut self, 
        id: ViewId,
        index: usize,
        view: T
    ) -> View<T> {
        self.layout.get_layout_mut().subview(id, index, view)
    }
    */

    pub fn renderer<'a>(&'a mut self) -> Option<UiRender<'a>> {
        match &self.view {
            Some(view) => {
                Some(UiRender::new(self.canvas.renderer(
                    &self.wgpu.device,
                    &self.wgpu.queue,
                    Some(&view.view)    
                )))
            },
            None => None
        }
    }

    pub fn renderer_viewless<'a>(&'a mut self) -> PlotRenderer<'a> {
        self.canvas.renderer(
            &self.wgpu.device, 
            &self.wgpu.queue, 
            None,
        )
    }

    pub fn renderer_draw<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(self.canvas.renderer(
                    &self.wgpu.device, 
                    &self.wgpu.queue, 
                    Some(&view.view)
                ))
            },
            None => None
        }
    }

    fn plot_renderer<'a>(&'a mut self) -> Option<PlotRenderer<'a>> {
        match &self.view {
            Some(view) => {
                Some(self.canvas.renderer(
                    &self.wgpu.device, 
                    &self.wgpu.queue, 
                    Some(&view.view)
                ))
            },
            None => None
        }
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        if let Some(mut renderer) = self.plot_renderer() {
            renderer.draw_path(path, style).unwrap();

            renderer.flush();
        }
    }

    pub fn draw_text(
        &mut self, 
        xy: impl Into<Point>, 
        text: &str, 
        text_style: &TextStyle
    ) {
        if let Some(view) = &self.view {
            let mut plot_renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
                Some(&view.view)
            );

            let style = PathStyle::new();
    
            plot_renderer.draw_text(xy.into(), text, 0., &style, text_style).unwrap();

            plot_renderer.flush();
        }
    }

    pub fn create_image(&mut self, colors: Tensor<u8>) -> Option<ImageId> {
        if let Some(view) = &self.view {
            let mut plot_renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
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
            let mut plot_renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
                Some(&view.view)
            );

            //canvas.clear_screen(&view);

            //let style = PathStyle::new();
    
            plot_renderer.draw_image_ref(pos, image).unwrap();
            plot_renderer.flush();
        }
    }

    pub fn draw_markers(
        &mut self, 
        path: &Path<Canvas>,
        xy: impl Into<Tensor>,
        sizes: impl Into<Tensor>,
        colors: &Vec<Color>,
    ) {
        if let Some(view) = &self.view {
            let mut plot_renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
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
            ).unwrap();

            plot_renderer.flush();
        }
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.canvas.resize(&self.wgpu.device, width, height);
        self.canvas.set_scale_factor(2.);
        self.set_stale();

        let mut renderer = self.canvas.renderer(
            &self.wgpu.device, 
            &self.wgpu.queue, 
            None, // Some(&view.view)
        );
        /*
        self.layout.get_layout_mut().resize(
            &mut renderer,
            &Bounds::from([width as f32, height as f32])
        );
        */
    }

    pub(crate) fn set_stale(&mut self) {
        self.is_stale = true;
    }
}

pub struct UiRender<'a> {
    renderer: PlotRenderer<'a>,
}

impl<'a> UiRender<'a> {
    fn new(renderer: PlotRenderer<'a>) -> Self {
        Self {
            renderer,
        }
    }

    pub fn renderer(&mut self) -> &mut PlotRenderer<'a> {
        &mut self.renderer
    }

    pub fn draw_path(&mut self, path: &Path<Canvas>, style: &PathStyle) {
        self.renderer.draw_path(path, style).unwrap();

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
        ).unwrap();
    }

    pub fn draw_image(&mut self, pos: &Bounds<Canvas>, image: ImageId) {
        self.renderer.draw_image_ref(pos, image).unwrap();
    }

    pub fn flush(&mut self) {
        self.renderer.flush();
    }

    pub fn font(&mut self, family: &str) -> FontTypeId {
        let mut style = FontStyle::new();
        
        style.family(family);
        
        self.renderer.font(&style).unwrap()
    }
}

impl<'a> Deref for UiRender<'a> {
    type Target = dyn Renderer + 'a;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

impl<'a> DerefMut for UiRender<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.renderer
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
    fn draw(&mut self, _renderer: &mut dyn Renderer) -> Result<()> {
        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
    }
}

pub struct UiBuilder<'a> {
    app: &'a mut App,
    page: PageBuilder,

    sub: UiSubBuilder<'a>,

    time: Duration,
}

impl<'a> UiBuilder<'a> {
    pub fn build(app: &'a mut App, f: impl FnOnce(&mut UiSubBuilder)) {
        let time = Duration::from_millis(30);
        let mut page = Page::builder();

        let mut builder = UiSubBuilder {
            app,
            page: page.vertical(),
        };

        (f)(&mut builder);

        let ui_canvas = UiCanvasPlugin {
            time: time,
            page: page.build(),
        };

        app.plugin(ui_canvas);
    }
    /*
    pub fn view<T>(&mut self, mut plugin: impl ViewPlugin<T> + 'static)
    where
        T: Drawable + Send + 'static
    {
        if let Some(view) = plugin.view(self.app) {
            self.page.view(view.clone());
        }

        self.app.plugin(plugin);
    }

    pub fn horizontal(&mut self, builder: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.horizontal();
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (builder)(&mut sub_ui);
    }

    pub fn horizontal_height(&mut self, height: f32, builder: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.horizontal_height(height);
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (builder)(&mut sub_ui);
    }

    pub fn vertical(&mut self, builder: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.vertical();
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (builder)(&mut sub_ui);
    }

    pub fn build(self) {
        let ui_canvas = UiCanvasPlugin {
            time: self.time,
            page: self.page.build(),
        };

        self.app.plugin(ui_canvas);
    }
    */
}

pub struct UiSubBuilder<'a> {
    app: &'a mut App,
    page: &'a mut PageBuilder,
}

impl UiSubBuilder<'_> {
    pub fn view(&mut self, mut plugin: impl IntoViewPlugin)
    {
        if let Some(view) = plugin.build_view(self.app) {
            self.page.view(view.clone());
        }

        plugin.build(self.app); // .plugin(plugin);
    }

    pub fn horizontal(&mut self, f: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.horizontal();
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (f)(&mut sub_ui);
    }

    pub fn horizontal_height(&mut self, h: f32, f: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.horizontal_height(h);
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (f)(&mut sub_ui);
    }

    pub fn vertical(&mut self, builder: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.vertical();
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (builder)(&mut sub_ui);
    }

    pub fn vertical_width(&mut self, w: f32, f: impl FnOnce(&mut UiSubBuilder)) {
        let sub_page = self.page.vertical_width(w);
        let mut sub_ui = UiSubBuilder {
            app: self.app,
            page: sub_page,
        };

        (f)(&mut sub_ui);
    }

}

pub trait ViewPlugin : Plugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc>;
}

pub trait IntoViewPlugin {
    fn build_view(&mut self, app: &mut App) -> Option<ViewArc>;
    fn build(self, app: &mut App);
}

impl<T: ViewPlugin + 'static> IntoViewPlugin for T {
    fn build_view(&mut self, app: &mut App) -> Option<ViewArc> {
        self.view(app).map(|v| v.clone())
    }

    fn build(self, app: &mut App) {
        app.plugin(self);
    }
}

impl<T1, T2> IntoViewPlugin for (T1, T2)
where
    T1: IntoViewPlugin,
    T2: IntoViewPlugin,
{
    fn build_view(&mut self, app: &mut App) -> Option<ViewArc> {
        poly_arc(&[
            self.0.build_view(app),
            self.1.build_view(app),
        ])
    }

    fn build(self, app: &mut App) {
        self.0.build(app);
        self.1.build(app);
    }
}

impl<T1, T2, T3> IntoViewPlugin for (T1, T2, T3)
where
    T1: IntoViewPlugin,
    T2: IntoViewPlugin,
    T3: IntoViewPlugin,
{
    fn build_view(&mut self, app: &mut App) -> Option<ViewArc> {
        poly_arc(&[
            self.0.build_view(app),
            self.1.build_view(app),
            self.2.build_view(app),
        ])
    }

    fn build(self, app: &mut App) {
        self.0.build(app);
        self.1.build(app);
        self.2.build(app);
    }
}

fn poly_arc(views: &[Option<ViewArc>]) -> Option<ViewArc> {
    let mut vec = Vec::<ViewArc>::new();

    for view in views {
        if let Some(view) = view {
            vec.push(view.clone())
        }
    }

    if vec.len() == 0 {
        None
    } else if vec.len() == 1 {
        vec.pop()
    } else {
        Some(ViewArc::from(PolyDraw {
            vec,
        }))
    }
}

struct PolyDraw {
    vec: Vec<ViewArc>
}

impl Drawable for PolyDraw {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        for arc in &self.vec {
            arc.drawable().draw(renderer)?;
        }

        Ok(())
    }
}

struct UiCanvasPlugin {
    time: Duration,
    page: Page,
}

impl UiCanvasPlugin {
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
        let ui_canvas = UiCanvas::new(wgpu, self.page.clone());

        app.event::<UiWindowEvent>();
        app.system(First, ui_canvas_window);

        app.insert_resource(ui_canvas);
        app.insert_resource_non_send(event_loop);

        app.system(PreUpdate, ui_canvas_pre_update);
        app.system(Update, ui_canvas_draw);
        app.system(PostUpdate, ui_canvas_post_update);

        let time = self.time.clone();
        app.runner(move |app| {
            main_loop(app, time, 1)
        });
    }
}

fn ui_canvas_pre_update(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.init_view();
}

fn ui_canvas_draw(mut ui_canvas: ResMut<UiCanvas>) {
    ui_canvas.draw();
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
