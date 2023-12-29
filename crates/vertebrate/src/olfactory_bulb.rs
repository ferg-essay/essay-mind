use std::collections::HashMap;

///
/// Olfactory bulb
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res, Event}, app::event::OutEvent};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType}, 
    util::{Angle, DirVector},
};

pub struct OlfactoryBulb {
    food: Option<OdorItem>,
    avoid: Option<OdorItem>,

    glomerules: Vec<Glomerule>,
    odor_map: HashMap<OdorType, usize>,
}

impl OlfactoryBulb {
    fn odor(&mut self, odor: OdorType) -> OdorId {
        let index = self.glomerules.len();

        self.glomerules.push(Glomerule::new(odor));
        self.odor_map.insert(odor, index);

        OdorId(index)
    }

    fn update(&mut self) {
        for glom in &mut self.glomerules {
            glom.update();
        }
    }

    pub fn value(&self, odor: OdorType) -> f32 {
        if let Some(index) = self.odor_map.get(&odor) {
            self.glomerules[*index].vector.value()
        } else {
            0.
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
impl Default for OlfactoryBulb {
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
    mut olf_bulb: ResMut<OlfactoryBulb>,
    mut ob_events: OutEvent<ObEvent>,
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

    for glomerule in &olf_bulb.glomerules {
        if glomerule.vector.value() > Glomerule::MIN {
            ob_events.send(ObEvent::Odor(glomerule.odor, glomerule.vector));
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OdorId(usize);

impl OdorId {
    #[inline]
    pub fn i(&self) -> usize {
        self.0
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
    odor: OdorType,
    vector: DirVector,
}

impl Glomerule {
    const MIN : f32 = 0.;

    fn new(odor: OdorType) -> Self {
        Self {
            odor,
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

#[derive(Clone, Copy, Debug, Event)]
pub enum ObEvent {
    Odor(OdorType, DirVector),
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
        let mut bulb = OlfactoryBulb::default();

        for odor in &self.odors {
            bulb.odor(*odor);
        }

        app.insert_resource(bulb);

        app.event::<ObEvent>();

        app.system(Tick, update_olfactory);
    }
}
