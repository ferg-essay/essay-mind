use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::{Body, BodyPlugin}, core_motive::{Motive, Wake}, util::{DecayValue, Seconds, Ticks, Turn}};

use super::{move_oscillator::OscillatorArs, move_startle::StartleMrs};

//
// Barandela et al 2023 - Lamprey early R.mrrn only from M.nmlf, DLR, MLR
// Berg et al 2018 - Zebrafish control of motoneurons, fast muscle, slow muscle
// Bhattachayya et al 2017 - M-cell: visual, tactile, vestibular, auditor, lateral-line
// Bouvier et al 2015 - Descending V2a/chx10 halt. Xenopus head contact halts movement.
// Brocard and Dubuc 2003 - Lamprey MRRN slow swimming, PRRN fast swimming
// Brocard et al 2010 - Lamprey unilateral MLR stim, bilateral MRRN
// Capantini et al 2017 - Lamprey M.pot to MRRN, PRRN (co, ip)
// Carbo-Tano et al 2023 - MLR equally ipsi/contra bilateral. Correlate LDT
//   Forward R.rs medial r7-8. Turning r3-6. r5 MiD2 bilat.
//   Lamprey, salamander, mammal correlation
//   r1-r2 ARRN/sRN r2-r3 R.pn.o
//   r5 MRRN/mRN, r5-r6 R.pn.c
//   r8 PRRN/iRN, r8 R.my
// Chen X et al 2018 - r3 R.artr. r1-2 specific turn sensor to behavior.
//   turn RoV3, MiV1, MiV2. r1-2 R.ars.
//   R.artr r3 correlate ipsi r5-6, CB.io and contra R.rs.a r1-2
// Cregg et al 2020 - R.gi turn/stop from OT.i contra, H.zi ipsi, M.pot ipsi, 
//   CB contra, C.moss bilat. R.gi also slow locomotion. Inh R.gi speeds.
//   LPGi project bilaterally
// Daghfous et al 2016 - Lamprey sustained R.rs via NMDA and Ca2+
// Deichler et al, 2020 - OT seek to R.my; OT avoid to R.pn
// do Carmo et al 2018 - C-start 45 deg.
// Doykos et al 2020 - OT seek to R.pn.c; only from S.nr, C.m2, CB, H.zi
// Dunn et al 2016 - OT.pv to M-cell, R.mc4, R.mc5/6. 
//   Explore r3 ARTR, r4-6 R.rs.m CB.io. ARTR to R.rs.m
//   ARTR 45% chain 5 same dir, long tail 14% 10
//   Two state Markov p(L to L) = 86% (varies 72% to 89%)
//   ARTR turn bias not movement initiation
// Gahtan et al 2005 - Hunting OT to nMLF MeLc and MeLr. J-turn < 10 deg (smaller than R-turn)
//   MeLc, MeLr ko disrupt hunting but not OKR and OMR
// Guan et al 2021 - V2a.chx10 descending from M-cell repeat and amplify
// Hacker 2020 - Mauthner ko abolish fast escape
// Huang et al 2013 - turn chx10 RoV3, MiV1, MiV2. Turn modulate underlying CPG
// Isa et al 2021 - Lamprey OT.i to MRRN. Mammal seek to M.pmrf, avoid to M.cnf
//   both to V.rn and CB.io, but not direct to R.rs.
// Karpenko et al 2020 - R.artr oscillator 20s period.
// Kohashi et al 2012 - early larva N5 head stim activate M-cell, later N5 shift
//   M-cell fire once per escape
// Koide et al 2018 - Slow avoid to R.tsn (r1-2)
// Marques et al 2018 - Zebrafish locomotor: J-turn, scoot, O-bend, sequence
// Martin-Cortecero et al 2023 - OT.l whisker to R.pn.c, R.my.gi, R.7n
// Medan and Preuss 2011 - DA to M-cell disrupt PPI startle
// Mu et al 2019 - Astrocyte suppress swimming
// Ocana et al 2015 - Lamprey Pa to M-cell, MRRN, OT.d, M.pot
// Orger et al 2011 - nMLR forward OMR, not turn
//   Forward: nMLF, RoL1, RoR1, RoM1, RoL2, MiR1, MiR2, MiT
//   Turn: RoM1r, RoV3, MiV1, MiV2
//   But groups heterogeneous
// Portugues and Engert 2009 - visual startle O-turn, not M-Cell
// Pujala and Koyama 2019 - early born escape dorsal, late born slow ventral lateral
//   dorsal: MiD2i, MiD3i, RoM2, RoM3, MiVi.d
//   ventral: Rov3, MiV1.v, MiV2
//   early escape large head displacement
// Ryczko et al 2010 - unilateral MLR to bilateral mRN (rostral) and iRN (caudal)
// Sankrithi and O'Malley 2010 - nMLR active head taps. Large neurons respond to
//   both slow and fast
// Severi et al 2014 - nMLR speed 0-10mm/s by bout duration, > 10mm/s by tail freq.
//   distinct slow bout (yaw 20 deg), fast (yaw 60 deg). Fast increase from 4% to 50%
// Severi et al 2021 - nMLF control swimming speed
// Severi et al 2015 - Zebrafish nMLF drive N.sp for optomotor
// Suzuki et al 2019 - Both OT.pn.co and OT.pn.ip to R.mrrn
// Wang and McLean 2014 - nMLF drive N.sp. nMLF temporally summation
// Wee et al 2019 - Oxytocin in R.rs sufficient for escape-like
// Wolf et al 2023 - Zebrafish ARTR with Ising model and mean-field theory.
// Xie et al 2021 - OT pitx2 (turn) to R.my.irt, R.my.gi, R.my.pcrt
// Zhu and Goodhill 2023 - Zebrafish hunting R.rs gad13
// Zwaka et al - OT bias M-cell turn - avoid barrier

// TODO: possibly ARRN sufficiently different that shouldn't be included.
// Hind

/// Currently using ARS, MRS, PRS similar to Lamprey naming 
/// convention: ARRN, MRRN, PRRN
/// 
/// HindLocomotion includes spinal cord integration
/// 
/// M.nmlf - midbrain
/// R.ars - r1-r3
/// R.mrs - r4-r6
/// R.prs - r7-r8
/// 
/// M.nmlf, R.mrs and R.prs are all spinal projecting, meaning that
/// conflicts may be resolved at the spinal level.
/// 
/// R.ars is not by itself spinal-projecting, but uses R.mrs.
/// 
/// MRS splits forward from turning. Different upstream drivers produce
/// forward drives from other upstream turn drivers.
/// 
/// Several action paths:
/// midbrain opto-motor: nMLF
/// r1-2 sensory integration
/// r3 levy walk: ARTR/hind brain oscillator
/// r4 startle: Mauthner cells
/// r5-6 MLR/DLR and OT turn: MRRN
/// 
pub struct HindMove {
    // mid - nMLF - opto-motor, OKR, OMR, visual hunting, phototaxis, looming
    optic_mid: OpticMid,

    // r1 ARS/ARRN - sensory integration
    sensor_r1: SensorArs,

    // r3 ARTR/HBO - hindbrain oscillator - random walk
    oscillator_r3: Option<OscillatorArs>,

    // r4 Mauthner cell - acoustic startle escape
    startle_r4: Option<StartleMrs>,

    // r5/r6 MRS/MRRN - Zebrafish MiD2
    // mammal LPGi
    forward_r5: ForwardMrs,

    // r5/r6 - Zebrafish RoV3, MiV1, MiV2
    // mammal Gi
    turn_r6: TurnMrs,

    action: Action,
    
    ss_forward: f32,
    ss_left: f32,
    ss_right: f32,

    // output for UI
    is_freeze: bool,

    mo_forward: f32,
    mo_left: f32,
    mo_right: f32,
}

impl HindMove {
    // transition between slow twitch and fast twitch
    pub const SLOW_FAST : f32 = 0.5;
    pub const MCELL : f32 = 1.0;

    fn new() -> Self {
        Self {
            optic_mid: OpticMid::new(),

            sensor_r1: SensorArs::new(),

            oscillator_r3: None,

            startle_r4: None,

            forward_r5: ForwardMrs::new(),
            turn_r6: TurnMrs::new(),

            action: Action::none(),

            ss_forward: 0.0,
            ss_left: 0.0,
            ss_right: 0.0,

            is_freeze: false,

            mo_forward: 0.0,
            mo_left: 0.0,
            mo_right: 0.0,
        }
    }

    fn update(&mut self, body: &mut Body, wake: &Motive<Wake>) {
        self.pre_update();

        // Startle Mauthner cell in r4
        if let Some(startle) = &mut self.startle_r4 {
            startle.update(body);

            self.ss_forward = startle.ss_forward().max(self.ss_forward);
            self.ss_left = startle.ss_left().max(self.ss_left);
            self.ss_right = startle.ss_right().max(self.ss_right);

            if self.action.allow_startle() {
                if let Some(action) = startle.next_action() {
                    self.action = action;
                    self.send_action(body);
                    return;
                }
            }
        }

        if self.action.is_active() {
            self.send_action(body);
            return;
        } else if ! wake.is_active() {
            return;
        }

        let mut turn = Turn::Unit(0.);
        let mut kind = ActionKind::None;
        
        // TODO: should be driven by outside
        kind = ActionKind::Roam;

        // ARTR in ARS r3 has lowest-priority turn
        if let Some(oscillator) = &mut self.oscillator_r3 {
            if let Some(next_turn) = oscillator.next_turn() {
                turn = next_turn;
            }
        }

        // optic - nMLF
        if let Some(optic_kind) = self.optic().action() {
            kind = optic_kind;
        }

        if let Some(action) = kind.action(turn) {
            self.action = action;
        }

        if self.action.is_active() {
            self.send_action(body);
        }
    }

    fn send_action(&mut self, body: &mut Body) {
        let Action { speed, turn, timeout, elapsed, .. } = self.action;

        // println!("Turn {:?} {:?}", self.action.kind, turn);

        let turn_per_tick = Turn::Unit(turn.to_unit() / timeout.ticks().max(1) as f32);

        body.action(
            speed, 
            turn_per_tick,
            Ticks(timeout.ticks() - elapsed.ticks())
        );

        self.mo_forward = speed;
        let turn = turn.to_unit();

        let turn_value = match self.action.kind {
            ActionKind::Startle => 0.5 + 2. * turn.abs(),
            _ => (2. * turn.abs()).min(0.49),
        };

        if turn < -1.0e-3 {
            self.mo_left = turn_value;
        } else if turn > 1.0e-3 {
            self.mo_right = turn_value;
        }
    }

    fn pre_update(&mut self) {
        self.action.update();
        self.optic_mid.update();

        self.ss_forward = 0.;
        self.ss_left = 0.;
        self.ss_right = 0.;

        self.is_freeze = false;

        self.mo_forward = 0.;
        self.mo_left = 0.;
        self.mo_right = 0.;
    }

    //
    // external updates
    //

    #[inline]
    pub fn optic(&mut self) -> &mut OpticMid {
        &mut self.optic_mid
    }

    //
    // UI output
    //

    #[inline]
    pub fn ss_forward(&self) -> f32 {
        self.ss_forward
    }

    #[inline]
    pub fn ss_left(&self) -> f32 {
        self.ss_left
    }

    #[inline]
    pub fn ss_right(&self) -> f32 {
        self.ss_right
    }

    #[inline]
    pub fn is_freeze(&self) -> bool {
        self.is_freeze
    }

    #[inline]
    pub fn mo_forward(&self) -> f32 {
        self.mo_forward
    }

    #[inline]
    pub fn mo_left(&self) -> f32 {
        self.mo_left
    }

    #[inline]
    pub fn mo_right(&self) -> f32 {
        self.mo_right
    }
}

struct SensorArs {
}

impl SensorArs {
    fn new() -> Self {
        Self {
        }
    }
}

pub struct OpticMid {
    escape: DecayValue,
    escape_kind: ActionKind,
    u_turn: DecayValue,
}

impl OpticMid {
    fn new() -> Self {
        Self {
            escape: DecayValue::new(Ticks(1)),
            escape_kind: ActionKind::None,
            u_turn: DecayValue::new(Ticks(1)),
        }
    }

    fn update(&mut self){
        self.escape.update();
        self.u_turn.update();
    }

    pub fn escape(&mut self, turn: Turn) {
        self.escape.set_max(1.);
        self.escape_kind = ActionKind::Escape(turn);
    }

    pub fn u_turn(&mut self) {
        self.u_turn.set_max(1.);
    }

    fn action(&self) -> Option<ActionKind> {
        if self.escape.is_active() {
            Some(self.escape_kind)
        } else if self.u_turn.is_active() {
            Some(ActionKind::UTurn)
        } else {
            None
        }
    }
}

struct ForwardMrs {
}

impl ForwardMrs {
    fn new() -> Self {
        Self {
        }
    }
}

struct AvoidMrrn {
}

impl AvoidMrrn {
    fn new() -> Self {
        Self {
        }
    }
}

struct TurnMrs {
}

impl TurnMrs {
    fn new() -> Self {
        Self {
        }
    }
}

struct TurnMyLgi {
}

impl TurnMyLgi {
    fn new() -> Self {
        Self {
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct ActionPair {
    kind: ActionKind,
    turn: Turn,
}

impl ActionPair {
    pub(super) fn new(kind: ActionKind, turn: Turn) -> Self {
        Self {
            kind,
            turn
        }
    }
}


#[derive(Clone, Debug)]
pub(super) struct Action {
    kind: ActionKind,
    speed: f32,
    turn: Turn,
    timeout: Ticks,
    elapsed: Ticks,
}

impl Action {
    pub(super) fn new(kind: ActionKind, speed: f32, turn: Turn, time: impl Into<Ticks>) -> Self {
        let timeout = time.into();

        Self {
            kind,
            speed,
            turn,
            timeout,
            elapsed: Ticks(0),
        }
    }

    pub(super) fn none() -> Self {
        Action::new(ActionKind::None, 0., Turn::Unit(0.), Seconds(1.))
    }

    fn update(&mut self) {
        self.elapsed = Ticks(self.elapsed.ticks() + 1);
    }

    fn is_active(&self) -> bool {
        self.elapsed.ticks() < self.timeout.ticks()
    }

    fn allow_startle(&self) -> bool {
        if ! self.is_active() {
            return true;
        } else {
            match self.kind {
                ActionKind::None => true,
                ActionKind::Roam => true,
                ActionKind::Seek => true,
                ActionKind::UTurn => false,
                ActionKind::Escape(_) => false,
                ActionKind::Startle => false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ActionKind {
    None,
    Roam,
    Seek,
    Escape(Turn),
    UTurn,
    Startle,
}

impl ActionKind {
    fn speed(&self) -> f32 {
        match self {
            ActionKind::None => 0.,
            ActionKind::Roam => 0.5,
            ActionKind::Seek => 0.5,
            ActionKind::Escape(_) => 0.75,
            ActionKind::UTurn => 0.55,
            ActionKind::Startle => 1.0,
        }
    }

    fn action(&self, turn: Turn) -> Option<Action> {
        match self {
            ActionKind::None => None,
            ActionKind::Roam | ActionKind::Seek => {
                Some(Action::new(*self, 0.5, turn, Seconds(1.)))
            }
            ActionKind::Escape(turn) => {
                Some(Action::new(*self, 0.75, *turn, Seconds(1.)))
            }
            ActionKind::UTurn => {
                Some(Action::new(*self, 0.55, Turn::Unit(0.25), Seconds(2.)))
            }
            ActionKind::Startle => {
                Some(Action::new(*self, 1., Turn::Unit(0.12), Seconds(1.)))
            }
        }
    }
}

fn update_hind_move(
    mut hind_move: ResMut<HindMove>,
    mut body: ResMut<Body>,
    wake: Res<Motive<Wake>>,
) {
    hind_move.update(body.get_mut(), wake.get());
}

pub struct HindMovePlugin;

impl Plugin for HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotionPlugin requires BodyPlugin");

        let mut hind_move = HindMove::new();
        hind_move.oscillator_r3 = Some(OscillatorArs::new());
        hind_move.startle_r4 = Some(StartleMrs::new());

        app.insert_resource(hind_move);

        app.system(Tick, update_hind_move);
    }
}
