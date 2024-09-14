use essay_ecs::{app::App, core::{Res, ResMut}};
use log::warn;
use mind_ecs::Tick;

use crate::{
    body::{BodyEatPlugin, BodyPlugin}, 
    hind_eat::{HindEat, HindEatPlugin}, 
    hind_move::HindMovePlugin, 
    hippocampus::HippocampusPlugin, 
    mid_move::{MidMovePlugin, MidSeekPlugin}, 
    motive::{Dwell, Forage, Motive, MotiveAlarmPlugin, MotiveAvoidPlugin, MotiveEatPlugin, MotiveForagePlugin, MotiveSleepPlugin}, 
    olfactory::{olfactory_bulb::{OlfactoryBulb, OlfactoryBulbPlugin}, OlfactoryCortexPlugin}, 
    retina::RetinaPlugin, 
    taxis::{klinotaxis::KlinotaxisPlugin, TaxisAvoidPlugin}, 
    tectum::{TectumLoomingPlugin, TectumPlugin} 
};

pub struct AnimalBuilder {
    body: BodyPlugin,
    body_eat: BodyEatPlugin,

    hind_move: HindMovePlugin,
    hind_eat: HindEatPlugin,

    olfactory_bulb: OlfactoryBulbPlugin,
    olfactory_cortex: OlfactoryCortexPlugin,
    retina: RetinaPlugin,

    is_motive_eating: bool,
    is_mid_seek: bool,
    is_mid_klinotaxis: bool,

    dwell: Option<DwellMode>,
}

impl AnimalBuilder {
    pub fn new() -> Self {
        Self {
            body: BodyPlugin::new(),
            body_eat: BodyEatPlugin,

            hind_move: HindMovePlugin,
            hind_eat: HindEatPlugin,

            olfactory_bulb: OlfactoryBulbPlugin::new(),
            olfactory_cortex: OlfactoryCortexPlugin::new(),
            retina: RetinaPlugin::new(),

            is_motive_eating: true,
            is_mid_seek: false,
            is_mid_klinotaxis: false,

            dwell: None,
        }
    }

    pub fn olfactory(&mut self) -> &mut OlfactoryBulbPlugin {
        &mut self.olfactory_bulb
    }

    pub fn retina(&mut self) -> &mut RetinaPlugin {
        &mut self.retina
    }

    pub fn motive(&mut self) -> MotiveBuilder {
        MotiveBuilder {
            builder: self,
        }
    }

    pub fn seek(&mut self) -> SeekBuilder {
        SeekBuilder {
            builder: self,
        }
    }

    pub fn dwell(&mut self, dwell: DwellMode) {
        self.dwell = Some(dwell);
    }

    pub fn build(self, app: &mut App) {
        app.plugin(self.body);
        app.plugin(self.body_eat);

        app.plugin(self.hind_move);
        app.plugin(self.hind_eat);

        app.plugin(self.olfactory_bulb);
        app.plugin(self.retina);

        app.plugin(TectumPlugin::new().striatum());
        app.plugin(TectumLoomingPlugin::new());

        app.plugin(self.olfactory_cortex);

        if self.is_motive_eating {
            app.plugin(MidMovePlugin);
            app.plugin(MotiveAlarmPlugin);
            app.plugin(MotiveEatPlugin);
            app.plugin(MotiveForagePlugin);
        }

        if self.is_mid_seek {
            if ! self.is_motive_eating {
                warn!("Tegmentum seek requires eating");
            }

            app.plugin(MidSeekPlugin::<OlfactoryBulb, Forage>::new());
        }

        app.plugin(TaxisAvoidPlugin::new());

        if self.is_mid_klinotaxis {
            if ! self.is_motive_eating {
                warn!("Tegmentum klinotaxis requires eating");
            }

            app.plugin(KlinotaxisPlugin::<OlfactoryBulb, Forage>::new());
        }
            // app.plugin(LateralLinePlugin);

        //app.plugin(MidMotorPlugin);

        app.plugin(MotiveSleepPlugin::new());
        // app.plugin(CoreExplorePlugin);
        // app.plugin(CorePeptidesPlugin);

        for dwell in &self.dwell {
            match dwell {
                DwellMode::Eat => {
                    app.system(Tick, dwell_eat);
                }
            }
        }

        // forebrain

        let mut ehc = HippocampusPlugin::new();
        ehc.digits(4).radix(4).seq(2);
        app.plugin(ehc);

        app.plugin(MotiveAvoidPlugin);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwellMode {
    Eat,
}

pub struct MotiveBuilder<'a> {
    builder: &'a mut AnimalBuilder,
}

impl MotiveBuilder<'_> {
    pub fn eat_enable(&mut self, is_enable: bool) -> &mut Self {
        self.builder.is_motive_eating = is_enable;

        self
    }
}

pub struct SeekBuilder<'a> {
    builder: &'a mut AnimalBuilder,
}

impl SeekBuilder<'_> {
    pub fn seek(&mut self, is_enable: bool) -> &mut Self {
        self.builder.is_mid_seek = is_enable;

        self
    }

    pub fn klinotaxis(&mut self, is_enable: bool) -> &mut Self {
        self.builder.is_mid_klinotaxis = is_enable;

        self
    }
}

///
/// Set the Dwell motive if the animal is eating
/// 
fn dwell_eat(
    mut dwell: ResMut<Motive<Dwell>>,
    eat: Res<HindEat>,
) {
    if eat.is_eating() {
        dwell.set_max(1.);
    }
}
