use super::action::ActionId;


///
/// modeling the nucleus isthmi, parabigeminal circuit with the optic tectum
/// 
/// Henriques PM, Rahman N, Jackson SE, Bianco IH. 
/// Nucleus Isthmi Is Required to Sustain Target Pursuit during Visually 
/// Guided Prey-Catching. Curr Biol. 2019 Jun 3
///

pub struct NucleusIsthmi {
    actions: Vec<AttentionItem>,
    decay: f32,
    threshold: f32,
}

impl NucleusIsthmi {
    pub const DECAY : f32 = 0.75;
    pub const THRESHOLD : f32 = 0.1;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            decay: Self::DECAY,
            threshold: Self::THRESHOLD,
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

    pub fn action_copy(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].action = value;
    }

    pub fn update(&mut self, target: &mut dyn Attention) {
        for item in &mut self.actions {
            let action = item.action;
            item.action = item.action * self.decay;

            if item.action < self.threshold {
                item.action = 0.;
            }

            target.attend(item.id, action);
        }
    }
}

pub trait Attention {
    fn attend(&mut self, id: ActionId, value: f32);
}

pub struct AttentionItem {
    id: ActionId,
    _name: String,
    action: f32,
}

impl AttentionItem {
    fn new(
        id: ActionId,
        name: &str,
    ) -> AttentionItem {
        Self {
            id,
            _name: String::from(name),
            action: 0.,
        }
    }
}
