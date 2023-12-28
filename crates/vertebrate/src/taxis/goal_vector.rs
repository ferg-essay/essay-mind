use crate::util::{DirVector, Angle, DecayValue};


pub struct GoalVector {
    dir: Angle,
    value: DecayValue,
}

impl GoalVector {
    pub fn new(half_life: usize) -> Self {
        Self {
            dir: Angle::Unit(0.),
            value: DecayValue::new(half_life),
        }
    }

    pub fn avoid(&mut self, dir: Angle, gradient: f32) {
        self.value.update();

        if gradient > 0. {
            let reverse_dir = Angle::Unit(dir.to_unit() + 0.5);

            self.add_vector(reverse_dir, gradient);
            //self.dir_gradient(reverse_dir).set_max(gradient);
            // self.goal_vector.update(reverse_dir, gradient);
        } else if gradient < 0. {
            //self.dir_gradient(head_dir).set_max(- gradient);
            // self.goal_vector.update(head_dir, - gradient);
            self.add_vector(dir, - gradient);
        }
    }

    pub fn approach(&mut self, dir: Angle, gradient: f32) {
        self.value.update();

        if gradient < 0. {
            let reverse_dir = Angle::Unit((dir.to_unit() + 0.5) % 1.);

            self.add_vector(reverse_dir, - gradient);
        } else if gradient > 0. {
            self.add_vector(dir, gradient);
        }
    }

    pub fn add_vector(&mut self, dir: Angle, value: f32) {
        if self.value.value() < value {
            self.dir = dir;
            self.value.set(value);
        }
    }

    pub fn to_vector(&self) -> DirVector {
        DirVector::new(self.dir, self.value.value())
    }
}
