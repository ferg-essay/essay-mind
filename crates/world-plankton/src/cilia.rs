use essay_ecs::prelude::*;

use crate::{Body, sense_peptide::Peptide};

#[derive(Component)]
pub struct Cilia {
    swim_rate: f32, // how fast the cilia are beating
    arrest: f32,    // timeout for cilia arrest
}

impl Cilia {
    pub const SWIM_RATE : f32 = 1.;    // default swim rate
    pub const DY_SWIM : f32 = -0.05;    // speed of the default swim rate
    pub const ARREST_DECAY : f32 = 1.; // linear arrest decay

    pub fn new() -> Self {
        Self {
            swim_rate: 0.,
            arrest: 0.
        }
    }
}

fn cilia_update(cilia: &mut Cilia, mut peptides: InEvent<Peptide>) {
}

pub struct CiliaPlugin;

impl Plugin for CiliaPlugin {
    fn build(&self, app: &mut App) {
        app.system(Startup, |mut c: Commands| {
            c.spawn(Cilia::new())
        });

        app.event::<Peptide>();

        app.system(Update, cilia_update);
    }
}