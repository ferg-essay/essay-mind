use super::{ticks::HalfLife, Angle, DecayValue, DirVector, Heading};


pub struct DirGradient {
    dir: Heading,
    gradient: DecayValue,
}

impl DirGradient {
    pub fn new(dir: Heading, half_life: impl Into<HalfLife>) -> Self {
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
