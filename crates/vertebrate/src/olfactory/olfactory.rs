use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, util::{Ticks, TimeoutValue}, world::World};

pub struct Olfactory {
    is_food_zone: TimeoutValue<bool>,
}

impl Olfactory {
    pub fn new() -> Self {
        Self {
            is_food_zone: TimeoutValue::<bool>::new(Ticks(3))
        }
    }

    #[inline]
    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone.is_active()
    }

    fn pre_update(&mut self) {
        self.is_food_zone.update();
    }
}

fn update_olfactory(
    mut olfactory: ResMut<Olfactory>,
    body: Res<Body>,
    world: Res<World>,
) {
    olfactory.pre_update();

    let is_food = world.is_food(body.head_pos());
    olfactory.is_food_zone.set(is_food);
}

pub struct OlfactoryPlugin {
}

impl OlfactoryPlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Plugin for OlfactoryPlugin {
    fn build(&self, app: &mut App) {
        let olfactory = Olfactory::new();

        app.insert_resource(olfactory);

        app.system(Tick, update_olfactory);
    }
}
