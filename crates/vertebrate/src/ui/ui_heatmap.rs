use essay_ecs::prelude::*;
use essay_graphics::layout::View;
use essay_plot::{
    api::Bounds, 
    artist::{ColorMaps, GridColor, GridColorOpt}, 
    chart::{Chart, ChartFrame}, 
};
use essay_tensor::Tensor;
use mind_ecs::PostTick;
use ui_graphics::{UiCanvas, ViewPlugin};
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
        mut grid_plot: GridColorOpt,
        extent: (usize, usize)
    ) -> Self {
        let factor = 1;

        let mut data = Vec::<f32>::new();
        data.resize(extent.0 * extent.1 * factor * factor, 0.);

        let init_data = Tensor::from(&data).reshape([extent.0, extent.1]);

        grid_plot.data(init_data);

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
    view: Option<View<ChartFrame>>,
    grid_plot: Option<GridColorOpt>,
}

impl UiHeatmapPlugin {
    pub fn new() -> Self {
        Self {
            view: None,
            grid_plot: None,
        }
    }
}

impl ViewPlugin<ChartFrame> for UiHeatmapPlugin {
    fn view(&mut self, app: &mut App) -> Option<&View<ChartFrame>> {
        let mut chart = Chart::default();
        let mut data = Vec::<f32>::new();
        // data.resize(extent.0 * extent.1 * factor * factor, 0.);

        chart.flip_y(true);
        chart.aspect(1.);
        chart.x().visible(false);
        chart.y().visible(false);
        // graph.colorbar();
        let init_data = Tensor::from(&data).reshape([0, 0]);
        let colormesh = GridColor::new(init_data);
        let mut grid_plot = chart.artist(colormesh);
        grid_plot.color_map(ColorMaps::RedYellow);

        self.grid_plot = Some(grid_plot);
        self.view = Some(chart.view().clone());

        self.view.as_ref()
    }
}

impl Plugin for UiHeatmapPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            if let Some(grid_plot) = &self.grid_plot {
                let world = app.resource::<World>();
                app.insert_resource(UiHeatmap::new(grid_plot.clone(), world.extent()));

                app.system(PostTick, ui_heatmap_update);
            }
        }
    }
}
