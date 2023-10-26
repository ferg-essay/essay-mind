use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, core::{store::FromStore, Store, Local}};
use mind_ecs::Tick;

use crate::{
    habenula::Habenula, 
    mid_locomotor::MidLocomotorPlugin, 
    olfactory::{OlfactoryPlugin, Olfactory}, 
    tectum::TectumLocomotionStn, action::Turn
};

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
