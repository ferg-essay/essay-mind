use std::f32::consts::TAU;

use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::renderer::{self, Drawable, Renderer};
use essay_plot::api::{Affine2d, Bounds, CapStyle, Color, JoinStyle, Path, PathStyle};
use essay_plot::artist::Markers;
use essay_plot::artist::paths::Unit;

use ui_graphics::ViewPlugin;
use crate::body::Body;
use crate::ui::ui_world_map::{UiWorldPlugin, UiWorld};
use crate::util::{Heading, Point, Turn};
use crate::world::World;

fn draw_body(
    body: Res<Body>, 
    world: Res<World>, 
    mut ui_body: ResMut<UiBody>
) {
    ui_body.view.write(|v| {
        v.update_body(body.get());
        v.update_world(world.get());
    });
    /*
    if let Some(mut ui) = ui_canvas.renderer() {
        let mut style = PathStyle::new();
        let transform = Affine2d::eye()
            .rotate(body.dir().to_radians())
            .translate(body.pos().x(), body.pos().y());

        let transform = world.to_canvas().matmul(&transform);

        let middle_len = 0.5 * body.middle_len();
        let head_len = 0.5 * (body.len() - middle_len);
        // let turn = body.turn().to_unit() * 0.5;
        let turn = body.turn();
        let (dy, dx) = turn.sin_cos();

        let head_pt = Point(
            middle_len + dx * head_len,
            dy * head_len, 
        );

        let tail_pt = Point(
            - middle_len - dx * head_len,
            dy * head_len, 
        );

        let body_path = Path::<Unit>::move_to(tail_pt.0, tail_pt.1)
            .line_to(- middle_len, 0.0)
            .line_to(middle_len, 0.0)
            .line_to(head_pt.0, head_pt.1)
            .to_path()
            .transform(&transform);

        let color = Color::from("azure");
        style.line_width(3.);
        style.join_style(JoinStyle::Round);
        style.cap_style(CapStyle::Round);
        style.color(color);

        ui.draw_path(&body_path, &style);

        let head_pt = body.head_pos();

        let transform = Affine2d::eye()
            .rotate(body.head_dir().to_radians() + TAU * 0.0)
            .scale(0.10, 0.10)
            .translate(head_pt.0, head_pt.1)
            .compose(&world.to_canvas());

        let head = Markers::TriLeft.get_path().transform(&transform);
        //    .transform(&transform);
        // let transform = world.to_canvas().matmul(&transform);

        style.color("red");
        style.line_width(3.);
        ui.draw_path(&head, &style);
    }
    */
}

pub struct UiBody {
    view: View<UiBodyView>,
}

pub struct UiBodyView {
    world_bounds: Bounds<UiWorld>,
    dir: Heading,
    pos: Point,
    turn: Turn,

    head_dir: Heading,
    head_pos: Point,

    len: f32,
    middle_len: f32,

}

impl UiBodyView {
    fn update_world(&mut self, world: &World) {
        self.world_bounds = Bounds::<UiWorld>::from([world.width() as f32, world.height() as f32]);
    }

    fn update_body(&mut self, body: &Body) {
        self.dir = body.dir();
        self.pos = body.pos();
        self.turn = body.turn();

        self.len = body.len();
        self.middle_len = body.middle_len();

        self.head_dir = body.head_dir();
        self.head_pos = body.head_pos();
    }
}

impl Default for UiBodyView {
    fn default() -> Self {
        Self { 
            world_bounds: Bounds::none(),
            dir: Heading::Unit(0.),
            pos: Point(0., 0.),
            turn: Turn::Unit(0.),
            head_dir: Heading::Unit(0.),
            head_pos: Point(0., 0.),
            len: Default::default(), 
            middle_len: Default::default() 
        }
    }
}

impl Drawable for UiBodyView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        let mut style = PathStyle::new();
        let transform = Affine2d::eye()
            .rotate(self.dir.to_radians())
            .translate(self.pos.x(), self.pos.y());

        let to_canvas = self.world_bounds.affine_to(ui.pos());

        let transform = to_canvas.matmul(&transform);

        let middle_len = 0.5 * self.middle_len;
        let head_len = 0.5 * (self.len - middle_len);
        // let turn = body.turn().to_unit() * 0.5;
        let turn = self.turn;
        let (dy, dx) = turn.sin_cos();

        let head_pt = Point(
            middle_len + dx * head_len,
            dy * head_len, 
        );

        let tail_pt = Point(
            - middle_len - dx * head_len,
            dy * head_len, 
        );

        let body_path = Path::<Unit>::move_to(tail_pt.0, tail_pt.1)
            .line_to(- middle_len, 0.0)
            .line_to(middle_len, 0.0)
            .line_to(head_pt.0, head_pt.1)
            .to_path()
            .transform(&transform);

        let color = Color::from("azure");
        style.line_width(3.);
        style.join_style(JoinStyle::Round);
        style.cap_style(CapStyle::Round);
        style.color(color);

        ui.draw_path(&body_path, &style)?;

        let head_pt = self.head_pos;

        let transform = Affine2d::eye()
            .rotate(self.head_dir.to_radians() + TAU * 0.0)
            .scale(0.10, 0.10)
            .translate(head_pt.0, head_pt.1)
            .compose(&to_canvas);

        let head = Markers::TriLeft.get_path().transform(&transform);
        //    .transform(&transform);
        // let transform = world.to_canvas().matmul(&transform);

        style.color("red");
        style.line_width(3.);
        ui.draw_path(&head, &style)
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

pub struct UiBodyPlugin {
    view: Option<View<UiBodyView>>,
}

impl UiBodyPlugin {
    pub fn new() -> Self {
        Self {
            view: None,
        }
    }
}

impl ViewPlugin for UiBodyPlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        self.view = Some(View::from(UiBodyView::default()));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl Plugin for UiBodyPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            if let Some(view) = &self.view {
                app.insert_resource(UiBody { view: view.clone() });
                app.system(Update, draw_body); // .phase(DrawAgent));
            }
        }
    }
}
