use crate::util::EgoVector;

pub struct Taxis {
    approach_dir: EgoVector,
    avoid_dir: EgoVector,
}

impl Taxis {
    pub fn approach_dir(&self) -> EgoVector {
        self.approach_dir
    }

    pub fn set_approach_dir(&mut self, dir: EgoVector) {
        self.approach_dir = dir;
    }

    pub fn avoid_dir(&self) -> EgoVector {
        self.avoid_dir
    }

    pub fn set_avoid_dir(&mut self, dir: EgoVector) {
        self.avoid_dir = dir;
    }
}

impl Default for Taxis {
    fn default() -> Self {
        Self { 
            approach_dir: EgoVector::zero(),
            avoid_dir: EgoVector::zero(),
        }
    }
}