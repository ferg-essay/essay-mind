use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::touch::Touch;
use crate::body::{Body, BodyAction, BodyPlugin};
use crate::core_motive::motive::Motive;
use crate::core_motive::Dwell;
use crate::util::{Angle, DirVector, Ticks};
use util::random::{random_pareto, random, random_normal};


pub struct HindLocomotor {
    left60: Turn,
    right60: Turn,
    left120: Turn,
    _right120: Turn,

    action_kind: ActionKind,
    action: Action,

    random_walk: RandomWalk,

    is_first: bool,
}

impl HindLocomotor {
    const _CPG_TIME : f32 = 1.;

    pub fn get_avoid_left(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.,
            ActionKind::Explore => self.random_walk.avoid_left(),
            ActionKind::StrongAvoidLeft => 1.,
            ActionKind::StrongAvoidRight => 0.,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_avoid_right(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.,
            ActionKind::Explore => self.random_walk.avoid_right(),
            ActionKind::StrongAvoidLeft => 0.,
            ActionKind::StrongAvoidRight => 1.,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_avoid_forward(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.5,
            ActionKind::Explore => self.random_walk.avoid_forward(),
            ActionKind::StrongAvoidLeft => 0.,
            ActionKind::StrongAvoidRight => 0.,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_forward_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.5,
            ActionKind::Explore => self.random_walk.forward_delta(),
            ActionKind::StrongAvoidLeft => 0.5,
            ActionKind::StrongAvoidRight => 0.5,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_left_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.5,
            ActionKind::Explore => self.random_walk.left_delta(),
            ActionKind::StrongAvoidLeft => 1.,
            ActionKind::StrongAvoidRight => 0.5,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_right_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::None => 0.5,
            ActionKind::Explore => self.random_walk.right_delta(),
            ActionKind::StrongAvoidLeft => 0.5,
            ActionKind::StrongAvoidRight => 1.,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    fn pre_update(&mut self) {
        self.is_first = true;
        self.action_kind = self.action_kind.pre_update();
        self.random_walk.pre_update();
    }

    fn event(&mut self, event: &HindLocomotorEvent) {
        //if self.is_first {
        //    self.is_first = false;
        //    self.explore.pre_update();
        //}

        match event {
            // collision/escape - strong avoid events
            HindLocomotorEvent::StrongAvoidLeft => {
                self.action_kind = self.action_kind.avoid_left();
            }
            HindLocomotorEvent::StrongAvoidRight => {
                self.action_kind = self.action_kind.avoid_right();
            }
            HindLocomotorEvent::StrongAvoidBoth => {
                self.action_kind = self.action_kind.avoid_left();
                self.action_kind = self.action_kind.avoid_right();
            }

            // gradient taxis
            HindLocomotorEvent::AvoidVector(vector) => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.add_avoid(*vector)
            },
            
            HindLocomotorEvent::ApproachVector(vector) => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.add_approach(*vector)
            },

            // explore/speed modes
            HindLocomotorEvent::Approach => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.approach();
            }
            HindLocomotorEvent::Avoid => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.avoid();
            }
            HindLocomotorEvent::AvoidUTurn => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.avoid_turn();
            }
            HindLocomotorEvent::Normal => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.normal();
            }
            HindLocomotorEvent::Roam => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.roam();
            }
            HindLocomotorEvent::Dwell => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.dwell();
            }
            HindLocomotorEvent::Stop => {
                self.action_kind = ActionKind::None;
                self.random_walk.stop();
            }

            // display events
            HindLocomotorEvent::ApproachDisplay(_) => {},
            HindLocomotorEvent::AvoidDisplay(_) => {},
        }
    }

    fn update(&mut self, body: &mut Body) {
        if self.action.pre_update() {
            return;
        }

        match self.action_kind {
            ActionKind::None => {
                // self.action = self.random_walk.update(body);

                // body.set_action(self.action.kind, self.action.speed, self.action.turn);
                body.stop();
            },
            ActionKind::Explore => {
                self.action = self.random_walk.update(body);

                body.set_action(self.action.kind, self.action.speed, self.action.turn);
            },
            ActionKind::StrongAvoidLeft => {
                body.avoid(1., self.right60.angle());
            },
            ActionKind::StrongAvoidRight => {
                body.avoid(1., self.left60.angle());
            },
            ActionKind::StrongAvoidBoth => {
                body.avoid(1., self.left120.angle());
                //todo!();
                /*
                if random_normal() < 0. {
                    body.locomotion_mut().avoid(self.left120.action(1.));
                } else {
                    body.locomotion_mut().avoid(self.right120.action(1.));
                }
                */
            },
        }
    }
}

impl Default for HindLocomotor {
    fn default() -> Self {
        HindLocomotor {
            left60: Turn::new(Angle::Deg(-60.), Angle::Deg(15.)),
            right60: Turn::new(Angle::Deg(60.), Angle::Deg(15.)),

            left120: Turn::new(Angle::Deg(-120.), Angle::Deg(60.)),
            _right120: Turn::new(Angle::Deg(120.), Angle::Deg(60.)),

            random_walk: RandomWalk::new(),
            action: Action::none(),
            action_kind: ActionKind::Explore,

            is_first: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Event)]
pub enum HindLocomotorEvent {
    // escape/collision
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,

    // gradient taxis
    ApproachVector(DirVector),
    AvoidVector(DirVector),

    ApproachDisplay(DirVector),
    AvoidDisplay(DirVector),

    // speed modes
    Approach,
    Avoid,
    AvoidUTurn,
    Normal,
    Roam,
    Dwell,
    Stop,
}

#[derive(Clone, Copy, Debug)]
enum ActionKind {
    None,
    Explore,
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,
}

impl ActionKind {
    fn pre_update(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::None,
            ActionKind::Explore => ActionKind::Explore,
            ActionKind::StrongAvoidLeft => ActionKind::Explore,
            ActionKind::StrongAvoidRight => ActionKind::Explore,
            ActionKind::StrongAvoidBoth => ActionKind::Explore,
        }
    }

    fn explore(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::Explore,
            _ => *self,
        }
    }

    fn avoid_left(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::StrongAvoidLeft,
            ActionKind::Explore => ActionKind::StrongAvoidLeft,
            ActionKind::StrongAvoidLeft => ActionKind::StrongAvoidLeft,
            ActionKind::StrongAvoidRight => ActionKind::StrongAvoidBoth,
            ActionKind::StrongAvoidBoth => ActionKind::StrongAvoidBoth,
        }
    }

    fn avoid_right(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::StrongAvoidRight,
            ActionKind::Explore => ActionKind::StrongAvoidRight,
            ActionKind::StrongAvoidLeft => ActionKind::StrongAvoidBoth,
            ActionKind::StrongAvoidRight => ActionKind::StrongAvoidRight,
            ActionKind::StrongAvoidBoth => ActionKind::StrongAvoidBoth,
        }
    }
}
struct RandomWalk {
    speed: f32,

    alpha: f32,
    low: f32,
    high: f32,

    turn_mean: f32,
    turn_std: f32,

    approach_left: f32,
    approach_right: f32,
    approach_forward: f32,
    approach_dir: DirVector,

    avoid_left: f32,
    avoid_right: f32,
    avoid_forward: f32,
    avoid_dir: DirVector,

    is_last_turn: bool,

    // action: Action,
    action_kind: BodyAction,
}

impl RandomWalk {
    const LOW : f32 = 1.;
    const HIGH : f32 = 5.;
    const ALPHA : f32 = 2.;

    const TURN_MEAN : f32 = 60.;
    const TURN_STD : f32 = 15.;

    const _CPG_TIME : f32 = 1.;

    fn new() -> Self {
        RandomWalk {
            speed: 1.,

            low: Self::LOW,
            high: Self::HIGH,
            alpha: Self::ALPHA,

            turn_mean: Self::TURN_MEAN,
            turn_std: Self::TURN_STD,

            approach_left: 0.,
            approach_right: 0.,
            approach_forward: 0.,
            approach_dir: DirVector::zero(),

            avoid_left: 0.,
            avoid_right: 0.,
            avoid_forward: 0.,
            avoid_dir: DirVector::zero(),

            is_last_turn: false,

            // action: Action::none(),
            action_kind: BodyAction::Roam,
        }
    }

    fn pre_update(&mut self) {
        self.normal();

        self.approach_left = 0.;
        self.approach_right = 0.;
        self.approach_forward = 0.;
        self.approach_dir = DirVector::zero();

        self.avoid_left = 0.;
        self.avoid_right = 0.;
        self.avoid_forward = 0.;
        self.avoid_dir = DirVector::zero();
    }

    fn normal(&mut self) {
        self.roam();
    }

    fn approach(&mut self) {
        self.dwell();
    }

    fn roam(&mut self) {
        self.speed = 1.;

        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;

        self.action_kind = BodyAction::Roam;
    }

    fn dwell(&mut self) {
        self.speed = 0.5;

        self.low = Self::LOW;
        self.high = Self::HIGH.min(2. * Self::LOW);
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;

        self.action_kind = BodyAction::Dwell;
    }

    fn stop(&mut self) {
        self.speed = 0.;

        self.action_kind = BodyAction::None;
    }

    fn add_avoid(&mut self, avoid_dir: DirVector) {
        if avoid_dir.value() > 0.05 {
            let offset = 2. * avoid_dir.sin(); // * avoid_dir.value();

            self.avoid_left = offset.clamp(0., 1.);
            self.avoid_right = (- offset).clamp(0., 1.);
            self.avoid_forward = avoid_dir.cos().clamp(0., 1.);
            self.avoid_dir = avoid_dir;
        }
    }

    fn add_approach(&mut self, approach_dir: DirVector) {
        if approach_dir.value() > 0.01 {
            let offset = 2. * approach_dir.dx(); // * approach_dir.value();

            self.approach_left = (- offset).clamp(0., 1.);
            self.approach_right = offset.clamp(0., 1.);
            self.approach_forward = - approach_dir.dy().clamp(-1., 0.);
            self.approach_dir = approach_dir;

            self.action_kind = BodyAction::Seek;
        }
    }

    fn avoid_forward(&self) -> f32 {
        self.avoid_forward
    }

    fn avoid_left(&self) -> f32 {
        self.avoid_left
    }

    fn avoid_right(&self) -> f32 {
        self.avoid_right
    }

    fn avoid(&mut self) {
        // prefer
        //self.low = 2. * Self::HIGH;
        //self.high = 2. * Self::HIGH;
        //self.alpha = 1.;

        //self.turn_mean = 2. * Self::TURN_MEAN;
        //self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = Self::ALPHA;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = 3.;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        self.low = Self::HIGH;
        self.high = 2. * Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = 0.5 * Self::TURN_MEAN;
        self.turn_std = 0.5 * Self::TURN_STD;
    }

    fn forward_delta(&self) -> f32 {
        0.5 * (self.avoid_forward - self.approach_forward + 1.)
    }

    fn left_delta(&self) -> f32 {
        0.5 * (self.avoid_left - self.approach_left + 1.)
    }

    fn right_delta(&self) -> f32 {
        0.5 * (self.avoid_right - self.approach_right + 1.)
    }

    fn avoid_turn(&mut self) {
        self.low = Self::LOW;
        self.high = 0.5 * Self::HIGH;
        self.alpha = 1.;

        self.turn_mean = 2. * Self::TURN_MEAN;
        self.turn_std = 3. * Self::TURN_STD;
    }

    fn _prefer(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;
    }

    fn _is_turn(&self) -> bool {
        self.is_last_turn
    }

    fn update(
        &mut self,
        body: &mut Body
    ) -> Action {
        //body.set_action(self.action);

        //if self.action.pre_update() {
        //    return None;
        //}

        let random = random();
        let mut mean = self.turn_mean;
        let mut std = self.turn_std;

        let avoid_forward = self.avoid_forward + self.approach_forward;
        if avoid_forward > 0. && random_normal().abs() < avoid_forward {
            mean = 2. * Self::TURN_MEAN;
            std = 3. * Self::TURN_STD;
        }

        let angle = mean + (random_normal() * std).clamp(-2. * std, 2. * std);

        // bounded pareto as approximation of Levy walk
        let low = self.low;
        let high = self.high; // 4.;
        let alpha = self.alpha;

        let f = 4.;
        let approach_left = (1. - f * self.approach_right - self.avoid_left).max(1.0e-6);
        let approach_right = (1. - f * self.approach_left - self.avoid_right).max(1.0e-6);
        let p_left = approach_left / (approach_left + approach_right).max(0.01);

        let speed = self.speed;

        // semi-brownian
        if self.is_last_turn {
            self.is_last_turn = false;

            let len = random_pareto(low, high, alpha);

            Action::new(self.action_kind, len, speed, Angle::Unit(0.))
        } else if random <= p_left {
            self.is_last_turn = true;

            Action::new(self.action_kind, 1., speed, Angle::Deg(- angle))
        } else {
            self.is_last_turn = true;

            Action::new(self.action_kind, 1., speed, Angle::Deg(angle))
        }
    }
}

struct Turn {
    mean: Angle,
    std: Angle,
}

impl Turn {
    fn new(mean: Angle, std: Angle) -> Self {
        Self {
            mean,
            std
        }
    }

    fn action(&self, speed: f32) -> Action {
        let mean = self.mean.to_unit();
        let std = self.std.to_unit();

        let angle = mean + std * random_normal().clamp(-2., 2.);

        Action::new(BodyAction::Roam, 1., speed, Angle::Unit(angle))
    }

    fn angle(&self) -> Angle {
        let mean = self.mean.to_unit();
        let std = self.std.to_unit();

        let angle = mean + std * random_normal().clamp(-2., 2.);

        Angle::unit(angle)
    }
}

#[derive(Clone, Debug)]
pub struct Action {
    kind: BodyAction,
    time: f32,
    speed: f32,
    turn: Angle,
}

impl Action {
    fn new(kind: BodyAction, time: f32, speed: f32, turn: Angle) -> Self {
        Self {
            kind,
            time,
            speed,
            turn,
        }
    }

    fn none() -> Self {
        Action::new(BodyAction::None, 0., 0., Angle::Unit(0.))
    }

    fn pre_update(&mut self) -> bool {
        self.time -= Ticks(1).to_seconds();

        return self.time >= 1.0e-6
    }
}

fn update_hind_locomotor(
    mut body: ResMut<Body>, 
    mut touch_events: InEvent<Touch>,
    mut locomotor_events: InEvent<HindLocomotorEvent>,
    mut hind_locomotor: ResMut<HindLocomotor>, 
    dwell: Res<Motive<Dwell>>,
) {
    hind_locomotor.pre_update();

    if dwell.is_active() {
        hind_locomotor.random_walk.dwell();
    }

    for touch in touch_events.iter() {
        match touch {
            Touch::CollideLeft => {
                hind_locomotor.event(&HindLocomotorEvent::StrongAvoidLeft);
            },
            Touch::CollideRight => {
                hind_locomotor.event(&HindLocomotorEvent::StrongAvoidRight);
            },
        }
    }

    for event in locomotor_events.iter() {
        hind_locomotor.event(event);

        match event {
            HindLocomotorEvent::ApproachDisplay(vector) => {
                body.set_approach_dir(*vector);
            }
            _ => {},
        }
    }

    hind_locomotor.update(body.get_mut());
}

pub struct HindLocomotorPlugin;

impl Plugin for HindLocomotorPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotorPlugin requires BodyPlugin");

        app.event::<HindLocomotorEvent>();
        app.init_resource::<HindLocomotor>();

        app.system(Tick, update_hind_locomotor);
        // app.system(Tick, update_hind_locomotor_motive);
    }
}
