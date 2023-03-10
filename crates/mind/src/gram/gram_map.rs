use std::collections::HashMap;

use crate::Gram;

pub struct GramMap<V> {
    map: HashMap<Gram,V>,
}

impl<V> GramMap<V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Gram,V>::new()
        }
    }

    pub fn get(&self, key: &Gram) -> Option<&V> {
        let key = key.to_med();

        self.map.get(&key)
    }

    pub fn insert(&mut self, k: Gram, v: V) -> Option<V> {
        let k = k.to_med();

        self.map.insert(k, v)
    }
}