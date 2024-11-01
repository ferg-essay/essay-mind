use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::{AppTick, Tick};
use crate::{
    hind_brain::{HindAvoid, HindEat, HindSearch, Serotonin}, 
    mid_brain::{MidMove, MidMovePlugin}, 
    motive::eat::MotiveEatPlugin, 
    olfactory::{OdorCortex, OlfactoryCortexPlugin}, 
    subpallium::StriatumTimeout, 
    util::{DecayValue, Seconds}
};

use super::{
    eat::MotiveEat, Motive,MotiveTrait, Motives, Sleep,
};

fn update_forage(
    mut forage: ResMut<Forage>,
    odor_cortex: Res<OdorCortex>,
    mid_move: Res<MidMove>,
    mut motive_eat: ResMut<MotiveEat>,
    mut foraging: ResMut<Motive<Forage>>,
    serotonin_avoid: Res<Serotonin<HindAvoid>>,
    mut serotonin_eat: ResMut<Serotonin<HindEat>>,
    mut serotonin_search: ResMut<Serotonin<HindSearch>>,
    tick: Res<AppTick>,
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

    // necessary each time because of striatum side effects (timeout)
    let is_food_zone = odor_cortex.is_food_zone() 
        && forage.food_zone_timeout.is_active(tick.get());

    if serotonin_avoid.is_active() {
        // avoidance higher priority
        // TODO: priority with hunger?
    } else if serotonin_eat.is_active() {
        // active eating suppresses foraging
    } else if is_food_zone {
        // H.l food zone from olfactory
        foraging.clear();

        serotonin_eat.excite(1.);
        serotonin_search.inhibit(1.);

        // TODO: remove/merge motive_eat food zone because redundant with
        // serotonin
        motive_eat.set_food_zone(true);
    } else {
        foraging.set_max(1.);

        // H.sum activation for roaming
        // mid_move.roam();
        serotonin_search.excite(1.);
        serotonin_eat.excite(0.);
    }
}

///
/// Forage includes R.pb, H.l, H.pstn, H.pv, H.sum, S.a, P.bst
/// specifically the food-related portions of those nuclei
///

pub struct Forage {
    timeout: DecayValue,
    food_zone_timeout: StriatumTimeout,
}

impl Forage {
    fn new() -> Self {
        let food_zone = StriatumTimeout::new().ltd(Seconds(2.)).decay(Seconds(2.));

        Self {
            timeout: DecayValue::new(2.),
            food_zone_timeout: food_zone,
        }
    }

    fn pre_update(&mut self) {
        self.timeout.update();
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
