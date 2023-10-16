use essay_tensor::Tensor;

use crate::tectum::ActionId;

pub struct StriatumAction {
    actions: Vec<ActionItem>,
    selected: Option<ActionId>,
    threshold: f32,
}

impl StriatumAction {
    pub const DECAY : f32 = 0.75;
    pub const INHIBIT : f32 = 0.2;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            selected: None,
            threshold: 0.5,
        }
    }

    pub fn add_action(
        &mut self, 
        id: ActionId,
        name: impl AsRef<str>,
    ) {
        assert_eq!(id.i(), self.actions.len());

        self.actions.push(ActionItem::new(id, name.as_ref()));
    }

    pub fn sense(&mut self, id: ActionId, sense: f32) {
        assert!(0. <= sense && sense <= 1.);

        self.actions[id.i()].sense = sense;
    }

    pub fn action_copy(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.i()].action = value;
    }

    pub fn update(&mut self, snr: &mut dyn StriatumSnr) {
        let mut best_sense = 0.;
        let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let sense = item.sense;
            let action = item.action;
            item.sense = 0.; // sense may also be accumulative
            item.action = item.action * Self::DECAY;

            let scaled_sense = sense * (1. + action) + random() * 0.01;

            if best_sense < scaled_sense {
                second = best_sense;
                best_sense = scaled_sense;
                best = Some(item.id);
            }
        }

        // TODO: reference for this heuristic (fixed block)
        let selected = if self.threshold <= best_sense - second {
            //if let Some(id) = best {
            //    self.actions[id.0].dopamine = 1.;
            //}

            best
        } else {
            None
        };

        self.selected = selected;

        for item in &self.actions {
            if Some(item.id) == selected {
                snr.attend(item.id, 1.);
            } else {
                snr.attend(item.id, Self::INHIBIT);
            }
        }
    }
}

pub trait StriatumSnr {
    fn attend(&mut self, id: ActionId, value: f32);
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub struct ActionItem {
    id: ActionId,
    _name: String,
    sense: f32,
    action: f32,
}

impl ActionItem {
    fn new(
        id: ActionId,
        name: &str,
    ) -> ActionItem {
        Self {
            id,
            _name: String::from(name),
            sense: 0.,
            action: 0.,
        }
    }
}
