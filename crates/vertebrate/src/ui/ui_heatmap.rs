use essay_ecs::prelude::*;
use essay_plot::{api::Bounds, artist::{ColorMaps, GridColor, GridColorOpt}, graph::Graph, prelude::Point};
use essay_tensor::Tensor;
use mind_ecs::PostTick;
use ui_graphics::UiCanvas;
use crate::world::World;
use crate::body::Body;
use crate::ui::ui_world_map::UiWorldPlugin;

struct UiHeatmap {
    factor: usize,
    width: usize,
    height: usize,
    data: Vec<f32>,
    grid_plot: GridColorOpt,
}

impl UiHeatmap {
    fn new(
        mut graph: Graph,
        extent: (usize, usize)
    ) -> Self {
        let factor = 1;

        let mut data = Vec::<f32>::new();
        data.resize(extent.0 * extent.1 * factor * factor, 0.);

        graph.flip_y(true);
        graph.aspect(1.);
        graph.x().visible(false);
        graph.y().visible(false);
        // graph.colorbar();
        let init_data = Tensor::from(&data).reshape([extent.0, extent.1]);
        let colormesh = GridColor::new(init_data);
        let mut grid_plot = graph.artist(colormesh);
        grid_plot.color_map(ColorMaps::RedYellow);

        Self {
            factor: 1,
            width: extent.0 * factor,
            height: extent.1 * factor,
            data,
            grid_plot,
        }
    }
}

fn ui_heatmap_update(
    mut ui_heatmap: ResMut<UiHeatmap>,
    body: Res<Body>
) {
    let loc = body.pos();
    let factor = ui_heatmap.factor;
    let (i, j) = ((factor as f32 * loc.x()) as usize, (factor as f32 * loc.y()) as usize);

    let (height, width) = (ui_heatmap.height, ui_heatmap.width);

    ui_heatmap.data[(height - j - 1) * width + i] += 1.;

    let data = Tensor::from(&ui_heatmap.data).reshape([height, width]);
    ui_heatmap.grid_plot.data(data);
}

pub struct UiHeatmapPlugin {
    xy: Point,
    wh: Point,
}

impl UiHeatmapPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
        }
    }
}

impl Plugin for UiHeatmapPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            if let Some(ui_canvas) = app.get_mut_resource::<UiCanvas>() {
                let graph = ui_canvas.graph(Bounds::new(self.xy, self.wh));

                let world = app.resource::<World>();
                app.insert_resource(UiHeatmap::new(graph, world.extent()));

                app.system(PostTick, ui_heatmap_update);
            }
        }
    }
}
