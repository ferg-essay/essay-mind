use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::{
    hippocampus::{Hippocampus, HippocampusPlugin}, 
    util::{Ticks, Timeout}
};

pub struct MotiveAvoid {
    is_avoid: Timeout,
}

impl MotiveAvoid {
    fn new() -> Self {
        Self {
            is_avoid: Timeout::new(Ticks(3)),
        }
    }

    fn is_avoid(&self) -> bool {
        self.is_avoid.is_active()
    }

    pub fn avoid(&mut self) {
        self.is_avoid.set();
    }

    fn update(&mut self) {
        self.is_avoid.update();
    }
}

pub struct MotiveAvoidPlugin;

fn update_avoid(
    mut avoid: ResMut<MotiveAvoid>,
    mut hc: ResMut<Hippocampus>,
) {
    avoid.update();

    if avoid.is_avoid() {
        hc.avoid();
    }
}

impl Plugin for MotiveAvoidPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HippocampusPlugin>(), "MotiveAvoidPlugin requires HippocampusPlugin");

        let avoid = MotiveAvoid::new();
        
        app.insert_resource(avoid);

        app.system(Tick, update_avoid);

        // Motives::insert::<Avoid>(app, Seconds(0.2));
    }
}

