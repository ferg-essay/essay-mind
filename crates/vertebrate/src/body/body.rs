use essay_ecs::prelude::*;

use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use util::random::random_uniform;
use crate::body::touch::Touch;

use crate::util::{Angle, Heading, Point, Seconds, Ticks, Turn};
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
    middle_len: f32,

    cast_delta: Angle,
    cast_angle: Angle,
    noise_threshold: f32,

    pos: Point,

    dir: Heading,
    cast_pos: Angle,

    action: Action,

    collide_left: bool,
    collide_right: bool,
    collide_forward: bool,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        let mut noise_threshold = 0.2;

        if cfg!(test) {
            noise_threshold = 0.;
        }

        Self {
            body_len: 1.,
            middle_len: 0.2,

            cast_angle: Angle::Unit(0.),
            cast_delta: Angle::Unit(0.),
            noise_threshold,

            pos,
            dir: Heading::Unit(0.),
            cast_pos: Angle::Unit(0.),

            action: Action::new(BodyAction::None, 0., Turn::Unit(0.)),

            collide_left: false,
            collide_right: false,
            collide_forward: false,
        }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.body_len
    }

    #[inline]
    pub fn middle_len(&self) -> f32 {
        self.middle_len
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn head_pos(&self) -> Point {
        self.calculate_head_pos()
    }

    fn calculate_head_pos(&self) -> Point {
        let Point(x, y) = self.pos;

        let (dy, dx) = self.dir().sin_cos();
        let mid_len = 0.5 * self.middle_len;

        let (x, y) = (x + dx * mid_len, y + dy * mid_len);

        let (dy, dx) = self.head_dir().sin_cos();

        let len = 0.5 * self.body_len - mid_len;

        // head location
        Point(x + dx * len, y + dy * len)
    }

    #[inline]
    pub fn dir(&self) -> Heading {
        self.dir
    }

    #[inline]
    pub fn head_dir(&self) -> Heading {
        let cast = Angle::unit(self.cast_pos.sin() * self.cast_angle.to_unit());
        
        self.dir + self.turn() + cast
    }

    #[inline]
    pub fn head_cast(&self) -> f32 {
        self.cast_pos.sin()
    }

    #[inline]
    pub fn set_cast_period(&mut self, cast_period: impl Into<Seconds>) {
        let period: Seconds = cast_period.into();

        if period.0 == 0. {
            self.cast_delta = Angle::Unit(0.)
        } else {
            let ticks = Ticks::TICKS_PER_SECOND as f32 * period.0;

            self.cast_delta = Angle::Unit(1. / ticks);
        }
    }

    #[inline]
    pub fn set_cast_angle(&mut self, cast_angle: impl Into<Angle>) {
        self.cast_angle = cast_angle.into();
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
    pub fn is_collide_forward(&self) -> bool {
        self.collide_forward
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
    pub fn action(&mut self, kind: BodyAction, speed: f32, turn: Turn) {
        self.action = Action::new(kind, speed, turn);
    }

    #[inline]
    pub fn stop(&mut self) {
        if self.action.speed > 0. {
            self.action(BodyAction::None, 0., Turn::Unit(0.))
        }
    }

    #[inline]
    pub fn stop_action(&mut self, kind: BodyAction) {
        self.action(kind, 0., Turn::Unit(0.))
    }

    pub fn eat(&mut self) {
        self.stop_action(BodyAction::Eat);
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        self.action.speed
    }

    #[inline]
    pub fn turn(&self) -> Turn {
        self.action.turn
    }

    ///
    /// Update the animal's position
    /// 
    pub fn update(&mut self, world: &World) {
        self.action.update();

        let speed = self.speed() / Ticks::TICKS_PER_SECOND as f32;

        let mut dir = self.dir.to_unit();
        let turn = self.turn().to_unit();
        dir += turn / Ticks::TICKS_PER_SECOND as f32;

        // random noise into direction
        if speed > 0. && random_uniform() < self.noise_threshold {
            if random_uniform() < 0.5 {
                dir += 0.005;
            } else {
                dir -= 0.005;
            }
        }

        self.dir = Heading::unit(dir);

        let Point(mut x, mut y) = self.pos;

        // head casting
        self.cast_pos = self.cast_pos + self.cast_delta;

        let head = self.head_pos();

        let s = 0.1;
        let (dy, dx) = self.dir.sin_cos();
        let (dy, dx) = (s * dy, s * dx);

        let sensor_forward = (head.0 + dx, head.1 + dy);
        self.collide_forward = world.is_collide(sensor_forward);

        let (dy, dx) = (0.707 * dy, 0.707 * dx);

        // sensor 45 deg to the side
        let sensor_left = (head.0 + dx - dy, head.1 + dy + dx);
        self.collide_left = world.is_collide(sensor_left);

        let sensor_right = (head.0 + dx + dy, head.1 + dy - dx);
        self.collide_right = world.is_collide(sensor_right);

        let prev = self.pos;

        x = (1. - speed) * x + speed * head.0;
        y = (1. - speed) * y + speed * head.1;

        self.pos = Point(x, y);

        if world.is_collide(self.pos()) || world.is_collide(self.head_pos()) {
            self.pos = prev;
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
    turn: Turn,
    timeout: usize,
}

impl Action {
    fn new(kind: BodyAction, speed: f32, turn: Turn) -> Self {
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
            self.turn = Turn::unit(0.);
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
    cast_period: Seconds,
}

impl BodyPlugin {
    pub fn new() -> Self {
        BodyPlugin {
            pos: Point(0.5, 0.5),
            cast_period: Seconds(0.),
        }
    }

    //
    // Sets the animal's initial position.
    //
    pub fn pos(mut self, pos: impl Into<Point>) -> Self {
        self.pos = pos.into();

        self
    }

    //
    // Sets the animal's casting.
    //
    pub fn cast_period(mut self, cast_period: impl Into<Seconds>) -> Self {
        let period = cast_period.into();

        assert!(period.0 >= 0.);

        self.cast_period = period;

        self
    }
}

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<WorldPlugin>(), "BodyPlugin requires WorldPlugin");

        let mut body = Body::new(Point(0.5, 0.5));

        if self.cast_period.0 >= 0. {
            body.set_cast_period(self.cast_period);
            body.set_cast_angle(Angle::Deg(20.));
        }

        app.insert_resource(body);

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

    use crate::{
        body::BodyAction, 
        util::{Heading, Point, Ticks, Turn}, 
        world::{World, WorldPlugin}
    };

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
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.4999999, 1.), app.eval(|x: Res<Body>| x.head_pos()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Turn::unit(0.), app.eval(|x: Res<Body>| x.turn()));
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
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.4999999, 1.), app.eval(|x: Res<Body>| x.head_pos()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Turn::unit(0.), app.eval(|x: Res<Body>| x.turn()));
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
            x.action(BodyAction::Roam, 1., Turn::unit(0.));
        });
        
        app.tick();

        assert_eq!(Point(0.49999997, 0.55), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999988, 1.05), app.eval(|x: Res<Body>| x.head_pos()));
        assert_eq!(true, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(1., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Turn::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::Roam, app.eval(|x: Res<Body>| x.action_kind()));
        
        app.tick();

        assert_eq!(Point(0.49999994, 0.6), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999985, 1.1), app.eval(|x: Res<Body>| x.head_pos()));
        assert_eq!(true, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(1., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Turn::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::Roam, app.eval(|x: Res<Body>| x.action_kind()));

        for _ in 0..Ticks::TICKS_PER_SECOND - 1 {
            app.tick();
        }

        assert_eq!(Point(0.49999985, 0.9999998), app.eval(|x: Res<Body>| x.pos()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.dir()));
        assert_eq!(Heading::unit(0.0), app.eval(|x: Res<Body>| x.head_dir()));
        assert_eq!(Point(0.49999976, 1.4999998), app.eval(|x: Res<Body>| x.head_pos()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_moving()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_left()));
        assert_eq!(false, app.eval(|x: Res<Body>| x.is_collide_right()));
        assert_eq!(0., app.eval(|x: Res<Body>| x.speed()));
        assert_eq!(Turn::unit(0.), app.eval(|x: Res<Body>| x.turn()));
        assert_eq!(BodyAction::None, app.eval(|x: Res<Body>| x.action_kind()));
    }
}