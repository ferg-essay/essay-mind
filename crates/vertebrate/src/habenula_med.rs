use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, core::{store::FromStore, Store, Local}};
use mind_ecs::Tick;

use crate::{
    mid_locomotor::MidLocomotorPlugin, 
    olfactory::{OlfactoryPlugin, Olfactory}, 
    tectum::TectumLocomotionStn, action::Turn
};

pub struct Habenula {
    decay: f32,

    value: f32
}

impl Habenula {
    pub fn new(half_life: f32) -> Self {
        Self {
            decay: 0.1 / half_life,
            value: 0.5,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn excite(&mut self, value: f32) -> &mut Self {
        self.value += self.decay * 0.5 * value.clamp(0., 1.);

        self
    }

    pub fn inhibit(&mut self, value: f32) -> &mut Self {
        self.value -= self.decay * 0.5 * value.clamp(0., 1.);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.value = self.value * (1. - self.decay) + 0.5 * self.decay;

        self
    }
}

///
/// Medial habenula
/// 
/// For this essay, used for motivated (odor-seeking) locomotion
///

struct HabenulaState {
}

impl HabenulaState {
    fn persevere(&mut self) -> bool {
        true
    }

    fn decay(&mut self) {
    }
}

impl FromStore for HabenulaState {
    fn init(_store: &mut Store) -> Self {
        HabenulaState {
        }
    }
}

fn update_habenula_med(
    odor: Res<Olfactory>, 
    mut hb: Local<HabenulaState>,
    mut tectum: ResMut<TectumLocomotionStn>,
) {
    hb.decay();
    
    // "where" / "how" path
    if let Some(angle) = odor.avoid_dir() {
        if hb.persevere() {
            if 0.05 <= angle.to_unit() && angle.to_unit() <= 0.5 {
                tectum.away_odor().turn(Turn::Right, 1.);
            } else {
                tectum.away_odor().turn(Turn::Left, 1.);
            }
        }
    }
}
pub struct HabenulaMedPlugin;

impl Plugin for HabenulaMedPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "HabenulaMedPlugin requires MidLocomotorPlugin");
        assert!(app.contains_plugin::<OlfactoryPlugin>(), "HabenulaMedPlugin requires OlfactoryPlugin");

        app.system(Tick, update_habenula_med);
    }
}
