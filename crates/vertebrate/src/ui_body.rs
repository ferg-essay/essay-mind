use std::ops::Deref;

use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::PathStyle, 
    artist::{GridColorOpt, ColorMaps, paths::Unit, Markers, Norms}
};
use essay_tensor::tf32;
use ui_graphics::{UiCanvas, ui_plot::{UiKey, UiPlot, UiFigurePlugin, UiFigure}};
use crate::world::{OdorType, World};
use crate::{ui_world::{UiSlugWorldPlugin, UiWorld}, body::Body};

use super::ui_world::DrawAgent;

#[derive(Component)]
pub struct UiBody {
    plot: UiPlot,

    peptides: GridColorOpt,
}

impl UiBody {
    fn new(figure: &UiFigure<BodyPlot>) -> Self {
        let mut plot = figure.plot_xy((0., 0.), (1.5, 1.));

        //plot.x_label("seconds");

        // plot.line(Key::Dir, "dir");
        // plot.line(Key::Speed, "speed");
        plot.line(Key::PFood, "p(food)");
        plot.line(Key::Arrest, "arrest");
        let is_single_habituate = false;
        if is_single_habituate {
            plot.line(Key::HabitFoodA, "habit");
        } else {
            plot.line(Key::HabitFoodA, "food-a");
            plot.line(Key::HabitOtherA, "other-a");
            plot.line(Key::IgnoreOdor, "ignore odor");
        }

        let z_peptides = tf32!([[0., 0.], [0., 0.], [0., 0.], [0., 0.]]);
        let mut peptides : GridColorOpt = figure.color_grid((1.6, 0.), (0.5, 1.), z_peptides);
        peptides.norm(Norms::Linear.vmin(0.).vmax(1.));
        peptides.color_map(ColorMaps::WhiteRed);

        Self {
            plot,

            peptides,
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
    body: Res<Body>,
    world: Res<World>,
) {
    ui_body.plot.push(&Key::PFood, body.p_food());
    ui_body.plot.push(&Key::Arrest, body.arrest());

    ui_body.plot.tick();

    let peptides = tf32!([
        [if body.is_touch_left() { 1. } else { 0. }, 
        if body.is_touch_right() { 1. } else { 0. }],
        [if body.is_food_left(world.deref()) { 1. } else { 0. }, 
        if body.is_food_right(world.deref()) { 1. } else { 0. }],
        [ if body.is_sensor_food() { 1. } else { 0. }, body.arrest() ],
        [ body.muscle_left(), body.muscle_right() ],
    ]);

    ui_body.peptides.data(peptides.reshape([4, 2]));
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiBody::new(plot.get_mut()))
}

pub enum Key {
    PFood,
    Arrest,
    HabitFoodA,
    HabitOtherA,
    IgnoreOdor,
}

impl UiKey for Key {
    fn index(&self) -> usize {
        match self {
            Key::PFood => 0,
            Key::Arrest => 1,
            Key::HabitFoodA => 2,
            Key::HabitOtherA => 3,
            Key::IgnoreOdor => 4,
        }
    }
}

pub struct BodyPlot;

pub struct UiSlugBodyPlugin {
    xy: Point,
    wh: Point,
}

impl UiSlugBodyPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
        }
    }
}

impl Plugin for UiSlugBodyPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiSlugWorldPlugin>() {
            app.system(Update, draw_body.phase(DrawAgent));

            app.plugin(UiFigurePlugin::<BodyPlot>::new(self.xy, self.wh));

            app.system(Startup, ui_body_spawn_plot);
            app.system(Update, ui_body_plot);
        }
    }
}
