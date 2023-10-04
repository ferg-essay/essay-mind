use essay_tensor::Tensor;

pub struct Striatum {
    actions: Vec<ActionItem>,
    selected: Option<ActionId>,
    threshold: f32,
}

impl Striatum {
    pub const DECAY : f32 = 0.75;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            selected: None,
            threshold: 0.5,
        }
    }

    pub fn add_action(
        &mut self, 
        name: impl AsRef<str>,
    ) -> ActionId {
        let len = self.actions.len();
        let id = ActionId(len);
        self.actions.push(ActionItem::new(id, name.as_ref()));

        id
    }

    pub fn set_value(&mut self, id: ActionId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.actions[id.0].value = value;
    }

    pub fn update(&mut self) -> Option<ActionId> {
        let mut best_value = 0.;
        let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let value = item.value;
            let dopamine = item.dopamine;
            item.value = 0.;
            item.dopamine = item.dopamine * Self::DECAY;

            let scaled_value = value * (1. + dopamine) + random() * 0.01;

            if best_value < scaled_value {
                second = best_value;
                best_value = scaled_value;
                best = Some(item.id);
            }
        }

        // TODO: reference for this heuristic (fixed block)
        self.selected = if self.threshold <= best_value - second {
            if let Some(id) = best {
                self.actions[id.0].dopamine = 1.;
            }

            best
        } else {
            None
        };

        self.selected
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionId(usize);

pub struct ActionItem {
    id: ActionId,
    _name: String,
    value: f32,
    dopamine: f32,
}

impl ActionItem {
    fn new(
        id: ActionId,
        name: &str,
    ) -> ActionItem {
        Self {
            id,
            _name: String::from(name),
            value: 0.,
            dopamine: 0.,
        }
    }
}
