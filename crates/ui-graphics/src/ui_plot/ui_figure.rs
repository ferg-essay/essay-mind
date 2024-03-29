use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use essay_ecs::prelude::*;
use essay_plot::artist::{Lines2d, LinesOpt, GridColorOpt, GridColor};
use essay_plot::frame::Layout;
use essay_plot::graph::{FigureInner, GraphId};
use essay_plot::prelude::driver::FigureApi;
use essay_plot::prelude::*;
use essay_tensor::Tensor;

use crate::UiCanvas;
use crate::ui_layout::{UiLayoutEvent, UiLayout, BoxId};

use super::{UiPlot, UiGraph};

pub struct UiFigure<Key> {
    box_id: BoxId,
    inner: UiFigureArc,
    bounds: Bounds::<Canvas>,

    marker: PhantomData<Key>
}

impl<K: Send + Sync + 'static> UiFigure<K> {
    fn new(box_id: BoxId) -> Self {
        let figure = FigureInner::new();
        Self {
            box_id,
            inner: UiFigureInner::new(figure),
            bounds: Bounds::<Canvas>::zero(),
            marker: PhantomData::default(),
        }
    }

    pub fn plot_xy(&self, xy: impl Into<Point>, wh: impl Into<Point>) -> UiPlot {
        let xy = xy.into();
        let wh = wh.into();

        let bounds = Bounds::new(xy, (xy.0 + wh.0, wh.1 + wh.1));

        self.inner.0.lock().unwrap().plot_xy(bounds)
    }

    pub fn plot_xy_old(&self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        self.inner.0.lock().unwrap().plot_old(x, y)
    }

    pub fn x_label(&self, label: impl AsRef<str>) {
        self.inner.0.lock().unwrap().x_label(label.as_ref())
    }

    pub fn color_grid(
        &self, 
        xy: impl Into<Point>,
        wh: impl Into<Point>,
        data: impl Into<Tensor>
    ) -> GridColorOpt {
        let xy = xy.into();
        let wh = wh.into();

        let bounds = Bounds::new(xy, (xy.x() + wh.x(), wh.y() + wh.y()));

        self.inner.0.lock().unwrap().color_grid(bounds, data)
    }

    pub fn graph(
        &self, 
        xy: impl Into<Point>,
        wh: impl Into<Point>,
    ) -> UiGraph {
        let xy = xy.into();
        let wh = wh.into();

        let bounds = Bounds::new(xy, (xy.x() + wh.x(), wh.y() + wh.y()));

        self.inner.0.lock().unwrap().graph(bounds)
    }

    fn draw(ui_plot: ResMut<UiFigure<K>>, mut ui_canvas: ResMut<UiCanvas>) {
        if let Some(mut renderer)= ui_canvas.plot_renderer() {
            let canvas = Canvas::new(ui_plot.bounds.clone(), 2.);
            ui_plot.inner.0.lock().unwrap().figure.update_canvas(&canvas);
            ui_plot.inner.0.lock().unwrap().figure.draw(&mut renderer, &ui_plot.bounds);
            
            renderer.flush_inner(&Clip::None);
        }
    }
    
    pub fn resize(
        mut ui_plot: ResMut<UiFigure<K>>, 
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

pub struct UiFigurePlugin<Key> {
    bounds: Bounds<UiLayout>,
    marker: PhantomData<Key>,
}

impl<K> UiFigurePlugin<K> {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            marker: PhantomData::default(),
        }
    }
}

impl<K: Send + Sync + 'static> Plugin for UiFigurePlugin<K> {
    fn build(&self, app: &mut App) {
        let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
        if ! app.contains_resource::<UiFigure<K>>() {
            let figure = UiFigure::<K>::new(box_id);
            app.insert_resource(figure);

            app.system(Update, UiFigure::<K>::resize);
            app.system(Update, UiFigure::<K>::draw);
        }
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
        let graph = self.figure.new_graph(grid);

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

    fn graph(&mut self, bounds: impl Into<Bounds<Layout>>) -> UiGraph {
        let graph = self.figure.new_graph(bounds);

        UiGraph::new(graph)
    }

    fn color_grid(
        &mut self, 
        bounds: impl Into<Bounds<Layout>>, 
        data: impl Into<Tensor>
    ) -> GridColorOpt {
        let mut graph = self.figure.new_graph(bounds);
        graph.flip_y(true);
        graph.aspect(1.);
        graph.x().visible(false);
        graph.y().visible(false);
        let colormesh = GridColor::new(data);

        graph.artist(colormesh)
    }
}
