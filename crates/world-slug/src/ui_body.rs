use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{PathStyle, LinesOpt}, 
    artist::{GridColorOpt, ColorMaps, paths::Unit, Markers, Norms}
};
use essay_tensor::tf32;
use ui_graphics::{UiCanvas, ui_plot::{UiPlot, UiPlotPlugin}};

use crate::{UiWorld, UiApicalWorldPlugin, ui_world::DrawAgent};

use super::Body;

#[derive(Component)]
pub struct UiBody {
    x: Vec<f32>,
    dir: Vec<f32>,
    dir_opt: LinesOpt,

    y_speed: Vec<f32>,
    speed: LinesOpt,

    y_arrest: Vec<f32>,
    arrest: LinesOpt,

    peptides: GridColorOpt,

    tick: usize,
}

impl UiBody {
    pub const LIM : usize = 100;

    pub fn new(plot: &UiPlot) -> Self {
        let x = Vec::new();

        let dir = Vec::new();
        let mut dir_opt = plot.plot_xy(&x, &dir);
        dir_opt.label("dir");

        let y_speed = Vec::new();
        let mut speed = plot.plot_xy(&x, &y_speed);
        speed.label("speed");

        let y_arrest = Vec::new();
        let mut arrest = plot.plot_xy(&x, &y_arrest);
        arrest.label("arrest");

        let z_peptides = tf32!([[0., 1.], [0., 0.], [0., 0.]]);
        let mut peptides : GridColorOpt = plot.color_grid(z_peptides);
        peptides.norm(Norms::Linear.vmin(0.).vmax(1.));
        peptides.color_map(ColorMaps::WhiteRed);

        Self {
            x,
            dir,
            dir_opt,
            y_speed,
            speed,
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

        while self.dir.len() > Self::LIM {
            self.dir.remove(0);
        }

        while self.y_speed.len() > Self::LIM {
            self.y_speed.remove(0);
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
    let mut style = PathStyle::new();
    let transform = Affine2d::eye()
        .rotate(body.dir().to_radians())
        .translate(body.pos().x(), body.pos().y());

    let transform = world.to_canvas().matmul(&transform);

    let head_len = 0.3;
    let mut head_dir = 0.;

    if body.muscle_left() > 0.1 {
        let turn = (0.1 * body.muscle_left()).clamp(0., 0.1);
        head_dir += Angle::Unit(turn).to_radians();
    }

    if body.muscle_right() > 0.1 {
        let turn = - (0.1 * body.muscle_right()).clamp(0., 0.1);
        head_dir += Angle::Unit(turn).to_radians();
    }

    let head_pt = Point(
        0.1 + head_dir.cos() * head_len, 
        head_dir.sin() * head_len
    );

    let tail_pt = Point(
        - 0.1 - head_dir.cos() * head_len, 
        head_dir.sin() * head_len
    );

    let body = Path::<Unit>::move_to(tail_pt.0, tail_pt.1)
        .line_to(-0.1, 0.0)
        .line_to(0.1, 0.0)
        .line_to(head_pt.0, head_pt.1)
        .to_path()
        .transform(&transform);

    let color = Color::from("azure");
    style.line_width(3.);
    style.join_style(JoinStyle::Round);
    style.cap_style(CapStyle::Round);
    style.color(color);

    ui.draw_path(&body, &style);

    let head = Markers::TriLeft.get_path()
        .rotate::<Canvas>(head_dir)
        .scale::<Canvas>(0.10, 0.10)
        .translate::<Canvas>(head_pt.0, head_pt.1)
        .transform(&transform);

    style.color("red");
    style.line_width(3.);
    ui.draw_path(&head, &style);
}

pub fn ui_body_plot(
    ui_body: &mut UiBody,
    body: Res<Body>
) {
    ui_body.dir.push(body.dir().to_unit());

    ui_body.y_speed.push(body.get_speed());
    ui_body.y_arrest.push(body.get_arrest());
    ui_body.tick();

    ui_body.dir_opt.set_xy(&ui_body.x, &ui_body.dir);

    ui_body.speed.set_xy(&ui_body.x, &ui_body.y_speed);
    ui_body.arrest.set_xy(&ui_body.x, &ui_body.y_arrest);

    ui_body.peptides.data(body.state().reshape([3, 2]));
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiPlot>
) {
    c.spawn(UiBody::new(plot.get_mut()))
}

pub struct UiApicalBodyPlugin;

impl Plugin for UiApicalBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<UiApicalWorldPlugin>());
        
        app.system(Update, draw_body.phase(DrawAgent));

        if app.contains_plugin::<UiPlotPlugin>() {
            app.system(Startup, ui_body_spawn_plot);
            app.system(Update, ui_body_plot);
        }
}
}
