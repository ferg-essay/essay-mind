use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_brain::SerotoninManager, util::{Seconds, Ticks, Turn}};

use super::{AvoidHere, HindEat, HindMove, Serotonin, SerotoninTrait};

fn update_hind_avoid(
    mut hind_avoid: ResMut<HindAvoid>,
    mut hind_move: ResMut<HindMove>,
    avoid_place: Option<Res<AvoidHere>>,
    mut serotonin_avoid: ResMut<Serotonin<HindAvoid>>,
    mut serotonin_eat: ResMut<Serotonin<HindEat>>,
) {
    // R.pb tac1 avoidance
    if let Some(avoid_place) = avoid_place {
        if avoid_place.is_avoid() {
            serotonin_avoid.excite(1.);
            serotonin_eat.inhibit(1.);
        }
    }

    if serotonin_avoid.is_active() {
        if ! hind_avoid.is_avoid {
            hind_move.optic().u_turn(Turn::unit(0.5));
        }
        hind_avoid.is_avoid = true;
        hind_move.avoid();
        // hind_move.
    } else {
        hind_avoid.is_avoid = false;
    }
}

pub struct HindAvoid {
    is_avoid: bool,
}

impl HindAvoid {
    pub(super) fn new() -> Self {
        Self {
            is_avoid: false,
        }
    }
}

impl SerotoninTrait for HindAvoid {}

pub struct HindAvoidPlugin {
    avoid_time: Ticks,
}

impl HindAvoidPlugin {
    pub fn new() -> Self {
        Self {
            avoid_time: Seconds(2.).into(),
        }
    }
}

impl Plugin for HindAvoidPlugin {
    fn build(&self, app: &mut App) {
        SerotoninManager::insert::<HindAvoid>(app, self.avoid_time);

        let hind_avoid = HindAvoid::new();

        app.insert_resource(hind_avoid);

        app.system(Tick, update_hind_avoid);
    }
}
