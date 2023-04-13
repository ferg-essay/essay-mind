///
/// See Bevy schedule.rs
/// 
use std::{hash::{Hash, Hasher}, collections::HashMap, any::Any};

use crate::{world::prelude::World, prelude::System, util::prelude::DynLabel};

pub type BoxedSystem<Out=()> = Box<dyn System<Out=Out>>;
pub type BoxedLabel = Box<dyn ScheduleLabel>;

pub struct Schedule {
    systems: Vec<BoxedSystem>,
}

pub struct Schedules {
    schedule: Schedule,

    schedule_map: HashMap<BoxedLabel, Schedule>,
}

pub trait ScheduleLabel : DynLabel {
    fn box_clone(&self) -> BoxedLabel;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MainLabel {
    A,
    B,
    C,
}

impl ScheduleLabel for MainLabel {
    fn box_clone(&self) -> BoxedLabel {
        Box::new(Clone::clone(self))
    }
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

    pub fn run(&mut self, world: &World) {
        for system in &mut self.systems {
            system.run(world);
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

    pub fn add_system(&mut self, label: &dyn ScheduleLabel, system: BoxedSystem) {
        self.schedule_map.get_mut(label)
            .expect("add_system with an unknown schedule")
            .push(system);
    }

    pub fn run(&mut self, label: &dyn ScheduleLabel, world: &World) {
        let (key, mut schedule) = self.schedule_map.remove_entry(label).unwrap();
        
        schedule.run(world);

        self.schedule_map.insert(key, schedule);
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
