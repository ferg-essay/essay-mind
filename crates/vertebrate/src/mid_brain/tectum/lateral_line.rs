use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    hind_brain::{lateral_line::{LateralLine, Segment}}, 
    type_short,
};

use super::OrientTectum;

fn lateral_line_update(
    lateral_line: Res<LateralLine>,
    mut orient: ResMut<OrientTectum>,
) {
    let head_left = lateral_line.max(Segment::HeadLeft);
    if head_left > 0. {
        orient.add_orient_left(head_left);
    }
    //orient.set_max_left(head_left);

    let head_right = lateral_line.max(Segment::HeadRight);
    if head_right > 0. {
        orient.add_orient_right(head_right);
    }
    //orient.set_max_right(head_right);
}

pub struct TectumLateralLinePlugin {
    is_enable: bool,
}

impl TectumLateralLinePlugin {
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

impl Plugin for TectumLateralLinePlugin {
    fn build(&self, app: &mut App) {
        if self.is_enable {
            assert!(app.contains_resource::<LateralLine>(),
                "{} requires LateralLine", type_short!(Self)
            );
            assert!(app.contains_resource::<OrientTectum>(),
                "{} requires ObstacleTectum", type_short!(Self)
            );
    
            app.system(Tick, lateral_line_update);
        }
    }
}
