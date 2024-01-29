use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use log::{log, Level};
use mind_ecs::Tick;

use crate::{
    body::{Body, BodyEat, BodyEatPlugin}, 
    util::{DecayValue, HalfLife}, 
    world::World
};

pub struct HindEat {
    eat_enable: DecayValue,
    is_eat_while_move: bool,
    is_food_zone: bool,
}

impl HindEat {
    pub const HALF_LIFE : HalfLife = HalfLife(0.4);

    fn is_enable(&self) -> bool {
        true
    } 

    fn is_eat_allowed(&self, body: &Body) -> bool {
        ! self.is_eat_while_move || body.speed() < 0.1
    } 

    #[inline]
    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone
    } 
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            eat_enable: DecayValue::new(HindEat::HALF_LIFE),
            is_eat_while_move: true,
            is_food_zone: false,
        }
    }
}

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
    world: Res<World>,
) {
    if ! hind_eat.is_enable() {
        return;
    }

    if ! body_eat.is_food_zone() {
        // log!(Level::Debug, "eating without sensor");
        return;
    }

    if ! hind_eat.is_eat_allowed(body.get()) {
        log!(Level::Info, "eating while moving");
        println!("Eating while moving");
        return
    }

    println!("Eating");
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
