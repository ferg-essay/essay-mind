use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use log::{log, Level};
use mind_ecs::Tick;

use crate::{body::{Body, BodyEat, BodyEatPlugin, BodyPlugin}, hind_motor::{HindLocomotor, HindLocomotorEvent}, util::{DecayValue, HalfLife}, world::World};

pub struct HindEat {
    eat_enable : DecayValue,
}

impl HindEat {
    pub const HALF_LIFE : HalfLife = HalfLife(0.4);
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            eat_enable: DecayValue::new(HindEat::HALF_LIFE),
        }
    }
}

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
    world: Res<World>,
) {
    if hind_eat.eat_enable.value() <= 0.1 {
        return;
    }

    if ! body_eat.is_sensor_food() {
        log!(Level::Debug, "eating without sensor");
        return;
    }

    if body.speed() > 0.01 {
        log!(Level::Info, "eating while moving");
        return
    }
    // if world.isbody.is
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        // app.init_resource::<Explore>();
        // app.event::<HindLocomotorEvent>();
        app.init_resource::<HindEat>();

        app.system(Tick, update_hind_eat);
    }
}
