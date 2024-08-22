use super::tectum::ActionId;


///
/// modeling the nucleus isthmi, parabigeminal circuit with the optic tectum
/// 
/// Henriques PM, Rahman N, Jackson SE, Bianco IH. 
/// Nucleus Isthmi Is Required to Sustain Target Pursuit during Visually 
/// Guided Prey-Catching. Curr Biol. 2019 Jun 3
///

pub struct TectumAttention {
    actions: Vec<AttentionItem>,
    _decay: f32,
    _threshold: f32,
}

impl TectumAttention {
    pub const DECAY : f32 = 0.75;
    pub const THRESHOLD : f32 = 0.1;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            _decay: Self::DECAY,
            _threshold: Self::THRESHOLD,
        }
    }

    pub fn add_action(
        &mut self, 
        id: ActionId,
        name: impl AsRef<str>,
    ) {
        assert_eq!(id.i(), self.actions.len());

        self.actions.push(AttentionItem::new(id, name.as_ref()));
    }

    pub fn _action_copy(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()]._action = value;
    }

    pub fn _update(&mut self, target: &mut dyn _Attention) {
        for item in &mut self.actions {
            let action = item._action;
            item._action = item._action * self._decay;

            if item._action < self._threshold {
                item._action = 0.;
            }

            target.attend(item._id, action);
        }
    }
}

pub trait _Attention {
    fn attend(&mut self, id: ActionId, value: f32);
}

pub struct AttentionItem {
    _id: ActionId,
    _name: String,
    _action: f32,
}

impl AttentionItem {
    fn new(
        id: ActionId,
        name: &str,
    ) -> AttentionItem {
        Self {
            _id: id,
            _name: String::from(name),
            _action: 0.,
        }
    }
}
