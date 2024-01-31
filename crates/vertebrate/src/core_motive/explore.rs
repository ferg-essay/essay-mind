use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_motor::HindEat, mid_motor::MidMotor, util::Seconds};

use super::{motive::{Motive, MotiveTrait, Motives}, Wake};

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

fn roam_update(
    mut roam: ResMut<Motive<Roam>>,
    hind_eat: Res<HindEat>,
    mid_move: Res<MidMotor>,
    wake: Res<Motive<Wake>>,
) {
    if hind_eat.is_eat() {
        roam.set_max(wake.value() * 0.2);
    } else {
        roam.set_max(wake.value());
    }

    if roam.is_active() {
        mid_move.explore();
    }   
}

fn dwell_update(
    dwell: Res<Motive<Dwell>>,
) {
    if dwell.value() > 0.1 {
        // taxis.send(HindLocomotorEvent::Dwell);
    }
}

pub struct CoreExplorePlugin;

impl Plugin for CoreExplorePlugin {
    fn build(&self, app: &mut App) {
        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, roam_update);
        app.system(Tick, dwell_update);
    }
}
