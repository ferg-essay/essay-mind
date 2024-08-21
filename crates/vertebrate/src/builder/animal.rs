use essay_ecs::{app::App, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    body::{BodyEatPlugin, BodyPlugin}, 
    core_motive::{CoreWakePlugin, Dwell, Motive}, 
    hind_motor::{HindEat, HindEatPlugin, HindMovePlugin}, 
    olfactory_bulb::OlfactoryPlugin, 
    retina::RetinaPlugin, 
    tectum::{TectumLoomingPlugin, TectumPlugin},
};

pub struct AnimalBuilder {
    body: BodyPlugin,
    body_eat: BodyEatPlugin,

    hind_move: HindMovePlugin,
    hind_eat: HindEatPlugin,

    olfactory: OlfactoryPlugin,
    retina: RetinaPlugin,

    dwell: Option<DwellMode>,
}

impl AnimalBuilder {
    pub fn new() -> Self {
        Self {
            body: BodyPlugin::new(),
            body_eat: BodyEatPlugin,

            hind_move: HindMovePlugin,
            hind_eat: HindEatPlugin,

            olfactory: OlfactoryPlugin::new(),
            retina: RetinaPlugin::new(),

            dwell: None,
        }
    }

    pub fn olfactory(&mut self) -> &mut OlfactoryPlugin {
        &mut self.olfactory
    }

    pub fn retina(&mut self) -> &mut RetinaPlugin {
        &mut self.retina
    }

    pub fn dwell(&mut self, dwell: DwellMode) {
        self.dwell = Some(dwell);
    }

    pub fn build(self, app: &mut App) {
        app.plugin(self.body);
        app.plugin(self.body_eat);

        //app.plugin(HindLevyPlugin);
        //app.plugin(_HindMovePlugin);
        app.plugin(self.hind_move);
        app.plugin(self.hind_eat);

        app.plugin(self.olfactory);
        app.plugin(self.retina);

        app.plugin(TectumPlugin::new().striatum());
        app.plugin(TectumLoomingPlugin::new());
        // app.plugin(ChemotaxisPlugin);
        // app.plugin(TegSeekPlugin::<OlfactoryBulb, FoodSearch>::new());
        //app.plugin(KlinotaxisPlugin::<OlfactoryBulb, FoodSearch>::new());
        // app.plugin(LateralLinePlugin);

        //app.plugin(MidMotorPlugin);

        app.plugin(CoreWakePlugin::new());
        // app.plugin(CoreExplorePlugin);
        // app.plugin(CorePeptidesPlugin);
        // app.plugin(CoreEatingPlugin);

        for dwell in &self.dwell {
            match dwell {
                DwellMode::Eat => {
                    app.system(Tick, dwell_eat);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwellMode {
    Eat,
}

///
/// Set the Dwell motive if the animal is eating
/// 
fn dwell_eat(
    mut dwell: ResMut<Motive<Dwell>>,
    eat: Res<HindEat>,
) {
    if eat.is_eat() {
        dwell.set_max(1.);
    }
}
