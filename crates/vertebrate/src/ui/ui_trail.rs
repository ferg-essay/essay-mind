use essay_ecs::{app::{App, Plugin, Update}, core::{Res, ResMut}};
use essay_graphics::{api, layout::{View, ViewArc}};
use essay_plot::api::{renderer::{self, Drawable, Renderer}, Bounds, Path, PathCode, PathStyle};
use mind_ecs::Tick;
use ui_graphics::ViewPlugin;

use crate::{body::Body, util::Point};

use super::ui_world_map::{UiWorld, UiWorldPlugin};



pub fn update_trail(
    mut ui_trail: ResMut<UiTrail>,
    body: Res<Body>,
) {
    ui_trail.add(Point(body.pos().0, body.pos().1));
}
/*
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
    */

pub struct UiTrail {
    view: View<UiTrailView>,
    points: Vec<api::Point>,
    head: usize,
}

impl UiTrail {
    fn new(view: View<UiTrailView>, size: usize) -> Self {
        assert!(size > 0);

        let mut points = Vec::new();
        points.resize(size, api::Point(0., 0.));

        Self {
            view,
            points,
            head: 0,
        }
    }

    fn add(&mut self, point: impl Into<api::Point>) -> &mut Self {
        self.points[self.head] = point.into();

        self.head = (self.head + 1) % self.points.len();

        let path = self.path(4);

        self.view.write(|v| v.path = path);

        self
    }

    fn path(&self, step: usize) -> Path<UiWorld> {
        let mut path_codes = Vec::<PathCode>::new();

        path_codes.push(PathCode::MoveTo(self.points[self.head]));

        let len = self.points.len();

        for i in (step - 1..len).step_by(step) {
            path_codes.push(PathCode::LineTo(self.points[(self.head + i) % len]));
        }

        Path::new(path_codes)
    }
}

struct UiTrailView {
    world_bounds: Bounds::<UiWorld>,
    path: Path<UiWorld>,
}
    
impl Default for UiTrailView {
    fn default() -> Self {
        Self { 
            world_bounds: Bounds::none(),
            path: Path::move_to(0., 0.).to_path(),
        }
    }
}
    
impl Drawable for UiTrailView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        let transform = self.world_bounds.affine_to(ui.pos());

        let trail = transform.transform_path(&self.path);

        let mut style = PathStyle::new();
        style.color("midnight blue");

        ui.draw_path(&trail, &style)
    }
}
    
pub struct UiTrailPlugin {
    view: Option<View<UiTrailView>>,
    len: usize,
}
    
impl UiTrailPlugin {
    pub fn new() -> Self {
        Self {
            view: None,
            len: 256,
        }
    }

    pub fn len(&mut self, len: usize) -> &mut Self {
        self.len = len;

        self
    }
}
    
impl ViewPlugin for UiTrailPlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        self.view = Some(View::from(UiTrailView::default()));
    
        self.view.as_ref().map(|v| v.arc())
    }
}
    
impl Plugin for UiTrailPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            if let Some(view) = &self.view {
                app.insert_resource(UiTrail::new(view.clone(), self.len));
                let world = app.resource::<UiWorld>().bounds();
                let mut view  = view.clone();
                view.write(|v| v.world_bounds = world);
                app.system(Tick, update_trail); // .phase(DrawAgent));
            }
        }
    }
}
