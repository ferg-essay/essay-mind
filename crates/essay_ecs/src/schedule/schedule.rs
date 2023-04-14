///
/// See Bevy schedule.rs
/// 
use std::{hash::{Hash, Hasher}, collections::HashMap};

use crate::{world::prelude::World, prelude::{System, IntoSystem}, util::prelude::DynLabel};

pub type BoxedSystem<Out=()> = Box<dyn System<Out=Out>>;
pub type BoxedLabel = Box<dyn ScheduleLabel>;

pub struct Schedule {
    new_systems: Vec<BoxedSystem>,

    systems: Vec<BoxedSystem>,
}

pub struct Schedules {
    schedule: Schedule,

    schedule_map: HashMap<BoxedLabel, Schedule>,
}

pub trait ScheduleLabel : DynLabel {
    fn box_clone(&self) -> BoxedLabel;
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            new_systems: Vec::new(),

            systems: Vec::new(),
        }
    }

    /*
    pub fn push(&mut self, system: BoxedSystem) {
        self.systems.push(system);
    }
    */

    pub(crate) fn add_system<M>(&mut self, system: impl IntoSystem<(),M>) {
        self.new_systems.push(Box::new(IntoSystem::into_system(system)));
    }

    pub fn init(&mut self, world: &mut World) {
        for mut system in self.new_systems.drain(..) {
            system.init(world);

            self.systems.push(system);
        }
    }

    pub fn run(&mut self, world: &mut World) {
        self.init(world);
        self.run_systems(world);
        self.flush(world);
    }

    pub fn run_systems(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }

    fn flush(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.flush(world);
        }
    }
}

impl Schedules {
    pub fn get(&self, label: &dyn ScheduleLabel) -> Option<&Schedule> {
        self.schedule_map.get(label)
    }

    pub fn insert(&mut self, label: impl ScheduleLabel, schedule: Schedule) -> Option<Schedule> {
        self.schedule_map.insert(label.box_clone(), schedule)
    }

    pub fn add_system<M>(
        &mut self, 
        label: &dyn ScheduleLabel, 
        system: impl IntoSystem<(),M>
    ) {
        self.schedule_map.get_mut(label)
            .expect("add_system with an unknown schedule")
            .add_system(system);
    }

    pub fn run(&mut self, label: &dyn ScheduleLabel, world: &mut World) {
        let (key, mut schedule) = self.schedule_map.remove_entry(label).unwrap();
        
        schedule.init(world);
        schedule.run(world);
        schedule.flush(world);

        self.schedule_map.insert(key, schedule);
    }

    pub(crate) fn remove(
        &mut self, 
        label: &dyn ScheduleLabel
    ) -> Option<Schedule> {
        self.schedule_map.remove(label)
    }

    pub(crate) fn remove_entry(
        &mut self, 
        label: &dyn ScheduleLabel
    ) -> Option<(BoxedLabel, Schedule)> {
        self.schedule_map.remove_entry(label)
    }
}

impl Default for Schedules {
    fn default() -> Self {
        Self { 
            schedule: Schedule::new(),
            schedule_map: HashMap::new(),
         }
    }
}

impl PartialEq for dyn ScheduleLabel {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn ScheduleLabel {}

impl Hash for dyn ScheduleLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}
