use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::{AppTick, Tick};
use crate::{
    hind_brain::{HindMove, HindMovePlugin}, 
    hypothalamus::{FoodZone, HypEat}, 
    subpallium::{MosaicType, Striatum, StriatumTimeout}, 
    util::{DecayValue, Seconds, Ticks}
};

use super::{
    Sleep,
};

fn update_roam(
    mut hyp_move: ResMut<HypMove>,
    mut hind_move: ResMut<HindMove>,
    hyp_eat: Res<HypEat>,
    food_zone: Res<FoodZone>,
    tick: Res<AppTick>,
    sleep: Res<Sleep>,
) {
    hyp_move.pre_update();

    if sleep.is_sleep() {
        return;
    } else if hyp_eat.is_arc_mor() {
        // morphine suppresses roaming
    } else if food_zone.is_food_zone() {
        // stop in food zone
    } else if hyp_move.roam_striatum.left_mut().is_active(tick.get()) {
        // suppressed by eating and satiation and food zone
        hind_move.ante().roam(1.);
    }
}

///
/// HypMove includes H.l, H.sum, and Po.l
///

pub struct HypMove {
    timeout: DecayValue,
    _food_zone_timeout: StriatumTimeout,

    roam_striatum: Striatum<HypRoam>,
}

impl HypMove {
    fn new() -> Self {
        let food_zone = StriatumTimeout::new().ltd(Seconds(2.)).decay(Seconds(2.));

        Self {
            timeout: DecayValue::new(2.),
            _food_zone_timeout: food_zone,
            roam_striatum: Striatum::new(),
        }
    }

    fn pre_update(&mut self) {
        self.timeout.update();
    }
}

struct HypRoam;
impl MosaicType for HypRoam {}

pub struct HypMovePlugin {
    is_enable: bool,

    roam_timeout: Ticks,
    roam_recover: Ticks,
}

impl HypMovePlugin {
    pub fn new() -> Self {
        Self {
            is_enable: true,
            roam_timeout: Seconds(120.).into(),
            roam_recover: Seconds(15.).into(),
        }
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }

    pub fn is_enable(&self) -> bool {
        self.is_enable
    }

    pub fn roam_timeout(&mut self, timeout: impl Into<Ticks>) -> &mut Self {
        self.roam_timeout = timeout.into();

        self
    }

    pub fn roam_recover(&mut self, timeout: impl Into<Ticks>) -> &mut Self {
        self.roam_recover = timeout.into();

        self
    }
}

impl Plugin for HypMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "HypMove requires HindMove");

        let mut hyp_move = HypMove::new();
        hyp_move.roam_striatum.timeout(self.roam_timeout);
        hyp_move.roam_striatum.recover(self.roam_recover);
        app.insert_resource(hyp_move);

        app.system(Tick, update_roam);
    }
}
