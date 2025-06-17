use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_brain::{
        lateral_line::{LateralLine, LateralLine2Plugin, Segment},
        HindMove, HindMovePlugin
    }, subpallium::{StriatumExclusive, StriatumId, StriatumTimeout}, util::{DecayValue, HalfLife, Seconds, Ticks, Turn}
};

fn update_thigmotaxis_tectum(
    mut hind_move: ResMut<HindMove>,
    mut thigmotaxis: ResMut<ThigmotaxisTectum>,
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

    /*
    if thigmotaxis.is_active() {
        //thigmotaxis_ui.set_value(thigmotaxis.active_value());
    }

    thigmotaxis_ui.set_left(if turn_left.is_some() { 1. } else { 0. });
    thigmotaxis_ui.set_right(if turn_right.is_some() { 1. } else { 0. });
    */
}

pub struct ThigmotaxisTectum {
    // memory of thigmotaxis side.
    left: ThigmotaxisSide,
    right: ThigmotaxisSide,
    exclusive: StriatumExclusive,
}

impl ThigmotaxisTectum {
    pub(super) fn new(plugin: &TectumThigmotaxisPlugin) -> Self {
        let mut exclusive = StriatumExclusive::default();
        
        ThigmotaxisTectum {
            left: ThigmotaxisSide::new(Side::Left, &mut exclusive, plugin), // half_life, plugin.turn),
            right: ThigmotaxisSide::new(Side::Right, &mut exclusive, plugin),
            exclusive,
        }
    }

    pub fn is_active(&self) -> bool {
        self.left.memory.is_active() || self.right.memory.is_active()
    }

    pub fn active_left(&self) -> bool {
        self.left.memory.is_active()
    }

    pub fn active_right(&self) -> bool {
        self.right.memory.is_active()
    }

    pub fn active_value(&self) -> f32 {
        self.left.memory.active_value().max(self.right.memory.active_value())
    }

    fn update(
        &mut self, 
        lateral_line: &LateralLine,
        tick: &AppTick,
    )  {
        self.exclusive.update(tick);
        /*
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

impl Default for ThigmotaxisTectum {
    fn default() -> Self {
        Self {
            left: ThigmotaxisSide::default(Side::Left),
            right: ThigmotaxisSide::default(Side::Right),
            exclusive: StriatumExclusive::default(),
        }
    }
}

struct ThigmotaxisSide {
    side: Side,

    ll_target: f32,
    turn_max: f32,

    // memory of thigmotaxis side.
    memory: DecayValue,

    id: StriatumId,
    timeout: StriatumTimeout,
}

impl ThigmotaxisSide {
    fn new(
        side: Side,
        exclusive: &mut StriatumExclusive,
        plugin: &TectumThigmotaxisPlugin,
    ) -> Self {
        let half_life = plugin.memory_time;
        let mut timeout = StriatumTimeout::new();

        if let Some(ticks) = plugin.timeout {
            timeout = timeout.ltd(ticks);
        }

        if let Some(ticks) = plugin.timeout_recover {
            timeout = timeout.decay(ticks);
        }

        let id = exclusive.alloc_id();

        Self {
            side,
            ll_target: 0.6,
            turn_max: plugin.turn.to_unit(),
            memory: DecayValue::new(half_life),

            id,
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
            id: StriatumId::default(),
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

pub struct TectumThigmotaxisPlugin {
    pub(super) is_enable: bool,
    pub(super) memory_time: HalfLife,
    pub(super) inhibited_value: f32,
    pub(super) turn: Turn,
    pub(super) timeout: Option<Ticks>,
    pub(super) timeout_recover: Option<Ticks>,
}

impl TectumThigmotaxisPlugin {
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

impl Default for TectumThigmotaxisPlugin {
    fn default() -> Self {
        Self { 
            is_enable: true, 
            memory_time: Seconds(1.).into(),
            turn: Turn::Unit(0.1),
            inhibited_value: 1.,
            timeout: None,
            timeout_recover: None,
        }
    }
}

impl Plugin for TectumThigmotaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "TectumThigmotaxis requires HindLocomotion");
        assert!(app.contains_plugin::<LateralLine2Plugin>(), "TectumThigmotaxis requires LateralLine");

        if self.is_enable {
            app.insert_resource(ThigmotaxisTectum::new(&self));
            app.system(Tick, update_thigmotaxis_tectum);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Left,
    Right
}