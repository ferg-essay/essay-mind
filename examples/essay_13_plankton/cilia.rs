use essay_ecs::prelude::*;
use essay_tensor::prelude::*;

use super::{Body, sense_peptide::{Peptide, PeptidePlugin}};

#[derive(Component)]
pub struct Cilia {
    peptides: Tensor,
    cilia_matrix: Tensor,
}

impl Cilia {
    pub const SWIM_RATE : f32 = 1.;    // default swim rate
    pub const DY_SWIM : f32 = 0.05;    // speed of the default swim rate

    pub const PEPTIDE_DECAY : f32 = 0.9;
    pub const PEPTIDE_INPUT : f32 = 0.5;

    pub fn new() -> Self {
        Self {
            peptides: Tensor::zeros([6]),
            cilia_matrix: tf32!([
                [0.2, 0.],   // Pressure
                [-1., 1.],   // Hot
                [0.1, 0.],   // Cold
                [-0.2, 0.2], // light
                [2.0, -1.0], // CO2
                [0., 0.],    // blank
            ]).t()
        }
    }
}

fn cilia_update(
    cilia: &mut Cilia, 
    mut body: ResMut<Body>,
    mut in_peptides: InEvent<Peptide>
) {
    // peptides decay each tick
    let mut peptides = &cilia.peptides * Cilia::PEPTIDE_DECAY;

    for peptide in in_peptides.iter() {
        match peptide {
            Peptide::Pressure => {
                let input = Tensor::one_hot([0], 6);
                peptides = peptides + input * Cilia::PEPTIDE_INPUT;
            }
            Peptide::TempHot => {
                let input = Tensor::one_hot([1], 6);
                peptides = peptides + input * Cilia::PEPTIDE_INPUT;
            }
            Peptide::TempCold => {
                let input = Tensor::one_hot([2], 6);
                peptides = peptides + input * Cilia::PEPTIDE_INPUT;
            }
            Peptide::Light => {
                let input = Tensor::one_hot([3], 6);
                peptides = peptides + input * Cilia::PEPTIDE_INPUT;
            }
            Peptide::CO2 => {
                let input = Tensor::one_hot([4], 6);
                peptides = peptides + input * Cilia::PEPTIDE_INPUT;
            },
        }
    }

    cilia.peptides = peptides;

    let control = cilia.cilia_matrix.matvec(&cilia.peptides);
    let swim = control[0];
    let arrest = control[1];

    body.set_peptides(&cilia.peptides);

    body.swim_rate(Cilia::SWIM_RATE + swim);
    body.arrest(arrest);
}

pub struct CiliaPlugin;

impl Plugin for CiliaPlugin {
    fn build(&self, app: &mut App) {
        if ! app.contains_plugin::<PeptidePlugin>() {
            app.plugin(PeptidePlugin);
        }
        
        app.system(Startup, |mut c: Commands| {
            c.spawn(Cilia::new())
        });

        app.event::<Peptide>();

        app.system(Update, cilia_update);
    }
}