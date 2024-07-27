use essay_ecs::{core::ResMut, prelude::{App, Plugin}};
use essay_tensor::Tensor;
use mind_ecs::PreTick;

use crate::{
    striatum::striatum::{Sense, StriatumId, StriatumSnr, StriatumStn}, util::{Angle, DecayValue, Heading}
};

use super::nucleus_isthmi::NucleusIsthmi;

pub struct TectumLocomotionStn {
    seek: TectumStnTurn,
    _away: TectumStnTurn,
    _away_odor: TectumStnTurn,
}

impl TectumLocomotionStn {
    fn new(plugin: &TectumPlugin) -> Self {
        let mut locomotion = Self {
            seek: TectumStnTurn::new(plugin),
            _away: TectumStnTurn::new(plugin),
            _away_odor: TectumStnTurn::new(plugin),
        };

        locomotion.seek.add_indirect(Turn::Left);

        locomotion
    }

    pub fn _seek(&mut self) -> &mut TectumStnTurn {
        &mut self.seek
    }

    pub fn _away(&mut self) -> &mut TectumStnTurn {
        &mut self._away
    }

    pub fn _away_odor(&mut self) -> &mut TectumStnTurn {
        &mut self._away_odor
    }

    pub fn _update(&mut self) {
        self.seek._update();
        self._away._update();
        self._away_odor._update();
    }
}

pub struct TectumStnTurn {
    actions_d: TectumStnActions,
    actions_i: TectumStnActions,
    striatum: StriatumStn,
    ach_attention: Option<NucleusIsthmi>,

    _last_action: Option<ActionId>,
    _last_default: Option<ActionId>,
}

impl TectumStnTurn {
    const COST : f32 = 0.005;

    fn new(plugin: &TectumPlugin) -> Self {
        let ach_attention = if plugin.is_ni {
            Some(NucleusIsthmi::new())
        } else {
            None
        };

        let mut striatum = StriatumStn::new();
        striatum.dopamine_mut().cost_decay(Self::COST * 0.5);

        let mut turn = Self {
            actions_d: TectumStnActions::new(),
            actions_i: TectumStnActions::new(),
            striatum,
            ach_attention,
            _last_action: None,
            _last_default: None,
        };

        turn.add_action(Turn::Left, "turn-left");
        turn.add_action(Turn::Right, "turn-right");

        turn
    }

    fn add_action(&mut self, turn: Turn, name: &str) {
        let id = turn.id();
        
        assert_eq!(id.i(), self.actions_d.actions.len());

        let id_d = self.striatum.direct_mut().add_action(name);

        self.actions_d.actions.push(TectumAction::new(id, id_d, turn));

        if let Some(attention) = &mut self.ach_attention {
            attention.add_action(id, name);
        }
    }

    fn add_indirect(&mut self, turn: Turn) {
        let id = turn.id();
        
        assert_eq!(id.i(), self.actions_i.actions.len());

        let id_i = self.striatum.indirect_mut().add_action("stub");

        self.actions_i.actions.push(TectumAction::new(id, id_i, turn));

        if let Some(attention) = &mut self.ach_attention {
            attention.add_action(id, "stub");
        }
    }

    pub fn _turn(&mut self, turn: Turn, value: f32) {
        let action = &mut self.actions_d.actions[turn.id().i()];

        action._value = value;

        let id_d = action._id_s;

        self.striatum.direct_mut().sense(id_d, Sense::High);
        self.striatum.dopamine_mut().effort(1.);
    }

    pub fn _action(&self) -> Option<Turn> {
        if let Some(action) = &self._last_action {
            Some(self.actions_d.actions[action.i()]._turn)
        } else {
            None
        }
    }

    pub fn _indirect(&self) -> bool {
        if let Some(_) = &self._last_default {
            true
        } else {
            false
        }
    }

    pub fn _action_copy(&mut self, turn: Turn) {
        let id_d = self.actions_d.actions[turn.id().i()]._id_s;

        let value = 1.;

        self.striatum.direct_mut().attend(id_d, Sense::High);

        if let Some(ach_attention) = &mut self.ach_attention {
            ach_attention._action_copy(turn.id(), value);
        }
    }

    fn _update(&mut self) {
        self._last_action = None;

        self.striatum.update(
            &mut self.actions_d, 
            &mut self.actions_i
        );

        let best_d = self.actions_d._best_action();
        let best_i = self.actions_i._best_action();

        // TODO: softmax
        self._last_action = best_d;
        self._last_default = best_i;
    }

    pub fn _default(&mut self) {
        self.striatum.indirect_mut().sense(Sense::Top);
        if self.actions_i.actions.len() > 0 {
            self.actions_i.actions[0]._value = 1.;
        }
    }

    pub fn _cost(&mut self) {
        self.striatum.dopamine_mut().cost(Self::COST);
    }

    pub fn _effort(&mut self) {
        self.striatum.dopamine_mut().effort(1.);
    }
}

pub struct TectumStnActions {
    actions: Vec<TectumAction>,
}

impl TectumStnActions {
    fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    fn _best_action(&mut self) -> Option<ActionId> {
        let mut best_sense = 0.;
        //let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let value = item._value;
            item._value = 0.; // sense may also be leaky-accumulative
            let snr = item.snr;
            item.snr = 0.;

            let noise = 1. + _random() * 0.01;
            let scaled_sense = value * item._ach * snr * noise;

            if best_sense < scaled_sense {
                //second = best_sense;
                best_sense = scaled_sense;
                best = Some(item._id);
            }
        }

        best
    }
}

impl StriatumSnr for TectumStnActions {
    fn attend(&mut self, id: StriatumId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].snr = value;
    }
}

fn _random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

struct TectumAction {
    _id: ActionId,
    _id_s: StriatumId,
    _turn: Turn,
    _value: f32,
    _ach: f32,
    snr: f32,
}

impl TectumAction {
    fn new(id: ActionId, id_s: StriatumId, turn: Turn) -> Self {
        Self {
            _id: id,
            _id_s: id_s,
            _turn: turn,
            _value: 0.,
            _ach: 1.,
            snr: 1.,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionId(usize);

impl ActionId {
    pub fn new(id: usize) -> Self {
        ActionId(id)
    }

    pub fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn id(&self) -> ActionId {
        match self {
            Turn::Left => ActionId::new(0),
            Turn::Right => ActionId::new(1),
        }
    }
}

pub struct TectumMap {
    pos_map: Vec<DecayValue>,
    neg_map: Vec<DecayValue>,
}

impl TectumMap {
    const THRESHOLD : f32 = 1.0e-1;
    pub const N : usize = 12;

    fn update(&mut self) {
        for value in &mut self.pos_map {
            value.update();
        }

        for value in &mut self.neg_map {
            value.update();
        }
    }

    pub fn neg(&mut self, dir: Heading, value: f32) {
        let da = 0.2 / Self::N as f32;
        let d1 = Heading::unit(dir.to_unit() + da);
        let d2 = Heading::unit(dir.to_unit() - da);

        //let i = (Self::N as f32 * dir.to_unit()).floor() as usize;
        //self.neg_map[i].set_max(value);

        let i = (Self::N as f32 * d1.to_unit()).floor() as usize;
        self.neg_map[i].set_max(value);

        let i = (Self::N as f32 * d2.to_unit()).floor() as usize;
        self.neg_map[i].set_max(value);
    }

    pub fn pos(&mut self, dir: Angle, value: f32) {
        let i = (Self::N as f32 * dir.to_unit()).floor() as usize;

        self.pos_map[i].set_max(value);
    }

    pub fn values(&self) -> Vec<f32> {
        let mut vec = Vec::<f32>::new();

        for (pos, neg) in self.pos_map.iter().zip(&self.neg_map) {
            let pos_value = pos.value();
            let neg_value = neg.value();

            vec.push(if neg_value < Self::THRESHOLD {
                0.5 + 0.5 * pos_value
            } else {
                0.5 - 0.5 * neg_value
            });
        }

        vec
    }
}

impl Default for TectumMap {
    fn default() -> Self {
        let mut pos_map = Vec::new();
        let mut neg_map = Vec::new();

        for _ in 0..Self::N {
            pos_map.push(DecayValue::new(0.5));
            neg_map.push(DecayValue::new(0.5));
        }

        Self { 
            pos_map,
            neg_map,
        }
    }
}

fn update_tectum(mut tectum_map: ResMut<TectumMap>) {
    tectum_map.update();
}

pub struct TectumPlugin {
    _is_striatum: bool,
    is_ni: bool,
}

impl TectumPlugin {
    pub fn new() -> Self {
        Self {
            _is_striatum: false,
            is_ni: false,
        }
    }

    pub fn striatum(self) -> Self {
        Self {
            _is_striatum: true,
            .. self
        }
    }

    pub fn ni(self) -> Self {
        Self {
            is_ni: true,
            .. self
        }
    }
}

impl Plugin for TectumPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TectumLocomotionStn::new(self));

        app.init_resource::<TectumMap>();
        app.system(PreTick, update_tectum);

        // app.system(Tick, update_tectum);
    }
}
