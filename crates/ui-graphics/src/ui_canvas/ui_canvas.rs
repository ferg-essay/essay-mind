use std::{marker::PhantomData, time::Duration};

use essay_ecs::{core::{schedule::{SystemMeta, UnsafeStore}, Local, Store}, prelude::*};
use essay_graphics::{
    api::renderer::{Drawable, Renderer, Result}, layout::{Page, PageBuilder, ViewArc, ViewId}, ui::{Ui, UiState}, wgpu::{PlotCanvas, PlotRenderer}
};
use essay_plot::api::{renderer, Bounds};
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

pub struct UiBuilder;

impl UiBuilder {
    pub fn build<R>(app: &mut App, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        assert!(! app.contains_resource::<UiCanvas>(), "UiCanvas already exists");

        let time = Duration::from_millis(30);
        let mut page = PageBuilder::new();

        let mut builder = UiSubBuilder {
            app,
            page: &mut page,
            tags: Vec::new(),
        };

        let result = (f)(&mut builder);
        let tags = builder.tags.drain(..)
            .collect::<Vec<Box<dyn TagBuilder>>>();
        let page = page.build();

        for tag in &tags {
            tag.build(&page, app);
        }

        app.init_resource::<WinitEvents>();

        let event_loop = EventLoop::new().unwrap();

        let wgpu = WgpuCanvas::new(&event_loop);
        let ui_canvas = UiCanvas::new(wgpu, page);

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
    tags: Vec<Box<dyn TagBuilder>>,
}

impl UiSubBuilder<'_> {
    pub fn app(&mut self) -> &mut App {
        self.app
    }

    pub fn plugin(&mut self, mut plugin: impl IntoViewPlugin) {
        if let Some(view) = plugin.build_view(self.app) {
            let view_id = self.page.view(view.drawable());

            plugin.set_view_id(view_id);
        }

        plugin.build(self.app); // .plugin(plugin);
    }

    pub fn canvas<T: 'static>(&mut self) {
        let view_id = self.page.view(Empty);

        self.tags.push(Box::new(Tag {
            id: view_id,
            marker: PhantomData::<fn(T)>::default(),
        }));
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.horizontal(|ui| {
            sub_builder(ui, f, &mut self.tags, &mut self.app)
        })
    }

    pub fn horizontal_size<R>(&mut self, size: f32, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.horizontal_size(size, |ui| {
            sub_builder(ui, f, &mut self.tags, &mut self.app)
        })
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.vertical(|ui| {
            sub_builder(ui, f, &mut self.tags, &mut self.app)
        })
    }

    pub fn vertical_size<R>(&mut self, size: f32, f: impl FnOnce(&mut UiSubBuilder) -> R) -> R {
        self.page.vertical_size(size, |ui| {
            sub_builder(ui, f, &mut self.tags, &mut self.app)
        })
    }
}

fn sub_builder<R>(
    ui: &mut PageBuilder, 
    f: impl FnOnce(&mut UiSubBuilder) -> R,
    tags: &mut Vec<Box<dyn TagBuilder>>,
    app: &mut App,
) -> R {
    let mut sub_ui = UiSubBuilder {
        app,
        page: ui,
        tags: Vec::new(),
    };

    let result = (f)(&mut sub_ui);

    tags.append(&mut sub_ui.tags);

    result
}

trait TagBuilder {
    fn build(&self, page: &Page, app: &mut App);
}

struct Tag<T: 'static> {
    id: ViewId,
    marker: PhantomData<fn(T)>,
}

impl<T: 'static> TagBuilder for Tag<T> {
    fn build(&self, page: &Page, app: &mut App) {
        let bounds = page.view_bounds(self.id);

        app.insert_resource(ViewPos {
            id: self.id,
            bounds,
            marker: PhantomData::<fn(T)>::default(),
        });
    }
}

pub struct ViewPos<T> {
    id: ViewId,
    bounds: Bounds<Page>,
    marker: PhantomData<fn(T)>,
}

impl<T> ViewPos<T> {
    pub fn id(&self) -> ViewId {
        self.id
    }

    pub fn bounds(&self) -> &Bounds<Page> {
        &self.bounds
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

    pub(crate) fn render<R>(
        &mut self, 
        id: ViewId,
        f: impl FnOnce(&mut dyn Renderer) -> renderer::Result<R>
    ) -> renderer::Result<R> {
        if let Some(view) = &self.view {
            let mut renderer = self.canvas.renderer(
                &self.wgpu.device, 
                &self.wgpu.queue, 
                Some(&view.view)
            );

            self.page.render(id, &mut renderer, f)
        } else {
            // todo: cleanup error handling
            Err(renderer::RenderErr::NotImplemented)
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

struct Empty;

impl Drawable for Empty {
    fn draw(&mut self, _renderer: &mut dyn Renderer) -> Result<()> {
        Ok(())
    }
}

pub trait ViewPlugin : Plugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc>;
}

pub trait IntoViewPlugin {
    fn build_view(&mut self, app: &mut App) -> Option<ViewArc>;
    fn build(self, app: &mut App);

    #[allow(unused_variables)]
    fn set_view_id(&mut self, id: ViewId) {
    }
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

pub struct UiPos<'w, 's, T> {
    canvas: ResMut<'w, UiCanvas>,
    pos: Res<'w, ViewPos<T>>,
    state: Local<'s, UiState>,

    // renderer: &'a dyn Renderer,
}

impl<T: 'static> UiPos<'_, '_, T> {
    pub fn draw<R>(&mut self, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.canvas.render(self.pos.id(), |renderer| {
            self.state.draw(renderer, |ui| {
                Ok((f)(ui))
            })
        }).unwrap()
    }
}

// TODO: create #[derive(Param)]

impl<'w, 's, T: 'static> Param for UiPos<'w, 's, T> {
    type Arg<'w1, 's1> = UiPos<'w1, 's1, T>;

    type Local = (
        <ResMut<'w, UiCanvas> as Param>::Local, 
        <Res<'w, ViewPos<T>> as Param>::Local, 
        <Local<'s, UiState> as Param>::Local,
    );

    fn init(meta: &mut SystemMeta, world: &mut Store) -> essay_ecs::core::error::Result<Self::Local> {
        Ok((
            ResMut::<UiCanvas>::init(meta, world)?,
            Res::<ViewPos<T>>::init(meta, world)?,
            Local::<UiState>::init(meta, world)?
        ))
    }

    fn arg<'w1, 's1>(
        world: &'w1 UnsafeStore,
        state: &'s1 mut Self::Local, 
    ) -> essay_ecs::core::error::Result<Self::Arg<'w1, 's1>> {
        let (c_st, v_st, s_st) = state;

        Ok(UiPos {
            canvas: ResMut::<UiCanvas>::arg(world, c_st)?,
            pos: Res::<ViewPos<T>>::arg(world, v_st)?,
            state: Local::<UiState>::arg(world, s_st)?,
        })
    }
}

#[derive(Event)]
pub enum UiWindowEvent {
    Resized(u32, u32),
}
