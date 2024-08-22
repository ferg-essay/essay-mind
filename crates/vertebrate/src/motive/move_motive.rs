use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    hind_eat::HindEat, 
    mid_move::{MidMove, MidMovePlugin}, 
    util::Seconds
};

use super::{motive::{Motive, MotiveTrait, Motives}, Wake};

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

fn roam_update(
    mut roam: ResMut<Motive<Roam>>,
    mut dwell: ResMut<Motive<Dwell>>,
    hind_eat: Res<HindEat>,
    mid_move: Res<MidMove>,
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

    if dwell.is_active() {
        mid_move.dwell();
    } else if roam.is_active() {
        mid_move.roam();
    }   
}

pub struct MotiveMovePlugin;

impl Plugin for MotiveMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveMove requires MidMove");

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, roam_update);
    }
}
