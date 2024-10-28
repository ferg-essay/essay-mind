use core::{fmt, hash::Hash};
use std::{collections::HashMap, hash::Hasher};
use essay_ecs::app::{App, Plugin};
use util::label::DynKey;

use crate::util::DecayValue;

fn update_mid_action() {

}

pub struct MidActionTectum {
    actions: Vec<MidActionTectum>,
}

struct Action {
    value: DecayValue,
    excite: DecayValue,
    inhibit: DecayValue,
}


#[derive(Clone, Copy, Debug)]

pub struct ActionId(usize);

pub trait ActionKey : DynKey + fmt::Debug {
    fn box_clone(&self) -> Box<dyn ActionKey>;
}

impl PartialEq for dyn ActionKey {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn ActionKey {
}

impl Hash for dyn ActionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

// struct ActionBox : 

pub struct MidActionPlugin {
    key_map: HashMap<Box<dyn ActionKey>, ActionId>,
}

impl MidActionPlugin {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::default(),
        }
    }

    fn id(&mut self, key: impl ActionKey) -> ActionId {
        let key = key.box_clone();
        let len = self.key_map.len();

        *self.key_map.entry(key).or_insert(ActionId(len))
    }
}

impl Plugin for MidActionPlugin {
    fn build(&self, app: &mut App) {
    
    }
}