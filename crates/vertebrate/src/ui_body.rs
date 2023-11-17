use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::PathStyle, 
    artist::{GridColorOpt, ColorMaps, paths::Unit, Markers, Norms}
};
use essay_tensor::tf32;
use ui_graphics::{UiCanvas, ui_plot::{UiKey, UiPlot, UiFigurePlugin, UiFigure}};
use crate::world::World;
use crate::{ui_world::{UiWorldPlugin, UiWorld}, body::Body};

use super::ui_world::DrawAgent;

#[derive(Component)]
pub struct UiBody {
    plot: UiPlot,

    // action_map: GridColorOpt,

    trail: UiTrail,
}

impl UiBody {
    fn new(figure: &UiFigure<BodyPlot>) -> Self {
        let mut plot = figure.plot_xy((0., 0.), (1., 1.));

        //plot.x_label("seconds");

        plot.graph_mut().ylim(-0.1, 1.1);
        // plot.line(Key::Dir, "dir");
        // plot.line(Key::Speed, "speed");
        plot.line(Key::PFood, "p(food)");
        // plot.line(Key::Arrest, "arrest");
        plot.line(Key::Turn, "turn");
        let is_single_habituate = false;
        if is_single_habituate {
            plot.line(Key::HabitFoodA, "habit");
        } else {
            plot.line(Key::HabitFoodA, "food-a");
            plot.line(Key::HabitOtherA, "other-a");
            plot.line(Key::IgnoreOdor, "ignore odor");
        }

        //let z_peptides = tf32!([[0., 0.], [0., 0.], [0., 0.], [0., 0.]]);
        //let mut action_map : GridColorOpt = figure.color_grid((1.6, 0.), (0.5, 1.), z_peptides);
        //action_map.norm(Norms::Linear.vmin(0.).vmax(1.));
        //action_map.color_map(ColorMaps::WhiteRed);

        Self {
            plot,

            // action_map,
            trail: UiTrail::new(400),
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

    if body.turn() < 0.5 {
        let turn = body.turn().clamp(0., 0.25);
        head_dir += Angle::Unit(0.5 * turn).to_radians();
    } else {
        let turn = body.turn().clamp(0.75, 1.) - 1.;
        head_dir += Angle::Unit(0.5 * turn).to_radians();
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
    body: Res<Body>,
    _world: Res<World>,
) {
    ui_body.plot.push(&Key::PFood, body.p_food());
    ui_body.plot.push(&Key::Turn, (body.turn() + 0.5) % 1.);

    ui_body.plot.tick();
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiBody::new(plot.get_mut()))
}

pub fn draw_trail(
    ui_body: &mut UiBody,
    body: Res<Body>,
    ui_world: Res<UiWorld>,
    mut ui: ResMut<UiCanvas>
) {
    ui_body.trail.add(body.pos());

    let transform = Affine2d::eye();
    let transform = ui_world.to_canvas().matmul(&transform);

    let trail: Path<Canvas> = ui_body.trail.path(4).transform(&transform);

    let mut style = PathStyle::new();
    style.color("midnight blue");

    ui.draw_path(&trail, &style);
}

struct UiTrail {
    points: Vec<Point>,
    head: usize,
}

impl UiTrail {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut points = Vec::new();
        points.resize(size, Point(0., 0.));

        Self {
            points,
            head: 0,
        }
    }

    fn add(&mut self, point: impl Into<Point>) -> &mut Self {
        self.points[self.head] = point.into();

        self.head = (self.head + 1) % self.points.len();

        self
    }

    fn path(&self, step: usize) -> Path<Canvas> {
        let mut path_codes = Vec::<PathCode>::new();

        path_codes.push(PathCode::MoveTo(self.points[self.head]));

        let len = self.points.len();

        for i in (step - 1..len).step_by(step) {
            path_codes.push(PathCode::LineTo(self.points[(self.head + i) % len]));
        }

        Path::new(path_codes)
    }
}

pub enum Key {
    PFood,
    Turn,
    HabitFoodA,
    HabitOtherA,
    IgnoreOdor,
}

impl UiKey for Key {
    fn index(&self) -> usize {
        match self {
            Key::PFood => 0,
            Key::Turn => 1,
            Key::HabitFoodA => 2,
            Key::HabitOtherA => 3,
            Key::IgnoreOdor => 4,
        }
    }
}

pub struct BodyPlot;

pub struct UiBodyPlugin {
    xy: Point,
    wh: Point,
}

impl UiBodyPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
        }
    }
}

impl Plugin for UiBodyPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            app.system(Update, draw_body.phase(DrawAgent));

            app.plugin(UiFigurePlugin::<BodyPlot>::new(self.xy, self.wh));

            app.system(Startup, ui_body_spawn_plot);
            app.system(Update, ui_body_plot);
        }
    }
}

pub struct UiBodyTrailPlugin;

impl Plugin for UiBodyTrailPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            app.system(Update, draw_trail.phase(DrawAgent));
        }
    }
}
