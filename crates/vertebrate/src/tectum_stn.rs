use essay_ecs::prelude::{App, Plugin};
use essay_tensor::Tensor;

use crate::{
    ach_attention::NucleusIsthmi,
    action::{ActionId, StriatumSnr}, 
    striatum_stn::{StriatumStn, Sense, Dopamine, DirectId}
};

pub struct TectumLocomotionStn {
    seek: TectumStnTurn,
    away: TectumStnTurn,
    away_odor: TectumStnTurn,
}

impl TectumLocomotionStn {
    fn new(plugin: &TectumStnPlugin) -> Self {
        Self {
            seek: TectumStnTurn::new(plugin),
            away: TectumStnTurn::new(plugin),
            away_odor: TectumStnTurn::new(plugin),
        }
    }

    pub fn seek(&mut self) -> &mut TectumStnTurn {
        &mut self.seek
    }

    pub fn away(&mut self) -> &mut TectumStnTurn {
        &mut self.away
    }

    pub fn away_odor(&mut self) -> &mut TectumStnTurn {
        &mut self.away_odor
    }

    pub fn update(&mut self) {
        self.seek.update();
        self.away.update();
        self.away_odor.update();
    }
}

pub struct TectumStnTurn {
    actions_d: TectumStnActions,
    actions_i: TectumStnActions,
    striatum: StriatumStn,
    ach_attention: Option<NucleusIsthmi>,

    last_action: Option<ActionId>,
}

impl TectumStnTurn {
    fn new(plugin: &TectumStnPlugin) -> Self {
        /*
        let striatum = if plugin.is_striatum {
            Some(StriatumStn::new())
        } else {
            None
        };
        */

        let ach_attention = if plugin.is_ni {
            Some(NucleusIsthmi::new())
        } else {
            None
        };

        let striatum = StriatumStn::new();

        let mut turn = Self {
            actions_d: TectumStnActions::new(),
            actions_i: TectumStnActions::new(),
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
        
        assert_eq!(id.i(), self.actions_d.actions.len());

        let id_d = self.striatum.direct_mut().add_action(id, name);

        self.actions_d.actions.push(TectumAction::new(id, id_d, turn));

        if let Some(attention) = &mut self.ach_attention {
            attention.add_action(id, name);
        }
    }

    pub fn turn(&mut self, turn: Turn, value: f32) {
        let action = &mut self.actions_d.actions[turn.id().i()];

        action.value = value;

        let id_d = action.id_d;

        self.striatum.direct_mut().sense(id_d, Sense::High);
    }

    pub fn action(&self) -> Option<Turn> {
        if let Some(action) = &self.last_action {
            Some(self.actions_d.actions[action.i()].turn)
        } else {
            None
        }
    }

    pub fn action_copy(&mut self, turn: Turn) {
        let id_d = self.actions_d.actions[turn.id().i()].id_d;

        let value = 1.;

        self.striatum.direct_mut().attend(id_d, Sense::High);

        if let Some(ach_attention) = &mut self.ach_attention {
            ach_attention.action_copy(turn.id(), value);
        }
    }

    fn update(&mut self) {
        self.last_action = None;

        self.striatum.update(
            Dopamine::High, 
            &mut self.actions_d, 
            &mut self.actions_i
        );

        /*
        if let Some(mut ach_attention) = self.ach_attention.take() {
            ach_attention.update(self);

            self.ach_attention = Some(ach_attention);
        }
        */

        let best_d = self.actions_d.best_action();
        let best_i = self.actions_i.best_action();

        // TODO: softmax
        self.last_action = best_d;
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

    fn best_action(&mut self) -> Option<ActionId> {
        let mut best_sense = 0.;
        //let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let value = item.value;
            item.value = 0.; // sense may also be leaky-accumulative

            let noise = 1. + random() * 0.01;
            let scaled_sense = value * item.ach * item.snr * noise;

            if best_sense < scaled_sense {
                //second = best_sense;
                best_sense = scaled_sense;
                best = Some(item.id);
            }
        }

        best
    }
}

impl StriatumSnr for TectumStnActions {
    fn attend(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].snr = value;
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

struct TectumAction {
    id: ActionId,
    id_d: DirectId,
    turn: Turn,
    value: f32,
    ach: f32,
    snr: f32,
}

impl TectumAction {
    fn new(id: ActionId, id_d: DirectId, turn: Turn) -> Self {
        Self {
            id,
            id_d,
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
            Turn::Left => ActionId::new(0),
            Turn::Right => ActionId::new(1),
        }
    }
}

//fn update_tectum(mut tectum: ResMut<TectumLocomotion>) {
//    tectum.update();
//}

pub struct TectumStnPlugin {
    is_striatum: bool,
    is_ni: bool,
}

impl TectumStnPlugin {
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

impl Plugin for TectumStnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TectumLocomotionStn::new(self));

        // app.system(Tick, update_tectum);
    }
}
