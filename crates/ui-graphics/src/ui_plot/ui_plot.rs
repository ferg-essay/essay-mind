use std::sync::{Arc, Mutex};

use essay_ecs::prelude::*;
use essay_plot::artist::{Lines2d, LinesOpt, GridColorOpt, GridColor};
use essay_plot::graph::{FigureInner, GraphId};
use essay_plot::prelude::driver::FigureApi;
use essay_plot::prelude::*;
use essay_plot::wgpu::PlotRenderer;
use essay_tensor::Tensor;

use crate::UiCanvas;
use crate::ui_layout::UiLayoutEvent;
use crate::{UiCanvasPlugin, ui_layout::{UiLayout, BoxId}};

#[derive(Component)]
pub struct UiPlot {
    box_id: BoxId,
    inner: ArcPlot,
    bounds: Bounds::<Canvas>,
}

impl UiPlot {
    fn new(box_id: BoxId) -> Self {
        let figure = FigureInner::new();
        // let graph_id = figure.new_graph([1., 1.]).id();
        
        Self {
            box_id,
            inner: PlotInner::new(figure),
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

    pub fn plot_xy(&self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        self.inner.0.lock().unwrap().plot(x, y)
    }

    pub fn x_label(&self, label: impl AsRef<str>) {
        self.inner.0.lock().unwrap().x_label(label.as_ref())
    }

    pub fn color_grid(&self, data: impl Into<Tensor>) -> GridColorOpt {
        self.inner.0.lock().unwrap().color_grid(data)
    }
}

pub struct ArcPlot(Arc<Mutex<PlotInner>>);

pub struct PlotInner {
    figure: FigureInner,
    graph_id: Option<GraphId>,
}

impl PlotInner {
    fn new(figure: FigureInner) -> ArcPlot {
        ArcPlot(Arc::new(Mutex::new(
            PlotInner {
                figure,
                graph_id: None,
            }            
        )))
    }

    fn plot(&mut self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
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

fn ui_plot_draw(mut ui_plot: ResMut<UiPlot>, mut ui_canvas: ResMut<UiCanvas>) {
    if let Some(mut renderer)= ui_canvas.plot_renderer() {
        ui_plot.draw(&mut renderer);
        
        renderer.flush();
    }
}

pub fn ui_plot_resize(
    mut ui_plot: ResMut<UiPlot>, 
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

    let ui_plot = UiPlot::new(id);

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