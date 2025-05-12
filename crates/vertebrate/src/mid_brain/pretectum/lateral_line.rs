use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_brain::{lateral_line::{LateralLine, Segment}, HindMove}, util::Turn};

fn lateral_line_update(
    lateral_line: Res<LateralLine>,
    pt_lateral_line: Res<PretectumLateralLine>,
    mut hind_move: ResMut<HindMove>
) {
    let head_left = lateral_line.max(Segment::HeadLeft);
    let head_right = lateral_line.max(Segment::HeadRight);

    if pt_lateral_line.threshold < head_left || pt_lateral_line.threshold < head_right {
        if head_right < head_left {
            hind_move.optic().escape(PretectumLateralLine::TURN);
            hind_move.set_ss_left(0.75);
        } else {
            hind_move.optic().escape(- PretectumLateralLine::TURN);
            hind_move.set_ss_right(0.75);
        }
    }
}


struct PretectumLateralLine {
    pub threshold: f32,
}

impl PretectumLateralLine {
    const THRESHOLD : f32 = 0.5;

    const TURN : Turn = Turn::Unit(0.10);
    const _U_TURN : Turn = Turn::Unit(0.40);

    fn new() -> Self {
        Self {
            threshold: Self::THRESHOLD,
        }
    }
}

pub struct PretectumLateralLinePlugin {
    is_enable: bool,
}

impl PretectumLateralLinePlugin {
    pub fn new() -> Self {
        Self {
            is_enable: true,
        }
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }
}

impl Plugin for PretectumLateralLinePlugin {
    fn build(&self, app: &mut App) {
        if self.is_enable {
            if ! app.contains_resource::<LateralLine>() {
                panic!("PretectumLateralLine requires LateralLine");
            }
    
            app.insert_resource(PretectumLateralLine::new());
    
            app.system(Tick, lateral_line_update);
        }
    }
}
