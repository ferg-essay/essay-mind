use essay_ecs::core::{Res, ResMut};

use crate::{
    hind_brain::lateral_line::Segment, 
    util::{DecayValue, HalfLife, Turn}
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
) {
    thigmotaxis.update(hind_move.get(), lateral_line.as_ref());

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
        thigmotaxis_ui.set_value(thigmotaxis.active_value());
    }
}

pub struct ThigmotaxisArtr {
    // memory of thigmotaxis side.
    left: ThigmotaxisSide,
    right: ThigmotaxisSide,
}

impl ThigmotaxisArtr {
    pub(super) fn new(plugin: &HindThigmotaxisPlugin) -> Self {
        let half_life = plugin.memory_time;

        ThigmotaxisArtr {
            left: ThigmotaxisSide::new(Side::Left, half_life, plugin.turn),
            right: ThigmotaxisSide::new(Side::Right, half_life, plugin.turn),
        }
    }

    fn is_active(&self) -> bool {
        self.left.memory.is_active() || self.right.memory.is_active()
    }

    fn active_value(&self) -> f32 {
        self.left.memory.active_value().max(self.right.memory.active_value())
    }

    fn update(&mut self, hind_move: &HindMove, lateral_line: &LateralLine)  {
        self.left.update(hind_move, lateral_line);
        self.right.update(hind_move, lateral_line);
    }
}

impl Default for ThigmotaxisArtr {
    fn default() -> Self {
        Self {
            left: ThigmotaxisSide::new(Side::Left, HalfLife::default(), Turn::Unit(0.25)),
            right: ThigmotaxisSide::new(Side::Right, HalfLife::default(), Turn::Unit(0.25)),
        }
    }
}

struct ThigmotaxisSide {
    side: Side,

    ll_target: f32,
    turn_max: f32,

    // memory of thigmotaxis side.
    memory: DecayValue,
}

impl ThigmotaxisSide {
    fn new(
        side: Side,
        half_life: HalfLife,
        turn_max: Turn,
    ) -> Self {
        Self {
            side,
            ll_target: 0.5,
            turn_max: turn_max.to_unit(),
            memory: DecayValue::new(half_life)
        }
    }

    fn update(&mut self, hind_move: &HindMove, lateral_line: &LateralLine) {
        self.memory.update();

        if hind_move.artr().side().map_or(false, |side| side == self.side)
        || self.memory.is_active() {
            let head = lateral_line.max(head(self.side));
            // let tail = lateral_line.max(tail(self.side));

            if head > 0. {
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
