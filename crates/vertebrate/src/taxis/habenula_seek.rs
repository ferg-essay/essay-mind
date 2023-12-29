use essay_ecs::{prelude::{Plugin, App}, core::ResMut, app::event::OutEvent};
use mind_ecs::Tick;

use crate::util::{DecayValue, DirVector, Angle};

use super::{taxis_pons::TaxisEvent, GoalVector};

pub struct HabenulaSeek {
}

impl HabenulaSeek {
    pub const HALF_LIFE : usize = 10;

    pub fn new(half_life: usize) -> Self {
        Self {
        }
    }

    pub fn pre_update(&mut self) {

    }

    pub fn toward(&mut self, value: f32) {

    }

    pub fn update(&mut self, taxis: &mut OutEvent<TaxisEvent>) {
        
    }
}

fn update_habenula_seek(
    mut seek: ResMut<HabenulaSeek>,
) {

}

pub struct HabenulaSeekItem {
    average: DecayValue,
    value: f32,

    short_average: DecayValue,

    goal_vector: GoalVector,
}

impl HabenulaSeekItem {
    pub const N_DIR : usize = 12;
    pub const GOAL_LIFE : usize = 20;
    pub const SAMPLE_LIFE : usize = 5;

    pub fn new(goal_life: usize, sample_life: usize) -> Self {
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

    pub fn goal_vector(&self) -> DirVector {
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

    pub fn update(&mut self, head_dir: Angle) {
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

pub struct HabenulaSeekPlugin;

impl Plugin for HabenulaSeekPlugin {
    fn build(&self, app: &mut App) {
        // assert!(app.contains_plugin::<MidLocomotorPlugin>(), "HabenulaSeekPlugin requires MidLocomotorPlugin");

        app.insert_resource(HabenulaSeek::new(HabenulaSeek::HALF_LIFE));

        app.system(Tick, update_habenula_seek);
    }
}
