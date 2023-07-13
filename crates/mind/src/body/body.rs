use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use test_log::{TestLog, TestLogPlugin};
use ui_graphics::{UiCanvasPlugin};

use crate::{world::{ApicalWorldPlugin, DrawItem, World}, body::ui_body::draw_body};

#[derive(Component)]
pub struct Body {
    pos: Point,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.spawn(Body::new(Point(2.5, 2.5)));
}

pub fn body_physics(
    body: &mut Body,
    world: Res<World>,
) {
    let Point(x, mut y) = body.pos;
    let [width, height] = world.extent();

    if y > 0.5 {
        y -= 0.05;
    }

    body.pos = Point(x, y);
}

pub fn body_log(
    body: &Body,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1})", body.pos.x(), body.pos.y()));
}

pub struct ApicalBodyPlugin;

impl Plugin for ApicalBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<ApicalWorldPlugin>(), "BodyPlugin requires WorldPlugin");
        app.system(Startup, spawn_body);

        app.system(Update, body_physics);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }

        if app.contains_plugin::<UiCanvasPlugin>() {
            app.system(Update, draw_body.phase(DrawItem));
        }
    }
}