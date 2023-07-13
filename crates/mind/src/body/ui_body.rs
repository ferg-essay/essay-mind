use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{PathStyle, paths}};
use ui_graphics::UiCanvas;

use crate::world::{World, UiWorld};

use super::Body;


pub fn draw_body(body: &Body, world: Res<UiWorld>, mut ui: ResMut<UiCanvas>) {
    let mut style = PathStyle::new();
    let transform = world.to_canvas()
        .matmul(&Affine2d::eye().translate(body.pos().x(), body.pos().y()));

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
