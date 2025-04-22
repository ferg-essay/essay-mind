use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_brain::{lateral_line::{LateralLine2Plugin, Segment}, HindMovePlugin}, util::{DecayValue, HalfLife, Seconds, Turn}};

use super::{lateral_line::LateralLine, HindMove};

fn update_thigmotaxis(
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
            thigmotaxis.left.set_max(0.5);
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
            thigmotaxis.right.set_max(0.5);
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
    // memory of thigmotaxis side.
    left: DecayValue,
    right: DecayValue,
}

impl Thigmotaxis {
    const MAX_THRESHOLD: f32 = 0.4;

    fn new(half_life: impl Into<HalfLife>) -> Self {
        let half_life = half_life.into();

        Thigmotaxis {
            left: DecayValue::new(half_life),
            right: DecayValue::new(half_life),
        }
    }

    pub fn is_active(&self) {
        self.left.is_active();
        self.right.is_active();
    }

    pub fn active_value(&self) -> f32 {
        self.left.active_value().max(self.right.active_value())
    }

    fn turn(&self, head: f32) -> Turn {
        Turn::Unit(0.1 * (Thigmotaxis::MAX_THRESHOLD - head))
    }

    fn update(&mut self)  {
        self.left.update();
        self.right.update();
    }
}

pub struct HindThigmotaxisPlugin;

impl Plugin for HindThigmotaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "HindThigmotaxis requires HindLocomotion");
        assert!(app.contains_plugin::<LateralLine2Plugin>(), "HindThigmotaxis requires LateralLine");

        /*
        let mut hind_move = HindMove::new();
        hind_move.oscillator_r3 = Some(OscillatorArs::new());
        hind_move.startle_r4 = Some(StartleR4::new());

        app.insert_resource(hind_move);

        */

        app.insert_resource(Thigmotaxis::new(Seconds(4.)));

        app.system(Tick, update_thigmotaxis);
    }
}
