use std::time::Duration;

use essay_ecs::prelude::*;
use essay_graphics::{
    api::renderer::{Drawable, Renderer, Result},
    layout::{ViewArc, PageBuilder, Page},
    wgpu::{PlotCanvas, PlotRenderer},
};
use winit::event_loop::EventLoop;

use super::{WgpuCanvas, CanvasView};
use super::winit_loop::{main_loop, WinitEvents};

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

pub struct UiCanvas {
    wgpu: WgpuCanvas,
    canvas: PlotCanvas,
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

    pub fn renderer_viewless<'a>(&'a mut self) -> PlotRenderer<'a> {
        self.canvas.renderer(
            &self.wgpu.device, 
            &self.wgpu.queue, 
            None,
        )
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.wgpu.window_bounds(width, height);
        self.canvas.resize(&self.wgpu.device, width, height);
        self.canvas.set_scale_factor(2.);
        self.set_stale();
    }

    pub(crate) fn set_stale(&mut self) {
        self.is_stale = true;
    }
}

pub struct UiBuilder;

impl UiBuilder {
    pub fn build<R>(app: &mut App, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        assert!(! app.contains_resource::<UiCanvas>(), "UiCanvas already exists");

        let time = Duration::from_millis(30);
        let mut page = PageBuilder::new();

        let mut builder = UiSubBuilder {
            app,
            page: &mut page,
        };

        let result = (f)(&mut builder);

        app.init_resource::<WinitEvents>();

        let event_loop = EventLoop::new().unwrap();

        let wgpu = WgpuCanvas::new(&event_loop);
        let ui_canvas = UiCanvas::new(wgpu, page.build());

        app.event::<UiWindowEvent>();
        app.system(First, ui_canvas_window);

        app.insert_resource(ui_canvas);
        app.insert_resource_non_send(event_loop);

        app.system(PreUpdate, ui_canvas_pre_update);
        app.system(Update, ui_canvas_draw);
        app.system(PostUpdate, ui_canvas_post_update);

        // let time = self.time.clone();
        let time = time;
        app.runner(move |app| {
            main_loop(app, time, 1)
        });

        result
    }
}

pub struct UiSubBuilder<'a> {
    app: &'a mut App,
    page: &'a mut PageBuilder,
}

impl UiSubBuilder<'_> {
    pub fn view(&mut self, mut plugin: impl IntoViewPlugin)
    {
        if let Some(view) = plugin.build_view(self.app) {
            self.page.view(view.drawable());
        }

        plugin.build(self.app); // .plugin(plugin);
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.horizontal(|ui| {
            let mut sub_ui = UiSubBuilder {
                app: self.app,
                page: ui,
            };

            (f)(&mut sub_ui)
        })
    }

    pub fn horizontal_size<R>(&mut self, size: f32, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.horizontal_size(size, |ui| {
            let mut sub_ui = UiSubBuilder {
                app: self.app,
                page: ui,
            };

            (f)(&mut sub_ui)
        })
    }

    pub fn vertical<R>(&mut self, builder: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.vertical(|ui| {
            let mut sub_ui = UiSubBuilder {
                app: self.app,
                page: ui,
            };

            (builder)(&mut sub_ui)
        })
    }

    pub fn vertical_size<R>(&mut self, size: f32, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.vertical_size(size, |ui| {
            let mut sub_ui = UiSubBuilder {
                app: self.app,
                page: ui,
            };

            (f)(&mut sub_ui)
        })
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
        layer_arc(&[
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
        layer_arc(&[
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

fn layer_arc(views: &[Option<ViewArc>]) -> Option<ViewArc> {
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

#[derive(Event)]
pub enum UiWindowEvent {
    Resized(u32, u32),
}
