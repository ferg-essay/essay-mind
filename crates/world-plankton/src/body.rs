use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use test_log::{TestLog, TestLogPlugin};
use ui_graphics::{UiCanvasPlugin};

use crate::{world::{PlanktonWorldPlugin, World}, ui_body::draw_body, DrawItem};

#[derive(Component)]
pub struct BodyPlankton {
    pos: Point,

    dy: f32,

    cilia: Cilia,
}

impl BodyPlankton {
    const DY_FALL : f32 = -0.05;

    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dy: 0.,
            cilia: Cilia { swim_rate: Cilia::SWIM_RATE, arrest: 0. },
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn arrest(&mut self, time: f32) {
        self.cilia.arrest = time.max(self.cilia.arrest);
    }
}

struct Cilia {
    swim_rate: f32, // how fast the cilia are beating
    arrest: f32,    // timeout for cilia arrest
}

impl Cilia {
    const SWIM_RATE : f32 = 1.;    // default swim rate
    const DY_SWIM : f32 = 0.05;    // speed of the default swim rate
    const ARREST_DECAY : f32 = 1.; // linear arrest decay
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.spawn(BodyPlankton::new(Point(2.5, 2.5)));
}

///
/// Update the plankton's position based on the cilia movement
/// 
pub fn body_physics(
    body: &mut BodyPlankton,
    world: Res<World>,
) {
    let Point(x, mut y) = body.pos;
    let [_width, height] = world.extent();

    // default movement is falling
    let mut dy = BodyPlankton::DY_FALL;

    // if cilia aren't arrested, rise by the swim rate
    if body.cilia.arrest <= 0. {
        dy += body.cilia.swim_rate * Cilia::DY_SWIM;
    }

    // update y, clamped to the world boundaries
    y = (y + dy).clamp(0.5, height as f32 - 0.5);

    // arrest decays linearly
    body.cilia.arrest = (body.cilia.arrest - Cilia::ARREST_DECAY).max(0.);
    // swim rate resets each tick
    body.cilia.swim_rate = Cilia::SWIM_RATE;

    body.pos = Point(x, y);
}

pub fn body_log(
    body: &BodyPlankton,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1}", body.pos.x(), body.pos.y(), body.dy));
    log.log(&format!("cilia: swim={:.1} arrest={:.1}", body.cilia.swim_rate, body.cilia.arrest));
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
            app.system(Update, draw_body.phase(DrawItem));
        }
    }
}