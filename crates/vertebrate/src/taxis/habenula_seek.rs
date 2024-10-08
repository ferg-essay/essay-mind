use crate::util::{DecayValue, EgoVector, HalfLife, Heading};

use super::GoalVector;

pub struct HabenulaSeekItem {
    average: DecayValue,
    value: f32,

    short_average: DecayValue,

    goal_vector: GoalVector,
}

impl HabenulaSeekItem {
    pub const N_DIR : usize = 12;
    pub const GOAL_LIFE : HalfLife = HalfLife(2.);
    pub const SAMPLE_LIFE : HalfLife = HalfLife(1.);

    pub fn new(
        goal_life: impl Into<HalfLife>, 
        sample_life: impl Into<HalfLife>
    ) -> Self {
        let goal_life = goal_life.into();
        
        Self { 
            // start with 20
            average: DecayValue::new(goal_life),
            short_average: DecayValue::new(sample_life),
            value: 0.,
            goal_vector: GoalVector::new(goal_life),
        }
    }
    
    pub fn average(&self) -> f32 {
        self.average.value()
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn gradient(&self) -> f32 {
        self.value() - self.average()
    }

    pub fn short_average(&self) -> f32 {
        self.short_average.value()
    }

    pub fn short_gradient(&self) -> f32 {
        self.value() - self.short_average()
    }

    pub fn goal_vector(&self) -> EgoVector {
        self.goal_vector.to_vector()
    }

    pub fn pre_update(&mut self) {
        self.value = 0.;
        self.average.update();
        self.short_average.update();
    }

    pub fn add(&mut self, value: f32) {
        self.value += value;
        self.average.add(value);
        self.short_average.add(value);
    }

    pub fn update(&mut self, head_dir: Heading) {
        let gradient = self.short_gradient();
        self.goal_vector.approach(head_dir, gradient);
    }
}

impl Default for HabenulaSeekItem {
    fn default() -> Self {
        HabenulaSeekItem::new(
            HabenulaSeekItem::GOAL_LIFE,
            HabenulaSeekItem::SAMPLE_LIFE,
        )
    }
}
