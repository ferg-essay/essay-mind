use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::{AppTick, Tick};
use crate::{
    hind_brain::{HindMove, HindMovePlugin},
    subpallium::{MosaicType, Striatum, StriatumTimeout}, 
    util::{DecayValue, Seconds}
};

use super::{
    Sleep,
};

fn update_roam(
    mut hyp_move: ResMut<HypMove>,
    mut hind_move: ResMut<HindMove>,
    tick: Res<AppTick>,
    sleep: Res<Sleep>,
) {
    hyp_move.pre_update();

    if sleep.is_sleep() {
        return;
    }
    
    // suppressed by eating and satiation and food zone
    if hyp_move.roam_striatum.left_mut().is_active(tick.get()) {
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
}

impl HypMovePlugin {
    pub fn new() -> Self {
        Self {
            is_enable: true,
        }
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }

    pub fn is_enable(&self) -> bool {
        self.is_enable
    }
}

impl Plugin for HypMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "HypMove requires HindMove");

        let mut hyp_move = HypMove::new();
        hyp_move.roam_striatum.timeout(Seconds(60.));
        hyp_move.roam_striatum.recover(Seconds(30.));
        app.insert_resource(hyp_move);

        app.system(Tick, update_roam);
    }
}
