use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    body::BodyEat, 
    hind_brain::{HindEat, HindEatPlugin}, 
    motive::{Dwell, Eat, Motives, Sated}, 
    util::{DecayValue, Seconds, TimeoutValue}
};

use super::{Motive, Sleep, Wake};

///
/// MotiveEat includes R.pb, H.pstn, H.arc, H.pv, S.a, P.bst
/// specifically the food-related portions of those nuclei with R.pb as the
/// central node.
///
/// H.l and H.sum are in Forage, the food seeking module
/// 
/// MotiveEat includes multiple areas to clarify the behavior as a system,
/// which would be obscured by implementing the areas in separate modules.
/// 
/// ## Ascidian references
/// [Dufour et al 2006] Phox2b: N5/N7, R.nts. Phox2a: V.lc, N3, N4
/// [Fritzsch et al 2017] Branchial motor neurons: N5, N7, N9, N10, N11
/// [Gigante et al 2023] Ciona neck pac2/5/8 and phox2
/// [Jeffery 2015] Ciona oral SubP, atrial CCK, GnRN
/// [Krau et al 2021] Ascidian filter feeders exposed to microbes: TRP to
///   CGRP immune avoidance. Pathogen detection: both avoidance and immune
///   priming.
/// [Šestak and Domazet-Lošo 2015] Early vertebrate fast-swimming filter feeder
/// [Woronowicz and Schneider 2019] Vertebrates likely active feeder before jaws
///
/// ## R.pb references
/// [Ahn et al 2020] Gut (fat) -> N10 -> R.nts -> R.pb -> Snc
/// [Arthurs et al 2023] R.pb tac1 counteract some CGRP. CGRP high proximity
///   freeze, CTA. CGRP never hide.
/// [Barik and Chesler 2020] R.pb.el -> S.a, P.bst, C.i, R.my (escape, pain)
/// [Bowen et al 2020] R.pb.cgrp stim freezing 0.4s. R.pb.cgrp anxiogenic (EPM)
/// [Campos et al 2016] Harc.agrp to R.pb.el.cgrp delay meal termination.
///   R.pb.cgrp ko eat 3x. R.pb.cgrp short term satiety, via S.a.pkcδ.
/// [Campos et al 2017] R.pb.cgrp ko prevents lethargy, anxiety, malaise from
///    cancer. R.pb -> Sa, P.bst -> H.l motivation.
/// [Campos et al 2018] R.pb CGRP active shock, heat, itch, LPS. Inhibited by
///   feeding, active satiation. Active novel food or shock cue. Food 
///   presentation (H.l?) suppress R.pb.cgrp. R.pb ascending N5, N.sp, N10.
/// [Carter et al 2013] AgRP ko ptoduce hyperactive R.pb anorexia. 
///   R.pb.el CGRP -> S.al suppress appetite. R.pb.el CGRP eat suppress but not
///   general aversion. R.pb CGRP to S.al, P.bst, sparse H.pstn, T.md, R.pb.d.
/// [Carter et al 2015] R.pb LiCl -> R.pb.el CGRP and CTA
/// [Chian et al 2019] R.pb limited acute pain but significant persistent pain
/// [Coizet et al 2010] R.pb to Vta / Snc. R.pb may be exclusive nociceptive
///   to RMTg.
/// [Essner et al 2017] R.pb.cgrp amylin (pancreas), CCK (small intestine),
///   LiCl (gastric discomfort), LPS (bacterial inflammation). H.arc.cgrp suppress
///   R.pb inhibition amylin, CCK, LiCl, but not LPS.
/// [Fetterly et al 2019] V.lc α2a.i suppress R.pb -> P.bst.crf
/// [Flavin et al 2014] R.pb only source of CGRP in P.bst, blocked by V.lc.
/// [Fu et al 2019] R.pb.satb2 sweet. Stim R.pb satb2 enhance lick. R.pb oxt
///   water satiation.
/// [Garfield et al 2014] R.pb.l.cck -> H.vm.sf1 for hypoglycemia
/// [Han W et al 2018] Gut -> N10 -> R.nts -> R.pb.dl -> Snc
/// [Huang et al 2021] R.pb to H.l strongly correlate H.l.ox
/// [Ito et al 2021] R.pb.el to S.a RTPA but only after 4 min and not escape.
///   Contrast escape R.pb to H.vm, M.pag.l
/// [Jaramillo et al 2021] All R.pb -> S.a/P.bst express CGRP or PACAP.
///   R.pb CGRP to S.am immediately suppress eating, but not CGRP to P.bst.
///   Primate R.pb not have gustatory.
/// [Jarvie et al 2021] R.pb ko only partially disrupt taste preference. KO of
///   both CGRP and satb2 eliminate antipreference for bitter and sour.
/// [Karthik et al 2022] R.pb two pops lmx1b and foxp2. Lmx1b to S.a/P.bst and
///   Phox2b hindbrain. Foxp2 to H.
/// [Katagiri and Kato 2020] CGRP neuropathic pain. R.pb.l -> H.l.ox wake. Pain wake.
/// [Li CS et al 2012] S.sh suppress R.pb taste. R.pb project to S.sh. 
///    S.sh inhibit 90% of R.pb
/// [Li J et al 2019] R.pb.el converge bitter, capsaicin (N5 pain), heat
/// [Li MM et al 2019] Satiety H.pv.mc4r and H.pv.pdyn distinct with distinct
///   targets in R.pb, each approx 50%. H.pv targets distinct from CGRP.
/// [Liu PC et al 2023] R.pb astrocytes wakefulness. Stim R.pb 11h wake.
/// [Lustin et al 2021]P.bst -> R.pb threat/food. P.bst.g -> R.pb (dyn, cgrp)
///   increase feeding when sated, also increase bitter.
/// [Nagashima et al 2022] R.pb -> S.a or R.pb.l -> H.l suppress feeding immediately.
/// [Nagashima et al 2023] R.pb -> Vta.g reinforce lever press. Both RTPA/RTPP
/// [Palmiter 2018] R.nts.cck -> R.pb.cgrp. H.arc.agrp -> R.pb.cgrp for visceral 
///   malaise.
/// [Qiao et al 2019] R.pb.l.cgrp -> Sa, Vta collateral for neurpathic pain
/// [Raver et al 2020] Chronic pain suppress Sa.g -> R.pb.l.
/// [Roman et al 2016] R.nts.cck -> R.pb.cgrp ko rescue H.arc.agrp anorexia
/// [Roman et al 2017] R.nts.cck -> R.pb, H.pv stop eating. CTA, CPA
/// [Sandoval-Rodríguez et al 2023] R.pb.m, R.nts.r -> S.d1 but not to S.d2
///   R.ap (GLP-1) -> S.d2. S.d1 -> R.pcrt, R.m12, R.dmv, S.d2 -> R.nts.c.
/// [Sciolino et al 2022] V.lc suppresses S.a -> R.pb.l, suppressed while eating.
/// [Shah et al 2022] R.pb.l.tac1 -> H.pstn possibly pain-related.
/// [Shen et al 2022] rewarding R.nts -> R.pb -> P.ms.sst.g -> Hb.l
/// [Singh Alvardo et al 2024] R.pb short stim, long effect (minutes) suppress
///   feeding. 10s tail shock suppress feeding 30-60s cAMP/PKA.
/// [Torruella-Suárez et al 2024] S.a -> R.pb tonic inhibition (GABA.b), 
/// suppressed by chronic pain
/// [Tsou et al 2023] R.pb to Vta active contextual fear, restraing, mild stress
///   (not fron rich food, malaise or thermal). Emotional not direct sensory.
/// [Uddin et al 2018] CCI-pain R.pb prolonged after discharge 3s to 8s.
/// [Wang et al 2023] R.pb cck -> H.pv predatory, looming defense
/// [Weiss et al 2014] R.pb some long-latency 1.5s possibly N10 gut.
/// [Yang B et al 2021] V.lc inhibit S.a.g -> R.pb via CB1i
/// 
/// ## S.a references
/// [Barik and Chesler 2020] R.pb.el -> S.a, P.bst, C.i, R.my (escape, pain)
/// [Barbier et al 2017] H.pstn and H.cbn as part of H.l most connect with S.a
///   A.bm.a -> H.cbn is the major H target of A.bm.a
/// [Boughter et al 2019] Taste -> R.pb.c -> S.a, H.l, T.vpm direct to Vta, S.v
/// [Bowen et al 2020] R.pb.cgrp to S.a.cap (40%) RTPA from R.pb.cgrp and
///   Sa.r, H.pstn, P.si. CPA from P.si, CTA from S.a, P.bst.ov
/// [Campos et al 2017] R.pb -> S.a, P.bst -> H.l motivation
/// [Carter et al 2013] R.pb.el CGRP -> S.al suppress eating.
/// [Carter et al 2015] R.pb to S.a necessary/sufficient for CTA
/// [Chiang et al 2019] CTA specific LiCl, not shock. CGRP to Sa hyperalgesia and
///   ultrasonic vocalization
/// [Chiang et al 2020] R.pb.l dyn to P.bst, S.am memory RTPA and CPA
/// [Chometton et al 2016] H.pstn simlar connectivity S.a bidirectional to S.a
/// [Huang et al 2021] R.pb LiCl but not CCK overlap with CGRP  
///   R.pb.el LiCl, CGRP -> S.al CCK satiety but reinforcing
/// [Ito et al 2021] R.pb.el to S.a substitute for pain is learned threat/fear
///   R.pb to S.a RTPA but only after 4 min, but not escape not CPA
/// [Jaramillo et al 2021] All R.pb -> S.a / P.bst express CGRP or PACAP
///   Extended amygdala: S.a, P.bst, S.sh. R.pb to S.a / P.bst basket axiosomatic.
///   R.pb to S.a response to stressor. CGRP to S.am immediately suppress eating
///   S.a.nts to R.pb increase drinking. R.pb to S.a hypercapnia (CO2)
/// [Karthik et al 2022] R.pb.lmx1b satb2, cgrp to S.a / P.bst
/// [Nagashima et al 2022] R.pb to S.a or R.pb.l to H.l suppress eating immediately.
///   RTPA after 2 min, fast escape. CPA.
/// [Palmiter 2018] R.pb.cgrp to S.a, Pv.
/// [Qiao et al 2019] R.pb.l.cgrp to Sa, Vta collateral for neuropathic pain
/// [Raver et al 2020] Chronic pain suppress S.a.g -> R.pb.l.
///   S.a dyn, CRH, SST to R.pb with dyn as largest.
/// [Sciolino et al 2022] V.lc suppresses S.a -> R.pb.l, is suppressed by eating.
/// [Shah et al 2022] H.pstn connectivity similar to S.a. H.pstn to S.al, S.am
///   S.am to H.pstn is unique to H.l. H.pstn eating rich food not seeking
/// [Söderpalm and Berridge 2020] GABA stim increase eating R.pb (S.a)
/// [Steinberg et al 2020] S.a to Snc.l
/// [Torres-Rodriguez et al 2024] R.pb to S.am pain sensitization not somatosensory
///   S.a as pain rheostatic decreased S.a.sst increased S.a.pkcδ.
/// [Torruella-Suárez et al 2020] Sa.nts activated by ethanol. Sa.nts -> R.pb
///   increase ethanol, sucrose drinking but not water or food. Directly increase
///   licking.
/// [Torruella-Suárez et al 2024] S.a to R.pb tonic inhibition GABA.b suppressed
///   by chronic pain.
/// [Tsou et al 2023] Malaise R.pb.cgrp to S.a. Injury produce S.a.g to R.pb 
///   plasticity, particularly presynaptic GABA.b
/// [Wilson et al 2019] S.a as pain rheostat. S.a.pkcδ sensitized by neuropathic
///   pain, S.a.sst inhibited by nerve injury.
/// [Yang B et al 2021] V.lc inhibit S.a.g to R.pb via CB1i, inhibit feeding
///   V.lc suppress S.a.g presynaptic. 
/// [Zheng D et al 2022] Sa.g.sst to M.dp licking, drinking 
/// 
/// ## P.bst references
/// [Barik and Chesler 2020] R.pb.el to S.a, P.bst, C.i, R.my (escape, pain)
/// [Bowen et al 2020] R.pb.cgrp to R.bst.ov 15%. Only R.pb to P.bst.ov increase
///   anxiety (EPM). CTA from S.a, P.bst.ov
/// [Campos et al 2017] R.pb to S.a, P.bst to H.l
/// [Chiang et al 2020] R.pb.l dyn to P.bst, S.am RTPA and CPA
/// [Fetterly et al 2019] V.lc α2a.i suppress R.pb to P.bst.crf. Acute restraint
///   stress P.bst.d.crf
/// [Flavin et al 2014] R.pb to P.bst only source of CGRP in P.bst, blocked by V.lc.
///   R.pb to P.bst large axosomatic
/// [Jaramillo et al 2021] All R.pb to S.a / P.bst express CGRP or PACAP.
///   R.pb CGRP to P.bst not suppress eating. P.bst.g to R.pb increase eating,
///   especially sucrose.
/// [Jarvie et al 201] R.pb satb2 to P.bst
/// [Karthik et al 2022] R.pb lmx1b to S.a / P.bst
/// [Lustin et al 2021] P.bst to R.pb threat / food. P.bst.g to R.pb increase
///   feeding when sated, also increase eating bitter or salty or nonfood.
///   P.bst to R.pb target dyn, cgrp. R.pb dyn engaged when feeding, has RTPA.
/// [Palmiter 2018] R.pb.el.cgrp to S.al, P.bst.ov.
/// [Tsou et al 2023] P.bst, V.lc to R.pb suppress feeding, also suppress locomotion,
///   but not reinforcing.
/// 
/// ## V.lc references
/// [Fetterly et al 2019] V.lc α2a.i suppress R.pb to P.bst.crf.
/// [Flavin et al 2014] V.lc block R.pb.cgrp to P.bst.
/// [Karthik et al 2022] V.lc in Phox2b area (is Phox2a)
/// [Sciolino et al 2022] V.lc suppressed while eating. V.lc suppresses
///   S.a to R.pb.l. V.lc activated for food cues. V.lc ko not affect food intake.
/// [Yang B et al 2021] V.lc inhibit S.a.g to R.pb via CB1i. V.lc suppress feeding
///   S.a.g presynaptic.
/// 
/// ## H.pstn references
/// [Barbier et al 2017] H.pstn and H.cbn as the part of H.l more connected
///   with S.a.
/// [Chometton et al 2016] H.pstn, H.cbn as more rich food than plain food.
///   H.pstn marked by tac1. H.pstn similar S.a and strong recip connectivity.
/// [Dufour et al 2006] Phox2a: V.lc, N3, N4
/// [Shat et al 2022] R.pb.l.tac1 to H.pstn possibly pain-related. H.pstn eating,
///   not seeking. H.pstn neophobia. H.pstn to S.al, S.am. H.pstn rich food.
/// 
/// ## R.plc references
/// [Ahn et al 2020] H.l.g to R.plc voracious feeding.
/// [Gasparini et al 2021] R.plc is foxp2, dyn. R.nts dyn from R.nts (Na), H.pv, H.arc.agrp
/// [Gong et al 2020] R.plc transition from "sampling to savoring" R.plc is part
///   of R.pn.cg, is glu flanked by gaba from P.ldt. Stim R.plc increase eating.
///   R.plc.glu inhibition (?) extends eating ~15s.
/// 
/// ## H.arc and H.pv references
/// [Andermann and Lowell 2017] meal termination predominantly N10 to R.nts.
///   R.nts to R.pb.l (cgrp) to S.a meal termination. H.arc increase circadian
///   at expected feeding times.
/// [Aponte et al 2021] H.arc.agrp stim feeding within minutes.
/// [Berrios et al 2021] Food cue drop AgRP. H.l.glu to H.dm.g to H.arc.agrp.
/// [Betley et al 2013] AgRP projection distinct subpops. AgRP increase eating
///   P.bst.a, H.pv, H.l and some T.pv, but no eating effect to S.a, R.pb.l.
/// [Campos et al 2016] H.arc.agrp to R.pb.el.cgrp delay meal termination.
/// [Essner et al 2017] H.arc.agrp suppress R.pb inhibition of amylin, CCK,
///   LiCl, but not LPS. H.arc.agrp ko starvation.
/// [Garfield et al 2015] H.arc.agrp to H.pv, P.bst.a, H.l, T.pv. H.pv.mc4 to
///   R.pb.l, R.nts, R.dmv. Only subset H.pv.mc4 to R.pb.l. H.pv.mc4 to R.pb.l
///   stim reduce eating, RTPP 75% when hungry.
/// [Korchynska et al 2022] H.pv.da (A14) to S.ls.sst active period
///   locomotion, H.scn.
/// [Li MM et al 2910] Satiety H.pv.mc4r and H.pv.pdyn distinct with distinct
///   targets in R.pb. Each approx 50%. Other H.pv distinct from both.
///   Both H.pv targets distinct from CGRP.
/// [Palmiter 2018] H.arc.agrp to R.pb.cgrp for visceral malaise. R.pb.cgrp ko
///   avoid starvation from H.arc.agrp ko. R.pb.cgrp active after large meal.
///   R.nts.cck, R.nts.dbh -> R.pb.cgrp.
/// [Roman et al 2017] R.nts.cck -> R.pb, H.pv both halt eating. After meal
///   R.nts.cck activated CTA, CPA.
/// [Ryan et al 2017] R.nts, H.pv.oxt to R.pb.oxt suppress drinking with R.nts
///   stronger affect. R.pb.oxt not affect eating or salt.
/// [Want et al 2023] R.pb cck -> H.pv predatory, looming defense.
/// 
/// ## S.v references
/// 

pub struct MotiveEat {
    // food_zone derives from H.l (Forage)
    is_food_zone: TimeoutValue<bool>,

    // R.pb CCK from gut satiation
    is_cck_sated: TimeoutValue<bool>,

    // H.arc AgRP hunger motivation
    is_agrp_hungry: TimeoutValue<bool>,

    // H.pv satiation
    is_pv_sated: TimeoutValue<bool>,

    sated: f32,

    // R.pb CGRP bitter
    cgrp_bitter: DecayValue,

    // R.pb CGRP LPS
    is_cgrp_sick: TimeoutValue<bool>,
}

impl MotiveEat {
    fn is_food_zone(&self) -> bool {
        self.is_food_zone.value_or(false)
    }

    // food_zone is set by H.l (Forage)
    pub(super) fn set_food_zone(&mut self, value: bool) {
        self.is_food_zone.set(value);
    }

    pub fn sated(&self) -> f32 {
        self.sated
    }

    /// gut satiation is short term satiation.
    /// R.pb CCK from gut satiation.
    #[inline]
    pub fn is_sated_gut(&self) -> bool {
        self.is_cck_sated.value_or(false)
    }

    /// AgRP hunger - seek hunger
    #[inline]
    pub fn is_hungry_agrp(&self) -> bool {
        self.is_agrp_hungry.value_or(false)
    }

    /// H.pv satiation - driven by H.arc
    #[inline]
    pub fn is_sated_pv(&self) -> bool {
        self.is_pv_sated.value_or(true)
    }

    /// R.pb LPS sick
    #[inline]
    pub fn is_sick(&self) -> bool {
        self.is_cgrp_sick.value_or(false)
    }

    #[inline]
    pub fn is_eat(&self) -> bool {
        ! self.is_sated_gut() && ! self.is_sick()
    }

    fn pre_update(&mut self) {
        self.is_food_zone.update();
    }

    fn update_hunger(
        &mut self,
        body_eat: &BodyEat,
        sleep: &Sleep
    ) {
        // R.pb CCK sated signal extends body signal
        if body_eat.sated_cck() > 0. {
            self.is_cck_sated.set(true);
        }

        // H.arc AgRP hunger
        if self.is_food_zone() {
            // H.l food zone immediately suppresses AgRP
            self.is_agrp_hungry.set(false);
        } else if body_eat.glucose() > 0.75 {
            self.is_agrp_hungry.set(false);
        } else if body_eat.glucose() < 0.1 {
            self.is_agrp_hungry.set(true);
        } else if sleep.is_forage() {
            // Circadian forage time in morning activates AgRP
            self.is_agrp_hungry.set(true);
        }

        if self.is_cck_sated.value_or(false) {
            self.sated = 1.;
        } else {
            self.sated = body_eat.glucose();
        }

        // H.pv MC4 satiation, defaults to true
        if self.is_hungry_agrp() {
            self.is_pv_sated.set(false);
        } else if body_eat.sated_cck() > 0. {
            self.is_pv_sated.set(true);
        }

        // R.pb CGRP for bitter/CCK
        let mut cgrp_bitter: f32 = 0.;
        if self.is_sated_gut() {
            cgrp_bitter = 1.;
        }

        cgrp_bitter = cgrp_bitter.max(body_eat.bitter());

        // TODO: capsaicin and pain s/b treated like bitter

        // AgRP suppresses CCK and bitter
        // TODO: But H.arc.AgRP not directly to R.pb CGRP?
        if self.is_hungry_agrp() {
            cgrp_bitter = 0.;
        }

        // S.am suppression of cgrp_bitter

        self.cgrp_bitter.set_max(cgrp_bitter);

        // LPS sickness

        if body_eat.sickness() > 0. {
            self.is_cgrp_sick.set(true);
        }
    }
}

impl Default for MotiveEat {
    fn default() -> Self {
        // R.pb activation has long sustain ~30s

        Self { 
            is_food_zone: Default::default(), 
            is_cck_sated: TimeoutValue::new(Seconds(30.)),
            is_agrp_hungry: TimeoutValue::new(Seconds(30.)),
            is_pv_sated: TimeoutValue::new(Seconds(30.)),
            sated: 0.,

            cgrp_bitter: DecayValue::new(Seconds(30.)),
            is_cgrp_sick: TimeoutValue::new(Seconds(30.)),
        }
    }
}

///
/// Update eat threat
/// 
fn update_eat_threat(
    mut eat: ResMut<MotiveEat>,
    body_eat: Res<BodyEat>,
) {
}

fn update_eat(
    mut eat: ResMut<MotiveEat>,
    body_eat: Res<BodyEat>,
    mut hind_eat: ResMut<HindEat>,
    sleep: Res<Sleep>,
) {
    eat.pre_update();

    eat.update_hunger(body_eat.get(), sleep.get());

    if ! sleep.is_wake() {
        return;
    } else if ! eat.is_food_zone() {
        // is_food_zone is from Forage (H.l) 
        return;
    }

    if ! eat.is_sated_gut() {
        hind_eat.eat();
    }

    // TODO: check current moving
}

pub struct MotiveEatPlugin;

impl Plugin for MotiveEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindEatPlugin>(), "MotiveEat requires HindEat");

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Dwell>(app, Seconds(4.));

        app.insert_resource(MotiveEat::default());

        app.system(Tick, update_eat);
    }
}
