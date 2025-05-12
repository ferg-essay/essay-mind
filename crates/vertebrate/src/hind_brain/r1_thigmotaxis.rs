use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_brain::{lateral_line::{LateralLine2Plugin, Segment}, r1_thigmotaxis_artr::{update_thigmaxis_artr, ThigmotaxisArtr}, HindMovePlugin}, util::{DecayValue, HalfLife, Seconds, Ticks, Turn}};

use super::{lateral_line::LateralLine, HindMove};

fn update_thigmaxis_direct(
    mut hind_move: ResMut<HindMove>,
    mut thigmotaxis: ResMut<Thigmotaxis>,
    lateral_line: Res<LateralLine>,
) {
    thigmotaxis.update();

    let mut left_turn = None;
    let mut right_turn = None;

    let left_head = lateral_line.max(Segment::HeadLeft);
    let left_trunk = lateral_line.max(Segment::TailLeft);

    if left_head > 0. || left_trunk > 0. {
        if ! thigmotaxis.right.is_active() {
            thigmotaxis.left.set_max(1.);
        } else {
            let inhibited_value = thigmotaxis.inhibited_value;
            thigmotaxis.right.set_max(inhibited_value);
        }
    }

    if thigmotaxis.left.is_active()
    && left_head < Thigmotaxis::MAX_THRESHOLD
    && (left_head < left_trunk || left_head == 0.) {
        left_turn = Some(- thigmotaxis.turn(left_head));
        //hind_move.ante().thigmotaxis(Turn::Unit(0.95));
    }

    let right_head = lateral_line.max(Segment::HeadRight);
    let right_trunk = lateral_line.max(Segment::TailRight);

    if right_head > 0. || right_trunk > 0. {
        if ! thigmotaxis.left.is_active() {
            thigmotaxis.right.set_max(1.);
        } else {
            let inhibited_value = thigmotaxis.inhibited_value;
            thigmotaxis.right.set_max(inhibited_value);
        }
    }

    if thigmotaxis.right.is_active()
    && right_head < Thigmotaxis::MAX_THRESHOLD
    && (right_head < right_trunk || right_head == 0.) {
        right_turn = Some(thigmotaxis.turn(right_head));
        //hind_move.ante().thigmotaxis(Turn::Unit(0.05));
    }

    if left_turn.is_some() && right_turn.is_some() {
        // decide left/right bias or disable thigmotaxis
    } else if let Some(turn) = left_turn {
        hind_move.ante().thigmotaxis(turn);
    } else if let Some(turn) = right_turn {
        hind_move.ante().thigmotaxis(turn);
    }
}

#[derive(Default)]
pub struct Thigmotaxis {
    // max turn rate
    turn: Turn, 

    // inhibited value
    inhibited_value: f32,

    // memory of thigmotaxis side.
    left: DecayValue,
    right: DecayValue,
}

impl Thigmotaxis {
    const MAX_THRESHOLD: f32 = 0.4;

    fn new(plugin: &HindThigmotaxisPlugin) -> Self {
        //let half_life = plugin.memory_time;
        let half_life = Ticks(4);

        Thigmotaxis {
            turn: plugin.turn,
            inhibited_value: plugin.inhibited_value,

            left: DecayValue::new(half_life),
            right: DecayValue::new(half_life),
        }
    }

    pub fn is_active(&self) -> bool {
        self.left.is_active() || self.right.is_active()
    }

    pub fn active_value(&self) -> f32 {
        self.left.active_value().max(self.right.active_value())
    }

    // todo: currently used as a UI hack
    pub(super) fn set_value(&mut self, value: f32) {
        self.left.set(value);
        self.right.set(value);
    }

    // todo: currently used as a UI hack
    pub(super) fn set_left(&mut self, value: f32) {
        self.left.set(value);
    }

    // todo: currently used as a UI hack
    pub(super) fn set_right(&mut self, value: f32) {
        self.right.set(value);
    }

    pub fn left_active(&self) -> bool {
        self.left.is_active()
    }

    pub fn right_active(&self) -> bool {
        self.right.is_active()
    }

    fn turn(&self, head: f32) -> Turn {
        Turn::Unit(self.turn.to_unit() * (Thigmotaxis::MAX_THRESHOLD - head))
    }

    fn update(&mut self)  {
        self.left.update();
        self.right.update();
    }
}

pub struct HindThigmotaxisPlugin {
    pub(super) is_enable: bool,
    pub(super) memory_time: HalfLife,
    pub(super) inhibited_value: f32,
    pub(super) turn: Turn,
    pub(super) timeout: Option<Ticks>,
    pub(super) timeout_recover: Option<Ticks>,

    strategy: ThigmotaxisStrategy,
}

impl HindThigmotaxisPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }

    pub fn memory_time(&mut self, timeout: impl Into<HalfLife>) -> &mut Self {
        self.memory_time = timeout.into();

        self
    }

    pub fn turn(&mut self, turn: impl Into<Turn>) -> &mut Self {
        self.turn = turn.into();

        self
    }


    pub fn strategy(&mut self, strategy: ThigmotaxisStrategy) -> &mut Self {
        self.strategy = strategy;

        self
    }

    // lateral inhibition value when both sides have lateral line values
    pub fn inhibited_value(&mut self, value: f32) -> &mut Self {
        self.inhibited_value = value;

        self
    }

    pub fn timeout(&mut self, ticks: impl Into<Ticks>) -> &mut Self {
        self.timeout = Some(ticks.into());

        self
    }

    pub fn timeout_recover(&mut self, ticks: impl Into<Ticks>) -> &mut Self {
        self.timeout_recover = Some(ticks.into());

        self
    }
}

impl Default for HindThigmotaxisPlugin {
    fn default() -> Self {
        Self { 
            is_enable: true, 
            memory_time: Seconds(1.).into(),
            turn: Turn::Unit(0.1),
            inhibited_value: 1.,
            strategy: ThigmotaxisStrategy::Artr,
            timeout: None,
            timeout_recover: None,
        }
    }
}

impl Plugin for HindThigmotaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "HindThigmotaxis requires HindLocomotion");
        assert!(app.contains_plugin::<LateralLine2Plugin>(), "HindThigmotaxis requires LateralLine");

        if self.is_enable {
            match self.strategy {
                ThigmotaxisStrategy::Direct => {
                    app.insert_resource(Thigmotaxis::new(&self));
                    app.system(Tick, update_thigmaxis_direct);
                }
                ThigmotaxisStrategy::Artr => {
                    app.insert_resource(ThigmotaxisArtr::new(&self));
                    app.system(Tick, update_thigmaxis_artr);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThigmotaxisStrategy {
    Direct,
    Artr
}
