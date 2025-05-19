use essay_ecs::core::{Res, ResMut};
use mind_ecs::AppTick;

use crate::{
    hind_brain::lateral_line::Segment, subpallium::StriatumTimeout, util::{DecayValue, HalfLife, Turn}
};

use super::{
    lateral_line::LateralLine, 
    r1_thigmotaxis::{HindThigmotaxisPlugin, Thigmotaxis}, 
    r2_artr::Side, 
    HindMove
};

pub(super) fn update_thigmaxis_artr(
    mut hind_move: ResMut<HindMove>,
    mut thigmotaxis: ResMut<ThigmotaxisArtr>,
    mut thigmotaxis_ui: ResMut<Thigmotaxis>,
    lateral_line: Res<LateralLine>,
    tick: Res<AppTick>,
) {
    thigmotaxis.update(hind_move.get(), lateral_line.as_ref(), tick.as_ref());

    let turn_left = thigmotaxis.left.turn(lateral_line.as_ref());
    let turn_right = thigmotaxis.right.turn(lateral_line.as_ref());

    if turn_left.is_some() && turn_right.is_some() {
        // todo
    } else if let Some(turn) = turn_left {
        hind_move.turn(turn);
    } else if let Some(turn) = turn_right {
        hind_move.turn(turn);
    }

    if thigmotaxis.is_active() {
        //thigmotaxis_ui.set_value(thigmotaxis.active_value());
    }

    thigmotaxis_ui.set_left(if turn_left.is_some() { 1. } else { 0. });
    thigmotaxis_ui.set_right(if turn_right.is_some() { 1. } else { 0. });
}

pub struct ThigmotaxisArtr {
    // memory of thigmotaxis side.
    left: ThigmotaxisSide,
    right: ThigmotaxisSide,
}

impl ThigmotaxisArtr {
    pub(super) fn new(plugin: &HindThigmotaxisPlugin) -> Self {
        ThigmotaxisArtr {
            left: ThigmotaxisSide::new(Side::Left, plugin), // half_life, plugin.turn),
            right: ThigmotaxisSide::new(Side::Right, plugin),
        }
    }

    fn is_active(&self) -> bool {
        self.left.memory.is_active() || self.right.memory.is_active()
    }

    fn _active_value(&self) -> f32 {
        self.left.memory.active_value().max(self.right.memory.active_value())
    }

    fn update(
        &mut self, 
        hind_move: &HindMove, 
        lateral_line: &LateralLine,
        tick: &AppTick,
    )  {
        self.left.update(hind_move, lateral_line, tick);
        self.right.update(hind_move, lateral_line, tick);
    }
}

impl Default for ThigmotaxisArtr {
    fn default() -> Self {
        Self {
            left: ThigmotaxisSide::default(Side::Left),
            right: ThigmotaxisSide::default(Side::Right),
        }
    }
}

struct ThigmotaxisSide {
    side: Side,

    ll_target: f32,
    turn_max: f32,

    // memory of thigmotaxis side.
    memory: DecayValue,

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

        Self {
            side,
            ll_target: 0.5,
            turn_max: plugin.turn.to_unit(),
            memory: DecayValue::new(half_life),
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
        }
    }

    fn update(
        &mut self, 
        hind_move: &HindMove, 
        lateral_line: &LateralLine,
        tick: &AppTick,
    ) {
        self.memory.update();

        if hind_move.artr().side().map_or(false, |side| side == self.side)
        || self.memory.is_active() {
            let head = lateral_line.max(head(self.side));
            // let tail = lateral_line.max(tail(self.side));

            if ! self.timeout.is_active(tick) {
                self.memory.set(0.);
            } else if head > 0. {
                self.memory.set_max(1.);
            }
        }
    }

    // turn pressure
    fn turn(&self, lateral_line: &LateralLine) -> Option<Turn> {
        if self.memory.is_active() {
            let head = lateral_line.max(head(self.side));

            let delta = head - self.ll_target;

            let turn = match self.side {
                Side::Left => Turn::Unit(0.5 * delta * self.turn_max),
                Side::Right => Turn::Unit(- 0.5 * delta * self.turn_max),
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
