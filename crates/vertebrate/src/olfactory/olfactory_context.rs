use core::fmt;
use std::{collections::HashMap, hash::Hash};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, hippocampus::Engram64, mid_brain::SeekContext, util::base64_rev, world::WorldHex};

pub struct OdorPlaceMap<K: Eq + Hash> {
    loc_map: HashMap<K, OlfactoryItem>,
}

impl<K: Eq + Hash> OdorPlaceMap<K> {
    fn find(&self, key: &K) -> Option<&OlfactoryItem> {
        self.loc_map.get(key)
    }
} 

#[derive(Default)]
pub struct OdorContext {
    loc: OlfactoryItem,
}

#[derive(Clone, Default)]
struct OlfactoryItem {
    vec: Vec<usize>,
}

impl SeekContext for OdorContext {
    fn context(&self) -> Engram64 {
        let mut value : u64 = 0;

        for item in &self.loc.vec {
            value = (value << 6) + *item as u64;
        }

        Engram64(value)
    }
}

fn update_olfactory<K: Eq + Hash + fmt::Debug + 'static>(
    mut odor_context: ResMut<OdorContext>, 
    odor_map: Res<OdorPlaceMap<K>>,
    body: Res<Body>,
    world_hex: Res<WorldHex<K>>,
) {
    let pos = body.head_pos();

    if let Some(item) = odor_map.find(&world_hex[pos]) {
        odor_context.loc = item.clone();
    } else {
        odor_context.loc = OlfactoryItem::default();
    }
}

pub struct OlfactoryContextPlugin<K> {
    loc_map: HashMap<K, OlfactoryItem>,
}

impl<K: Eq + Hash + Send + fmt::Debug> OlfactoryContextPlugin<K> {
    pub fn new() -> Self {
        Self {
            loc_map: HashMap::default(),
        }
    }

    pub fn add(mut self, key: K, str: &str) -> Self {
        let mut vec = Vec::new();

        for c in str.chars() {
            let value = base64_rev(c).unwrap() as usize;

            vec.push(value);
        }

        vec.sort();

        self.loc_map.insert(key, OlfactoryItem {
            vec
        });

        self
    }
}

impl<K: Eq + Hash + fmt::Debug + Clone + Send + 'static> Plugin for OlfactoryContextPlugin<K> {
    fn build(&self, app: &mut App) {
        let odor_map: OdorPlaceMap<K> = OdorPlaceMap {
            loc_map: self.loc_map.clone(),
        };

        app.insert_resource(odor_map);
        app.insert_resource(OdorContext::default());

        app.system(Tick, update_olfactory::<K>);
    }
}
