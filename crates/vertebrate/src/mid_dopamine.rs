use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, core::{store::FromStore, Store, Local}};
use mind_ecs::Tick;

use crate::{mid_locomotor::{MidLocomotorPlugin, ApproachMlr}, body::Body, world::World, habenula::Habenula, olfactory::{OlfactoryPlugin, Olfactory}};

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
    fn persist(&mut self) -> bool {
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
    mut approach: ResMut<ApproachMlr>,
) {
    da.decay();
    
    // "where" / "how" path
    if let Some(angle) = odor.dir() {
        if da.persist() {
            approach.turn(angle);
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
