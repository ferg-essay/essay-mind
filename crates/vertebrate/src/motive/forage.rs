use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::Tick;
use crate::{
    hind_brain::{HindEat, HindSearch, Serotonin}, mid_brain::{MidMove, MidMovePlugin}, motive::eat::MotiveEatPlugin, olfactory::{OlfactoryCortex, OlfactoryCortexPlugin}, util::{DecayValue, Seconds}
};

use super::{
    eat::MotiveEat, Motive,MotiveTrait, Motives, Sleep,
};

//
// Forage includes R.pb, H.l, H.pstn, H.pv, H.sum, S.a, P.bst
// specifically the food-related portions of those nuclei
//

pub struct Forage {
    timeout: DecayValue,
}

impl Forage {
    fn new() -> Self {
        Self {
            timeout: DecayValue::new(2.),
        }
    }

    fn pre_update(&mut self) {
        self.timeout.update();
    }
}

fn update_forage(
    mut forage: ResMut<Forage>,
    olfactory: Res<OlfactoryCortex>,
    mid_move: Res<MidMove>,
    mut motive_eat: ResMut<MotiveEat>,
    mut foraging: ResMut<Motive<Forage>>,
    mut serotonin_eat: ResMut<Serotonin<HindEat>>,
    mut serotonin_search: ResMut<Serotonin<HindSearch>>,
    sleep: Res<Sleep>,
) {
    forage.pre_update();

    if sleep.is_sleep() {
        return;
    } else if motive_eat.is_alarm() {
        mid_move.avoid();
        return;
    } else if motive_eat.sated() > 0. {
        // TODO: roam not strictly justified, but w/o this the animal remains 
        // paused at the food
        mid_move.roam();
        return;
    }
    
    if olfactory.is_food_zone() {
        // H.l food zone from olfactory
        foraging.clear();

        motive_eat.set_food_zone(true);
        serotonin_eat.excite(1.);
        serotonin_search.inhibit(1.);
    } else {
        foraging.set_max(1.);

        // H.sum activation for roaming
        // mid_move.roam();
        serotonin_search.excite(1.);
        serotonin_eat.excite(0.);
    }
}
pub struct Eat;
impl MotiveTrait for Eat {}

pub struct Sated;
impl MotiveTrait for Sated {}

// pub struct Forage;
impl MotiveTrait for Forage {}

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Alarm;
impl MotiveTrait for Alarm {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

pub struct MotiveForagePlugin;

impl Plugin for MotiveForagePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveForage requires MidMove");
        assert!(app.contains_plugin::<MotiveEatPlugin>(), "MotiveForage requires MotiveEat");
        assert!(app.contains_plugin::<OlfactoryCortexPlugin>(), "MotiveForage requires Olfactory");

        let feeding = Forage::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Forage>(app, Seconds(0.1));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));
        Motives::insert::<Alarm>(app, Seconds(4.));

        app.system(Tick, update_forage);
    }
}
