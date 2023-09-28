///
/// Olfactory bulb
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use essay_plot::prelude::Angle;
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType}
};

pub struct Olfactory {
    odor: Option<OdorType>,
    dir: Option<Angle>,
}

impl Olfactory {
    pub fn dir(&self) -> Option<Angle> {
        self.dir
    }
}
impl Default for Olfactory {
    fn default() -> Self {
        Self { 
            odor: None,
            dir: None,
        }
    }
}

fn update_olfactory(
    body: Res<Body>, 
    world: Res<World>, 
    mut olfactory: ResMut<Olfactory>,
) {
    if let Some((odor, angle)) = body.odor_turn(world.get()) {
        olfactory.odor = Some(odor);
        olfactory.dir = Some(angle);
    } else {
        olfactory.odor = None;
        olfactory.dir = None;
    }
}
pub struct OlfactoryPlugin;

impl Plugin for OlfactoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Olfactory>();

        app.system(Tick, update_olfactory);
    }
}
