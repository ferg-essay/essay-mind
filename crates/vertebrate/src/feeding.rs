use essay_ecs::app::{Plugin, App};
use mind_macros::Peptide;
use crate as vertebrate;

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
    }
}
