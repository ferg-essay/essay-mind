use essay_ecs::{app::{Plugin, App}, core::{ResMut, store::FromStore, Store}};
use mind_ecs::PreTick;

use crate::util::{DecayValue, HalfLife};

pub struct MidPeptides {
    // eating - consummation
    glucose: DecayValue,

    // eating - consummation
    near_food: DecayValue,

    // seek/explore
    explore_food: DecayValue,
    cue_seek_food: DecayValue,
    cue_avoid_food: DecayValue,
    seek_food: DecayValue,
    give_up_seek_food: DecayValue,
    urgency_food: DecayValue,
}

impl MidPeptides {
    const HALF_LIFE : HalfLife = HalfLife(1.);

    pub fn new() -> Self {
        let half_life = Self::HALF_LIFE;

        Self {
            glucose: DecayValue::new(half_life),

            near_food: DecayValue::new(half_life),

            explore_food: DecayValue::new(half_life),
            cue_seek_food: DecayValue::new(half_life),
            cue_avoid_food: DecayValue::new(half_life),
            seek_food: DecayValue::new(half_life),
            give_up_seek_food: DecayValue::new(half_life),
            urgency_food: DecayValue::new(half_life),
        }
    }

    //
    // blood sensors
    //

    /// glucose - blood sugar
    pub fn glucose(&self) -> f32 {
        self.glucose.value()
    }

    /// glucose
    pub fn glucose_mut(&mut self) -> &mut DecayValue {
        &mut self.glucose
    }

    //
    // eating group
    //

    /// AgRP
    pub fn near_food(&self) -> f32 {
        self.near_food.value()
    }

    /// AgRP
    pub fn near_food_mut(&mut self) -> &mut DecayValue {
        &mut self.near_food
    }

    /// orexin
    pub fn explore_food(&self) -> f32 {
        self.explore_food.value()
    }

    /// orexin
    pub fn explore_food_mut(&mut self) -> &mut DecayValue {
        &mut self.explore_food
    }

    /// dopamine
    pub fn seek_food(&self) -> f32 {
        self.seek_food.value()
    }

    /// dopamine
    pub fn seek_food_mut(&mut self) -> &mut DecayValue {
        &mut self.seek_food
    }

    /// habenula
    pub fn give_up_seek_food(&self) -> f32 {
        self.give_up_seek_food.value()
    }

    /// habenula
    pub fn give_up_seek_food_mut(&mut self) -> &mut DecayValue {
        &mut self.give_up_seek_food
    }

    pub fn urgency(&self) -> f32 {
        self.urgency_food.value()
    }

    pub fn urgency_mut(&mut self) -> &mut DecayValue {
        &mut self.urgency_food
    }

    pub fn cue_seek_food(&self) -> f32 {
        self.cue_seek_food.value()
    }

    pub fn cue_seek_food_mut(&mut self) -> &mut DecayValue {
        &mut self.cue_seek_food
    }

    pub fn cue_avoid_food(&self) -> f32 {
        self.cue_avoid_food.value()
    }

    pub fn cue_avoid_food_mut(&mut self) -> &mut DecayValue {
        &mut self.cue_avoid_food
    }

    fn update(&mut self) {
        self.glucose.update();

        self.near_food.update();

        self.explore_food.update();
        self.cue_seek_food.update();
        self.cue_avoid_food.update();
        self.seek_food.update();
        self.give_up_seek_food.update();
        self.urgency_food.update();
    }
}

impl FromStore for MidPeptides {
    fn init(_world: &mut Store) -> Self {
        MidPeptides::new()
    }
}

fn update_peptides(mut peptides: ResMut<MidPeptides>) {
    peptides.update()
}

pub struct CorePeptidesPlugin;

impl Plugin for CorePeptidesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidPeptides>();

        app.system(PreTick, update_peptides);
    }
}
