use std::collections::HashMap;

///
/// Olfactory bulb
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType}, 
    util::{Angle, DirVector},
};

pub struct Olfactory {
    food: Option<OdorItem>,
    avoid: Option<OdorItem>,

    glomerules: Vec<Glomerule>,
    odor_map: HashMap<OdorType, usize>,
}

impl Olfactory {
    fn odor(&mut self, odor: OdorType) {
        let index = self.glomerules.len();

        self.glomerules.push(Glomerule::new(odor));
        self.odor_map.insert(odor, index);
    }

    fn update(&mut self) {
        for glom in &mut self.glomerules {
            glom.update();
        }
    }

    pub fn food_dir(&self) -> Option<Angle> {
        if let Some(food) = &self.food {
            Some(food.dir)
        } else {
            None
        }
    }

    pub fn avoid_dir(&self) -> Option<Angle> {
        if let Some(avoid) = &self.avoid {
            Some(avoid.dir)
        } else {
            None
        }
    }
}
impl Default for Olfactory {
    fn default() -> Self {
        Self { 
            food: None,
            avoid: None,
            glomerules: Vec::new(),
            odor_map: HashMap::new(),
        }
    }
}

fn update_olfactory(
    body: Res<Body>, 
    world: Res<World>, 
    mut olf_bulb: ResMut<Olfactory>,
) {
    olf_bulb.food = None;
    olf_bulb.avoid = None;

    olf_bulb.update();

    for (odor, vector) in world.odors_by_head(body.pos_head()) {
        let index = *olf_bulb.odor_map.get(&odor).unwrap();

        olf_bulb.glomerules[index].odor(vector);
        /*
        if odor.is_food() {
            olfactory.food = Some(OdorItem::new(odor, angle));
        } else {
            olfactory.avoid = Some(OdorItem::new(odor, angle));
        }
        */
    }
}

struct OdorItem {
    _odor: OdorType,
    dir: Angle,
}

impl OdorItem {
    fn new(odor: OdorType, dir: Angle) -> Self {
        Self {
            _odor: odor,
            dir,
        }
    }
}

struct Glomerule {
    _odor: OdorType,
    vector: DirVector,
}

impl Glomerule {
    fn new(odor: OdorType) -> Self {
        Self {
            _odor: odor,
            vector: DirVector::zero(),
        }
    }

    fn update(&mut self) {
        self.vector = DirVector::zero();
    }

    fn odor(&mut self, vector: DirVector) {
        self.vector = vector;
    }
}

pub struct OlfactoryPlugin {
    odors: Vec<OdorType>,
}

impl OlfactoryPlugin {
    pub fn new() -> Self {
        Self {
            odors: Vec::new(),
        }
    }

    pub fn odor(mut self, odor: OdorType) -> Self {
        self.odors.push(odor);

        self
    }
}

impl Plugin for OlfactoryPlugin {
    fn build(&self, app: &mut App) {
        let mut bulb = Olfactory::default();

        for odor in &self.odors {
            bulb.odor(*odor);
        }

        app.insert_resource(bulb);

        app.system(Tick, update_olfactory);
    }
}
