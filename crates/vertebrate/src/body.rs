use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::{Tensor, tf32};
use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use ui_graphics::UiCanvasPlugin;

use crate::world::{OdorType, World, SlugWorldPlugin};

use super::{
    ui_body::UiSlugBodyPlugin,
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

    tick_food: usize,
    ticks: usize,

    is_single_habituate: bool,
    odor_habituate: Vec<Habituate>,

    muscle_left: f32,
    muscle_right: f32,

    state: Tensor, // TODO: cleanup and move to cilia
}

impl Body {
    const ARREST_DECAY : f32 = -0.1;
    const ARREST_THRESHOLD : f32 = 0.4;

    const MUSCLE_DECAY : f32 = -0.05;
    const MUSCLE_THRESHOLD : f32 = 0.2;

    const SPEED : f32 = 0.025;

    const FOOD_DIST : f32 = 1.5;

    pub fn new(pos: Point) -> Self {
        let mut odor_habituate = Vec::new();

        for i in 0..OdorType::count() {
            odor_habituate.push(Habituate::new(i));
        }

        Self {
            pos,
            dir: Angle::Unit(0.),
            speed: 1.,
            arrest: 0.,

            sensor_left: false,
            sensor_right: false,
            sensor_food: false,

            tick_food: 0,
            ticks: 0,

            is_single_habituate: false,
            odor_habituate,

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

    pub fn is_touch_left(&self) -> bool {
        self.sensor_left
    }

    pub fn is_touch_right(&self) -> bool {
        self.sensor_right
    }

    pub fn is_sensor_food(&self) -> bool {
        self.sensor_food
    }

    pub fn p_food(&self) -> f32 {
        self.tick_food as f32 / self.ticks.max(1) as f32
    }

    pub fn odor_turn(&self, world: &World) -> Option<(OdorType, Angle)> {
        if let Some((odor, angle)) = world.odor(self.pos) {
            let turn = (2. + angle.to_unit() - self.dir.to_unit()) % 1.;

            if self.is_odor_active(odor) {
                Some((odor, Angle::Unit(turn)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_food_left(&mut self, world: &World) -> bool {
        if let Some((odor, angle)) = world.odor(self.pos) {
            let turn = (2. + angle.to_unit() - self.dir.to_unit()) % 1.;

            turn <= 0.5 && self.is_odor_active(odor)
        } else {
            false
        }
    }

    pub fn is_food_right(&mut self, world: &World) -> bool {
        if let Some((odor, angle)) = world.odor(self.pos) {
            let turn = (2. + angle.to_unit() - self.dir.to_unit()) % 1.;

            0.5 < turn && self.is_odor_active(odor)
        } else {
            false
        }
    }

    pub fn is_single_habituate(&self) -> bool {
        self.is_single_habituate
    }

    fn is_odor_active(&self, odor: OdorType) -> bool {
        if self.is_single_habituate {
            self.odor_habituate[0].is_active()
        } else {
            self.odor_habituate[odor.index()].is_active()
        }
    }

    pub fn get_food_habituate(&self, odor: OdorType) -> f32 {
        self.odor_habituate[odor.index()].food
    }

    pub fn odor_habituate(&mut self, i: usize, odor: Option<OdorType>) {
        if self.is_single_habituate {
            if odor.is_some() {
                self.odor_habituate[0].update(Some(OdorType::from(0)));
            } else {
                self.odor_habituate[0].update(None);
            }
        } else {
            self.odor_habituate[i].update(odor);
        }
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

    pub fn _speed(&mut self, speed: f32) {
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

struct Habituate {
    odor: OdorType,
    food: f32,
}

impl Habituate {
    pub const INCREASE: f32 = 0.002;
    pub const DECAY: f32 = 0.002;
    pub const THRESHOLD: f32 = 0.75;

    fn new(i: usize) -> Self {
        Self {
            odor: OdorType::from(i),
            food: 1.,
        }
    }

    fn update(&mut self, odor: Option<OdorType>) {
        if Some(self.odor) == odor {
            self.food = (self.food + Self::INCREASE).clamp(0., 1.);
        } else {
            self.food = (self.food - Self::DECAY).clamp(0., 1.);
        }
    }

    fn is_active(&self) -> bool {
        self.food < Self::THRESHOLD
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.insert_resource(Body::new(Point(0.5, 0.5)));
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

        // random noise into direction
        if random() < 0.2 {
            if random() < 0.5 {
                dir += 0.005;
            } else {
                dir -= 0.005;
            }
        }
    }

    if body.muscle_left > Body::MUSCLE_THRESHOLD {
        dir += 0.015 * (1. + 0.2 * (random() - 0.5));
    } else if body.muscle_right > Body::MUSCLE_THRESHOLD {
        dir -= 0.015 * (1. + 0.2 * (random() - 0.5));
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

    if body.sensor_food {
        body.tick_food += 1;
    }
    body.ticks += 1;

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

pub fn body_habit(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    let odor = match world.odor(body.pos) {
        Some((odor, _)) => Some(odor),
        None => None,
    };

    for i in 0..OdorType::count() {
        body.odor_habituate(i, odor);
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
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

        app.system(Tick, body_physics);
        app.system(Tick, body_habit);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }

        if ! app.contains_plugin::<SlugControlPlugin>() {
            app.plugin(SlugControlPlugin);
        }
    }
}