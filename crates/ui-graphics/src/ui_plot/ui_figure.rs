use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use essay_ecs::core::Store;
use essay_ecs::prelude::*;
use essay_plot::artist::{Lines2d, LinesOpt, GridColorOpt, GridColor};
use essay_plot::frame::Layout;
use essay_plot::graph::{FigureInner, GraphId};
use essay_plot::prelude::driver::FigureApi;
use essay_plot::prelude::*;
use essay_plot::wgpu::PlotRenderer;
use essay_tensor::Tensor;

use crate::UiCanvas;
use crate::ui_layout::UiLayoutEvent;
use crate::{UiCanvasPlugin, ui_layout::{UiLayout, BoxId}};

use super::UiPlot;

pub struct UiFigures {
}



pub struct UiFigure2<Key> {
    box_id: BoxId,
    inner: UiFigureArc,
    bounds: Bounds::<Canvas>,

    marker: PhantomData<Key>
}

impl<K: Send + Sync + 'static> UiFigure2<K> {
    fn new(box_id: BoxId) -> Self {
        let figure = FigureInner::new();
        Self {
            box_id,
            inner: UiFigureInner::new(figure),
            bounds: Bounds::<Canvas>::zero(),
            marker: PhantomData::default(),
        }
    }

    pub fn plot_xy(&self, xy_grid: impl Into<Point>, wh_grid: impl Into<Point>) -> UiPlot {
        self.inner.0.lock().unwrap().plot_xy(Bounds::new(xy_grid, wh_grid))
    }

    pub fn plot_xy_old(&self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        self.inner.0.lock().unwrap().plot_old(x, y)
    }

    pub fn x_label(&self, label: impl AsRef<str>) {
        self.inner.0.lock().unwrap().x_label(label.as_ref())
    }

    pub fn color_grid(&self, data: impl Into<Tensor>) -> GridColorOpt {
        self.inner.0.lock().unwrap().color_grid(data)
    }

    fn draw(ui_plot: ResMut<UiFigure2<K>>, mut ui_canvas: ResMut<UiCanvas>) {
        if let Some(mut renderer)= ui_canvas.plot_renderer() {
            let canvas = Canvas::new(ui_plot.bounds.clone(), 2.);
            ui_plot.inner.0.lock().unwrap().figure.update_canvas(&canvas);
            ui_plot.inner.0.lock().unwrap().figure.draw(&mut renderer, &ui_plot.bounds);
            
            renderer.flush();
        }
    }
    
    pub fn resize(
        mut ui_plot: ResMut<UiFigure2<K>>, 
        ui_layout: Res<UiLayout>,
        mut read: InEvent<UiLayoutEvent>
    ) {
        for _ in read.iter() {
            let bounds = ui_layout.get_box(ui_plot.box_id).clone();
    
            let canvas = Canvas::new(bounds.clone(), 2.);

            ui_plot.bounds = bounds.clone();
    
            ui_plot.inner.0.lock().unwrap().figure.update_canvas(&canvas);
        }
    }
    }

pub struct UiFigure2Plugin<Key> {
    bounds: Bounds<UiLayout>,
    marker: PhantomData<Key>,
}

impl<K> UiFigure2Plugin<K> {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            marker: PhantomData::default(),
        }
    }
}

impl<K: Send + Sync + 'static> Plugin for UiFigure2Plugin<K> {
    fn build(&self, app: &mut App) {
        let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
        if ! app.contains_resource::<UiFigure2<K>>() {
            let figure = UiFigure2::<K>::new(box_id);
            app.insert_resource(figure);

            app.system(Update, UiFigure2::<K>::resize);
            app.system(Update, UiFigure2::<K>::draw);
        }
    }
}

#[derive(Component)]
pub struct UiFigure {
    box_id: BoxId,
    inner: UiFigureArc,
    bounds: Bounds::<Canvas>,
}

impl UiFigure {
    fn new(box_id: BoxId) -> Self {
        let figure = FigureInner::new();
        // let graph_id = figure.new_graph([1., 1.]).id();
        
        Self {
            box_id,
            inner: UiFigureInner::new(figure),
            bounds: Bounds::<Canvas>::zero(),
        }
    }

    fn canvas_bounds(&mut self, bounds: &Bounds<Canvas>) {
        let canvas = Canvas::new(bounds.clone(), 2.);

        self.bounds = bounds.clone();

        self.inner.0.lock().unwrap().figure.update_canvas(&canvas);
    }

    fn draw(&mut self, renderer: &mut PlotRenderer) {
        let canvas = Canvas::new(self.bounds.clone(), 2.);
        self.inner.0.lock().unwrap().figure.update_canvas(&canvas);

        self.inner.0.lock().unwrap().figure.draw(renderer, &self.bounds);
    }

    pub fn plot_xy(&self, xy_grid: impl Into<Point>, wh_grid: impl Into<Point>) -> UiPlot {
        self.inner.0.lock().unwrap().plot_xy(Bounds::new(xy_grid, wh_grid))
    }

    pub fn plot_xy_old(&self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        self.inner.0.lock().unwrap().plot_old(x, y)
    }

    pub fn x_label(&self, label: impl AsRef<str>) {
        self.inner.0.lock().unwrap().x_label(label.as_ref())
    }

    pub fn color_grid(&self, data: impl Into<Tensor>) -> GridColorOpt {
        self.inner.0.lock().unwrap().color_grid(data)
    }
}

pub struct UiFigureArc(Arc<Mutex<UiFigureInner>>);

pub struct UiFigureInner {
    figure: FigureInner,
    graph_id: Option<GraphId>,
}

impl UiFigureInner {
    fn new(figure: FigureInner) -> UiFigureArc {
        UiFigureArc(Arc::new(Mutex::new(
            UiFigureInner {
                figure,
                graph_id: None,
            }            
        )))
    }

    fn plot_xy(&mut self, grid: Bounds::<Layout>) -> UiPlot {
        let mut graph = match self.graph_id {
            Some(graph_id) => self.figure.get_graph(graph_id),

            None => {
                let graph = self.figure.new_graph(grid);
                self.graph_id = Some(graph.id());
                graph
            }
        };

        UiPlot::new(graph)
    }

    fn plot_old(&mut self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        let mut graph = match self.graph_id {
            Some(graph_id) => self.figure.get_graph(graph_id),

            None => {
                let graph = self.figure.new_graph([0., 0., 1.5, 1.]);
                self.graph_id = Some(graph.id());
                graph
            }
        };
        let lines = Lines2d::from_xy(x, y);

        graph.artist(lines)
    }

    fn x_label(&mut self, label: &str) {
        let mut graph = match self.graph_id {
            Some(graph_id) => self.figure.get_graph(graph_id),

            None => {
                let graph = self.figure.new_graph([0., 0., 1.5, 1.]);
                self.graph_id = Some(graph.id());
                graph
            }
        };

        graph.x_label(label);
    }

    fn color_grid(&mut self, data: impl Into<Tensor>) -> GridColorOpt {
        let mut graph = self.figure.new_graph([1.5, 0., 2., 1.]);
        graph.flip_y(true);
        graph.x().visible(false);
        graph.y().visible(false);
        let colormesh = GridColor::new(data);

        graph.artist(colormesh)
    }
}

pub struct UiPlotPlugin;

/*
fn ui_plot_draw(mut ui_plot: ResMut<UiFigure>, mut ui_canvas: ResMut<UiCanvas>) {
    if let Some(mut renderer)= ui_canvas.plot_renderer() {
        ui_plot.draw(&mut renderer);
        
        renderer.flush();
    }
}

pub fn ui_plot_resize(
    mut ui_plot: ResMut<UiFigure>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let bounds = ui_layout.get_box(ui_plot.box_id).clone();

        ui_plot.canvas_bounds(&bounds);
    }
}

pub fn ui_plot_spawn(
    mut commands: Commands,
    mut ui_layout: ResMut<UiLayout>,
) {
    // spawn_world(commands);

    let id = ui_layout.add_box(Bounds::new(Point(0., 1.), Point(1., 2.)));

    let ui_plot = UiFigure::new(id);

    commands.insert_resource(ui_plot);
}

impl Plugin for UiPlotPlugin {
    fn build(&self, app: &mut App) {
        if ! app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiCanvasPlugin);
        }

        app.system(PreStartup, ui_plot_spawn);

        app.system(Update, ui_plot_resize);
        app.system(Update, ui_plot_draw);
    }
}
*/