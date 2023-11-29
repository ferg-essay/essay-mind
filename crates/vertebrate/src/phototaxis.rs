///
/// phototaxis
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use essay_plot::prelude::Angle;
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType},
};

pub struct Phototaxis {
}

impl Phototaxis {
}

impl Default for Phototaxis {
    fn default() -> Self {
        Self { 
        }
    }
}

fn update_phototaxis(
    body: Res<Body>, 
    world: Res<World>, 
    mut phototaxis: ResMut<Phototaxis>,
) {
    let light = world.light(body.pos_head());

    // println!("Light {:?}", light);
}

pub struct PhototaxisPlugin;

impl Plugin for PhototaxisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Phototaxis>();

        app.system(Tick, update_phototaxis);
    }
}
