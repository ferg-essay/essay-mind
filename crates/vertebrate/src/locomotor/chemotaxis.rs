///
/// phototaxis
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::World, 
    locomotor::{
        mid_explore::MidExplore, 
        mid_locomotor::MidLocomotorPlugin, 
        habenula_seek::HabenulaSeek,
    },
    util::{DecayValue, DirVector, Angle}, olfactory_bulb::OlfactoryBulb, 
};

pub struct Chemotaxis {
    habenula: HabenulaSeek,
}

impl Chemotaxis {
    pub const N_DIR : usize = 12;

    pub fn new() -> Self {
        Self {
            habenula: HabenulaSeek::new(),
        }

    }

    pub fn update(&mut self) {
    }
    
    fn goal_vector(&self) -> DirVector {
        todo!()
    }
}

fn update_chemotaxis(
    mut chemotaxis: ResMut<Chemotaxis>,
) {
}

pub struct ChemotaxisPlugin;

impl Plugin for ChemotaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_resource::<HabenulaSeek>(), "chemotaxis requires HabenulaSeek");
        assert!(app.contains_resource::<OlfactoryBulb>(), "chemotaxis requires OlfactoryBulb");

        let chemotaxis = Chemotaxis::new();

        app.insert_resource(chemotaxis);

        app.system(Tick, update_chemotaxis);
    }
}
