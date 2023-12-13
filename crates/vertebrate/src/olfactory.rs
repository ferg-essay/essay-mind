///
/// Olfactory bulb
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType}, util::Angle,
};

pub struct Olfactory {
    food: Option<OdorItem>,
    avoid: Option<OdorItem>,
}

impl Olfactory {
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
        }
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

fn update_olfactory(
    body: Res<Body>, 
    world: Res<World>, 
    mut olfactory: ResMut<Olfactory>,
) {
    olfactory.food = None;
    olfactory.avoid = None;

    if let Some((odor, angle)) = body.odor_turn(world.get()) {
        if odor.is_food() {
            olfactory.food = Some(OdorItem::new(odor, angle));
        } else {
            olfactory.avoid = Some(OdorItem::new(odor, angle));

        }
    }
}

pub struct OlfactoryPlugin;

impl Plugin for OlfactoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Olfactory>();

        app.system(Tick, update_olfactory);
    }
}
