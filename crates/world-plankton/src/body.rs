use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use test_log::{TestLog, TestLogPlugin};
use ui_graphics::{UiCanvasPlugin, ui_plot::UiPlotPlugin};

use crate::{world::{PlanktonWorldPlugin, World}, ui_body::{draw_body, ui_body_spawn_plot, UiApicalBodyPlugin}, DrawItem, cilia::{Cilia, CiliaPlugin}};

#[derive(Component)]
pub struct Body {
    pos: Point,

    dy: f32,

    swim_rate: f32,
    arrest: f32,
}

impl Body {
    const DY_FALL : f32 = -0.05;

    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dy: 0.,
            swim_rate: 0.,
            arrest: 0.,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn pressure(&self) -> f32 {
        self.pos.y() * 0.1
    }

    pub fn swim_rate(&mut self, swim: f32) {
        self.swim_rate = swim;
    }

    pub fn arrest(&mut self, time: f32) {
        self.arrest = time;
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.insert_resource(Body::new(Point(2.5, -2.5)));
}

///
/// Update the plankton's position based on the cilia movement
/// 
pub fn body_physics(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    let Point(x, mut y) = body.pos;
    let [_width, height] = world.extent();

    // default movement is falling
    let mut dy = Body::DY_FALL;

    // if cilia aren't arrested, rise by the swim rate
    if body.arrest <= 0. {
        dy += body.swim_rate * Cilia::DY_SWIM;
    }

    // update y, clamped to the world boundaries
    y = (y + dy).clamp(- (height as f32) + 0.5, -0.5);

    body.pos = Point(x, y);
}

pub fn body_log(
    body: &Body,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1} swim={:.1} arrest={:.1}",
        body.pos.x(), body.pos.y(), body.dy, body.swim_rate, body.arrest
    ));
}

pub struct PlanktonBodyPlugin;

impl Plugin for PlanktonBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<PlanktonWorldPlugin>(), "BodyPlugin requires WorldPlugin");
        app.system(Startup, spawn_body);

        app.system(Update, body_physics);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }

        if app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiApicalBodyPlugin);
        }

        if ! app.contains_plugin::<CiliaPlugin>() {
            app.plugin(CiliaPlugin);
        }
    }
}