use std::collections::HashMap;

///
/// Olfactory bulb
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res, Event}, app::event::OutEvent};
use mind_ecs::Tick;

use crate::{
    body::Body, mid_motor::SeekInput, pallidum::basal_forebrain::{AttendId, AttendValue, BasalForebrain}, util::{Angle, DirVector}, world::{OdorType, World}
};

pub struct OlfactoryBulb {
    food: Option<OdorItem>,
    avoid: Option<OdorItem>,

    glomerules: Vec<Glomerule>,
    odor_map: HashMap<OdorType, usize>,

    active_odors: Vec<OdorId>,

    attention: BasalForebrain,
}

impl OlfactoryBulb {
    fn new() -> Self {
        Self { 
            food: None,
            avoid: None,
            glomerules: Vec::new(),
            active_odors: Vec::new(),
            odor_map: HashMap::new(),
            attention: BasalForebrain::new(),
        }
    }

    fn odor(&mut self, odor: OdorType) -> OdorId {
        let index = self.glomerules.len();

        let attend_id = self.attention.push();

        self.glomerules.push(Glomerule::new(odor, attend_id));
        self.odor_map.insert(odor, index);

        OdorId(index)
    }

    fn pre_update(&mut self) {
        self.attention.pre_update();
        
        for glom in &mut self.glomerules {
            glom.pre_update();
        }
    }

    fn update_odor(&mut self, index: usize, vector: DirVector) {
        self.glomerules[index].odor(vector);

        let attend_id = self.glomerules[index].attend_id;
        let value = self.glomerules[index].attend_value();

        self.attention.add(attend_id, value);
    }

    fn update(&mut self) {
        self.attention.update();

        self.active_odors.clear();

        for (i, glom) in self.glomerules.iter_mut().enumerate() {
            let attend_id = glom.attend_id;
            let attend = self.attention.attend(attend_id);

            glom.set_attend(attend);

            glom.update();

            if glom.vector.value() > Glomerule::MIN {
                self.active_odors.push(OdorId(i));
            }
        }
    }

    #[inline]
    pub fn value(&self, odor: OdorType) -> f32 {
        let attend_value = self.value_pair(odor);

        attend_value.value * attend_value.attend
    }

    pub fn value_pair(&self, odor: OdorType) -> AttendValue {
        if let Some(index) = self.odor_map.get(&odor) {
            let glom = &self.glomerules[*index];

            let value = glom.vector.value();
            let factor = self.attention.attend(glom.attend_id);

            AttendValue::new(value, factor)
        } else {
            AttendValue::new(0., 0.)
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

impl SeekInput for OlfactoryBulb {
    fn seek_dir(&self) -> Option<DirVector> {
        for id in &self.active_odors {
            let glom = &self.glomerules[id.0];

            if glom.odor.is_food() {
                return Some(glom.vector);
            }
        }

        None
    }
}

fn update_olfactory(
    body: Res<Body>, 
    world: Res<World>, 
    mut olf_bulb: ResMut<OlfactoryBulb>,
    mut ob_events: OutEvent<ObEvent>,
) {
    // olf_bulb.food = None;
    // olf_bulb.avoid = None;

    olf_bulb.pre_update();

    for (odor, vector) in world.odors_by_head(body.head_pos()) {
        let index = *olf_bulb.odor_map.get(&odor).unwrap();

        let vector = vector.to_ego(body.head_dir());

        // olf_bulb.glomerules[index].odor(vector);
        olf_bulb.get_mut().update_odor(index, vector);
    }

    olf_bulb.update();

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
    fn _new(odor: OdorType, dir: Angle) -> Self {
        Self {
            _odor: odor,
            dir,
        }
    }
}

struct Glomerule {
    odor: OdorType,
    vector: DirVector,
    attend_id: AttendId,
    attend: f32,
}

impl Glomerule {
    const MIN : f32 = 0.;

    fn new(odor: OdorType, attend_id: AttendId) -> Self {
        Self {
            odor,
            vector: DirVector::zero(),
            attend_id,
            attend: 1.,
        }
    }

    #[inline]
    fn value(&self) -> f32 {
        self.vector.value()
    }

    fn attend(&self) -> f32 {
        self.attend
    }

    #[inline]
    fn attend_value(&self) -> f32 {
        self.attend() * self.value()
    }

    fn pre_update(&mut self) {
        self.vector = DirVector::zero();
    }

    fn set_attend(&mut self, attend: f32) {
        self.attend = attend;
    }

    fn odor(&mut self, vector: DirVector) {
        self.vector = vector;
    }

    fn update(&mut self) {
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

    pub fn odor(&mut self, odor: OdorType) -> &mut Self {
        self.odors.push(odor);

        self
    }
}

impl Plugin for OlfactoryPlugin {
    fn build(&self, app: &mut App) {
        let mut bulb = OlfactoryBulb::new();

        for odor in &self.odors {
            bulb.odor(*odor);
        }

        app.insert_resource(bulb);

        app.event::<ObEvent>();

        app.system(Tick, update_olfactory);
    }
}
