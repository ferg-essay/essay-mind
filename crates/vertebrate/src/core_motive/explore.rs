use essay_ecs::{app::{event::OutEvent, App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_motor::HindLocomotorEvent, util::Seconds};

use super::{motive::{Motive, MotiveTrait, Motives}, Wake};

fn roam_update(
    mut roam: ResMut<Motive<Roam>>,
    wake: Res<Motive<Wake>>,
) {
    roam.set_max(wake.value());
}

fn dwell_update(
    dwell: Res<Motive<Dwell>>,
    mut _taxis: OutEvent<HindLocomotorEvent>,
) {
    if dwell.value() > 0.1 {
        // taxis.send(HindLocomotorEvent::Dwell);
    }
}

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

pub struct CoreExplorePlugin;

impl Plugin for CoreExplorePlugin {
    fn build(&self, app: &mut App) {
        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(1.));

        app.system(Tick, roam_update);
        app.system(Tick, dwell_update);
    }
}
