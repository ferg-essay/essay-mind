use essay_ecs::{app::{Plugin, App}, core::{Res, ResMut}};
use mind_ecs::Tick;
use mind_macros::Peptide;
use crate::{self as vertebrate, mid_peptides::{MidPeptides, PeptideId}, olfactory::Olfactory, habenula_med::Habenula, body::Body, world::World, tectum::TectumLocomotionStn};

struct MidFeeding {
    give_up_hb: Habenula,
    
    explore_id: PeptideId, 
    urgency_id: PeptideId, 
    give_up_id: PeptideId, 

    cue_seek_id: PeptideId, 
    cue_avoid_id: PeptideId, 
    seek_id: PeptideId, 

    food_near_id: PeptideId, 
    blood_sugar_id: PeptideId, 

    eat_id: PeptideId, 

}

impl MidFeeding {
    fn new(peptides: &mut MidPeptides) -> Self {
        Self {
            give_up_hb: Habenula::new(40.),
            explore_id: peptides.peptide(ExploreFood).id(),
            urgency_id: peptides.peptide(UrgencySeekFood).id(),
            give_up_id: peptides.peptide(GiveUpSeekFood).id(),
            cue_seek_id: peptides.peptide(CueSeekFood).id(),
            cue_avoid_id: peptides.peptide(CueAvoidFood).half_life(5.).id(),
            seek_id: peptides.peptide(SeekFood).id(),

            food_near_id: peptides.peptide(NearFood).id(),
            blood_sugar_id: peptides.peptide(Glucose).id(),

            eat_id: peptides.peptide(EatFood).id(),

        }
    }
}

fn update_feeding(
    mut feeding: ResMut<MidFeeding>,
    mut peptides: ResMut<MidPeptides>
) {
    // orexin - base exploratory drive
    let explore_v = 0.5;
    peptides.add(feeding.explore_id, explore_v);

    // habenula - give-up timer
    feeding.give_up_hb.update();

    // TODO: should be action-based
    if peptides[feeding.seek_id] > 0.25 {
        feeding.give_up_hb.excite(1.);
    }

    // serotonin - high serotonin increases persistence
    let patience_5ht = (peptides[feeding.urgency_id] - 0.7).clamp(0., 0.25);
    feeding.give_up_hb.inhibit(patience_5ht);

    peptides.add(feeding.give_up_id, feeding.give_up_hb.value());

    // serotonin - urgency
    let urgency_v = (
        peptides[feeding.explore_id]
        - (peptides[feeding.give_up_id] - 0.5)
    ).clamp(0., 1.);
    peptides.add(feeding.urgency_id, urgency_v);

    // dopamine - trigger for seeking a food cue
    let mut seek = 0.;
    // baseline DA from orexin
    seek += peptides[feeding.explore_id] * 0.4;
    // ghrelin - food cue (ghrelin) prompts
    seek += peptides[feeding.cue_seek_id];
    // nts - neurotensin suppresses food seeking
    seek -= peptides[feeding.cue_avoid_id];
    // orexin - high orexin avoids
    seek -= (peptides[feeding.explore_id] - 0.5).max(0.);

    // habenula - give-up circuit suppresses
    seek -= 2. * (peptides[feeding.give_up_id] - 0.5).max(0.);

    peptides.add(feeding.seek_id, seek.clamp(0., 1.));
}

fn update_feeding_olfactory(
    feeding: Res<MidFeeding>,
    olfactory: Res<Olfactory>,
    mut peptides: ResMut<MidPeptides>
) {
    if olfactory.food_dir().is_some() {
        peptides.add(feeding.cue_seek_id, 0.8);
    }    

}

fn update_near_food(
    body: Res<Body>, 
    world: Res<World>, 
    feeding: Res<MidFeeding>,
    mut peptides: ResMut<MidPeptides>
) {
    if body.eat().is_sensor_food() {
        peptides.add(feeding.food_near_id, 1.0);
    }
}

fn update_eat(
    mut peptides: ResMut<MidPeptides>,
    mut body: ResMut<Body>,
) {
    if let Some(item) = peptides.get_peptide(&NearFood) {
        if peptides[item.id()] > 0.5 {
            if body.eat().glucose() < 0.8 && body.eat().is_eating()
                || body.eat().glucose() < 0.3 {
                body.locomotion_mut().arrest();
                body.eat_mut().eat();
            }
        }
    }

    let id = peptides.get_peptide(&Glucose).unwrap().id();
    peptides.add(id, body.eat().glucose());
}

/// Orexin
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct ExploreFood;

// ghrelin - possibly MCH
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct CueSeekFood;

// neurotensin
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct CueAvoidFood;

// DA - possibly MCH
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct SeekFood;

// Hb - habenula
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct GiveUpSeekFood;

// 5HT - serotonin
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct UrgencySeekFood;

// DA - possibly two DA?
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct NearFood;

// Cck - probably something else
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct EatFood;

// MCH/Leptin
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct Glucose;


pub struct MidFeedingPlugin;

impl Plugin for MidFeedingPlugin {
    fn build(&self, app: &mut App) {
        let peptides = app.resource_mut::<MidPeptides>();

        let feeding = MidFeeding::new(peptides);

        app.insert_resource(feeding);
        app.system(Tick, update_feeding);

        app.system(Tick, update_near_food);
        app.system(Tick, update_eat);

        if app.contains_resource::<Olfactory>() {
            app.system(Tick, update_feeding_olfactory);
        }
    }
}
