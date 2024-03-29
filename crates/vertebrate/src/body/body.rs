use essay_ecs::prelude::*;

use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use util::random::random_uniform;
use crate::body::touch::Touch;

use crate::util::{Angle, Point, Ticks};
use crate::world::{World, WorldPlugin};

///
/// Body is the locomotive core of the animal.
/// 
/// The body contains a position, direction, a current action,
/// and the state of the last collision.
/// 
/// Actions are movement, turn pairs and they timeout after a
/// simulation second. Typically higher layers will refresh the action.
/// 
/// Movement is mildly stochastic, meaning the speed and turn direction
/// aren't precise or perfectly predictable. 
/// 
pub struct Body {
    body_len: f32,
    noise_threshold: f32,

    pos: Point,

    dir: Angle,

    action: Action,

    collide_left: bool,
    collide_right: bool,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        let mut noise_threshold = 0.2;

        if cfg!(test) {
            noise_threshold = 0.;
        }

        Self {
            body_len: 1.,
            noise_threshold,

            pos,
            dir: Angle::Unit(0.),

            action: Action::new(BodyAction::None, 0., Angle::Unit(0.)),

            collide_left: false,
            collide_right: false,
        }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.body_len
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
    pub fn action_kind(&self) -> BodyAction {
        self.action.kind
    }

    #[inline]
    pub fn is_moving(&self) -> bool {
        self.action.kind != BodyAction::None
    }

    #[inline]
    pub fn is_collide_left(&self) -> bool {
        self.collide_left
    }

    #[inline]
    pub fn is_collide_right(&self) -> bool {
        self.collide_right
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

    ///
    /// Update the animal's position
    /// 
    pub fn update(&mut self, world: &World) {
        self.action.update();

        let speed = self.speed() / Ticks::TICKS_PER_SECOND as f32;

        let mut dir = self.dir.to_unit();
        let turn_unit = self.turn().to_turn();
        dir += turn_unit / Ticks::TICKS_PER_SECOND as f32;

        // random noise into direction
        if speed > 0. && random_uniform() < self.noise_threshold {
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
/// Update the animal's position
/// 
fn body_update(
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
}

#[derive(Clone, Debug)]
struct Action {
    kind: BodyAction,
    speed: f32,
    turn: Angle,
    timeout: usize,
}

impl Action {
    fn new(kind: BodyAction, speed: f32, turn: Angle) -> Self {
        assert!(-1. <= speed && speed <= 1.);

        Self {
            kind,
            speed,
            turn,
            timeout: Ticks::TICKS_PER_SECOND,
        }
    }

    fn update(&mut self) {
        if self.timeout > 0 {
            self.timeout -= 1;
        } else {
            self.kind = BodyAction::None;
            self.speed = 0.;
            self.turn = Angle::unit(0.);
            self.timeout = 0x1000;
        }
    }
}

///
/// Descriptive movement actions
/// 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BodyAction {
    None,
    Sleep,
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

///
/// Animal base locomotor body contains a position, direction and
/// a locomotion action.
/// 
pub struct BodyPlugin {
    pos: Point,
}

impl BodyPlugin {
    pub fn new() -> Self {
        BodyPlugin {
            pos: Point(0.5, 0.5)
        }
    }

    //
    // Sets the animal's initial position.
    //
    pub fn pos(mut self, pos: impl Into<Point>) -> Self {
        self.pos = pos.into();

        self
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

#[cfg(test)]
mod test {
    use essay_ecs::core::{Res, ResMut};
    use mind_ecs::MindApp;

    use crate::{body::BodyAction, util::{Angle, Point, Ticks}, world::{World, WorldPlugin}};

    use super::{Body, BodyPlugin};

    #[test]
    fn default_body() {
        let mut app = MindApp::new();
        app.plugin(WorldPlugin::new(7, 13));
        app.plugin(BodyPlugin::new());

        assert_eq!((7, 13), app.eval(|x: Res<World>| x.extent()));

        assert_eq!(1., app.eval(|x: Res<Body>| x.len()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.noise_threshold));
        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.4999999, 1.), app.eval(|x: Res<Body>| x.pos_head()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Angle::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::None, app.eval(|x: Res<Body>| x.action_kind()));
    }

    #[test]
    fn default_move() {
        let mut app = MindApp::new();
        app.plugin(WorldPlugin::new(7, 13));
        app.plugin(BodyPlugin::new());

        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));

        for _ in 0..100 {
            app.tick();
        }
        
        assert_eq!((7, 13), app.eval(|x: Res<World>| x.extent()));
        assert_eq!(1., app.eval(|x: Res<Body>| x.len()));
        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.4999999, 1.), app.eval(|x: Res<Body>| x.pos_head()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Angle::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::None, app.eval(|x: Res<Body>| x.action_kind()));
    }

    #[test]
    fn move_1() {
        let mut app = MindApp::new();
        app.plugin(WorldPlugin::new(7, 13));
        app.plugin(BodyPlugin::new());
        app.setup();

        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));

        app.tick();
        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));

        app.eval(|mut x: ResMut<Body>| {
            x.set_action(BodyAction::Roam, 1., Angle::unit(0.));
        });
        
        app.tick();

        assert_eq!(Point(0.49999997, 0.55), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999988, 1.05), app.eval(|x: Res<Body>| x.pos_head()));
        assert_eq!(true, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(1., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Angle::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::Roam, app.eval(|x: Res<Body>| x.action_kind()));
        
        app.tick();

        assert_eq!(Point(0.49999994, 0.6), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999985, 1.1), app.eval(|x: Res<Body>| x.pos_head()));
        assert_eq!(true, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(1., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Angle::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::Roam, app.eval(|x: Res<Body>| x.action_kind()));

        for _ in 0..Ticks::TICKS_PER_SECOND - 1 {
            app.tick();
        }

        assert_eq!(Point(0.49999985, 0.9999998), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Angle::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999976, 1.4999998), app.eval(|x: Res<Body>| x.pos_head()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Angle::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::None, app.eval(|x: Res<Body>| x.action_kind()));
    }
}