use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::{Body, BodyAction, BodyPlugin};
use crate::core_motive::{Motive, Wake};
use crate::util::{Command, DecayValue, DirVector, Seconds, Ticks, Turn};
use util::random::{random_normal, random_pareto, random_uniform};

use super::move_hind::{Action, ActionKind};

// Karpenko et al 2020 - ARTR oscillator 20s period

///
/// HindMove represents hindbrain motor areas, particularly reticulospinal
/// areas (R.rs).
/// 
/// Zebrafish R.rs contains Brownian search oscillators (ARTR) as well as 
/// stimulus-response escape neurons (giant Mauthner cells).
/// 
/// HindMove actions have a time length that they execute, and will ignore
/// higher level commands until the execution completes, simulating fish tail
/// movement, which have a slow time period compared to fast neuron cycles.
/// 
/// Movement mode and turn directions are independent.
/// 
/// Turn directions are encoded as either approach or avoid. Avoidance 1.0
/// is a wall next to the first. An open area is 0.0.
/// 
/// 
pub struct OscillatorArs {
    move_commands: Command<MoveCommand>,
    turn_commands: Command<TurnCommand>,

    next_action: ActionKind,
    action: Action,

    avoid: DecayValue,
    roam: DecayValue,
    dwell: DecayValue,
    seek: DecayValue,

    sleep: DecayValue,

    approach_left: DecayValue,
    approach_right: DecayValue,
    approach_forward: DecayValue,

    avoid_left: DecayValue,
    avoid_right: DecayValue,
    avoid_forward: DecayValue,

    is_last_turn: bool,
}

impl OscillatorArs {
    const HALF_LIFE : f32 = 0.2;

    const _TURN_MEAN : f32 = 60.;
    const _TURN_STD : f32 = 15.;

    const UTURN_MEAN : f32 = 160.;
    const UTURN_STD : f32 = 15.;

    pub(super) fn new() -> Self {
        Self {
            move_commands: Command::new(),
            turn_commands: Command::new(),

            action: Action::none(),
            next_action: ActionKind::Roam,

            avoid: DecayValue::new(Self::HALF_LIFE),
            roam: DecayValue::new(Self::HALF_LIFE),
            dwell: DecayValue::new(Self::HALF_LIFE),
            seek: DecayValue::new(Self::HALF_LIFE),
            sleep: DecayValue::new(Self::HALF_LIFE),

            approach_left: DecayValue::new(Self::HALF_LIFE),
            approach_right: DecayValue::new(Self::HALF_LIFE),
            approach_forward: DecayValue::new(Self::HALF_LIFE),

            avoid_left: DecayValue::new(Self::HALF_LIFE),
            avoid_right: DecayValue::new(Self::HALF_LIFE),
            avoid_forward: DecayValue::new(Self::HALF_LIFE),

            is_last_turn: false,
        }
    }

    ///
    /// Returns the strength of left avoidance, such as a wall on the left.
    /// Returns 0 if there is nothing to avoid.
    /// Returns 1 for a wall immediately to the left.
    /// 
    pub fn get_avoid_left(&self) -> f32 {
        self.avoid_left.value()
    }

    ///
    /// Returns the strength of left avoidance, such as a wall on the left.
    /// Returns 0 if there is nothing to avoid.
    /// Returns 1 for a wall immediately to the left.
    /// 
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
        self.next_action == ActionKind::None
    }

    ///
    /// Stops any current motion, curtailing the current action.
    /// 
    #[inline]
    pub fn stop(&self) {
        self.send_move(MoveCommand::Stop);
    }

    ///
    /// Set sleep/inhibition pressure. Represents the output of Snr.
    /// 
    #[inline]
    pub fn sleep(&mut self) {
        self.send_move(MoveCommand::Sleep);
    }

    ///
    /// Move in a roaming mode, which has few turns and higher speed.
    /// 
    #[inline]
    pub fn roam(&self) {
        self.send_move(MoveCommand::Roam);
    }


    ///
    /// Move in a dwell mode, which has many turns and lower speed, for 
    /// area restricted search.
    /// 
    #[inline]
    pub fn dwell(&self) {
        self.send_move(MoveCommand::Dwell);
    }

    #[inline]
    pub fn avoid(&self) {
        self.send_move(MoveCommand::Avoid);
    }

    #[inline]
    fn send_move(&self, command: MoveCommand) {
        self.move_commands.send(command);
    }

    #[inline]
    pub fn turn(&self, command: TurnCommand) {
        self.turn_commands.send(command);
    }

    fn pre_update(&mut self) {
        self.next_action = self.next_action.pre_update();

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
        self.sleep.update();
    }

    fn update_move_commands(&mut self) {
        for command in self.move_commands.drain() {
            self.move_command(&command);
        }
    }

    fn move_command(&mut self, event: &MoveCommand) {
        /*
        match event {
            // explore/speed modes
            MoveCommand::SeekRoam => {
                self.next_action = self.next_action.explore();
                self.roam.set_max(1.);
            }
            MoveCommand::SeekDwell => {
                self.next_action = self.next_action.explore();
                self.roam.set_max(1.);
            }
            MoveCommand::Avoid => {
                self.next_action = self.next_action.explore();
                self.avoid.set_max(1.);
            }
            MoveCommand::Roam => {
                self.next_action = self.next_action.explore();
                self.roam.set_max(1.);
            }
            MoveCommand::Dwell => {
                self.next_action = self.next_action.explore();
                self.dwell.set_max(1.);
            }
            MoveCommand::Stop => {
                self.next_action = ActionKind::Stop;
                //self.random_walk.stop();
            }
            MoveCommand::Sleep => {
                self.next_action = ActionKind::Stop;
                //self.random_walk.stop();
            }
        }
        */
    }

    fn update_turn_commands(&mut self) {
        for command in self.turn_commands.drain() {
            self.turn_command(command);
        }
    }

    fn turn_command(&mut self, event: TurnCommand) {
        /*
        match event {
            // collision/escape - strong avoid events
            TurnCommand::StrongAvoidLeft => {
                self.avoid_left.set(1.);
                self.next_action = self.next_action.avoid_left();
            }
            TurnCommand::StrongAvoidRight => {
                self.avoid_right.set(1.);
                self.next_action = self.next_action.avoid_right();
            }
            TurnCommand::StrongAvoidBoth => {
                self.avoid_left.set(1.);
                self.avoid_right.set(1.);
                self.avoid_forward.set(1.);
                self.next_action = self.next_action.avoid_left();
                self.next_action = self.next_action.avoid_right();
            }
            TurnCommand::AvoidLeft(v) => {
                self.avoid_left.set_max(v);

                if v > 0.75 {
                    self.next_action = self.next_action.avoid_left();
                }
            }
            TurnCommand::AvoidRight(v) => {
                self.avoid_right.set_max(v);

                if v > 0.75 {
                    self.next_action = self.next_action.avoid_right();
                }
            }

            // taxis gradient
            TurnCommand::AvoidVector(vector) => {
                self.next_action = self.next_action.seek();
                self.add_avoid(vector)
            },
            
            TurnCommand::ApproachVector(vector) => {
                self.next_action = self.next_action.seek();
                self.add_approach(vector)
            },

            TurnCommand::AvoidUTurn => {
                // self.action_kind = self.action_kind.explore();
                self.avoid_forward.set_max(1.);
                self.next_action = self.next_action.avoid_left();
                self.next_action = self.next_action.avoid_right();
            }
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

    fn update(
        &mut self, 
        body: &mut Body,
        wake: &Motive<Wake>,
    ) {
        /*
        self.action.pre_update();

        if self.next_action.is_curtail() {
            self.action.curtail();
        }

        if ! self.action.is_active() {
            self.action = self.update_action(wake);
            self.next_action = ActionKind::None;
        }

        body.action(self.action.speed, self.action.turn, 1., Seconds(1.));
        */
    }

    ///
    /// update_action selects the next action
    /// 
    fn update_action(
        &mut self,
        wake: &Motive<Wake>,
    ) -> Action {
        /*
        let move_command = self.get_move();

        if move_command == MoveCommand::Stop {
            if wake.is_active() {
                Action::new(BodyAction::None, 0.25, 0., Turn::unit(0.))
            } else {
                Action::new(BodyAction::Sleep, 1., 0., Turn::unit(0.))
            }
        } else if self.sleep.is_active() {
            Action::new(BodyAction::Sleep, 1., 0., Turn::unit(0.))
        } else if self.is_last_turn {
            self.is_last_turn = false;

            self.action_run(move_command)
        } else {
            self.is_last_turn = true;

            self.action_turn(move_command)
        }
        */
        todo!()
    }

    fn get_move(&self) -> MoveCommand {
        todo!();
        /*
        if self.next_action == ActionKind::Stop {
            MoveCommand::Stop
        } else if self.avoid.is_active() {
            MoveCommand::Avoid
        } else if self.seek.is_active() {
            if self.dwell.is_active() {
                MoveCommand::SeekDwell
            } else {
                MoveCommand::SeekRoam
            }
        } else if self.dwell.is_active() {
            MoveCommand::Dwell
        } else if self.roam.is_active() {
            MoveCommand::Roam
        } else if self.sleep.is_active() {
            MoveCommand::Sleep
        } else {
            MoveCommand::Stop
        }
        */
    }

    pub(super) fn next_action(&mut self) -> Action {

        // semi-brownian
        if random_uniform() <= 0.5 {
            Action::new(ActionKind::Roam, 0.5, Turn::Deg(0.), Seconds(1.))
        } else if random_uniform() <= 0.5 {
            let turn = Turn::Deg(-30.);

            Action::new(ActionKind::Roam, 0.5, turn, Seconds(1.))
        } else {
            let turn = Turn::Deg(30.);

            Action::new(ActionKind::Roam, 0.5, turn, Seconds(1.))
        }
    }

    pub(super) fn next_turn(&mut self) -> Option<Turn> {
        // semi-brownian
        if random_uniform() <= 0.5 {
            Some(Turn::Deg(0.))
        } else if random_uniform() <= 0.5 {
            Some(Turn::Deg(-30.))
        } else {
            Some(Turn::Deg(30.))
        }
    }


    ///
    /// "run" is a straight movement in a run and tumble search
    /// 
    fn action_run(
        &mut self,
        move_command: MoveCommand,
    ) -> Action {
        let speed = move_command.speed();

        let len = move_command.run_len();

        Action::new(move_command.kind(), speed, Turn::Unit(0.), Seconds(len as f32))
    }

    ///
    /// "turn" is the turn movement in a run and tumble search
    /// 
    fn action_turn(
        &mut self,
        move_command: MoveCommand,
    ) -> Action {
        let speed = move_command.speed();
        let len = 1.;

        let mut turn = move_command.turn();

        let avoid_forward = self.avoid_forward.value() + self.approach_forward.value();
        if avoid_forward > 0.01 && random_normal().abs() < avoid_forward {
            turn = turn_angle(Self::UTURN_MEAN, Self::UTURN_STD);
        }

        let f = 4.;
        let approach_left = (1. - f * self.approach_right.value() - self.avoid_left.value()).max(1.0e-6);
        let approach_right = (1. - f * self.approach_left.value() - self.avoid_right.value()).max(1.0e-6);
        let p_left = approach_left / (approach_left + approach_right).max(0.01);

        // semi-brownian
        if random_uniform() <= p_left {
            let turn = Turn::unit(- turn.to_unit());

            Action::new(move_command.kind(), speed, turn, Seconds(len as f32))
        } else {
            Action::new(move_command.kind(), speed, turn, Seconds(len as f32))
        }
    }
}

impl Default for OscillatorArs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)] // , Event)]
enum MoveCommand {
    SeekRoam,
    SeekDwell,
    Avoid,
    Roam,
    Dwell,
    Sleep,
    Stop,
}

impl MoveCommand {
    const ROAM_LOW : f32 = 1.;
    const ROAM_HIGH : f32 = 5.;

    const DWELL_LOW : f32 = 1.;
    const DWELL_HIGH : f32 = 1.;

    const AVOID_LOW : f32 = 1.;
    
    const SLEEP : f32 = 5.;

    const ALPHA : f32 = 2.;

    fn run_len(&self) -> f32 {
        match self {
            MoveCommand::Roam => {
                random_pareto(Self::ROAM_LOW, Self::ROAM_HIGH, Self::ALPHA)
            }
            MoveCommand::Avoid => {
                Self::AVOID_LOW
            },
            MoveCommand::Dwell => {
                random_pareto(Self::DWELL_LOW, Self::DWELL_HIGH, Self::ALPHA)
            },
            MoveCommand::Stop => 0.,
            MoveCommand::Sleep => Self::SLEEP,

            MoveCommand::SeekRoam => {
                Self::ROAM_LOW
            },
            MoveCommand::SeekDwell => {
                Self::DWELL_LOW
            },
        }
    }

    fn speed(&self) -> f32 {
        match self {
            MoveCommand::Roam => 0.5,
            MoveCommand::Avoid => 1.,
            MoveCommand::Dwell => 0.4,
            MoveCommand::Stop => 0.,
            MoveCommand::Sleep => 0.,

            MoveCommand::SeekRoam => 0.5,
            MoveCommand::SeekDwell => 0.4,
        }
    }

    fn turn(&self) -> Turn {
        match self {
            MoveCommand::SeekRoam => turn_angle(60., 30.),
            MoveCommand::SeekDwell => turn_angle(60., 30.),
            MoveCommand::Avoid => turn_angle(90., 30.),
            MoveCommand::Roam => turn_angle(30., 30.),
            MoveCommand::Dwell => turn_angle(60., 60.),
            MoveCommand::Stop => Turn::unit(0.),
            MoveCommand::Sleep => Turn::unit(0.),
        }
    }

    fn kind(&self) -> ActionKind {
        /*
        match self {
            MoveCommand::SeekRoam => BodyAction::Seek,
            MoveCommand::SeekDwell => BodyAction::Seek,
            MoveCommand::Avoid => BodyAction::Avoid,
            MoveCommand::Roam => BodyAction::Roam,
            MoveCommand::Dwell => BodyAction::Dwell,
            MoveCommand::Sleep => BodyAction::Sleep,
            MoveCommand::Stop => BodyAction::None,
        }
        */

        match self {
            MoveCommand::SeekRoam => ActionKind::Roam,
            MoveCommand::SeekDwell => ActionKind::Roam,
            MoveCommand::Avoid => ActionKind::Roam,
            MoveCommand::Roam => ActionKind::Roam,
            MoveCommand::Dwell => ActionKind::Roam,
            MoveCommand::Sleep => ActionKind::Roam,
            MoveCommand::Stop => ActionKind::None,
        }
    }
}

fn turn_angle(mean: f32, std: f32) -> Turn {
    Turn::Deg(mean + (random_normal() * std).clamp(-2. * std, 2. * std))
}

#[derive(Clone, Copy, Debug)] // , Event)]
pub enum TurnCommand {
    // escape/collision
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,

    AvoidLeft(f32),
    AvoidRight(f32),
    // taxis gradient
    ApproachVector(DirVector),
    AvoidVector(DirVector),

    AvoidUTurn,
}

impl ActionKind {
    fn pre_update(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::None,
            _ => ActionKind::Roam,
        }
    }

    fn explore(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::Roam,
            _ => *self,
        }
    }

    fn seek(&self) -> Self {
        match self {
            ActionKind::None => ActionKind::Seek,
            ActionKind::Roam => ActionKind::Seek,
            _ => *self,
        }
    }

    /*
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

    fn is_curtail(&self) -> bool {
        match self {
            ActionKind::Stop => true,
            ActionKind::StrongAvoidLeft => true,
            ActionKind::StrongAvoidRight => true,
            ActionKind::StrongAvoidBoth => true,
            _ => false,
        }
    }
    */
}

fn update_hind_move(
    mut body: ResMut<Body>, 
    wake: Res<Motive<Wake>>,
    mut hind_move: ResMut<OscillatorArs>, 
) {
    hind_move.pre_update();

    if body.is_collide_left() {
        hind_move.turn_command(TurnCommand::StrongAvoidLeft);
    }

    if body.is_collide_right() {
        hind_move.turn_command(TurnCommand::StrongAvoidRight);
    }

    hind_move.update_turn_commands();
    hind_move.update_move_commands();

    hind_move.update(body.get_mut(), wake.get());
}

pub struct LevyWalkPlugin;