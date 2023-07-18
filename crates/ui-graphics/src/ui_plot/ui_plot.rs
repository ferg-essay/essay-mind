use std::sync::{Arc, Mutex};

use essay_ecs::prelude::*;
use essay_plot::artist::{Lines2d, LinesOpt};
use essay_plot::graph::{FigureInner, GraphId};
use essay_plot::prelude::driver::FigureApi;
use essay_plot::{frame::Layout, prelude::Figure, graph::Graph};
use essay_plot::prelude::*;
use essay_plot_wgpu::PlotRenderer;
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
}

pub struct ArcPlot(Arc<Mutex<PlotInner>>);

pub struct PlotInner {
    figure: FigureInner,
}

impl PlotInner {
    fn new(figure: FigureInner) -> ArcPlot {
        ArcPlot(Arc::new(Mutex::new(
            PlotInner {
                figure,
            }            
        )))
    }

    fn plot(&mut self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> LinesOpt {
        let graph = self.figure.new_graph(());
        let lines = Lines2d::from_xy(x, y);

        graph.add_plot_artist(lines)
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