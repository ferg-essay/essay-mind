use essay_ecs::prelude::*;
use essay_plot::{prelude::Point, artist::{GridColorOpt, ColorMaps}};
use essay_tensor::Tensor;
use ui_graphics::ui_plot::{UiFigurePlugin, UiFigure};

use crate::{ui_world::{UiSlugWorldPlugin, DrawAgent}, body::Body, ui_body::BodyPlot, world::World};

#[derive(Component)]
pub struct UiLocMap {
    width: usize,
    height: usize,
    data: Vec<f32>,
    grid_plot: GridColorOpt,
}

impl UiLocMap {
    fn new(
        ui_figure: &mut UiFigure<LocMap>,
        extent: (usize, usize)
    ) -> Self {
        let grid = Tensor::zeros([extent.1, extent.0]);
        let mut data = Vec::<f32>::new();
        data.resize(extent.0 * extent.1 * 4, 0.);

        let mut grid_plot = ui_figure.color_grid((0., 0.), (1., 1.), grid);
        grid_plot.color_map(ColorMaps::RedYellow);

        Self {
            width: extent.0 * 2,
            height: extent.1 * 2,
            data,
            grid_plot,
        }
    }
}

pub fn ui_locmap_update(
    ui_locmap: &mut UiLocMap,
    body: Res<Body>
) {
    let loc = body.pos();
    let (i, j) = ((2. * loc.x()) as usize, (2. * loc.y()) as usize);

    ui_locmap.data[(ui_locmap.height - j - 1) * ui_locmap.width + i] += 1.;

    ui_locmap.grid_plot.data(Tensor::from(&ui_locmap.data).reshape([ui_locmap.height, ui_locmap.width]));
    // ui_locmap.peptides.data(body.state().reshape([3, 2]));
}

pub fn ui_locmap_spawn_plot(
    mut c: Commands,
    world: Res<World>,
    mut plot: ResMut<UiFigure<LocMap>>
) {
    c.spawn(UiLocMap::new(plot.get_mut(), world.extent()))
}

pub struct LocMap;

pub struct UiSlugLocationPlugin {
    xy: Point,
    wh: Point,
}

impl UiSlugLocationPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
        }
    }
}

impl Plugin for UiSlugLocationPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiSlugWorldPlugin>() {
            // app.system(Update, draw_body.phase(DrawAgent));

            app.plugin(UiFigurePlugin::<LocMap>::new(self.xy, self.wh));

            app.system(Startup, ui_locmap_spawn_plot);
            app.system(Update, ui_locmap_update);
        }
    }
}
