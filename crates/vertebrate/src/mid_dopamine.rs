use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, core::{store::FromStore, Store, Local}};
use mind_ecs::Tick;

use crate::{
    habenula::Habenula, 
    mid_locomotor::MidLocomotorPlugin, 
    olfactory::{OlfactoryPlugin, Olfactory}, 
    tectum_action::{TectumLocomotion, Turn}
};

///
/// Midbrain dopamine region
/// 
/// posterior tubuculum in Zebrafish
/// Snr in mammals (substantia nigra pars compacta)
/// 
/// For this essay, used for motivated (odor-seeking) locomotion
///

struct DopamineState {
    hb: Habenula,
}

impl DopamineState {
    fn persevere(&mut self) -> bool {
        self.hb.persist()
    }

    fn decay(&mut self) {
        self.hb.decay()
    }
}

impl FromStore for DopamineState {
    fn init(store: &mut Store) -> Self {
        DopamineState {
            hb: Habenula::init(store),
        }
    }
}

fn update_mid_dopamine(
    odor: Res<Olfactory>, 
    mut da: Local<DopamineState>,
    mut tectum: ResMut<TectumLocomotion>,
) {
    da.decay();
    
    // "where" / "how" path
    if let Some(angle) = odor.dir() {
        if da.persevere() {
            if 0.05 <= angle.to_unit() && angle.to_unit() <= 0.5 {
                tectum.toward().turn(Turn::Left, 1.);
            } else if 0.5 <= angle.to_unit() && angle.to_unit() <= 0.95 {
                tectum.toward().turn(Turn::Right, 1.);
            }
        }
    }
}
pub struct MidDopaminePlugin;

impl Plugin for MidDopaminePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "MidDopaminePlugin requires MidLocomotorPlugin");
        assert!(app.contains_plugin::<OlfactoryPlugin>(), "MidDopaminePlugin requires OlfactoryPlugin");

        app.system(Tick, update_mid_dopamine);
    }
}
