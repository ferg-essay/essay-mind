use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::touch::Touch;
use crate::body::{Body, BodyAction, BodyPlugin};
use crate::util::{Angle, DecayValue, DirVector, Command, Seconds, Ticks};
use util::random::{random, random_normal, random_pareto, random_uniform};


pub struct HindMove {
    left60: Turn,
    right60: Turn,
    left120: Turn,
    _right120: Turn,

    move_commands: Command<MoveCommand>,
    turn_commands: Command<TurnCommand>,

    action_kind: ActionKind,
    action: Action,

    avoid: DecayValue,
    roam: DecayValue,
    dwell: DecayValue,
    seek: DecayValue,

    approach_left: DecayValue,
    approach_right: DecayValue,
    approach_forward: DecayValue,
    // approach_dir: DirVector,

    avoid_left: DecayValue,
    avoid_right: DecayValue,
    avoid_forward: DecayValue,
    // avoid_dir: DirVector,

    is_last_turn: bool,

    // action_kind: BodyAction,

    is_first: bool,
}

impl HindMove {
    const HALF_LIFE : f32 = 0.2;
    const LEN_LOW : f32 = 1.;
    const LEN_HIGH : f32 = 5.;
    const LEN_TURN : f32 = 1.;
    const ALPHA : f32 = 2.;

    const TURN_MEAN : f32 = 60.;
    const TURN_STD : f32 = 15.;


    fn new() -> Self {
        Self {
            left60: Turn::new(Angle::Deg(-60.), Angle::Deg(15.)),
            right60: Turn::new(Angle::Deg(60.), Angle::Deg(15.)),

            left120: Turn::new(Angle::Deg(-120.), Angle::Deg(60.)),
            _right120: Turn::new(Angle::Deg(120.), Angle::Deg(60.)),

            //random_walk: RandomWalk::new(),
            move_commands: Command::new(),
            turn_commands: Command::new(),

            action: Action::none(),
            action_kind: ActionKind::Roam,

            avoid: DecayValue::new(Self::HALF_LIFE),
            roam: DecayValue::new(Self::HALF_LIFE),
            dwell: DecayValue::new(Self::HALF_LIFE),
            seek: DecayValue::new(Self::HALF_LIFE),

            approach_left: DecayValue::new(Self::HALF_LIFE),
            approach_right: DecayValue::new(Self::HALF_LIFE),
            approach_forward: DecayValue::new(Self::HALF_LIFE),

            avoid_left: DecayValue::new(Self::HALF_LIFE),
            avoid_right: DecayValue::new(Self::HALF_LIFE),
            avoid_forward: DecayValue::new(Self::HALF_LIFE),

            is_last_turn: false,

            is_first: true,
        }
    }

    pub fn get_avoid_left(&self) -> f32 {
        self.avoid_left.value()
    }

    pub fn get_avoid_right(&self) -> f32 {
        self.avoid_right.value()
    }

    pub fn get_avoid_forward(&self) -> f32 {
        self.avoid_forward.value()
    }

    pub fn get_forward_delta(&self) -> f32 {
        0.5 * (self.avoid_forward.value() - self.approach_forward.value() + 1.)
    }

    pub fn get_left_delta(&self) -> f32 {
        0.5 * (self.avoid_left.value() - self.approach_left.value() + 1.)
    }

    pub fn get_right_delta(&self) -> f32 {
        0.5 * (self.avoid_right.value() - self.approach_right.value() + 1.)
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.action_kind == ActionKind::Stop
    }

    #[inline]
    pub fn send_move(&self, command: MoveCommand) {
        self.move_commands.send(command);
    }

    #[inline]
    pub fn stop(&self) {
        self.send_move(MoveCommand::Stop);
    }

    #[inline]
    pub fn roam(&self) {
        self.send_move(MoveCommand::Roam);
    }

    #[inline]
    pub fn dwell(&self) {
        self.send_move(MoveCommand::Dwell);
    }

    #[inline]
    pub fn send_turn(&self, command: TurnCommand) {
        self.turn_commands.send(command);
    }

    fn pre_update(&mut self) {
        self.action_kind = self.action_kind.pre_update();

        self.approach_left.update();
        self.approach_right.update();
        self.approach_forward.update();

        self.avoid_left.update();
        self.avoid_right.update();
        self.avoid_forward.update();

        self.avoid.update();
        self.seek.update();
        self.roam.update();
        self.dwell.update();
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
            MoveCommand::Seek => {
                self.action_kind = self.action_kind.explore();
                //self.random_walk.approach();
                self.roam.set_max(1.);
            }
            MoveCommand::Avoid => {
                self.action_kind = self.action_kind.explore();
                //self.random_walk.avoid();
                self.avoid.set_max(1.);
            }
            MoveCommand::Normal => {
                self.action_kind = self.action_kind.explore();
                //self.random_walk.normal();
                self.roam.set_max(1.);
            }
            MoveCommand::Roam => {
                self.action_kind = self.action_kind.explore();
                //self.random_walk.roam();
                self.roam.set_max(1.);
            }
            MoveCommand::Dwell => {
                self.action_kind = self.action_kind.explore();
                //self.random_walk.dwell();
                self.dwell.set_max(1.);
            }
            MoveCommand::Stop => {
                self.action_kind = ActionKind::Stop;
                //self.random_walk.stop();
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
                self.avoid_left.set(1.);
                self.action_kind = self.action_kind.avoid_left();
            }
            TurnCommand::StrongAvoidRight => {
                self.avoid_right.set(1.);
                self.action_kind = self.action_kind.avoid_right();
            }
            TurnCommand::StrongAvoidBoth => {
                self.avoid_left.set(1.);
                self.avoid_right.set(1.);
                self.avoid_forward.set(1.);
                self.action_kind = self.action_kind.avoid_left();
                self.action_kind = self.action_kind.avoid_right();
            }

            // taxis gradient
            TurnCommand::AvoidVector(vector) => {
                self.action_kind = self.action_kind.seek();
                self.add_avoid(vector)
                // self.roam();
            },
            
            TurnCommand::ApproachVector(vector) => {
                self.action_kind = self.action_kind.seek();
                self.add_approach(vector)
                // self.approach(); 
            },

            TurnCommand::AvoidUTurn => {
                self.action_kind = self.action_kind.explore();
                // self.avoid_turn();
                todo!()
            }
        }
    }

    fn update(&mut self, body: &mut Body) {
        self.action.pre_update();

        if self.action_kind == ActionKind::Stop {
            self.stop();
            body.stop();
        }

        if self.action.is_active() {
            return;
        }

        self.action = self.update_action();

        body.set_action(self.action.kind, self.action.speed, self.action.turn);

        /*
        match self.action_kind {
            ActionKind::Stop => {
                if body.speed() > 0. {
                    body.stop();
                }
            },
            ActionKind::Roam => {
                self.action = self.random_walk.update();

                body.roam(self.action.speed, self.action.turn);
            },
            ActionKind::Seek => {
                self.action = self.random_walk.update();

                body.seek(self.action.speed, self.action.turn);
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
        */
    }

    fn add_avoid(&mut self, avoid_dir: DirVector) {
        if avoid_dir.value() > 0.05 {
            let offset = 2. * avoid_dir.sin(); // * avoid_dir.value();

            self.avoid_left.set_max(offset.clamp(0., 1.));
            self.avoid_right.set_max((- offset).clamp(0., 1.));
            self.avoid_forward.set_max(avoid_dir.cos().clamp(0., 1.));
            // self.avoid_dir = avoid_dir;
        }
    }

    fn add_approach(&mut self, approach_dir: DirVector) {
        if approach_dir.value() > 0.01 {
            let offset = 2. * approach_dir.dx(); // * approach_dir.value();

            self.approach_left.set_max((- offset).clamp(0., 1.));
            self.approach_right.set_max(offset.clamp(0., 1.));
            self.approach_forward.set_max(- approach_dir.dy().clamp(-1., 0.));
            // self.approach_dir = approach_dir;

            self.seek.set_max(1.);
        }
    }

    fn avoid_forward(&self) -> f32 {
        self.avoid_forward.value()
    }

    fn avoid_left(&self) -> f32 {
        self.avoid_left.value()
    }

    fn avoid_right(&self) -> f32 {
        self.avoid_right.value()
    }

    fn forward_delta(&self) -> f32 {
        0.5 * (self.avoid_forward.value() - self.approach_forward.value() + 1.)
    }

    fn left_delta(&self) -> f32 {
        0.5 * (self.avoid_left.value() - self.approach_left.value() + 1.)
    }

    fn right_delta(&self) -> f32 {
        0.5 * (self.avoid_right.value() - self.approach_right.value() + 1.)
    }

    fn update_action(
        &mut self,
    ) -> Action {
        let move_command = self.get_move();

        if move_command == MoveCommand::Stop {
            Action::none()
        } else if self.is_last_turn {
            self.is_last_turn = false;

            self.action_run(move_command)
        } else {
            self.is_last_turn = true;

            self.action_turn(move_command)
        }
    }

    fn get_move(&self) -> MoveCommand {
        if self.avoid.is_active() {
            MoveCommand::Avoid
        } else if self.dwell.is_active() {
            MoveCommand::Dwell
        } else if self.seek.is_active() {
            MoveCommand::Seek
        } else if self.roam.is_active() {
            MoveCommand::Roam
        } else {
            MoveCommand::Stop
        }
    }

    fn action_run(
        &mut self,
        move_command: MoveCommand,
    ) -> Action {
        let speed = move_command.speed();

        let len = move_command.run_len();

        Action::new(move_command.body(), len, speed, Angle::Unit(0.))
    }

    fn action_turn(
        &mut self,
        move_command: MoveCommand,
    ) -> Action {
        //body.set_action(self.action);

        //if self.action.pre_update() {
        //    return None;
        //}

        let speed = move_command.speed();
        let len = 1.;

        let mut turn = move_command.turn();

        let avoid_forward = self.avoid_forward.value() + self.approach_forward.value();
        if avoid_forward > 0.01 && random_normal().abs() < avoid_forward {
            turn = turn_angle(2. * Self::TURN_MEAN, 3. * Self::TURN_STD);
        }

        let f = 4.;
        let approach_left = (1. - f * self.approach_right.value() - self.avoid_left.value()).max(1.0e-6);
        let approach_right = (1. - f * self.approach_left.value() - self.avoid_right.value()).max(1.0e-6);
        let p_left = approach_left / (approach_left + approach_right).max(0.01);

        // semi-brownian
        if random_uniform() <= p_left {
            let turn = Angle::unit(- turn.to_unit());

            Action::new(self.action_kind.body(), len, speed, turn)
        } else {
            Action::new(self.action_kind.body(), len, speed, turn)
        }
    }
}

impl Default for HindMove {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)] // , Event)]
pub enum MoveCommand {
    Seek,
    Avoid,
    Normal,
    Roam,
    Dwell,
    Stop,
}

impl MoveCommand {
    const ROAM_LOW : f32 = 1.;
    const ROAM_HIGH : f32 = 5.;

    const DWELL_LOW : f32 = 1.;
    const DWELL_HIGH : f32 = 1.;
    
    const ALPHA : f32 = 2.;

    const LEN_TURN : f32 = 1.;

    fn run_len(&self) -> f32 {
        match self {
            MoveCommand::Roam => {
                random_pareto(Self::ROAM_LOW, Self::ROAM_HIGH, Self::ALPHA)
            }
            MoveCommand::Avoid => {
                0.
            },
            MoveCommand::Dwell => {
                random_pareto(Self::DWELL_LOW, Self::DWELL_HIGH, Self::ALPHA)
            },
            MoveCommand::Stop => 0.,

            MoveCommand::Normal => todo!(),
            MoveCommand::Seek => {
                Self::DWELL_LOW
            },
        }
    }

    fn speed(&self) -> f32 {
        match self {
            MoveCommand::Roam => 0.5,
            MoveCommand::Avoid => 1.,
            MoveCommand::Dwell => 0.25,
            MoveCommand::Stop => 0.,

            MoveCommand::Normal => todo!(),
            MoveCommand::Seek => 0.5,
        }
    }

    fn turn(&self) -> Angle {
        match self {
            MoveCommand::Seek => turn_angle(60., 30.),
            MoveCommand::Avoid => turn_angle(60., 30.),
            MoveCommand::Normal => turn_angle(30., 15.),
            MoveCommand::Roam => turn_angle(30., 30.),
            MoveCommand::Dwell => turn_angle(60., 60.),
            MoveCommand::Stop => Angle::unit(0.),
        }
    }

    fn body(&self) -> BodyAction {
        match self {
            MoveCommand::Seek => BodyAction::Seek,
            MoveCommand::Avoid => BodyAction::Avoid,
            MoveCommand::Normal => BodyAction::Roam,
            MoveCommand::Roam => BodyAction::Roam,
            MoveCommand::Dwell => BodyAction::Dwell,
            MoveCommand::Stop => BodyAction::None,
        }
    }
}

fn turn_angle(mean: f32, std: f32) -> Angle {
    Angle::Deg(mean + (random_normal() * std).clamp(-2. * std, 2. * std))
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
    None,
    Stop,
    Roam,
    Dwell,
    Seek,
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,
}

impl ActionKind {
    fn pre_update(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Stop,
            _ => ActionKind::Roam,
        }
    }

    fn body(&self) -> BodyAction {
        match self {
            ActionKind::None => BodyAction::None,
            ActionKind::Stop => BodyAction::None,
            ActionKind::Roam => BodyAction::Roam,
            ActionKind::Dwell => BodyAction::Dwell,
            ActionKind::Seek => BodyAction::Seek,
            ActionKind::StrongAvoidLeft => BodyAction::Avoid,
            ActionKind::StrongAvoidRight => BodyAction::Avoid,
            ActionKind::StrongAvoidBoth => BodyAction::Avoid,
        }
    }

    fn explore(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Roam,
            _ => *self,
        }
    }

    fn seek(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Seek,
            ActionKind::Roam => ActionKind::Seek,
            _ => *self,
        }
    }

    fn avoid_left(&self) -> Self {
        match self {
            ActionKind::StrongAvoidLeft => ActionKind::StrongAvoidLeft,
            ActionKind::StrongAvoidRight => ActionKind::StrongAvoidBoth,
            ActionKind::StrongAvoidBoth => ActionKind::StrongAvoidBoth,
            _ => ActionKind::StrongAvoidLeft,
        }
    }

    fn avoid_right(&self) -> Self {
        match self {
            ActionKind::StrongAvoidLeft => ActionKind::StrongAvoidBoth,
            ActionKind::StrongAvoidRight => ActionKind::StrongAvoidRight,
            ActionKind::StrongAvoidBoth => ActionKind::StrongAvoidBoth,
            _ => ActionKind::StrongAvoidRight,
        }
    }
}

struct RandomWalk {
    speed: f32,

    alpha: f32,

    len_low: f32,
    len_high: f32,
    turn_len: f32,

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

    action_kind: BodyAction,
}

impl RandomWalk {
    const LEN_LOW : f32 = 1.;
    const LEN_HIGH : f32 = 5.;
    const LEN_TURN : f32 = 1.;
    const ALPHA : f32 = 2.;

    const TURN_MEAN : f32 = 60.;
    const TURN_STD : f32 = 15.;

    fn new() -> Self {
        RandomWalk {
            speed: 1.,

            len_low: Self::LEN_LOW,
            len_high: Self::LEN_HIGH,
            alpha: Self::ALPHA,
            turn_len: 1.,

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
        self.roam();

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

        self.len_low = Self::LEN_LOW;
        self.len_high = Self::LEN_HIGH;
        self.alpha = Self::ALPHA;
        self.turn_len = Self::LEN_TURN;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;

        self.action_kind = BodyAction::Roam;
    }

    fn dwell(&mut self) {
        self.speed = 0.5;

        self.len_low = Self::LEN_LOW;
        self.len_high = Self::LEN_HIGH.min(2. * Self::LEN_LOW);
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

        self.len_low = Self::LEN_HIGH;
        self.len_high = 2. * Self::LEN_HIGH;
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
        self.len_low = Self::LEN_LOW;
        self.len_high = 0.5 * Self::LEN_HIGH;
        self.alpha = 1.;

        self.turn_mean = 2. * Self::TURN_MEAN;
        self.turn_std = 3. * Self::TURN_STD;
    }

    fn _prefer(&mut self) {
        self.len_low = Self::LEN_LOW;
        self.len_high = Self::LEN_HIGH;
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
        let low = self.len_low;
        let high = self.len_high; // 4.;
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

            Action::new(self.action_kind, self.turn_len, speed, Angle::Deg(- angle))
        } else {
            self.is_last_turn = true;

            Action::new(self.action_kind, self.turn_len, speed, Angle::Deg(angle))
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

    fn pre_update(&mut self) {
        self.time -= Ticks(1).to_seconds();
    }

    fn is_active(&self) -> bool {
        self.time >= 1.0e-6
    }

    fn update_stop(&mut self) {
        self.speed -= 0.1;

        if self.speed <= 0. {
            self.time = 0.;
        }
    }

    fn _is_turn(&self) -> bool {
        self.turn.to_unit().abs() <= 1e-3
    }
}

fn update_hind_move(
    mut body: ResMut<Body>, 
    mut touch_events: InEvent<Touch>,
    // mut locomotor_events: InEvent<HindMoveCommand>,
    mut hind_locomotor: ResMut<HindMove>, 
) {
    hind_locomotor.pre_update();

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
