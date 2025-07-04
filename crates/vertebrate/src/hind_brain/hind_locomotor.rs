use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use util::random::random_uniform;

use crate::{
    body::{Body, BodyPlugin}, hind_brain::Serotonin, hypothalamus::{Motive, Wake}, util::{DecayValue, Seconds, Ticks, TimeoutValue, Turn}
};

use super::{r4_startle::StartleR4, ArtrR2, HindEat};

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
// Cregg et al 2020 - R.mrs.gi turn/stop from OT.i contra, H.zi ipsi, M.pot ipsi, 
//   CB contra, C.moss bilat. R.mrs.gi also slow locomotion. Inh R.mrs.gi speeds.
//   LPGi project bilaterally
// Daghfous et al 2016 - Lamprey sustained R.rs via NMDA and Ca2+
// Deichler et al, 2020 - OT seek to R.my; OT avoid to R.pn
// do Carmo et al 2018 - C-start 45 deg.
// Doykos et al 2020 - OT seek to R.pn.c; only from S.nr, C.m2, CB, H.zi
// Dunn et al 2016 - OT.pv to M-cell, R.mc4, R.mc5/6. 
//   Explore r3 ARTR, r4-6 R.mrs CB.io. ARTR to R.mrs
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
// Martin-Cortecero et al 2023 - OT.l whisker to R.pn.c, R.mrs.gi, R.7n
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
// Xie et al 2021 - OT pitx2 (turn) to R.my.irt, R.mrs.gi, R.my.pcrt
// Zhu and Goodhill 2023 - Zebrafish hunting R.rs gad13
// Zwaka et al - OT bias M-cell turn - avoid barrier

// TODO: possibly ARRN sufficiently different that shouldn't be included.
// Hind


fn update_hind_move(
    mut hind_move: ResMut<HindMove>,
    hind_eat: Res<HindEat>,
    mut body: ResMut<Body>,
    wake: Res<Motive<Wake>>,
) {
    hind_move.pre_update();

    // Acoustic startle in r4 Mauthner cells is an immediate reflex
    if hind_move.update_startle(body.get_mut()) {
    } else if ! wake.is_active() {
    } else if hind_eat.is_active() {
        // lateral inhibition by hindbrain eating circuits
    } else {
        hind_move.update_voluntary_move(body.get_mut());
    }
}


/// Currently using ARS, MRS, PRS similar to Lamprey naming 
/// convention: ARRN, MRRN, PRRN
/// 
/// HindLocomotion includes spinal cord integration
/// 
/// M.nmlf - midbrain
/// R.ars - r1-r3 (pons) - ARRN, pn.o
/// R.mrs - r4-r6 (medulla) - MRRN, M-cell, pn.c, my.gi, my.lgi
/// R.prs - r7-r8 (medulla) - PRRN
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
    optic_mb: OpticMid,

    // r1.a - sensory integration/anterior turns
    ante_r1: AnteR1,

    // r2 ARTR/HBO - hindbrain oscillator - random walk
    artr_r2: ArtrR2,

    // r4 Mauthner cell - acoustic startle escape
    startle_r4: StartleR4,

    // r5/r6 MRS/MRRN - Zebrafish MiD2
    // mammal LPGi
    forward_r5: ForwardR5,

    // r5/r6 - Zebrafish RoV3, MiV1, MiV2
    // mammal Gi
    turn_r5: TurnR5,

    // S.nr or lateral (eat) disable of movement
    is_disable: TimeoutValue<bool>,

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
            optic_mb: OpticMid::new(),

            ante_r1: AnteR1::new(),

            artr_r2: ArtrR2::new(),

            startle_r4: StartleR4::new(),

            forward_r5: ForwardR5::new(),
            turn_r5: TurnR5::new(),

            is_disable: TimeoutValue::default(),

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

    #[inline]
    pub fn is_active(&self) -> bool {
        ! self.action_kind().is_stop()
    }

    #[inline]
    pub fn is_obstacle(&self) -> bool {
        self.action_kind().is_obstacle()
    }

    #[inline]
    pub fn is_avoid(&self) -> bool {
        self.action_kind().is_avoid()
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.action_kind().is_stop()
    }

    //
    // external updates
    //

    ///
    /// Optic locomotion: nMLF
    /// 
    #[inline]
    pub fn optic(&mut self) -> &mut OpticMid {
        &mut self.optic_mb
    }

    ///
    /// Startle R4 Mauthner cell
    /// 
    #[inline]
    pub fn startle(&mut self) -> &mut StartleR4 {
        &mut self.startle_r4
    }

    ///
    /// R1.a anterior hindbrain - modeled as weak
    /// 
    #[inline]
    pub fn ante(&mut self) -> &mut AnteR1 {
        &mut self.ante_r1
    }

    ///
    /// R2.artr anterior hindbrain turning region
    /// 
    #[inline]
    pub fn artr(&self) -> &ArtrR2 {
        &self.artr_r2
    }

    #[inline]
    pub fn forward(&mut self, _value: f32) {
        self.forward_r5.roam();
    }

    #[inline]
    pub fn roam(&mut self) {
        self.forward_r5.roam();
    }

    #[inline]
    pub fn seek(&mut self) {
        self.forward_r5.seek();
    }

    #[inline]
    pub fn avoid(&mut self) {
        self.forward_r5.avoid();
    }

    #[inline]
    pub fn halt(&mut self) {
        self.forward_r5.halt();
    }

    #[inline]
    pub fn turn(&mut self, turn: impl Into<Turn>) {
        self.turn_r5.turn(turn.into());
    }

    #[inline]
    pub fn turn_if_new(&mut self, turn: impl Into<Turn>) {
        self.turn_r5.turn_if_new(turn.into());
    }

    //
    // tick updates
    //

    fn pre_update(&mut self) {
        self.action.update();
        self.ante_r1.update();
        self.artr_r2.update();
        self.optic_mb.update();
        self.is_disable.update();

        self.ss_forward = 0.;
        self.ss_left = 0.;
        self.ss_right = 0.;

        self.is_freeze = false;

        self.mo_forward = 0.;
        self.mo_left = 0.;
        self.mo_right = 0.;
    }

    ///
    /// Acoustic startle driving Mauthner cells in r4
    /// 
    fn update_startle(&mut self, body: &mut Body) -> bool {
        self.pre_update();

        // Startle Mauthner cell in r4
        self.startle_r4.update(body);

        self.ss_forward = self.startle_r4.ss_forward().max(self.ss_forward);
        self.ss_left = self.startle_r4.ss_left().max(self.ss_left);
        self.ss_right = self.startle_r4.ss_right().max(self.ss_right);

        if self.action.allow_startle() {
            if let Some(action) = self.startle_r4.next_action() {
                self.action = action;
                self.send_action(body);
                return true;
            }
        }

        false
    }

    ///
    /// Voluntary movement
    /// 
    fn update_voluntary_move(&mut self, body: &mut Body) {
        // S.nr top-down disable
        if self.is_disable.value_or(false) {
            return;
        }

        let mut turn = Turn::Unit(0.);

        let mut kind = if self.ante_r1.is_roam() {
            MoveKind::Roam
        } else {
            MoveKind::None
        };
        
        // TODO: should be driven by outside such as H.sum/MLR
        kind = self.forward_r5.take().or(kind);

        // ARTR - R1.a
        turn = self.artr_r2.next_turn().unwrap_or(turn);

        // thigmotaxis - R1.a
        kind = self.ante().action().unwrap_or(kind);

        if random_uniform() < 0.5 {
            turn = Turn::Unit(0.);
        }

        // optic - nMLF
        kind = self.optic().action().unwrap_or(kind);

        // r6 chx10 overrides nmlf 
        turn = self.turn_r5.take().unwrap_or(turn);
        
        // CPG can only change on certain phases
        if self.action.allow_override(kind) {
            if let Some(action) = kind.action(turn) {
                self.artr_r2.on_turn(turn);
                self.action = action;
            }
        }

        if self.action.is_active() {
            self.send_action(body);
        }
    }

    fn send_action(&mut self, body: &mut Body) {
        let Action { speed, turn, timeout, elapsed, .. } = self.action;

        let turn_per_tick = Turn::Unit(turn.to_unit() / timeout.ticks().max(1) as f32);

        body.action(
            speed, 
            turn_per_tick,
            Ticks(timeout.ticks() - elapsed.ticks())
        );

        self.mo_forward = speed;
        let turn = turn.to_unit();

        let turn_value = match self.action.kind {
            MoveKind::Startle => 0.5 + 2. * turn.abs(),
            MoveKind::Escape(_) => 0.75,
            MoveKind::UTurn(_) => 0.75,
            _ => (2. * turn.abs()).min(0.49),
        };

        if turn < -1.0e-3 {
            self.mo_left = turn_value;
        } else if turn > 1.0e-3 {
            self.mo_right = turn_value;
        }
    }

    //
    // UI updates
    //

    #[inline]
    pub fn action_kind(&self) -> MoveKind {
        if self.action.is_active() {
            self.action.kind
        } else {
            MoveKind::None
        }
    }

    #[inline]
    pub fn set_ss_forward(&mut self, value: f32) {
        self.ss_forward = self.ss_forward.max(value);
    }

    #[inline]
    pub fn set_ss_left(&mut self, value: f32) {
        self.ss_left = self.ss_left.max(value);
    }

    #[inline]
    pub fn set_ss_right(&mut self, value: f32) {
        self.ss_right = self.ss_right.max(value);
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

impl Default for HindMove {
    fn default() -> Self {
        HindMove::new()
    }
}

pub struct AnteR1 {
    roam: DecayValue,
    attract: DecayValue,
    attract_kind: MoveKind,
}

impl AnteR1 {
    fn new() -> Self {
        let mut attract = DecayValue::default();
        attract.set_threshold(0.4);

        let roam  = DecayValue::new(Seconds(2.));

        Self {
            roam,
            attract,
            attract_kind: MoveKind::None,
        }
    }

    fn update(&mut self){
        self.attract.update();
        self.roam.update();
    }

    fn is_roam(&self) -> bool {
        self.roam.is_active()
    }

    pub fn roam(&mut self, value: f32) {
        self.roam.add(value);
    }

    pub fn thigmotaxis(&mut self, turn: Turn) {
        self.attract.set_max(0.8);

        self.attract_kind = MoveKind::Thigmotaxis(turn);
    }

    fn action(&self) -> Option<MoveKind> {
        // TODO: disable with MOR
        if self.attract.is_active() {
            Some(self.attract_kind)
        } else if self.roam.is_active() {
            Some(MoveKind::Roam)
        } else {
            None
        }
    }
}

pub struct OpticMid {
    escape: DecayValue,
    escape_kind: MoveKind,
    u_turn: DecayValue,
}

impl OpticMid {
    fn new() -> Self {
        let mut escape = DecayValue::default();
        escape.set_threshold(0.4);

        Self {
            escape,
            escape_kind: MoveKind::None,
            u_turn: DecayValue::new(Seconds(1.)),
        }
    }

    fn update(&mut self){
        self.escape.update();
        self.u_turn.update();
    }

    pub fn escape(&mut self, turn: Turn) {
        self.escape.set_max(1.);

        if ! self.u_turn.is_active() {
            self.escape_kind = MoveKind::Escape(turn);
        }
    }

    pub fn u_turn(&mut self, turn: Turn) {
        self.escape.set_max(1.);
        self.u_turn.set_max(1.);
        self.escape_kind = MoveKind::UTurn(turn);
    }

    fn action(&self) -> Option<MoveKind> {
        if self.escape.is_active() {
            Some(self.escape_kind)
        } else {
            None
        }
    }
}

struct _AvoidMrrn {
}

impl _AvoidMrrn {
    fn _new() -> Self {
        Self {
        }
    }
}

struct ForwardR5 {
    kind: MoveKind,
}

impl ForwardR5 {
    fn new() -> Self {
        Self {
            kind: MoveKind::None
        }
    }

    pub fn roam(&mut self) {
        self.kind = self.kind.roam();
    }

    pub fn seek(&mut self) {
        self.kind = self.kind.seek();
    }

    pub fn avoid(&mut self) {
        self.kind = self.kind.avoid();
    }

    pub fn halt(&mut self) {
        self.kind = self.kind.halt();
    }

    fn take(&mut self) -> MoveKind {
        let kind = self.kind;
        self.kind = MoveKind::None;

        kind
    }
}

struct TurnR5 {
    turn: Option<Turn>
}

impl TurnR5 {
    fn new() -> Self {
        Self {
            turn: None,
        }
    }

    fn turn(&mut self, turn: Turn) {
        self.turn = Some(turn);
    }

    fn turn_if_new(&mut self, turn: Turn) {
        if self.turn.is_none() {
            self.turn = Some(turn);
        }
    }

    fn take(&mut self) -> Option<Turn> {
        self.turn.take()
    }
}

struct _TurnMyLgi {
}

impl _TurnMyLgi {
    fn _new() -> Self {
        Self {
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct _ActionPair {
    kind: MoveKind,
    turn: Turn,
}

impl _ActionPair {
    pub(super) fn _new(kind: MoveKind, turn: Turn) -> Self {
        Self {
            kind,
            turn
        }
    }
}


#[derive(Clone, Debug)]
pub(super) struct Action {
    kind: MoveKind,
    speed: f32,
    turn: Turn,
    timeout: Ticks,
    elapsed: Ticks,
}

impl Action {
    pub(super) fn new(kind: MoveKind, speed: f32, turn: Turn, time: impl Into<Ticks>) -> Self {
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
        Action::new(MoveKind::None, 0., Turn::Unit(0.), Seconds(1.))
    }

    fn update(&mut self) {
        self.elapsed = Ticks(self.elapsed.ticks() + 1);

        if ! self.is_active() {
            self.kind = MoveKind::None;
        }        
    }

    fn is_active(&self) -> bool {
        self.elapsed.ticks() < self.timeout.ticks()
    }

    fn allow_startle(&self) -> bool {
        if ! self.is_active() {
            return true;
        } else {
            match self.kind {
                MoveKind::UTurn(_) => false,
                MoveKind::Escape(_) => false,
                MoveKind::Startle => false,
                _ => true,
            }
        }
    }

    fn allow_override(&self, next: MoveKind) -> bool {
        if ! self.is_active() {
            return true;
        } else {
            match self.kind {
                MoveKind::Roam | MoveKind::Seek => {
                    match next {
                        MoveKind::UTurn(_) | MoveKind::Escape(_) | MoveKind::Startle => {
                            true
                        },
                        _ => false,
                    }
                },
                _ => false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveKind {
    None,
    Halt,
    Roam,
    Seek,
    Avoid,
    Thigmotaxis(Turn),
    Escape(Turn),
    UTurn(Turn),
    Startle,
}

impl MoveKind {
    fn roam(&self) -> MoveKind {
        match self {
            MoveKind::None => MoveKind::Roam,
            _ => *self
        }
    }

    fn seek(&self) -> MoveKind {
        match self {
            MoveKind::None => MoveKind::Seek,
            MoveKind::Roam => MoveKind::Seek,
            MoveKind::Thigmotaxis(_) => MoveKind::Seek,
            _ => *self
        }
    }

    fn avoid(&self) -> MoveKind {
        match self {
            MoveKind::None => MoveKind::Avoid,
            MoveKind::Roam => MoveKind::Avoid,
            MoveKind::Seek => MoveKind::Avoid,
            MoveKind::Thigmotaxis(_) => MoveKind::Seek,
            _ => *self
        }
    }

    fn halt(&self) -> MoveKind {
        match self {
            MoveKind::Startle => *self,
            MoveKind::Escape(_) => *self,
            _ => MoveKind::Halt
        }
    }

    fn is_stop(&self) -> bool {
        match self {
            MoveKind::None => true,
            MoveKind::Halt => true,
            _ => false
        }
    }

    fn is_obstacle(&self) -> bool {
        match self {
            MoveKind::Escape(_) => true,
            MoveKind::UTurn(_) => true,
            _ => false
        }
    }

    fn is_avoid(&self) -> bool {
        match self {
            MoveKind::Avoid => true,
            _ => false
        }
    }

    fn or(self, other: MoveKind) -> Self {
        match self {
            MoveKind::None => other,
            MoveKind::Halt => other,
            _ => self
        }
    }
    
    fn _turn(&self) -> Option<Turn> {
        match self {
            MoveKind::Thigmotaxis(turn) => Some(*turn),
            MoveKind::Escape(turn) => Some(*turn),
            MoveKind::UTurn(turn) => Some(*turn),
            _ => None,
        }
    }

    fn _speed(&self) -> f32 {
        match self {
            MoveKind::None => 0.,
            MoveKind::Halt => 0.,
            MoveKind::Thigmotaxis(_) => 0.5,
            MoveKind::Roam => 0.5,
            MoveKind::Seek => 0.5,
            MoveKind::Avoid => 0.75,
            MoveKind::Escape(_) => 0.75,
            MoveKind::UTurn(_) => 0.75,
            MoveKind::Startle => 1.0,
        }
    }

    fn _is_random_turn(&self) -> bool {
        match self {
            MoveKind::Roam => true,
            _ => false,
        }
    }

    fn action(&self, turn: Turn) -> Option<Action> {
        match self {
            MoveKind::None => None,
            MoveKind::Halt => {
                Some(Action::new(*self, 0., turn, Seconds(1.)))
            }
            MoveKind::Roam | MoveKind::Seek => {
                Some(Action::new(*self, 0.5, turn, Seconds(1.)))
            }
            MoveKind::Thigmotaxis(turn) => {
                Some(Action::new(*self, 0.5, *turn, Seconds(0.5)))
            }
            MoveKind::Avoid => {
                Some(Action::new(*self, 0.75, turn, Seconds(1.)))
            }
            MoveKind::Escape(turn) => {
                Some(Action::new(*self, 0.75, *turn, Seconds(1.)))
            }
            MoveKind::UTurn(turn) => {
                Some(Action::new(*self, 0.75, *turn, Seconds(2.)))
            }
            MoveKind::Startle => {
                Some(Action::new(*self, 1., Turn::Unit(0.12), Seconds(1.)))
            }
        }
    }
}

pub struct HindMovePlugin;

impl Plugin for HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotionPlugin requires BodyPlugin");

        let mut hind_move = HindMove::new();
        // hind_move.artr_r2 = Some(OscillatorArs::new());
        hind_move.startle_r4 = StartleR4::new();

        app.insert_resource(hind_move);
        app.init_resource::<Serotonin<ArtrR2>>();

        app.system(Tick, update_hind_move);
    }
}
