use core::fmt;
use std::{collections::HashMap, hash::{Hash, Hasher}, ops::Index};

use essay_ecs::{app::{Plugin, App}, core::{ResMut, store::FromStore, Store}};
use mind_ecs::{PostTick, PreTick};
use util::label::DynLabel;

pub struct MidPeptides {
    peptide_map: HashMap<Box<dyn Peptide>, PeptideId>,
    peptides: Vec<PeptideItem>,
    values: Vec<f32>,
}

impl MidPeptides {
    pub fn new() -> Self {
        Self {
            peptide_map: HashMap::new(),
            peptides: Vec::new(),
            values: Vec::new(),
        }
    }
    
    pub fn peptide(&mut self, peptide: impl Peptide) -> &mut PeptideItem {
        let id = *self.peptide_map
            .entry(peptide.box_clone())
            .or_insert_with(|| {
            PeptideId(self.peptides.len())
        });

        self.peptides.push(PeptideItem::new(id, peptide));
        self.values.resize(self.peptides.len(), 0.);

        &mut self.peptides[id.i()]
    }
    
    pub fn get_peptide(&self, peptide: &dyn Peptide) -> Option<&PeptideItem> {
        match self.peptide_map.get(peptide) {
            Some(id) => Some(&self.peptides[id.i()]),
            None => None
        }
    }

    #[inline]
    pub fn add(&mut self, id: PeptideId, delta: f32) {
        assert!(0. <= delta && delta <= 1.);

        let decay = self.peptides[id.i()].decay;

        self.values[id.i()] = (self.values[id.i()] + delta * decay).clamp(0., 1.);
        //self.values[id.i()] = (self.values[id.i()] + delta).clamp(0., 1.);
    }

    fn update(&mut self) {
        for (item, value) in self.peptides.iter().zip(&mut self.values) {
            *value = (*value * (1. - item.decay)).clamp(0., 1.);
        }
    }
}

impl FromStore for MidPeptides {
    fn init(_world: &mut Store) -> Self {
        MidPeptides::new()
    }
}

impl Index<PeptideId> for MidPeptides {
    type Output = f32;

    #[inline]
    fn index(&self, id: PeptideId) -> &Self::Output {
        &self.values[id.i()]
    }
}

impl Index<&dyn Peptide> for MidPeptides {
    type Output = PeptideItem;

    #[inline]
    fn index(&self, peptide: &dyn Peptide) -> &Self::Output {
        self.get_peptide(peptide).unwrap()
    }
}

pub struct PeptideItem {
    id: PeptideId,
    peptide: BoxPeptide,
    decay: f32,
}

impl PeptideItem {
    fn new(id: PeptideId, peptide: impl Peptide) -> Self {
        Self {
            id,
            peptide: peptide.box_clone(),
            decay: 0.1,
        }
    }

    pub fn id(&self) -> PeptideId {
        self.id
    }

    pub fn peptide(&self) -> &BoxPeptide {
        &self.peptide
    }

    pub fn half_life(&mut self, decay_s: f32) -> &mut Self {
        self.decay = (0.1 / decay_s.max(1e-6)).clamp(0., 1.);

        self
    }
}

pub type BoxPeptide = Box<dyn Peptide>;

pub trait Peptide : Send + Sync + DynLabel + fmt::Debug {
    fn box_clone(&self) -> Box<dyn Peptide>;
}

impl PartialEq for dyn Peptide {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn Peptide {}

impl Hash for dyn Peptide {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PeptideId(usize);

impl PeptideId {
    pub fn i(&self) -> usize {
        self.0
    }
}

fn update_peptide_canal(mut peptides: ResMut<MidPeptides>) {
    peptides.update()
}

pub struct MidPeptidesPlugin;

impl Plugin for MidPeptidesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidPeptides>();

        app.system(PreTick, update_peptide_canal);
    }
}


#[cfg(test)]
mod test {
    use mind_macros::Peptide;
    use crate as vertebrate;

    use super::MidPeptides;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Peptide)]
    struct TestPeptide;

    #[test]
    fn test() {
        let mut peptides = MidPeptides::new();
        peptides.peptide(TestPeptide).half_life(0.5);
        let id = peptides.get_peptide(&TestPeptide).unwrap().id();
        peptides.add(id, 0.1);

        assert_eq!(peptides[id], 0.1);
    }
}
