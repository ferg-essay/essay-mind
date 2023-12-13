use super::{DecayValue, DirVector, Angle};


pub struct DirGradient {
    dir: Angle,
    gradient: DecayValue,
}

impl DirGradient {
    pub fn new(dir: Angle, half_life: usize) -> Self {
        Self {
            dir,
            gradient: DecayValue::new(half_life),
        }        
    }

    pub fn set_max(&mut self, value: f32) {
        self.gradient.set_max(value);
    }

    pub fn update(&mut self) {
        self.gradient.update();
    }

    pub fn to_vector(&self) -> DirVector {
        DirVector::new(self.dir, self.gradient.value())
    }
}
