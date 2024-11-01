use core::fmt;
use std::{collections::HashMap, hash::Hash};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, world::{WorldHex, WorldHexTrait}};

fn update_avoid_place<K: WorldHexTrait + Eq + Hash + fmt::Debug + 'static>(
    mut avoid_place: ResMut<AvoidPlace>, 
    avoid_map: Res<AvoidPlaceMap<K, AvoidItem>>,
    body: Res<Body>,
    world_hex: Res<WorldHex<K>>,
) {
    let pos = body.head_pos();

    if let Some(item) = avoid_map.find(&world_hex[pos]) {
        avoid_place.avoid = item.clone();
    } else {
        avoid_place.avoid = AvoidItem::default();
    }
}

///
/// AvoidPlace represents R.pb triggers from irritation/painful areas.
/// R.pb has at least two distinct negative triggers: tac1 and cgrp, which
/// project to distinct areas.
/// 
#[derive(Default)]
pub struct AvoidPlace {
    avoid: AvoidItem,
}

impl AvoidPlace {
    #[inline]
    pub fn is_avoid(&self) -> bool {
        self.avoid.is_avoid
    }
}

pub struct AvoidPlaceMap<K: Eq + Hash, T> {
    avoid_map: HashMap<K, T>,
}

impl<K: Eq + Hash, T> AvoidPlaceMap<K, T> {
    fn find(&self, key: &K) -> Option<&T> {
        self.avoid_map.get(key)
    }
} 

#[derive(Clone, Default)]
struct AvoidItem {
    is_avoid: bool,
}

pub struct AvoidPlacePlugin<K> {
    avoid_map: HashMap<K, AvoidItem>,
}

impl<K: Eq + Hash + Send + fmt::Debug> AvoidPlacePlugin<K> {
    pub fn new() -> Self {
        Self {
            avoid_map: HashMap::default(),
        }
    }

    pub fn avoid(mut self, key: K, is_avoid: bool) -> Self {
        self.avoid_map.insert(key, AvoidItem {
            is_avoid
        });

        self
    }
}

impl<K: WorldHexTrait + Eq + Hash + fmt::Debug + Send> Plugin for AvoidPlacePlugin<K> {
    fn build(&self, app: &mut App) {
        let avoid_map: AvoidPlaceMap<K, AvoidItem> = AvoidPlaceMap {
            avoid_map: self.avoid_map.clone(),
        };

        app.insert_resource(avoid_map);
        app.insert_resource(AvoidPlace::default());

        app.system(Tick, update_avoid_place::<K>);
    }
}
