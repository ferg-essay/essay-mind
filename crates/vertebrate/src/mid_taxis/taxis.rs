use crate::util::DirVector;

pub struct Taxis {
    approach_dir: DirVector,
    avoid_dir: DirVector,
}

impl Taxis {
    pub fn approach_dir(&self) -> DirVector {
        self.approach_dir
    }

    pub fn set_approach_dir(&mut self, dir: DirVector) {
        self.approach_dir = dir;
    }

    pub fn avoid_dir(&self) -> DirVector {
        self.avoid_dir
    }

    pub fn set_avoid_dir(&mut self, dir: DirVector) {
        self.avoid_dir = dir;
    }
}

impl Default for Taxis {
    fn default() -> Self {
        Self { 
            approach_dir: DirVector::zero(),
            avoid_dir: DirVector::zero(),
        }
    }
}