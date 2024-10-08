use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use log::error;
use mind_ecs::Tick;

use crate::{
    body::{Body, BodyEat, BodyEatPlugin},
    util::{Seconds, Ticks, TimeoutValue} 
};

use super::HindMove;

///
/// HindEat corresponds to Phox2b correlates of adult tunicate Ciona brain,
/// including R.nts, R.ap, and R.my but does not include R.pb
///
/// For simplicity HindEat only includes actions, not relay taste functions because
///   BodyEat already includes taste.
/// 
/// ## References
/// [Ahn et al 2020] R.nts dbh/th orexigenic. Gut (fat) -> N10 -> R.nts
/// [Boughter et al 2019] Taste -> R.nts -> R.pb.c -> T.vpm
/// [Chiang et al 2019] R.nts: taste, viscerosensory, respiration, fluid balance,
///   cardiovascular. R.ap chemosensory.
/// [Gasparini et al 2021] R.nts (NA) -> R.plc.dyn
/// [Han W et al 2018] Gut -> N10 -> R.nts -> R.pb.dl -> Snc. N10 left/right 
///   asymmetry: R.nts vs R.ap. R.nts RTPP 85% reinforcing.
/// [Karthik et al 2022] V.lc in Phox2b area.
/// [Kinzeler and Travers 2008] Bitter stim R.nts.r produce gaping by N9.
///    R.nts.r stim produce gaping and also licking.
/// [Palmiter 2018] N10 -> CCK, GLP-1. LPS / LiCl stim R.nts.
/// [Parker 2003] Mouse/Rat specific gaping because can't vomit.
/// [Roman et al 2016] R.nts.cck -> R.pb, H.pv both stop eating. After meal
///   R.nts.cck activated
/// [Rosen et al 2010] Specific water response in R.nts and R.pb. Only 30% 
///   of R.nts project to R.pb
/// [Roussin et al 2012] R.nts process sense different active vs passive.
///   R.nts reflects active licking. R.nts include touch, hot, cold, water.
/// [Ryan et al 2017] R.nts, H.pv.oxt -> R.pb.oxt suppress drinking with R.nts
///   stronger effect.
/// [Travers et al 1997] Licking R.my.irt. Orosensory to R.my.irt.
/// [Weiss et al 2014] R.nts -> R.my orofacial reflexes. R.nts ensemble of
///   broadly tuned neurons. Some long-latency 1.5 possibly N10 gut.
/// 
/// ## Tunicate References
/// [Dufour et al 2006] Phox2b: N5/N7 branchial arch muscles face, jaw, neck, 
///   pharynx, R.nts. Rphox2b visceral nervous system.
/// [Fritzsch et al 2017] Branchial motor neurons are distinct motor neuron pop
///   N5, N7, N9, N10, N11. BMN ancestral to chordates.
/// [Kraus et al 2021] Ascidian filter feedres regularly exposed to microbes.
///   TRP to CGRP immune avoidance.
/// [Šestak and Domazet-Lošo 2015] Early vertebrates fast-swimming filter feeders.
///
pub struct HindEat {
    is_eat_request: TimeoutValue<bool>,
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
    allow_eat_while_move: bool,
}

impl HindEat {
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
    pub fn eat(&mut self) {
        self.is_eat_request.set(true);
    }

    #[inline]
    fn is_eat_request(&self) -> bool {
        self.is_eat_request.value_or(false)
    } 

    #[inline]
    pub fn stop(&mut self) {
        self.is_stop_request.set(true);
    }

    fn is_stop_request(&self) -> bool {
        self.is_stop_request.value_or(false)
    } 

    fn is_eat_allowed(&self, body: &Body) -> bool {
        ! self.allow_eat_while_move || body.speed() < 0.1
    } 

    fn pre_update(&mut self) {
        self.is_eating.update();
        self.is_gaping.update();
        self.is_vomiting.update();

        self.is_eat_request.update();
        self.is_stop_request.update();
    }
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            is_eat_request: TimeoutValue::new(Ticks(3)),
            is_stop_request: TimeoutValue::new(Seconds(1.)),
            is_eating: TimeoutValue::new(Seconds(2.)),
            is_gaping: TimeoutValue::new(Seconds(5.)),
            is_vomiting: TimeoutValue::new(Seconds(15.)),
            allow_eat_while_move: true,
        }
    }
}

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    mut hind_move: ResMut<HindMove>,
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
) {
    hind_eat.pre_update();

    if hind_eat.is_stop_request() {
        hind_eat.is_eating.set(false);
    } else if hind_eat.is_eat_request() {
        if hind_eat.is_eat_allowed(body.get()) {
            hind_eat.is_eating.set(true);
        } else {
            error!("eating while moving");
        }
    }


    if body_eat.sickness() > 0. {
        // rodent lack vomiting
        hind_eat.is_vomiting.set(true);
    } else if body_eat.bitter() > 0. {
        // rodent gaping is in R.nts [cite]
        hind_eat.is_gaping.set(true);
    } else if hind_eat.is_eating() {
        body_eat.eat();
    }

    // R.my blocking of movement while gaping or vomiting
    if hind_eat.is_vomiting() || hind_eat.is_gaping() {
        hind_move.halt();
    }
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        app.init_resource::<HindEat>();

        app.system(Tick, update_hind_eat);
    }
}
