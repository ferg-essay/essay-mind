use essay_ecs::{app::{event::{InEvent, OutEvent}, App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::hind_motor::{HindLocomotor, HindLocomotorEvent};

pub struct MidSustain {
    is_explore: bool,
    is_eat: bool,
}

impl MidSustain {
    fn is_explore(&self) -> bool {
        self.is_explore
    }
}

impl Default for MidSustain {
    fn default() -> Self {
        Self { 
            is_explore: true,
            is_eat: true,
        }
    }
}

fn update_mid_sustain(
    mut sustain: ResMut<MidSustain>,
    mut locomotor_events: OutEvent<HindLocomotorEvent>,
    mut hind_locomotor: ResMut<HindLocomotor>, 
) {
    if sustain.is_explore() {
        locomotor_events.send(HindLocomotorEvent::Normal);
    }
}

pub struct MidSustainPlugin;

impl Plugin for MidSustainPlugin {
    fn build(&self, app: &mut App) {
        //assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        //assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        app.init_resource::<MidSustain>();

        app.system(Tick, update_mid_sustain);
    }
}
