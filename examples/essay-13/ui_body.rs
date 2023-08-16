use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{PathStyle, paths, LinesOpt}, artist::{GridColorOpt, ColorMaps}};
use essay_tensor::tf32;
use ui_graphics::{UiCanvas, ui_plot::{UiFigurePlugin, UiFigure}};

use super::{UiWorld, World, UiApicalWorldPlugin, DrawItem};

use super::Body;

#[derive(Component)]
pub struct UiBody {
    x: Vec<f32>,
    y_pressure: Vec<f32>,
    pressure: LinesOpt,
    y_light: Vec<f32>,
    light: LinesOpt,

    y_temp: Vec<f32>,
    temp: LinesOpt,

    y_swim: Vec<f32>,
    swim: LinesOpt,

    y_arrest: Vec<f32>,
    arrest: LinesOpt,

    peptides: GridColorOpt,

    tick: usize,
}

impl UiBody {
    pub const LIM : usize = 100;

    pub fn new(plot: &UiFigure<BodyPlot>) -> Self {
        let x = Vec::new();

        let y_pressure = Vec::new();
        let mut pressure = plot.plot_xy_old(&x, &y_pressure);
        pressure.label("pressure");

        let y_light = Vec::new();
        let mut light = plot.plot_xy_old(&x, &y_light);
        light.label("light");

        let y_temp = Vec::new();
        let mut temp = plot.plot_xy_old(&x, &y_temp);
        temp.label("temp");

        let y_swim = Vec::new();
        let mut swim = plot.plot_xy_old(&x, &y_swim);
        swim.label("swim");

        let y_arrest = Vec::new();
        let mut arrest = plot.plot_xy_old(&x, &y_arrest);
        arrest.label("arrest");

        let z_peptides = tf32!([[0., 1.], [0., 0.], [0., 0.]]);
        let mut peptides : GridColorOpt = plot.color_grid((1.5, 0.), (0.5, 1.), z_peptides);
        peptides.color_map(ColorMaps::WhiteRed);

        Self {
            x,
            y_pressure,
            pressure,
            y_light,
            light,
            y_temp,
            temp,
            y_swim,
            swim,
            y_arrest,
            arrest,
            peptides,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.x.push(self.tick as f32);

        while self.x.len() > Self::LIM {
            self.x.remove(0);
        }

        while self.y_pressure.len() > Self::LIM {
            self.y_pressure.remove(0);
        }

        while self.y_light.len() > Self::LIM {
            self.y_light.remove(0);
        }

        while self.y_temp.len() > Self::LIM {
            self.y_temp.remove(0);
        }

        while self.y_swim.len() > Self::LIM {
            self.y_swim.remove(0);
        }

        while self.y_arrest.len() > Self::LIM {
            self.y_arrest.remove(0);
        }
    }
}

pub fn draw_body(
    body: Res<Body>, 
    world: Res<UiWorld>, 
    mut ui: ResMut<UiCanvas>
) {
    let [_width, height] = world.extent();

    let mut style = PathStyle::new();
    let transform = world.to_canvas()
        .matmul(&Affine2d::eye().translate(body.pos().x(), height + body.pos().y()));

    let body_circle = paths::circle()
        .scale::<World>(0.4, 0.4)
        .transform(&transform);

    let color = Color::from("azure");
    style.color(color);

    ui.draw_path(&body_circle, &style);

    let body_apical = paths::unit_asterisk(10)
        .scale::<World>(0.3, 0.3)
        .translate::<World>(0., 0.3)
        .transform(&transform);

    style.color(Color::from("red"));
    style.line_width(3.);

    ui.draw_path(&body_apical, &style);

    let fringe_cut = Path::<World>::move_to(-0.3, 0.25)
        .line_to(0.3, 0.25)
        .line_to(0.3, 0.0)
        .close_poly(-0.3, 0.0)
        .to_path()
        .transform::<Canvas>(&transform);

    style.color(color);

    ui.draw_path(&fringe_cut, &style);

    let body_fringe = Path::<World>::move_to(-0.45, 0.)
        .line_to(0.45, 0.)
        .line_to(0.4, -0.2)
        .close_poly(-0.4, -0.2)
        .to_path()
        .transform(&transform);

    style.color(Color::from("aquamarine"));

    ui.draw_path(&body_fringe, &style);
}

pub fn ui_body_plot(
    ui_body: &mut UiBody,
    body: Res<Body>
) {
    ui_body.y_pressure.push(body.pressure());
    ui_body.y_light.push(body.light());
    ui_body.y_temp.push(body.temperature());

    ui_body.y_swim.push(body.get_swim_rate());
    ui_body.y_arrest.push(body.get_arrest());
    ui_body.tick();

    ui_body.pressure.set_xy(&ui_body.x, &ui_body.y_pressure);
    ui_body.light.set_xy(&ui_body.x, &ui_body.y_light);
    ui_body.temp.set_xy(&ui_body.x, &ui_body.y_temp);

    ui_body.swim.set_xy(&ui_body.x, &ui_body.y_swim);
    ui_body.arrest.set_xy(&ui_body.x, &ui_body.y_arrest);

    ui_body.peptides.data(body.peptides().reshape([3, 2]));
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiBody::new(plot.get_mut()))
}

pub struct BodyPlot;

pub struct UiApicalBodyPlugin;

impl Plugin for UiApicalBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<UiApicalWorldPlugin>());
        
        app.system(Update, draw_body.phase(DrawItem));

        app.plugin(UiFigurePlugin::<BodyPlot>::new((0., 1.), (2., 1.)));

        app.system(Startup, ui_body_spawn_plot);
        app.system(Update, ui_body_plot);
        /*
    if app.contains_plugin::<UiPlotPlugin>() {
            app.system(Startup, ui_body_spawn_plot);
            app.system(Update, ui_body_plot);
        }
    }
    */
   }
}
