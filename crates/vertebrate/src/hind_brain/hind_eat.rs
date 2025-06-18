use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};

use mind_ecs::Tick;

use crate::{
    body::{BodyEat, BodyEatPlugin}, hind_brain::SerotoninManager, hypothalamus::Sleep, util::{Seconds, Ticks, TimeoutValue} 
};

use super::{HindMove, Serotonin, SerotoninTrait};

// ## References
// [Ahn et al 2020] R.nts dbh/th orexigenic. Gut (fat) -> N10 -> R.nts
// [Boughter et al 2019] Taste -> R.nts -> R.pb.c -> T.vpm
// [Chiang et al 2019] R.nts: taste, viscerosensory, respiration, fluid balance,
//   cardiovascular. R.ap chemosensory.
// [Gasparini et al 2021] R.nts (NA) -> R.plc.dyn
// [Han W et al 2018] Gut -> N10 -> R.nts -> R.pb.dl -> Snc. N10 left/right 
//   asymmetry: R.nts vs R.ap. R.nts RTPP 85% reinforcing.
// [Karthik et al 2022] V.lc in Phox2b area.
// [Kinzeler and Travers 2008] Bitter stim R.nts.r produce gaping by N9.
//    R.nts.r stim produce gaping and also licking.
// [Palmiter 2018] N10 -> CCK, GLP-1. LPS / LiCl stim R.nts.
// [Parker 2003] Mouse/Rat specific gaping because can't vomit.
// [Roman et al 2016] R.nts.cck -> R.pb, H.pv both stop eating. After meal
//   R.nts.cck activated
// [Rosen et al 2010] Specific water response in R.nts and R.pb. Only 30% 
//   of R.nts project to R.pb
// [Roussin et al 2012] R.nts process sense different active vs passive.
//   R.nts reflects active licking. R.nts include touch, hot, cold, water.
// [Ryan et al 2017] R.nts, H.pv.oxt -> R.pb.oxt suppress drinking with R.nts
//   stronger effect.
// [Travers et al 1997] Licking R.my.irt. Orosensory to R.my.irt.
// [Weiss et al 2014] R.nts -> R.my orofacial reflexes. R.nts ensemble of
//   broadly tuned neurons. Some long-latency 1.5 possibly N10 gut.
// 
// ## Tunicate References
// [Dufour et al 2006] Phox2b: N5/N7 branchial arch muscles face, jaw, neck, 
//   pharynx, R.nts. Rphox2b visceral nervous system.
// [Fritzsch et al 2017] Branchial motor neurons are distinct motor neuron pop
//   N5, N7, N9, N10, N11. BMN ancestral to chordates.
// [Kraus et al 2021] Ascidian filter feedres regularly exposed to microbes.
//   TRP to CGRP immune avoidance.
// [Šestak and Domazet-Lošo 2015] Early vertebrates fast-swimming filter feeders.
//

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    body_eat: Res<BodyEat>,
    hind_move: Res<HindMove>,
    mut serotonin_eat: ResMut<Serotonin<HindEat>>,
) {
    hind_eat.pre_update();

    if serotonin_eat.is_active() && body_eat.sated_leptin() <= 0. && body_eat.taste_food() > 0. {
        serotonin_eat.excite(1.);
    }

    //if body_eat.is_eating() {
    //    hind_eat.is_eating.set(true);
    //}

    if hind_eat.is_stop_request() {
        hind_eat.is_eating.set(false);
    }

    if hind_move.is_active() {
        hind_eat.is_eating.set(false);
    }

    if body_eat.sickness() > 0. {
        // rodent lack vomiting
        hind_eat.is_vomiting.set(true);
        hind_eat.is_eating.set(false);
    } else if body_eat.taste_bitter() > 0. {
        // rodent gaping is in R.nts
        hind_eat.is_gaping.set(true);
        hind_eat.is_eating.set(false);
    }
}

fn mammal_feed(
    mut hind_eat: ResMut<HindEat>,
    hind_move: Res<HindMove>,
    mut body_eat: ResMut<BodyEat>,
    serotonin_eat: Res<Serotonin<HindEat>>,
    sleep: Res<Sleep>,
) {
    if sleep.is_sleep() {
        // sleeping
    } else if hind_eat.is_vomiting() || hind_eat.is_gaping() {
        // R.my blocking of movement while gaping or vomiting
    } else if hind_move.is_active() {
        // lateral inhibition
    } else if serotonin_eat.is_active() {
        hind_eat.is_eating.set(true);
        body_eat.eat();
    }
}

fn filter_feed(
    hind_move: Res<HindMove>,
    mut body_eat: ResMut<BodyEat>,
    hind_eat: Res<HindEat>,
    sleep: Res<Sleep>,
) {
    if sleep.is_sleep() {
    } else if hind_move.is_active() {
    } else if hind_eat.is_vomiting() || hind_eat.is_gaping() {
    } else if body_eat.is_sated_stretch() {
    } else {
        body_eat.eat();
    }
}

///
/// HindEat corresponds to Phox2b correlates of adult tunicate Ciona brain,
/// including R.nts, R.ap, and R.my but does not include R.pb
///
/// For simplicity HindEat only includes actions, not relay taste functions because
///   BodyEat already includes taste.
/// 
pub struct HindEat {
    is_stop_request: TimeoutValue<bool>,

    is_eating: TimeoutValue<bool>,

    // Mouse gaping is a reflexive orofacial expression to expel food in
    // the mouth, functionally like spitting
    is_gaping: TimeoutValue<bool>,

    // Some animals like mice can't vomit
    is_vomiting: TimeoutValue<bool>,

    // Configuration

    // animals that can eat while moving, such as worms or swimming
    // filter feeders like manta rays
    _allow_eat_while_move: bool,
}

impl HindEat {
    #[inline]
    pub fn is_active(&self) -> bool {
        self.is_eating()
        || self.is_gaping()
        || self.is_vomiting()    
    }

    #[inline]
    pub fn is_eating(&self) -> bool {
        self.is_eating.value_or(false)
    } 

    #[inline]
    pub fn is_gaping(&self) -> bool {
        self.is_gaping.value_or(false)
    } 

    #[inline]
    pub fn is_vomiting(&self) -> bool {
        self.is_vomiting.value_or(false)
    } 

    #[inline]
    pub fn stop(&mut self) {
        self.is_stop_request.set(true);
    }

    fn is_stop_request(&self) -> bool {
        self.is_stop_request.value_or(false)
    } 

    fn pre_update(&mut self) {
        self.is_eating.update();
        self.is_gaping.update();
        self.is_vomiting.update();

        self.is_stop_request.update();
    }
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            is_stop_request: TimeoutValue::new(Seconds(1.)),
            is_eating: TimeoutValue::new(Seconds(2.)),
            is_gaping: TimeoutValue::new(Seconds(5.)),
            is_vomiting: TimeoutValue::new(Seconds(15.)),
            _allow_eat_while_move: true,
        }
    }
}

impl SerotoninTrait for HindEat {}

pub struct HindEatPlugin {
    eat_time: Ticks,
    strategy: EatStrategy,
}

impl HindEatPlugin {
    pub fn new() -> Self {
        Self {
            eat_time: Seconds(2.).into(),
            strategy: EatStrategy::Mammal,
        }
    }

    pub fn strategy(&mut self, strategy: EatStrategy) -> &mut Self {
        self.strategy = strategy;

        self
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EatStrategy {
    FilterFeed,
    Mammal,
}

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        SerotoninManager::insert::<HindEat>(app, self.eat_time);

        let mut hind_eat = HindEat::default();

        hind_eat.is_eating = TimeoutValue::new(self.eat_time);

        app.insert_resource(hind_eat);

        app.system(Tick, update_hind_eat);

        match self.strategy {
            EatStrategy::FilterFeed => {
                app.system(Tick, filter_feed);
            }
            EatStrategy::Mammal => {
                app.system(Tick, mammal_feed);
            }
        }
    }
}
