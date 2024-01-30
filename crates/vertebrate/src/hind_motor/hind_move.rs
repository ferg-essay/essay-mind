use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::touch::Touch;
use crate::body::{Body, BodyAction, BodyPlugin};
use crate::core_motive::motive::Motive;
use crate::core_motive::Dwell;
use crate::util::{Angle, DecayValue, DirVector, Command, Seconds, Ticks};
use util::random::{random_pareto, random, random_normal};


pub struct HindMove {
    left60: Turn,
    right60: Turn,
    left120: Turn,
    _right120: Turn,

    move_commands: Command<MoveCommand>,
    turn_commands: Command<TurnCommand>,

    action_kind: ActionKind,
    action: Action,

    left_approach: DecayValue,
    left_avoid: DecayValue,
    right_approach: DecayValue,
    right_avoid: DecayValue,
    forward_avoid: DecayValue,

    random_walk: RandomWalk,

    is_first: bool,
}

impl HindMove {
    const _CPG_TIME : f32 = 1.;
    const HALF_LIFE : f32 = 0.5;

    fn new() -> Self {
        Self {
            left60: Turn::new(Angle::Deg(-60.), Angle::Deg(15.)),
            right60: Turn::new(Angle::Deg(60.), Angle::Deg(15.)),

            left120: Turn::new(Angle::Deg(-120.), Angle::Deg(60.)),
            _right120: Turn::new(Angle::Deg(120.), Angle::Deg(60.)),

            left_approach: DecayValue::new(Self::HALF_LIFE),
            left_avoid: DecayValue::new(Self::HALF_LIFE),
            right_approach: DecayValue::new(Self::HALF_LIFE),
            right_avoid: DecayValue::new(Self::HALF_LIFE),
            forward_avoid: DecayValue::new(Self::HALF_LIFE),

            random_walk: RandomWalk::new(),
            move_commands: Command::new(),
            turn_commands: Command::new(),
            action: Action::none(),
            action_kind: ActionKind::Explore,

            is_first: true,
        }
    }

    pub fn get_avoid_left(&self) -> f32 {
        self.left_avoid.value()
    }

    pub fn get_avoid_right(&self) -> f32 {
        self.right_avoid.value()
    }

    pub fn get_avoid_forward(&self) -> f32 {
        self.forward_avoid.value()
    }

    pub fn get_forward_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::Stop => 0.5,
            ActionKind::Explore => self.random_walk.forward_delta(),
            ActionKind::StrongAvoidLeft => 0.5,
            ActionKind::StrongAvoidRight => 0.5,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_left_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::Stop => 0.5,
            ActionKind::Explore => self.random_walk.left_delta(),
            ActionKind::StrongAvoidLeft => 1.,
            ActionKind::StrongAvoidRight => 0.5,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    pub fn get_right_delta(&self) -> f32 {
        match self.action_kind {
            ActionKind::Stop => 0.5,
            ActionKind::Explore => self.random_walk.right_delta(),
            ActionKind::StrongAvoidLeft => 0.5,
            ActionKind::StrongAvoidRight => 1.,
            ActionKind::StrongAvoidBoth => 1.,
        }
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.action_kind == ActionKind::Stop
    }

    #[inline]
    pub fn explore(&self) {
        self.send_move(MoveCommand::Roam);
    }

    #[inline]
    pub fn stop(&self) {
        self.send_move(MoveCommand::Stop);
    }

    #[inline]
    pub fn send_move(&self, command: MoveCommand) {
        self.move_commands.send(command);
    }

    #[inline]
    pub fn send_turn(&self, command: TurnCommand) {
        self.turn_commands.send(command);
    }

    fn pre_update(&mut self) {
        self.is_first = true;
        self.action_kind = self.action_kind.pre_update();
        self.random_walk.pre_update();
    }

    fn update_move_commands(&mut self) {
        for command in self.move_commands.drain() {
            self.move_command(&command);
        }
    }

    fn move_command(&mut self, event: &MoveCommand) {
        //if self.is_first {
        //    self.is_first = false;
        //    self.explore.pre_update();
        //}

        match event {
            // explore/speed modes
            MoveCommand::Approach => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.approach();
            }
            MoveCommand::Avoid => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.avoid();
            }
            MoveCommand::Normal => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.normal();
            }
            MoveCommand::Roam => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.roam();
            }
            MoveCommand::Dwell => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.dwell();
            }
            MoveCommand::Stop => {
                self.action_kind = ActionKind::Stop;
                self.random_walk.stop();
            }
        }
    }

    fn update_turn_commands(&mut self) {
        for command in self.turn_commands.drain() {
            self.turn_command(command);
        }
    }

    fn turn_command(&mut self, event: TurnCommand) {
        match event {
            // collision/escape - strong avoid events
            TurnCommand::StrongAvoidLeft => {
                self.left_avoid.set(1.);
                self.action_kind = self.action_kind.avoid_left();
            }
            TurnCommand::StrongAvoidRight => {
                self.right_avoid.set(1.);
                self.action_kind = self.action_kind.avoid_right();
            }
            TurnCommand::StrongAvoidBoth => {
                self.left_avoid.set(1.);
                self.right_avoid.set(1.);
                self.action_kind = self.action_kind.avoid_left();
                self.action_kind = self.action_kind.avoid_right();
            }

            // taxis gradient
            TurnCommand::AvoidVector(vector) => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.add_avoid(vector)
            },
            
            TurnCommand::ApproachVector(vector) => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.add_approach(vector)
            },

            TurnCommand::AvoidUTurn => {
                self.action_kind = self.action_kind.explore();
                self.random_walk.avoid_turn();
            }
        }
    }

    fn update(&mut self, body: &mut Body) {
        if self.action.pre_update() {
            return;
        }

        match self.action_kind {
            ActionKind::Stop => {
                // self.action = self.random_walk.update(body);

                // body.set_action(self.action.kind, self.action.speed, self.action.turn);
                body.stop();
            },
            ActionKind::Explore => {
                self.action = self.random_walk.update();

                body.set_action(self.action.kind, self.action.speed, self.action.turn);
            },
            ActionKind::StrongAvoidLeft => {
                self.action = Action::new(BodyAction::Avoid, 0.25, 1., self.right60.angle());

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

impl Default for HindMove {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)] // , Event)]
pub enum MoveCommand {
    Approach,
    Avoid,
    Normal,
    Roam,
    Dwell,
    Stop,
}

#[derive(Clone, Copy, Debug)] // , Event)]
pub enum TurnCommand {
    // escape/collision
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,

    // taxis gradient
    ApproachVector(DirVector),
    AvoidVector(DirVector),

    AvoidUTurn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ActionKind {
    Stop,
    Explore,
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,
}

impl ActionKind {
    fn pre_update(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Stop,
            ActionKind::Explore => ActionKind::Explore,
            ActionKind::StrongAvoidLeft => ActionKind::Explore,
            ActionKind::StrongAvoidRight => ActionKind::Explore,
            ActionKind::StrongAvoidBoth => ActionKind::Explore,
        }
    }

    fn explore(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Explore,
            _ => *self,
        }
    }

    fn avoid_left(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::StrongAvoidLeft,
            ActionKind::Explore => ActionKind::StrongAvoidLeft,
            ActionKind::StrongAvoidLeft => ActionKind::StrongAvoidLeft,
            ActionKind::StrongAvoidRight => ActionKind::StrongAvoidBoth,
            ActionKind::StrongAvoidBoth => ActionKind::StrongAvoidBoth,
        }
    }

    fn avoid_right(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::StrongAvoidRight,
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
    fn new(kind: BodyAction, time: impl Into<Seconds>, speed: f32, turn: Angle) -> Self {
        Self {
            kind,
            time: time.into().0,
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

fn update_hind_move(
    mut body: ResMut<Body>, 
    mut touch_events: InEvent<Touch>,
    // mut locomotor_events: InEvent<HindMoveCommand>,
    mut hind_locomotor: ResMut<HindMove>, 
    dwell: Res<Motive<Dwell>>,
) {
    hind_locomotor.pre_update();

    if dwell.is_active() {
        hind_locomotor.random_walk.dwell();
    }

    for touch in touch_events.iter() {
        match touch {
            Touch::CollideLeft => {
                hind_locomotor.turn_command(TurnCommand::StrongAvoidLeft);
            },
            Touch::CollideRight => {
                hind_locomotor.turn_command(TurnCommand::StrongAvoidRight);
            },
        }
    }

    hind_locomotor.update_turn_commands();
    hind_locomotor.update_move_commands();
    //for event in hind_locomotor.commands() {
    //    println!("HindC1 {:?}", event);
    //    hind_locomotor.move_command(&event);
    //}

    /*
    for event in locomotor_events.iter() {
        println!("HindC2 {:?}", event);
        hind_locomotor.event(event);
    }
    */

    hind_locomotor.update(body.get_mut());
}

pub struct HindMovePlugin;

impl Plugin for HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindMovePlugin requires BodyPlugin");

        // app.event::<HindLocomotorEvent>();
        app.init_resource::<HindMove>();

        app.system(Tick, update_hind_move);
        // app.system(Tick, update_hind_locomotor_motive);
    }
}
