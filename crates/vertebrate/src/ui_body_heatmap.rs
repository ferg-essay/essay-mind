use essay_ecs::prelude::*;
use essay_plot::{prelude::Point, artist::{GridColorOpt, ColorMaps, TextCoords, TextOpt}};
use essay_tensor::Tensor;
use ui_graphics::ui_plot::{UiFigurePlugin, UiFigure};
use crate::world::World;
use crate::{ui_world::UiSlugWorldPlugin, body::Body};

#[derive(Component)]
pub struct UiLocMap {
    factor: usize,
    width: usize,
    height: usize,
    data: Vec<f32>,
    grid_plot: GridColorOpt,
    p_food_text: TextOpt,
}

impl UiLocMap {
    fn new(
        ui_figure: &mut UiFigure<LocMap>,
        extent: (usize, usize)
    ) -> Self {
        let factor = 1;
        let grid = Tensor::zeros([extent.1, extent.0]);
        let mut data = Vec::<f32>::new();
        data.resize(extent.0 * extent.1 * factor * factor, 0.);

        let mut graph = ui_figure.graph((0., 0.), (1., 1.));
        let mut grid_plot = graph.color_grid(grid);
        grid_plot.color_map(ColorMaps::RedYellow);

        let mut text = graph.graph().text((0., 0.95), "hello");
        text.coord(TextCoords::FrameFraction).color("k");

        Self {
            factor: 1,
            width: extent.0 * factor,
            height: extent.1 * factor,
            data,
            grid_plot,
            p_food_text: text,
        }
    }
}

pub fn ui_locmap_update(
    ui_locmap: &mut UiLocMap,
    body: Res<Body>
) {
    let loc = body.pos();
    let factor = ui_locmap.factor;
    let (i, j) = ((factor as f32 * loc.x()) as usize, (factor as f32 * loc.y()) as usize);

    ui_locmap.data[(ui_locmap.height - j - 1) * ui_locmap.width + i] += 1.;

    ui_locmap.grid_plot.data(Tensor::from(&ui_locmap.data).reshape([ui_locmap.height, ui_locmap.width]));
    // ui_locmap.peptides.data(body.state().reshape([3, 2]));

    ui_locmap.p_food_text.text(format!("p(food) = {:.3}", body.p_food()));
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
