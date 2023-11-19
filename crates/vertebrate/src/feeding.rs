use essay_ecs::{app::{Plugin, App}, core::{Res, ResMut}};
use mind_ecs::Tick;
use mind_macros::Peptide;
use crate::{self as vertebrate, mid_peptide_canal::{PeptideCanal, PeptideId}};

struct MidFeeding {
    explore_id: PeptideId, 
    seek_id: PeptideId, 
    eat_id: PeptideId, 

}

impl MidFeeding {
    fn new(peptides: &mut PeptideCanal) -> Self {
        Self {
            explore_id: peptides.peptide(ExploreFood).id(),
            seek_id: peptides.peptide(SeekFood).id(),
            eat_id: peptides.peptide(EatFood).id(),
        }
    }
}

fn update_feeding(
    feeding: Res<MidFeeding>,
    mut peptides: ResMut<PeptideCanal>
) {
    if peptides[feeding.explore_id] < 0.8 {
        peptides.add(feeding.explore_id, 1.);
    }

    if peptides[feeding.seek_id] < 0.2 {
        peptides.add(feeding.seek_id, 0.2);
    }
}

/// Orexin
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct ExploreFood;

// MCH
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct SeekFood;

// Cck
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Peptide)]
pub struct EatFood;


pub struct MidFeedingPlugin;

impl Plugin for MidFeedingPlugin {
    fn build(&self, app: &mut App) {
        let peptides = app.resource_mut::<PeptideCanal>();

        let feeding = MidFeeding::new(peptides);

        app.insert_resource(feeding);

        app.system(Tick, update_feeding);
    }
}
