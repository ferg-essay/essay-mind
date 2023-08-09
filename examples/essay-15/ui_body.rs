use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{PathStyle, LinesOpt}, 
    artist::{GridColorOpt, ColorMaps, paths::Unit, Markers, Norms}
};
use essay_tensor::tf32;
use ui_graphics::{UiCanvas, ui_plot::{UiFigure, UiPlotPlugin, UiKey, UiPlot, UiFigure2Plugin, UiFigure2}};

use crate::{ui_world::{UiSlugWorldPlugin, UiWorld}, body::Body};

use super::ui_world::DrawAgent;

#[derive(Component)]
pub struct UiBody {
    plot: UiPlot,

    peptides: GridColorOpt,
}

impl UiBody {
    fn new(figure: &UiFigure2<BodyPlot>) -> Self {
        let mut plot = figure.plot_xy((0., 0.), (1.5, 1.));

        plot.x_label("seconds");

        plot.line(Key::DIR, "dir");
        plot.line(Key::SPEED, "speed");
        plot.line(Key::ARREST, "arrest");

        let z_peptides = tf32!([[0., 1.], [0., 0.], [0., 0.]]);
        let mut peptides : GridColorOpt = figure.color_grid(z_peptides);
        peptides.norm(Norms::Linear.vmin(0.).vmax(1.));
        peptides.color_map(ColorMaps::WhiteRed);

        Self {
            plot,

            peptides,
        }
    }

    pub fn tick(&mut self) {
        self.plot.tick();
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
    } else if body.muscle_right() > 0.1 {
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
    ui_body.plot.push(&Key::DIR, body.dir().to_unit());
    ui_body.plot.push(&Key::SPEED, body.get_speed());
    ui_body.plot.push(&Key::ARREST, body.get_arrest());

    ui_body.tick();

    ui_body.peptides.data(body.state().reshape([3, 2]));
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure2<BodyPlot>>
) {
    c.spawn(UiBody::new(plot.get_mut()))
}

pub enum Key {
    DIR,
    SPEED,
    ARREST,
}

impl UiKey for Key {
    fn index(&self) -> usize {
        match self {
            Key::DIR => 0,
            Key::SPEED => 1,
            Key::ARREST => 2,
        }
    }
}

pub struct BodyPlot;

pub struct UiSlugBodyPlugin;

impl Plugin for UiSlugBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<UiSlugWorldPlugin>());
        
        app.system(Update, draw_body.phase(DrawAgent));

        app.plugin(UiFigure2Plugin::<BodyPlot>::new((0., 1.), (2., 1.)));

        app.system(Startup, ui_body_spawn_plot);
        app.system(Update, ui_body_plot);
    }
}
