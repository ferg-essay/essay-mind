use super::system::System;

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            systems: Vec::new(),
        }
    }

    pub fn push(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.run();
        }
    }
}
