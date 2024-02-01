use essay_ecs::prelude::*;

use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use util::random::random_uniform;
use crate::body::touch::Touch;

use crate::util::{Angle, Point, Ticks};
use crate::world::{OdorType, World, WorldPlugin};

pub struct Body {
    pos: Point,

    dir: Angle,

    body_len: f32,

    action: Action,

    collide_left: bool,
    collide_right: bool,

    ticks: usize,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dir: Angle::Unit(0.),
            body_len: 1.,

            action: Action::new(BodyAction::None, 0., Angle::Unit(0.)),

            collide_left: false,
            collide_right: false,

            ticks: 0,
        }
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn pos_head(&self) -> Point {
        let Point(x, y) = self.pos;

        let (dy, dx) = self.dir.sin_cos();

        let len = self.body_len;
        // head location
        let head = Point(x + dx * 0.5 * len, y + dy * 0.5 * len);

        head
    }

    #[inline]
    pub fn dir(&self) -> Angle {
        self.dir
    }

    #[inline]
    pub fn head_dir(&self) -> Angle {
        self.dir()
    }

    #[inline]
    pub fn set_action(&mut self, kind: BodyAction, speed: f32, turn: Angle) {
        self.action = Action::new(kind, speed, turn);
    }

    #[inline]
    pub fn stop(&mut self) {
        if self.action.speed > 0. {
            self.set_action(BodyAction::None, 0., Angle::Unit(0.))
        }
    }

    #[inline]
    pub fn stop_action(&mut self, kind: BodyAction) {
        self.set_action(kind, 0., Angle::Unit(0.))
    }

    pub fn eat(&mut self) {
        self.stop_action(BodyAction::Eat);
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        self.action.speed
    }

    #[inline]
    pub fn turn(&self) -> Angle {
        self.action.turn
    }

    #[inline]
    pub fn action_kind(&self) -> BodyAction {
        self.action.kind
    }

    #[inline]
    pub fn is_collide_left(&self) -> bool {
        self.collide_left
    }

    #[inline]
    pub fn is_collide_right(&self) -> bool {
        self.collide_right
    }

    // TODO: move out of body
    pub fn odor_turn(&self, world: &World) -> Option<(OdorType, Angle)> {
        if let Some((odor, angle)) = world.odor(self.pos_head()) {
            let turn = (2. + angle.to_unit() - self.dir().to_unit()) % 1.;

            Some((odor, Angle::Unit(turn)))
        } else {
            None
        }
    }

    ///
    /// Update the animal's position
    /// 
    pub fn update(&mut self, world: &World) {
        let speed = self.speed() / Ticks::TICKS_PER_SECOND as f32;

        let mut dir = self.dir.to_unit();
        let turn_unit = self.turn().to_turn();
        dir += turn_unit / Ticks::TICKS_PER_SECOND as f32;

        // random noise into direction
        if speed > 0. && random_uniform() < 0.2 {
            if random_uniform() < 0.5 {
                dir += 0.005;
            } else {
                dir -= 0.005;
            }
        }

        self.dir = Angle::unit(dir);

        let Point(mut x, mut y) = self.pos;

        let (dy, dx) = self.dir.sin_cos();

        // head location
        let head = self.pos_head();

        // sensor ahead and to the side
        let sensor_left = (head.0 + dx * 0.1 - dy * 0.1, head.1 + dy * 0.1 + dx * 0.1);
        self.collide_left = world.is_collide(sensor_left);

        let sensor_right = (head.0 + dx * 0.1 + dy * 0.1, head.1 + dy * 0.1 - dx * 0.1);
        self.collide_right = world.is_collide(sensor_right);

        x = (1. - speed) * x + speed * head.0;
        y = (1. - speed) * y + speed * head.1;

        if ! world.is_collide((x, y)) {
            self.pos = Point(x, y);
        } else if ! self.collide_left && ! self.collide_right {
            self.collide_left = true;
            self.collide_right = true;
        }
    }
}


///
/// Update the animal's position based on the cilia movement
/// 
pub fn body_update(
    mut body: ResMut<Body>,
    mut touch_event: OutEvent<Touch>,
    world: Res<World>,
) {
    body.update(world.get());

    if body.is_collide_left() {
        touch_event.send(Touch::CollideLeft);
    } 
    
    if body.is_collide_right() {
        touch_event.send(Touch::CollideRight);
    }

    body.ticks += 1;
}

#[derive(Clone, Debug)]
struct Action {
    kind: BodyAction,
    speed: f32,
    turn: Angle,
}

impl Action {
    fn new(kind: BodyAction, speed: f32, turn: Angle) -> Self {
        assert!(-1. <= speed && speed <= 1.);

        Self {
            kind,
            speed,
            turn,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BodyAction {
    None,
    Roam,
    Dwell,
    Avoid,
    Seek,
    Eat,
}

pub fn body_log(
    body: Res<Body>,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1} speed={:.1} turn={:.1}",
        body.pos().x(), body.pos().y(), body.dir().to_unit(), body.speed(), body.turn().to_unit()
    ));
}

pub struct BodyPlugin {
}

impl BodyPlugin {
    pub fn new() -> Self {
        BodyPlugin {
        }
    }
}

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<WorldPlugin>(), "BodyPlugin requires WorldPlugin");

        app.insert_resource(Body::new(Point(0.5, 0.5)));

        app.event::<Touch>();

        app.system(Tick, body_update);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }
    }
}