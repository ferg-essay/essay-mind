use core::fmt;
use std::{collections::HashMap, hash::Hash};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, world::{WorldHex, WorldHexTrait}};

fn update_avoid_here<K: WorldHexTrait + Eq + Hash + fmt::Debug + 'static>(
    mut avoid_here: ResMut<AvoidHere>, 
    avoid_map: Res<AvoidHereMap<K, AvoidItem>>,
    body: Res<Body>,
    world_hex: Res<WorldHex<K>>,
) {
    let pos = body.head_pos();

    if let Some(item) = avoid_map.find(&world_hex[pos]) {
        avoid_here.avoid = item.clone();
    } else {
        avoid_here.avoid = AvoidItem::default();
    }
}

///
/// AvoidHere represents R.pb triggers from irritation/painful areas.
/// R.pb has at least two distinct negative triggers: tac1 and cgrp, which
/// project to distinct areas.
/// 
/// This function is named AvoidHere not AvoidPlace because R.pb doesn't have
/// a general knowledge of place, but only that the current location is 
/// bad.
/// 
#[derive(Default)]
pub struct AvoidHere {
    avoid: AvoidItem,
}

impl AvoidHere {
    #[inline]
    pub fn is_avoid(&self) -> bool {
        self.avoid.is_avoid
    }
}

pub struct AvoidHereMap<K: Eq + Hash, T> {
    avoid_map: HashMap<K, T>,
}

impl<K: Eq + Hash, T> AvoidHereMap<K, T> {
    fn find(&self, key: &K) -> Option<&T> {
        self.avoid_map.get(key)
    }
} 

#[derive(Clone, Default)]
struct AvoidItem {
    is_avoid: bool,
}

pub struct AvoidHerePlugin<K> {
    avoid_map: HashMap<K, AvoidItem>,
}

impl<K: Eq + Hash + Send + fmt::Debug> AvoidHerePlugin<K> {
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

impl<K: WorldHexTrait + Eq + Hash + fmt::Debug + Send> Plugin for AvoidHerePlugin<K> {
    fn build(&self, app: &mut App) {
        let avoid_map: AvoidHereMap<K, AvoidItem> = AvoidHereMap {
            avoid_map: self.avoid_map.clone(),
        };

        app.insert_resource(avoid_map);
        app.insert_resource(AvoidHere::default());

        app.system(Tick, update_avoid_here::<K>);
    }
}
