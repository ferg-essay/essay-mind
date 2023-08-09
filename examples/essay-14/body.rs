use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::{Tensor, tf32};
use test_log::{TestLog, TestLogPlugin};
use ui_graphics::UiCanvasPlugin;

use super::{
    world::{SlugWorldPlugin, World}, ui_body::UiSlugBodyPlugin,
    control::SlugControlPlugin
};

#[derive(Component)]
pub struct Body {
    pos: Point,

    dir: Angle,

    speed: f32,
    arrest: f32,

    sensor_left: bool,
    sensor_right: bool,
    sensor_food: bool,

    satiety: f32,

    muscle_left: f32,
    muscle_right: f32,

    state: Tensor, // TODO: cleanup and move to cilia
}

impl Body {
    const ARREST_DECAY : f32 = -0.1;
    const ARREST_THRESHOLD : f32 = 0.4;

    const MUSCLE_DECAY : f32 = -0.05;
    const MUSCLE_THRESHOLD : f32 = 0.2;

    const SATIETY_INCREATE : f32 = 0.025;
    const SATIETY_DECAY : f32 = 0.0025;

    const SPEED : f32 = 0.05;

    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dir: Angle::Unit(0.),
            speed: 1.,
            arrest: 0.,

            sensor_left: false,
            sensor_right: false,
            sensor_food: false,
            satiety: 0.,

            muscle_left: 1.,
            muscle_right: 0.,

            state: Tensor::zeros([3, 2]),
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn dir(&self) -> Angle {
        self.dir
    }

    pub fn is_sensor_left(&self) -> bool {
        self.sensor_left
    }

    pub fn is_sensor_right(&self) -> bool {
        self.sensor_right
    }

    pub fn is_sensor_food(&self) -> bool {
        self.sensor_food
    }

    pub fn get_satiety(&self) -> f32 {
        self.satiety
    }

    pub fn muscle_left(&self) -> f32 {
        self.muscle_left
    }

    pub fn set_muscle_left(&mut self, muscle: f32) {
        // simulate refraction by only updating when zero.
        if self.muscle_left <= 0. {
            self.muscle_left = self.muscle_left.max(muscle).clamp(0., 1.);
        }
    }

    pub fn _is_muscle_left(&self) -> bool {
        self.muscle_left >= 0.
    }

    pub fn muscle_right(&self) -> f32 {
        self.muscle_right
    }

    pub fn _is_muscle_right(&self) -> bool {
        self.muscle_right >= 0.
    }

    pub fn set_muscle_right(&mut self, muscle: f32) {
        // simulate refraction by only updating when zero.
        if self.muscle_right <= 0. {
            self.muscle_right = self.muscle_right.max(muscle).clamp(0., 1.);
        }
    }

    pub fn speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_arrest(&self) -> f32 {
        self.arrest
    }

    ///
    /// Stop the muco-cilia beating for a period of time
    /// 
    pub fn arrest(&mut self, time: f32) {
        if self.arrest <= 0. {
            self.arrest = time;
        }
    }

    pub fn state(&self) -> &Tensor {
        &self.state
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.insert_resource(Body::new(Point(2.5, 2.5)));
}

///
/// Update the plankton's position based on the cilia movement
/// 
pub fn body_physics(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    let Point(mut x, mut y) = body.pos;
    let mut dir = body.dir.to_unit();

    // default movement is falling
    let (dy, dx) = body.dir.to_radians().sin_cos();

    let speed = body.speed * Body::SPEED;
    body.speed = 1.;

    // if cilia aren't arrested, move in the direction
    if body.arrest <= Body::ARREST_THRESHOLD {
        x = x + dx * speed;
        y = y + dy * speed;

        if body.muscle_left > Body::MUSCLE_THRESHOLD {
            dir += 0.01;
        } else if body.muscle_right > Body::MUSCLE_THRESHOLD {
            dir -= 0.01;
        }
    }

    // update y, clamped to the world boundaries
    let head = Point(x + dx * 0.5, y + dy * 0.5);

    // sensor ahead and to the side
    let sensor_left = (head.0 + dx * 0.1 - dy * 0.1, head.1 + dy * 0.1 + dx * 0.1);
    body.sensor_left = world.is_collide(sensor_left);

    let sensor_right = (head.0 + dx * 0.1 + dy * 0.1, head.1 + dy * 0.1 - dx * 0.1);
    body.sensor_right = world.is_collide(sensor_right);

    if ! world.is_collide(head) {
        body.pos = Point(x, y);
    } else if ! body.sensor_left && ! body.sensor_right {
        body.sensor_left = true;
        body.sensor_right = true;
    }

    body.sensor_food = world.is_food((x, y));

    body.satiety = (body.satiety - Body::SATIETY_DECAY).max(0.);

    if world.is_food((x, y)) {
        body.satiety = (body.satiety + Body::SATIETY_INCREATE).clamp(0., 1.);
    }

    body.dir = Angle::Unit((dir + 1.) % 1.);

    body.arrest = (body.arrest + Body::ARREST_DECAY).max(0.);
    body.muscle_left = (body.muscle_left + Body::MUSCLE_DECAY).max(0.);
    body.muscle_right = (body.muscle_right + Body::MUSCLE_DECAY).max(0.);

    body.state = tf32!([
        [if body.sensor_left { 1. } else { 0. }, 
        if body.sensor_right { 1. } else { 0. }],
        [ body.muscle_left, body.muscle_right ],
        [ if body.sensor_food { 1. } else { 0. }, body.arrest ]
    ])
}

pub fn body_log(
    body: &Body,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1} swim={:.1} arrest={:.1}",
        body.pos.x(), body.pos.y(), body.dir.to_unit(), body.speed, body.arrest
    ));
}

pub struct SlugBodyPlugin;

impl Plugin for SlugBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<SlugWorldPlugin>(), "BodyPlugin requires WorldPlugin");
        app.system(Startup, spawn_body);

        app.system(Update, body_physics);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }

        if app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiSlugBodyPlugin);
        }

        if ! app.contains_plugin::<SlugControlPlugin>() {
            app.plugin(SlugControlPlugin);
        }
    }
}