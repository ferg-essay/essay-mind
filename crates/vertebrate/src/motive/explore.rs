use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    hind_eat::HindEat, 
    mid_motor::{MidMotor, MidMotorPlugin}, 
    util::Seconds
};

use super::{motive::{Motive, MotiveTrait, Motives}, Wake};

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

fn explore_update(
    mut roam: ResMut<Motive<Roam>>,
    mut dwell: ResMut<Motive<Dwell>>,
    hind_eat: Res<HindEat>,
    mid_move: Res<MidMotor>,
    wake: Res<Motive<Wake>>,
) {
    if ! wake.is_active() {
        return;
    }

    if hind_eat.is_eat() {
        roam.set_max(wake.value() * 0.2);
        dwell.set_max(wake.value());
    } else {
        roam.set_max(wake.value());
    }

    if roam.is_active() || dwell.is_active() {
        mid_move.explore();
    }   
}

pub struct CoreExplorePlugin;

impl Plugin for CoreExplorePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMotorPlugin>(), "CoreExplore requires MidMotor");

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, explore_update);
    }
}
