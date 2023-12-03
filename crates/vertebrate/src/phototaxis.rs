///
/// phototaxis
///

use essay_ecs::prelude::{Plugin, App, ResMut, Res};
use essay_plot::prelude::Angle;
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::{World, OdorType}, mid_explore::MidExplore, mid_locomotor::MidLocomotorPlugin, util::DecayValue,
};

pub struct Phototaxis {
    average: DecayValue,
    value: f32,
}

impl Phototaxis {
    pub fn average(&self) -> f32 {
        self.average.value()
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn update(&mut self, value: f32) {
        self.value = value;
        self.average.update();
        self.average.add(value);
    }

    pub fn gradient(&self) -> f32 {
        self.value() - self.average()
    }
}

impl Default for Phototaxis {
    fn default() -> Self {
        Self { 
            average: DecayValue::new(20),
            value: 0.,
        }
    }
}

fn update_phototaxis(
    body: Res<Body>, 
    world: Res<World>, 
    mut explore: ResMut<MidExplore>,
    mut phototaxis: ResMut<Phototaxis>,
) {
    let light = world.light(body.pos_head());

    // TODO: negative light is an error value
    if light >= 0.0 {
        // update the light average
        phototaxis.update(light);
    }

    // the 5HT cells average the past light
    let average = phototaxis.average.value();

    let diff = phototaxis.gradient();

    if diff >= 0.2 {
        // positive gradient, move forward, avoiding the current area
        explore.avoid(); 
    } else if diff <= -0.2 {
        // negative gradient, turn avound
        explore.avoid_turn();
    } else if light > 0.5 {
        explore.normal();
    } else {
        explore.avoid();
    }
}

pub struct PhototaxisPlugin;

impl Plugin for PhototaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>());

        app.init_resource::<Phototaxis>();

        app.system(Tick, update_phototaxis);
    }
}
