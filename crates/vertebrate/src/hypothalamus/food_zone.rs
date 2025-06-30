use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{olfactory::{OdorCortex, OlfactoryCortexPlugin}, subpallium::Striatum};

fn update_food_zone(
    mut food_zone: ResMut<FoodZone>,
    odor_cortex: Res<OdorCortex>,
    tick: Res<AppTick>,
) {
    food_zone.pre_update();

    // necessary each time because of striatum side effects (timeout)
    food_zone.is_food_zone = odor_cortex.is_food_zone() 
        && food_zone.food_zone_striatum.left_mut().is_active(tick.get());
}

pub struct FoodZone {
    is_food_zone: bool,
    food_zone_striatum: Striatum<FoodZone>,
}

impl FoodZone {
    fn new() -> Self {
        Self {
            is_food_zone: false,
            food_zone_striatum: Striatum::new(),
        }
    }

    fn pre_update(&mut self) {
    }

    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone
    }
}

pub struct FoodZonePlugin {
}

impl FoodZonePlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Plugin for FoodZonePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<OlfactoryCortexPlugin>(), "FoodZone requires Olfactory");

        let food_zone = FoodZone::new();
        app.insert_resource(food_zone);

        app.system(Tick, update_food_zone);
    }
}