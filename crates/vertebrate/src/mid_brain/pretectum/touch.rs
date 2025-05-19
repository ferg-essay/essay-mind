use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::{
    body::Body, mid_brain::pretectum::obstacle::ObstaclePretectum, type_short
};

fn touch_update(
    body: ResMut<Body>,
    mut obstacle: ResMut<ObstaclePretectum>,
) {
    if body.is_collide_left() {
        obstacle.set_max_left(1.);
    } 
    
    if body.is_collide_right() {
        obstacle.set_max_right(1.);
    }
}

pub struct PretectumTouchPlugin {
    is_enable: bool,
}

impl PretectumTouchPlugin {
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

impl Plugin for PretectumTouchPlugin {
    fn build(&self, app: &mut App) {
        if self.is_enable {
            assert!(app.contains_resource::<ObstaclePretectum>(),
                "{} requires ObstaclePretectum", type_short!(Self)
            );
    
            app.system(Tick, touch_update);
        }
    }
}
