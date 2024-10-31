use core::fmt;
use std::{collections::HashMap, hash::Hash};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, hippocampus::Engram64, mid_brain::SeekContext, util::base64_rev, world::{WorldHex, WorldHexTrait}};

fn update_odor_place<K: WorldHexTrait + Eq + Hash + fmt::Debug + 'static>(
    mut odor_context: ResMut<OdorPlace>, 
    odor_map: Res<OdorPlaceMap<K, OdorItem>>,
    body: Res<Body>,
    world_hex: Res<WorldHex<K>>,
) {
    let pos = body.head_pos();

    if let Some(item) = odor_map.find(&world_hex[pos]) {
        odor_context.odor = item.clone();
    } else {
        odor_context.odor = OdorItem::default();
    }
}

pub struct OdorPlaceMap<K: Eq + Hash, T> {
    loc_map: HashMap<K, T>,
}

impl<K: Eq + Hash, T> OdorPlaceMap<K, T> {
    fn find(&self, key: &K) -> Option<&T> {
        self.loc_map.get(key)
    }
} 

#[derive(Default)]
pub struct OdorPlace {
    odor: OdorItem,
}

impl SeekContext for OdorPlace {
    fn context(&self) -> Engram64 {
        self.odor.engram()
    }
}

#[derive(Clone, Default)]
struct OdorItem {
    vec: Vec<usize>,
}

impl OdorItem {
    fn engram(&self) -> Engram64 {
        let mut value : u64 = 0;

        for item in &self.vec {
            value = (value << 6) + *item as u64;
        }

        Engram64(value)
    }
}

pub struct OdorPlacePlugin<K> {
    loc_map: HashMap<K, OdorItem>,
}

impl<K: Eq + Hash + Send + fmt::Debug> OdorPlacePlugin<K> {
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

        self.loc_map.insert(key, OdorItem {
            vec
        });

        self
    }
}

impl<K: WorldHexTrait + Eq + Hash + fmt::Debug + Send> Plugin for OdorPlacePlugin<K> {
    fn build(&self, app: &mut App) {
        let odor_map: OdorPlaceMap<K, OdorItem> = OdorPlaceMap {
            loc_map: self.loc_map.clone(),
        };

        app.insert_resource(odor_map);
        app.insert_resource(OdorPlace::default());

        app.system(Tick, update_odor_place::<K>);
    }
}
