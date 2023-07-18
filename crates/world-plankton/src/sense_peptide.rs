use essay_ecs::prelude::*;

use crate::Body;

fn sense_pressure(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.pressure() > 0.5 {
        peptides.send(Peptide::Pressure)
    }
}

#[derive(Event, Debug)]
pub enum Peptide {
    Pressure,
}

pub struct PeptidePlugin {

}

impl Plugin for PeptidePlugin {
    fn build(&self, app: &mut App) {
        app.system(Update, sense_pressure);
    }
}