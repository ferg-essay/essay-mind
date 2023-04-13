use crate::world::prelude::World;

use super::system::System;

pub type BoxedSystem<Out=()> = Box<dyn System<Out=Out>>;

pub struct Schedule {
    systems: Vec<BoxedSystem>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            systems: Vec::new(),
        }
    }

    pub fn push(&mut self, system: BoxedSystem) {
        self.systems.push(system);
    }

    pub fn update(&mut self, world: &World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}
