use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use mind_ecs::Tick;

use crate::{
    mid_seek::mid_locomotor::MidLocomotorPlugin, 
    olfactory_bulb::{OlfactoryPlugin, OlfactoryBulb}, 
    action::Turn, motivation::mid_peptides::MidPeptides, 
};

use super::tectum::TectumLocomotionStn;

///
/// Midbrain dopamine region
/// 
/// posterior tubuculum in Zebrafish
/// Snr in mammals (substantia nigra pars compacta)
/// 
/// For this essay, used for motivated (odor-seeking) locomotion
///

fn update_tuberculum(
    odor: Res<OlfactoryBulb>, 
    peptides: Res<MidPeptides>,
    mut tectum: ResMut<TectumLocomotionStn>,
) {
    // "where" / "how" path
    if let Some(angle) = odor.food_dir() {
        // tectum.seek().effort();
        // tectum.seek().cost();
    
        if peptides.seek_food() > 0.3 {
            if 0.05 <= angle.to_unit() && angle.to_unit() <= 0.5 {
                tectum.seek().turn(Turn::Left, 1.);
            } else if 0.5 <= angle.to_unit() && angle.to_unit() <= 0.95 {
                tectum.seek().turn(Turn::Right, 1.);
            }
        }
    }
}
pub struct TuberculumPlugin;

impl Plugin for TuberculumPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "MidDopaminePlugin requires MidLocomotorPlugin");
        assert!(app.contains_plugin::<OlfactoryPlugin>(), "MidDopaminePlugin requires OlfactoryPlugin");

        app.system(Tick, update_tuberculum);
    }
}
