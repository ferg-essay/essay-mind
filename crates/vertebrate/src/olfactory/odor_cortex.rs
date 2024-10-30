use essay_ecs::{app::{App, Plugin}, core::{Query, Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, util::{Ticks, TimeoutValue}, world::Food};

use super::odor_place::OdorPlace;

fn update_odor_cortex(
    mut odor_cortex: ResMut<OdorCortex>,
    odor_place: ResMut<OdorPlace>,
    body: Res<Body>,
    food: Query<&Food>,
) {
    odor_cortex.pre_update();

    let is_food = food.iter()
        .any(|food| food.is_pos(body.head_pos()));

    

    odor_cortex.is_food_zone.set(is_food);
}

pub struct OdorCortex {
    is_food_zone: TimeoutValue<bool>,
}

impl OdorCortex {
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
        let olfactory = OdorCortex::new();

        app.insert_resource(olfactory);

        app.system(Tick, update_odor_cortex);
    }
}
