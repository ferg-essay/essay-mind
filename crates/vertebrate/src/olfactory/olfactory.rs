use essay_ecs::{app::{App, Plugin}, core::{Query, Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, util::{Ticks, TimeoutValue}, world::Food};

pub struct OlfactoryCortex {
    is_food_zone: TimeoutValue<bool>,
}

impl OlfactoryCortex {
    pub fn new() -> Self {
        Self {
            is_food_zone: TimeoutValue::<bool>::new(Ticks(3))
        }
    }

    #[inline]
    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone.value().unwrap_or(false)
    }

    fn pre_update(&mut self) {
        self.is_food_zone.update();
    }
}

fn update_olfactory(
    mut olfactory: ResMut<OlfactoryCortex>,
    body: Res<Body>,
    food: Query<&Food>,
) {
    olfactory.pre_update();

    let is_food = food.iter()
        .any(|food| food.is_pos(body.head_pos()));

    olfactory.is_food_zone.set(is_food);
}

pub struct OlfactoryCortexPlugin {
}

impl OlfactoryCortexPlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Plugin for OlfactoryCortexPlugin {
    fn build(&self, app: &mut App) {
        let olfactory = OlfactoryCortex::new();

        app.insert_resource(olfactory);

        app.system(Tick, update_olfactory);
    }
}
