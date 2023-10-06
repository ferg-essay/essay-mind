use essay_ecs::prelude::{App, Plugin, ResMut};
use essay_tensor::Tensor;
use mind_ecs::Tick;

use crate::{striatum_action::{StriatumAction, StriatumSnr}, ach_attention::{NucleusIsthmi, Attention}};

pub struct TectumLocomotion {
    toward: TectumTurn,
    away: TectumTurn,
}

impl TectumLocomotion {
    fn new(plugin: &TectumPlugin) -> Self {
        Self {
            toward: TectumTurn::new(plugin, "toward"),
            away: TectumTurn::new(plugin, "away"),
        }
    }

    pub fn toward(&mut self) -> &mut TectumTurn {
        &mut self.toward
    }

    pub fn away(&mut self) -> &mut TectumTurn {
        &mut self.away
    }

    pub fn update(&mut self) {
        self.toward.update();
        self.away.update();
    }
}

pub struct TectumTurn {
    name: String, 
    actions: Vec<TectumAction>,
    striatum: Option<StriatumAction>,
    ach_attention: Option<NucleusIsthmi>,

    last_action: Option<ActionId>,
}

impl TectumTurn {
    fn new(plugin: &TectumPlugin, name: &str) -> Self {
        //let left = striatum.add_action("turn-left");
        //let right = striatum.add_action("turn-right");

        let striatum = if plugin.is_striatum {
            Some(StriatumAction::new())
        } else {
            None
        };

        let ach_attention = if plugin.is_ni {
            Some(NucleusIsthmi::new())
        } else {
            None
        };

        let mut turn = Self {
            name: String::from(name),
            actions: Vec::new(),
            striatum,
            ach_attention,
            last_action: None,
        };

        turn.add_action(Turn::Left, "turn-left");
        turn.add_action(Turn::Right, "turn-right");

        turn
    }

    fn add_action(&mut self, turn: Turn, name: &str) {
        let id = turn.id();
        
        assert_eq!(id.0, self.actions.len());

        self.actions.push(TectumAction::new(id, turn, name));

        if let Some(striatum) = &mut self.striatum {
            striatum.add_action(id, name);
        }

        if let Some(attention) = &mut self.ach_attention {
            attention.add_action(id, name);
        }
    }

    pub fn turn(&mut self, turn: Turn, value: f32) {
        let action = &mut self.actions[turn.id().i()];

        action.value = value;

        if let Some(striatum) = &mut self.striatum {
            striatum.sense(turn.id(), value);
        }
    }

    pub fn action(&self) -> Option<Turn> {
        if let Some(action) = &self.last_action {
            Some(self.actions[action.i()].turn)
        } else {
            None
        }
    }

    pub fn action_copy(&mut self, turn: Turn) {
        let value = 1.;

        if let Some(striatum) = &mut self.striatum {
            striatum.action_copy(turn.id(), value);
        }

        if let Some(ach_attention) = &mut self.ach_attention {
            ach_attention.action_copy(turn.id(), value);
        }
    }

    fn update(&mut self) {
        self.last_action = None;

        if let Some(mut striatum) = self.striatum.take() {
            striatum.update(self);

            self.striatum = Some(striatum);
        }

        if let Some(mut ach_attention) = self.ach_attention.take() {
            ach_attention.update(self);

            self.ach_attention = Some(ach_attention);
        }

        let mut best_sense = 0.;
        let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let value = item.value;
            item.value = 0.; // sense may also be leaky-accumulative

            let scaled_sense = value * item.ach * item.snr * (1. + random() * 0.01);

            if best_sense < scaled_sense {
                second = best_sense;
                best_sense = scaled_sense;
                best = Some(item.id);
            }
        }

        // TODO: softmax
        self.last_action = best;
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

impl Attention for TectumTurn {
    fn attend(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].ach = 1. + value;
    }
}

impl StriatumSnr for TectumTurn {
    fn attend(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].snr = value;
    }
}

struct TectumAction {
    id: ActionId,
    _name: String,
    turn: Turn,
    value: f32,
    ach: f32,
    snr: f32,
}

impl TectumAction {
    fn new(id: ActionId, turn: Turn, name: &str) -> Self {
        Self {
            id,
            _name: String::from(name),
            turn,
            value: 0.,
            ach: 1.,
            snr: 1.,
        }
    }
}



#[derive(Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    fn id(&self) -> ActionId {
        match self {
            Turn::Left => ActionId(0),
            Turn::Right => ActionId(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionId(usize);

impl ActionId {
    pub fn i(&self) -> usize {
        self.0
    }
}

//fn update_tectum(mut tectum: ResMut<TectumLocomotion>) {
//    tectum.update();
//}

pub struct TectumPlugin {
    is_striatum: bool,
    is_ni: bool,
}

impl TectumPlugin {
    pub fn new() -> Self {
        Self {
            is_striatum: false,
            is_ni: false,
        }
    }

    pub fn striatum(self) -> Self {
        Self {
            is_striatum: true,
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
        app.insert_resource(TectumLocomotion::new(self));

        // app.system(Tick, update_tectum);
    }
}
