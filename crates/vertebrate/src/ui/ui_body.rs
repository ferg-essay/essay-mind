use std::f32::consts::TAU;

use essay_ecs::prelude::*;
use essay_plot::artist::{PathStyle, Markers};
use essay_plot::artist::paths::Unit;
use essay_plot::prelude::*;

use crate::util as util;

use mind_ecs::PostTick;
use ui_graphics::UiCanvas;
use crate::body::Body;
use crate::ui::ui_world::{UiWorldPlugin, UiWorld};

use super::ui_world::DrawAgent;

pub fn draw_body(
    body: Res<Body>, 
    world: Res<UiWorld>, 
    mut ui_canvas: ResMut<UiCanvas>
) {
    if let Some(mut ui) = ui_canvas.renderer(Clip::None) {
        let mut style = PathStyle::new();
        let transform = Affine2d::eye()
            .rotate(body.dir().to_radians())
            .translate(body.pos().x(), body.pos().y());

        let transform = world.to_canvas().matmul(&transform);

        let head_len = 0.3;
        let turn = util::Angle::Unit(body.turn().to_unit_zero() * 0.5);

        let tail_pt = Point(
            - 0.1 - turn.sin() * head_len, 
            - turn.cos() * head_len,
        );

        let head_pt = Point(
            0.1 + turn.sin() * head_len, 
            - turn.cos() * head_len,
        );

        let body = Path::<Unit>::move_to(tail_pt.0, tail_pt.1)
            .line_to(-0.1, 0.)
            .line_to(0.1, 0.)
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
            .rotate::<Canvas>(turn.to_radians() + TAU * 0.75)
            .scale::<Canvas>(0.10, 0.10)
            .translate::<Canvas>(head_pt.0, head_pt.1)
            .transform(&transform);

        style.color("red");
        style.line_width(3.);
        ui.draw_path(&head, &style);
    }
}

pub fn update_trail(
    mut ui_trail: ResMut<UiTrail>,
    body: Res<Body>,
) {
    ui_trail.add(Point(body.pos().0, body.pos().1));
}

pub fn draw_trail(
    ui_trail: Res<UiTrail>,
    ui_world: Res<UiWorld>,
    mut ui: ResMut<UiCanvas>
) {
    // ui_trail.add(body.pos());

    let transform = Affine2d::eye();
    let transform = ui_world.to_canvas().matmul(&transform);

    let trail: Path<Canvas> = ui_trail.path(4).transform(&transform);

    let mut style = PathStyle::new();
    style.color("midnight blue");

    ui.draw_path(&trail, &style);
}

pub struct UiTrail {
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

pub struct BodyPlot;

pub struct UiBodyPlugin;

impl Plugin for UiBodyPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            app.system(Update, draw_body.phase(DrawAgent));
        }
    }
}

pub struct UiBodyTrailPlugin;

impl Plugin for UiBodyTrailPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            app.insert_resource(UiTrail::new(400));
            app.system(PostTick, update_trail);
            app.system(Update, draw_trail.phase(DrawAgent));
        }
    }
}
