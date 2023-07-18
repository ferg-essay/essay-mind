use essay_ecs::prelude::*;

use crate::Body;

fn sense_pressure(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.pressure() > 0.5 {
        peptides.send(Peptide::Pressure)
    }
}

fn sense_hot(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.temperature() > 0.75 {
        peptides.send(Peptide::TempHot)
    }
}

fn sense_cold(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.temperature() < 0.25 {
        peptides.send(Peptide::TempCold)
    }
}

fn sense_co2(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.co2() > 0.5 {
        peptides.send(Peptide::CO2)
    }
}

fn sense_light(body: Res<Body>, mut peptides: OutEvent<Peptide>) {
    if body.light() > 0.75 {
        peptides.send(Peptide::Light)
    }
}

#[derive(Event, Debug)]
pub enum Peptide {
    Pressure,
    TempHot,
    TempCold,
    Light,
    CO2,
}

pub struct PeptidePlugin;

impl Plugin for PeptidePlugin {
    fn build(&self, app: &mut App) {
        app.system(Update, sense_pressure);
        app.system(Update, sense_hot);
        app.system(Update, sense_cold);
        app.system(Update, sense_light);
        app.system(Update, sense_co2);
    }
}