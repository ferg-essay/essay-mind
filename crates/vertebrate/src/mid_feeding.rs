use essay_ecs::{app::{Plugin, App}, core::{Res, ResMut}};
use mind_ecs::Tick;
use mind_macros::Peptide;
use crate::{self as vertebrate, mid_peptides::{MidPeptides, PeptideId}, olfactory::Olfactory};

struct MidFeeding {
    explore_id: PeptideId, 
    cue_seek_id: PeptideId, 
    cue_avoid_id: PeptideId, 
    seek_id: PeptideId, 
    eat_id: PeptideId, 

}

impl MidFeeding {
    fn new(peptides: &mut MidPeptides) -> Self {
        Self {
            explore_id: peptides.peptide(ExploreFood).id(),
            cue_seek_id: peptides.peptide(CueSeekFood).id(),
            cue_avoid_id: peptides.peptide(CueAvoidFood).half_life(5.).id(),
            seek_id: peptides.peptide(SeekFood).id(),
            eat_id: peptides.peptide(EatFood).id(),
        }
    }
}

fn update_feeding(
    feeding: Res<MidFeeding>,
    mut peptides: ResMut<MidPeptides>
) {
    // orexin - base exploratory drive
    let explore_v = 0.5;

    peptides.add(feeding.explore_id, explore_v);

    // dopamine - trigger for seeking a food cue
    let mut seek = 0.;
    // ghrelin - food cue (ghrelin) prompts
    seek += peptides[feeding.cue_seek_id];
    // nts - neurotensin suppresses
    seek -= peptides[feeding.cue_avoid_id];
    // orexin - high orexin avoids
    seek -= (peptides[feeding.explore_id] - 0.5).max(0.);

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

// Cck - probably something else
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct EatFood;


pub struct MidFeedingPlugin;

impl Plugin for MidFeedingPlugin {
    fn build(&self, app: &mut App) {
        let peptides = app.resource_mut::<MidPeptides>();

        let feeding = MidFeeding::new(peptides);

        app.insert_resource(feeding);
        app.system(Tick, update_feeding);

        if app.contains_resource::<Olfactory>() {
            app.system(Tick, update_feeding_olfactory);
        }
    }
}
