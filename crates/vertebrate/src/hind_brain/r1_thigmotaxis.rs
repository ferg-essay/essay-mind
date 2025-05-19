use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{hind_brain::{lateral_line::{LateralLine2Plugin, Segment}, r1_thigmotaxis_artr::{update_thigmaxis_artr, ThigmotaxisArtr}, HindMovePlugin}, subpallium::{StriatumExclusive, StriatumId, StriatumTimeout}, util::{DecayValue, HalfLife, Seconds, Ticks, Turn}};

use super::{lateral_line::LateralLine, r2_artr::Side, HindMove};

fn update_thigmaxis_direct(
    mut hind_move: ResMut<HindMove>,
    mut thigmotaxis: ResMut<Thigmotaxis>,
    lateral_line: Res<LateralLine>,
    tick: Res<AppTick>,
) {
    thigmotaxis.update(lateral_line.as_ref(), tick.as_ref());

    let turn_left = thigmotaxis.left.turn(lateral_line.as_ref());
    let turn_right = thigmotaxis.right.turn(lateral_line.as_ref());

    if turn_left.is_some() && turn_right.is_some() {
        // todo
    } else if let Some(turn) = turn_left {
        hind_move.turn(turn);
    } else if let Some(turn) = turn_right {
        hind_move.turn(turn);
    }

    // thigmotaxis.update();

    //let mut left_turn = None;
    //let mut right_turn = None;

    let left_head = lateral_line.max(Segment::HeadLeft);
    let left_trunk = lateral_line.max(Segment::TailLeft);

    /*
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
    */
}

pub struct Thigmotaxis {
    // max turn rate
    // turn: Turn, 

    // memory of thigmotaxis side.
    left: ThigmotaxisSide,
    right: ThigmotaxisSide,

    ui_left: DecayValue,
    ui_right: DecayValue,

}

impl Default for Thigmotaxis {
    fn default() -> Self {
        let left = ThigmotaxisSide::default(Side::Left);
        let right = ThigmotaxisSide::default(Side::Right);

        Self { 
            left,
            right,
            ui_left: Default::default(), 
            ui_right: Default::default() 
        }
    }
}

impl Thigmotaxis {
    const MAX_THRESHOLD: f32 = 0.4;

    fn new(plugin: &HindThigmotaxisPlugin) -> Self {
        //let half_life = plugin.memory_time;
        let half_life = Ticks(4);

        Thigmotaxis {
            left: ThigmotaxisSide::new(Side::Left, plugin), // half_life, plugin.turn),
            right: ThigmotaxisSide::new(Side::Right, plugin),
            // exclusive,

            ui_left: DecayValue::new(half_life),
            ui_right: DecayValue::new(half_life),
        }
    }

    pub fn is_active(&self) -> bool {
        self.ui_left.is_active() || self.ui_right.is_active()
    }

    pub fn active_value(&self) -> f32 {
        self.ui_left.active_value().max(self.ui_right.active_value())
    }

    // todo: currently used as a UI hack
    pub(super) fn _set_value(&mut self, value: f32) {
        self.ui_left.set(value);
        self.ui_right.set(value);
    }

    // todo: currently used as a UI hack
    pub(super) fn set_left(&mut self, value: f32) {
        self.ui_left.set(value);
    }

    // todo: currently used as a UI hack
    pub(super) fn set_right(&mut self, value: f32) {
        self.ui_right.set(value);
    }

    pub fn left_active(&self) -> bool {
        self.ui_left.is_active()
    }

    pub fn right_active(&self) -> bool {
        self.ui_right.is_active()
    }

    fn turn(&self, head: f32) -> Turn {
        // Turn::Unit(self.turn.to_unit() * (Thigmotaxis::MAX_THRESHOLD - head))
        todo!()
    }

    fn update(
        &mut self, 
        lateral_line: &LateralLine,
        tick: &AppTick,
    )  {
        self.ui_left.update();
        self.ui_right.update();
        /*
        self.exclusive.update(tick);
        
        if self.exclusive.is_active(self.left.id, tick) {
            if self.left.update(lateral_line, tick) {
                self.exclusive.update_id(self.left.id, tick);
            }
        } else if self.exclusive.is_active(self.right.id, tick) {
            if self.right.update(lateral_line, tick) {
                self.exclusive.update_id(self.right.id, tick);
            }
        } else if self.exclusive.is_idle() {

        }
        */
    }
}

struct ThigmotaxisSide {
    side: Side,

    ll_target: f32,
    turn_max: f32,

    // memory of thigmotaxis side.
    memory: DecayValue,

    // id: StriatumId,
    timeout: StriatumTimeout,
}

impl ThigmotaxisSide {
    fn new(
        side: Side,
        plugin: &HindThigmotaxisPlugin,
    ) -> Self {
        let half_life = plugin.memory_time;
        let mut timeout = StriatumTimeout::new();

        if let Some(ticks) = plugin.timeout {
            timeout = timeout.ltd(ticks);
        }

        if let Some(ticks) = plugin.timeout_recover {
            timeout = timeout.decay(ticks);
        }

        // let id = exclusive.alloc_id();

        Self {
            side,
            ll_target: 0.6,
            turn_max: plugin.turn.to_unit(),
            memory: DecayValue::new(half_life),

            // id,
            timeout,
        }
    }

    fn default(
        side: Side,
    ) -> Self {
        let half_life = HalfLife::default();
        Self {
            side,
            ll_target: 0.5,
            turn_max: Turn::Unit(0.25).to_unit(),
            memory: DecayValue::new(half_life),
            timeout: StriatumTimeout::new(),
            // id: StriatumId::default(),
        }
    }

    fn update(
        &mut self, 
        lateral_line: &LateralLine,
        tick: &AppTick,
    ) -> bool {
        self.memory.update();

        let head = lateral_line.max(head(self.side));

        if head > 0. || self.memory.is_active() {
            if ! self.timeout.is_active(tick) {
                self.memory.set(0.);
            } else if head > 0. {
                self.memory.set_max(1.);
            }
        }

        self.memory.is_active()
    }

    // turn pressure
    fn turn(&self, lateral_line: &LateralLine) -> Option<Turn> {
        if self.memory.is_active() {
            let head = lateral_line.max(head(self.side));

            let delta = self.ll_target - head;

            let turn = match self.side {
                Side::Left => Turn::Unit(- 0.5 * delta * self.turn_max),
                Side::Right => Turn::Unit(0.5 * delta * self.turn_max),
            };

            Some(turn)
        } else {
            None
        }
    }
}

fn head(side: Side) -> Segment {
    match side {
        Side::Left => Segment::HeadLeft,
        Side::Right => Segment::HeadRight,
    }    
}

fn _tail(side: Side) -> Segment {
    match side {
        Side::Left => Segment::TailLeft,
        Side::Right => Segment::TailRight,
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
