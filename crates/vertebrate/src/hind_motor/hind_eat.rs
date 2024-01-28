use essay_ecs::{app::{App, Plugin}, core::Res};
use mind_ecs::Tick;

use crate::{body::{Body, BodyPlugin}, hind_motor::{HindLocomotor, HindLocomotorEvent}, world::World};

pub struct HindEat {

}

fn update_hind_eat(
    body: Res<Body>,
    world: Res<World>,
) {
    // if world.isbody.is
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotorPlugin requires BodyPlugin");
        // assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        // app.init_resource::<Explore>();
        app.event::<HindLocomotorEvent>();
        app.init_resource::<HindLocomotor>();

        app.system(Tick, update_hind_eat);
    }
}
