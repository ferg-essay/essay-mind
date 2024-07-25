use crate::util::{Angle, DecayValue, DirVector, HalfLife, Heading};


pub struct GoalVector {
    dir: Heading,
    value: DecayValue,
}

impl GoalVector {
    pub fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            dir: Heading::Unit(0.),
            value: DecayValue::new(half_life),
        }
    }

    pub fn avoid(&mut self, dir: Heading, gradient: f32) {
        self.value.update();

        if gradient > 0. {
            let reverse_dir = Heading::Unit(dir.to_unit() + 0.5);

            self.add_vector(reverse_dir, gradient);
            //self.dir_gradient(reverse_dir).set_max(gradient);
            // self.goal_vector.update(reverse_dir, gradient);
        } else if gradient < 0. {
            //self.dir_gradient(head_dir).set_max(- gradient);
            // self.goal_vector.update(head_dir, - gradient);
            self.add_vector(dir, - gradient);
        }
    }

    pub fn approach(&mut self, dir: Heading, gradient: f32) {
        self.value.update();

        if gradient < 0. {
            let reverse_dir = Heading::Unit((dir.to_unit() + 0.5) % 1.);

            self.add_vector(reverse_dir, - gradient);
        } else if gradient > 0. {
            self.add_vector(dir, gradient);
        }
    }

    pub fn add_vector(&mut self, dir: Heading, value: f32) {
        // let dt = (dir.to_unit() - self.dir.to_unit()).abs();

        if self.value.value() < value { // || dt > 0.25 {
            let value = value.clamp(0., 1.);
            self.set_vector(dir, value);
            self.dir = dir;
            self.value.set(value);
        }
    }

    pub fn set_vector(&mut self, dir: Heading, value: f32) {
        self.dir = dir;
        self.value.set(value);
    }

    pub fn to_vector(&self) -> DirVector {
        DirVector::new(self.dir, self.value.value())
    }
}
