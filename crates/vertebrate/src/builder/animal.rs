use essay_ecs::app::App;

use crate::{
    body::{BodyEatPlugin, BodyPlugin}, 
    core_motive::CoreWakePlugin, 
    hind_motor::{HindEatPlugin, HindMovePlugin}, 
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
        }
    }

    pub fn olfactory(&mut self) -> &mut OlfactoryPlugin {
        &mut self.olfactory
    }

    pub fn retina(&mut self) -> &mut RetinaPlugin {
        &mut self.retina
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

        // app.system(Tick, dwell_olfactory);
        //app.system(Tick, dwell_eat);
    }
}
