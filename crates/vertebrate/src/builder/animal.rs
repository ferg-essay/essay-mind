use essay_ecs::{app::App, core::{Res, ResMut}};
use log::warn;
use mind_ecs::Tick;

use crate::{
    body::{BodyEatPlugin, BodyPlugin}, 
    hind_brain::{
        lateral_line::LateralLine2Plugin, r1_thigmotaxis::HindThigmotaxisPlugin, 
        HindAvoidPlugin, HindEat, HindEatPlugin, HindMovePlugin,
    }, 
    hippocampus::HippocampusPlugin, 
    mid_brain::{
        pretectum::{lateral_line::PretectumLateralLinePlugin, ObstaclePretectumPlugin, PretectumTouchPlugin}, 
        taxis::{klinotaxis::KlinotaxisPlugin, TaxisAvoidPlugin}, 
        tectum::{TectumLoomingPlugin, TectumOrientPlugin, TectumPlugin}, 
        MidMovePlugin, MidSeekContextPlugin, MidSeekPlugin
    }, 
    motive::{Dwell, Forage, Motive, MotiveAvoidPlugin, MotiveEatPlugin, MotiveForagePlugin, MotiveSleepPlugin}, 
    olfactory::{odor_place::OdorPlace, olfactory_bulb::{OlfactoryBulb, OlfactoryBulbPlugin}, OlfactoryCortexPlugin}, 
    retina::RetinaPlugin, 
    util::Seconds 
};

pub struct AnimalBuilder {
    body: BodyPlugin,
    body_eat: BodyEatPlugin,

    hind_avoid: HindAvoidPlugin,
    hind_eat: HindEatPlugin,
    hind_move: HindMovePlugin,
    hind_thigmotaxis: HindThigmotaxisPlugin,

    lateral_line: LateralLine2Plugin,
    olfactory_bulb: OlfactoryBulbPlugin,
    olfactory_cortex: OlfactoryCortexPlugin,
    retina: RetinaPlugin,

    pretectum_obstacle: ObstaclePretectumPlugin,
    pretectum_touch: PretectumTouchPlugin,
    pretectum_lateral_line: PretectumLateralLinePlugin,

    tectum_looming: TectumLoomingPlugin,
    tectum_thigmotaxis: TectumOrientPlugin,

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

            hind_avoid: HindAvoidPlugin::new(),
            hind_eat: HindEatPlugin::new(),
            hind_move: HindMovePlugin,
            hind_thigmotaxis: HindThigmotaxisPlugin::new(),

            lateral_line: LateralLine2Plugin,
            olfactory_bulb: OlfactoryBulbPlugin::new(),
            olfactory_cortex: OlfactoryCortexPlugin::new(),
            retina: RetinaPlugin::new(),

            pretectum_obstacle: ObstaclePretectumPlugin::new(),
            pretectum_touch: PretectumTouchPlugin::new(),
            pretectum_lateral_line: PretectumLateralLinePlugin::new(),

            tectum_thigmotaxis: TectumOrientPlugin::new(),
            tectum_looming: TectumLoomingPlugin::new(),

            is_motive_eating: true,
            is_mid_seek: false,
            is_mid_klinotaxis: false,

            dwell: None,
        }
    }

    pub fn lateral_line(&mut self) -> &mut LateralLine2Plugin {
        &mut self.lateral_line
    }

    pub fn olfactory(&mut self) -> &mut OlfactoryBulbPlugin {
        &mut self.olfactory_bulb
    }

    pub fn pretectum_obstacle(&mut self) -> &mut ObstaclePretectumPlugin {
        &mut self.pretectum_obstacle
    }

    pub fn pretectum_touch(&mut self) -> &mut PretectumTouchPlugin {
        &mut self.pretectum_touch
    }

    pub fn pretectum_lateral_line(&mut self) -> &mut PretectumLateralLinePlugin {
        &mut self.pretectum_lateral_line
    }

    pub fn tectum_looming(&mut self) -> &mut TectumLoomingPlugin {
        &mut self.tectum_looming
    }

    pub fn retina(&mut self) -> &mut RetinaPlugin {
        &mut self.retina
    }

    pub fn hind_eat(&mut self) -> &mut HindEatPlugin {
        &mut self.hind_eat
    }

    pub fn hind_thigmotaxis(&mut self) -> &mut HindThigmotaxisPlugin {
        &mut self.hind_thigmotaxis
    }

    pub fn tectum_thigmotaxis(&mut self) -> &mut TectumOrientPlugin {
        &mut self.tectum_thigmotaxis
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

        app.plugin(self.lateral_line);
        app.plugin(self.olfactory_bulb);
        app.plugin(self.retina);

        app.plugin(self.hind_avoid);
        app.plugin(self.hind_eat);
        app.plugin(self.hind_move);
        app.plugin(self.hind_thigmotaxis);

        app.plugin(self.pretectum_obstacle);
        app.plugin(self.pretectum_touch);
        app.plugin(self.pretectum_lateral_line);

        app.plugin(self.tectum_thigmotaxis);

        app.plugin(TectumPlugin::new().striatum());
        app.plugin(self.tectum_looming);

        app.plugin(self.olfactory_cortex);

        if self.is_motive_eating {
            app.plugin(MidMovePlugin);
            app.plugin(MotiveEatPlugin);
            app.plugin(MotiveForagePlugin);
        }

        if self.is_mid_seek {
            if ! self.is_motive_eating {
                warn!("Tegmentum seek requires eating");
            }

            let is_context = true;
            if is_context {
                app.plugin(MidSeekContextPlugin::<OlfactoryBulb, OdorPlace, Forage>::new().decay(Seconds(180.)));
            } else {
                app.plugin(MidSeekPlugin::<OlfactoryBulb, Forage>::new());
            }
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

        if let Some(dwell) = &self.dwell {
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
