use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    hind_brain::{lateral_line::{LateralLine, Segment}}, 
    mid_brain::pretectum::obstacle::ObstaclePretectum, type_short,
};

fn lateral_line_update(
    lateral_line: Res<LateralLine>,
    mut obstacle: ResMut<ObstaclePretectum>,
) {
    let head_left = lateral_line.max(Segment::HeadLeft);
    obstacle.set_max_left(head_left);

    let head_right = lateral_line.max(Segment::HeadRight);
    obstacle.set_max_right(head_right);
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
            assert!(app.contains_resource::<LateralLine>(),
                "{} requires LateralLine", type_short!(Self)
            );
            assert!(app.contains_resource::<ObstaclePretectum>(),
                "{} requires ObstaclePretectum", type_short!(Self)
            );
    
            app.system(Tick, lateral_line_update);
        }
    }
}
