use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{PathStyle, paths, LinesOpt}};
use ui_graphics::{UiCanvas, ui_plot::{UiPlot, UiPlotPlugin}};

use crate::{UiWorld, World, UiApicalWorldPlugin, DrawItem};

use super::Body;

#[derive(Component)]
pub struct UiBody {
    x: Vec<f32>,
    y: Vec<f32>,
    plot: LinesOpt,
    tick: usize,
}

impl UiBody {
    pub const LIM : usize = 100;

    pub fn new(plot: &UiPlot) -> Self {
        let mut x = Vec::new();
        let mut y = Vec::new();

        x.push(0.);
        y.push(1.);

        let lines = plot.plot_xy(&x, &y);

        Self {
            x,
            y,
            plot: lines,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.x.push(self.tick as f32);

        while self.x.len() > Self::LIM {
            self.x.remove(0);
        }

        while self.y.len() > Self::LIM {
            self.y.remove(0);
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
    ui_body.y.push(body.pos().y());
    ui_body.tick();

    ui_body.plot.set_xy(&ui_body.x, &ui_body.y);
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
        
        app.system(Update, draw_body.phase(DrawItem));

        if app.contains_plugin::<UiPlotPlugin>() {
            app.system(Startup, ui_body_spawn_plot);
            app.system(Update, ui_body_plot);
        }
}
}
