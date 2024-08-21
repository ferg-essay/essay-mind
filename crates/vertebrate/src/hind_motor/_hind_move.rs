use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::{Body, BodyAction, BodyPlugin};
use crate::core_motive::{Motive, Wake};
use crate::util::{Command, DecayValue, HalfLife, Seconds, Ticks, Turn};

///
/// HindMove represents MRRN/PRRN of the reticulospinal areas, in mammals
/// Gi and LPGi, which encodes forward movement and left and right braking.
/// 
/// Turns are encoded by braking the left or the right. Braking both stops
/// the animal.
/// 
pub struct _HindMove {
    commands: Command<MoveCommand>,

    next_action: ActionKind,
    action: Action,

    forward: TimeoutValue,

    left_brake: TimeoutValue,
    right_brake: TimeoutValue,
    u_turn: TimeoutValue,

    sleep: DecayValue,
}

impl _HindMove {
    const HALF_LIFE : f32 = 0.2;

    fn new() -> Self {
        Self {
            commands: Command::new(),

            action: Action::none(),
            next_action: ActionKind::Roam,

            forward: TimeoutValue::new(Self::HALF_LIFE),

            left_brake: TimeoutValue::new(Self::HALF_LIFE),
            right_brake: TimeoutValue::new(Self::HALF_LIFE),
            u_turn: TimeoutValue::new(Self::HALF_LIFE),

            sleep: DecayValue::new(Self::HALF_LIFE),
        }
    }

    ///
    /// Returns the forward velocity
    /// 
    pub fn get_forward(&self) -> f32 {
        self.forward.value()
    }

    ///
    /// Returns the strength of left brake.
    /// Returns 0 if there is no left brake.
    /// Returns 1 if left movement is fully suppressed.
    /// 
    pub fn get_left_brake(&self) -> f32 {
        self.left_brake.value()
    }

    ///
    /// Returns the strength of left brake.
    /// Returns 0 if there is no left brake.
    /// Returns 1 if left movement is fully suppressed.
    /// 
    pub fn get_right_brake(&self) -> f32 {
        self.right_brake.value()
    }

    pub fn get_u_turn(&self) -> f32 {
        self.u_turn.value()
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.next_action == ActionKind::Stop
    }

    #[inline]
    pub fn forward(&self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.send_move(MoveCommand::Forward(value));
    }

    #[inline]
    pub fn backward(&self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.send_move(MoveCommand::Backward(value));
    }

    #[inline]
    pub fn left_brake(&self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.send_move(MoveCommand::LeftBrake(value));
    }

    #[inline]
    pub fn u_turn(&self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.send_move(MoveCommand::UTurn(value));
    }

    #[inline]
    pub fn right_brake(&self, value: f32) {
        assert!(0. <= value && value <= 1.);
        
        self.send_move(MoveCommand::RightBrake(value));
    }

    #[inline]
    pub fn freeze(&self, value: f32) {
        assert!(0. <= value && value <= 1.);
        
        self.send_move(MoveCommand::LeftBrake(value));
        self.send_move(MoveCommand::RightBrake(value));
    }

    ///
    /// Set sleep/inhibition pressure. Represents the output of Snr.
    /// 
    #[inline]
    pub fn sleep(&mut self) {
        self.send_move(MoveCommand::Sleep);
    }

    #[inline]
    fn send_move(&self, command: MoveCommand) {
        self.commands.send(command);
    }

    fn pre_update(&mut self) {
        self.next_action = self.next_action.pre_update();

        self.forward.update();

        self.left_brake.update();
        self.right_brake.update();
        self.u_turn.update();

        self.sleep.update();
    }

    fn update_commands(&mut self) {
        for command in self.commands.drain() {
            self.command(&command);
        }
    }

    fn command(&mut self, event: &MoveCommand) {
        match event {
            MoveCommand::Forward(value) => {
                self.forward.set_max(*value);
            },
            MoveCommand::Backward(_value) => {
                todo!();
            },
            MoveCommand::LeftBrake(value) => {
                self.left_brake.set_max(*value);
            },
            MoveCommand::RightBrake(value) => {
                self.right_brake.set_max(*value);
            },
            MoveCommand::UTurn(value) => {
                self.u_turn.set_max(*value);
            },
            MoveCommand::Sleep => todo!(),
        }
    }

    fn update(
        &mut self, 
        body: &mut Body,
        wake: &Motive<Wake>,
    ) {
        self.pre_update();
        self.action.pre_update();

        self.update_commands();

        if ! self.action.is_active() {
            self.action = self.update_action(wake);
            self.next_action = ActionKind::None;
        }

        if self.action.is_active() {
            body.action(self.action.speed, self.action.turn, Seconds(1.));
        }
    }

    ///
    /// update_action selects the next action
    /// 
    fn update_action(
        &mut self,
        wake: &Motive<Wake>,
    ) -> Action {
        if ! wake.is_active() {
            return Action::none();
        }

        let mut forward = self.forward.value();

        let mut left = self.left_brake.value();
        let mut right = self.right_brake.value();
        let u_turn = self.u_turn.value();
        let brake = left.min(right);

        //if forward > 0. {
        //}

        forward -= brake;
        left -= brake;
        right -= brake;

        left += u_turn;
        right -= u_turn;

        // println!("Fwd {} L {} R {} B {}", forward, left, right, brake);

        if forward <= 0. {
            Action::none()
        } else if left > 0. {
            Action::new(BodyAction::Roam, 0.2, forward, Turn::unit(-0.25 * left))
        } else {
            Action::new(BodyAction::Roam, 0.2, forward, Turn::unit(0.25 * right))
        }
    }
}

impl Default for _HindMove {
    fn default() -> Self {
        Self::new()
    }
}

struct TimeoutValue {
    value: f32,
    timeout: DecayValue,
}

impl TimeoutValue {
    fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            value: 0.,
            timeout: DecayValue::new(half_life),
        }
    }

    #[inline]
    fn update(&mut self) {
        self.timeout.update();
    }
    
    #[inline]
    fn set_max(&mut self, value: f32) {
        if self.timeout.is_active() {
            // TODO: not strictly correct. Should be next_value pattern
            self.value = self.value.max(value);
        } else {
            self.value = value;
        }
        self.timeout.set(1.);
    }

    fn value(&self) -> f32 {
        if self.timeout.is_active() {
            self.value        
        } else {
            0.
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)] // , Event)]
enum MoveCommand {
    Forward(f32),
    Backward(f32),
    LeftBrake(f32),
    RightBrake(f32),
    UTurn(f32),
    Sleep,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ActionKind {
    None,
    Stop,
    Roam,
}

impl ActionKind {
    fn pre_update(&self) -> Self {
        match self {
            ActionKind::Stop => ActionKind::Stop,
            _ => ActionKind::Roam,
        }
    }

    fn _is_curtail(&self) -> bool {
        match self {
            ActionKind::Stop => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Action {
    _kind: BodyAction,
    time: f32,
    speed: f32,
    turn: Turn,
}

impl Action {
    fn new(kind: BodyAction, time: impl Into<Seconds>, speed: f32, turn: Turn) -> Self {
        Self {
            _kind: kind,
            time: time.into().0,
            speed,
            turn,
        }
    }

    fn none() -> Self {
        Action::new(BodyAction::None, 0., 0., Turn::Unit(0.))
    }

    fn pre_update(&mut self) {
        self.time -= Ticks(1).to_seconds();
    }

    fn _curtail(&mut self) {
        if self.turn.to_unit() == 0. {
            self.time = 0.;
        }
    }

    fn is_active(&self) -> bool {
        self.time >= 1.0e-6
    }
}

fn update_hind_move(
    mut body: ResMut<Body>, 
    wake: Res<Motive<Wake>>,
    mut hind_move: ResMut<_HindMove>, 
) {
    //hind_move.pre_update();

    //hind_move.update_commands();

    hind_move.update(body.get_mut(), wake.get());
}

pub struct _HindMovePlugin;

impl Plugin for _HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindMovePlugin requires BodyPlugin");

        app.init_resource::<_HindMove>();

        app.system(Tick, update_hind_move);
    }
}

#[cfg(test)]
mod test {
    use essay_ecs::core::Res;
    use mind_ecs::MindApp;

    use crate::{body::{Body, BodyPlugin}, hind_motor::_HindMovePlugin, util::Point, world::WorldPlugin};

    #[test]
    fn test_default() {
        let mut app = MindApp::test();
        app.plugin(WorldPlugin::new(7, 13));
        app.plugin(BodyPlugin::new());
        app.plugin(_HindMovePlugin);

        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));

        for _ in 0..100 {
            app.tick();
        }

        assert_eq!(Point(0.5, 0.5), app.eval(|x: Res<Body>| x.pos()));
    }
}
