///
/// see bevy ecs/../app.rs
/// 

use crate::{
    system::prelude::{IntoSystem, System}, 
    world::prelude::{World}, entity::{prelude::{Insert, EntityId}}, schedule::prelude::{Schedule, Schedules, ScheduleLabel},
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tick(u64);

pub struct App {
    world: World<'static>,
}

impl Tick {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoreSchedule {
    Startup,
    Main,
    Outer,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn empty() -> Self {
        let mut world = World::new();

        world.init_resource::<Schedules>();

        App {
            world: world,
        }
    }

    pub fn add_system<M>(&mut self, into_system: impl IntoSystem<(), M>) -> &mut Self
    {
        let system = Box::new(IntoSystem::into_system(
            into_system,
            &mut self.world,
        ));

        self.resource_mut::<Schedules>().add_system(&CoreSchedule::Main, system);
    
        self
    }

    pub fn spawn<T:Insert>(&mut self, value: T) -> EntityId {
        self.world.spawn(value)
    }

    pub fn get_resource<T:'static>(&mut self) -> Option<&T> {
        self.world.get_resource::<T>()
    }

    pub fn get_mut_resource<T:'static>(&mut self) -> Option<&mut T> {
        self.world.get_resource_mut::<T>()
    }

    pub fn resource<T:'static>(&mut self) -> &T {
        self.world.get_resource::<T>().expect("unassigned resource")
    }

    pub fn resource_mut<T:'static>(&mut self) -> &mut T {
        self.world.get_resource_mut::<T>().expect("unassigned resource")
    }

    pub fn insert_resource<T:'static>(&mut self, value: T) {
        self.world.insert_resource(value);
    }

    pub fn default_schedule(&mut self) -> &mut Self {
        self.add_schedule(CoreSchedule::Main, CoreSchedule::main_schedule());
        self.add_schedule(CoreSchedule::Startup, CoreSchedule::startup_schedule());
        self.add_schedule(CoreSchedule::Outer, CoreSchedule::outer_schedule());

        self
    }

    pub fn add_schedule(
        &mut self, 
        label: impl ScheduleLabel, 
        schedule: Schedule
    ) -> &mut Self {
        self.resource_mut::<Schedules>().insert(label, schedule);

        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.world.resource_mut::<Tick>().0 += 1;
        self.world.resource_mut::<Schedules>().run(&CoreSchedule::Main, &self.world);
        self
    }

    pub fn eval<R, M>(&mut self, fun: impl IntoSystem<R, M>) -> R
    {
        let mut system = IntoSystem::into_system(
            fun,
            &mut self.world,
        );

        system.run(&mut self.world)
    }
}

impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();

        app.insert_resource(Tick(0));

        app.default_schedule();

        app
    }
}

impl CoreSchedule {
    fn main_schedule() -> Schedule {
        Schedule::new()
    }

    fn outer_schedule() -> Schedule {
        Schedule::new()
    }

    fn startup_schedule() -> Schedule {
        Schedule::new()
    }
}

impl ScheduleLabel for CoreSchedule {
    fn box_clone(&self) -> Box<dyn ScheduleLabel> {
        Box::new(Clone::clone(self))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{prelude::*, world::prelude::Res};

    #[test]
    fn app_system() {
        let mut app = App::new();
        let value = Vec::<String>::new();
        let value = Rc::new(RefCell::new(value));
        let ptr = Rc::clone(&value);

        app.add_system(move || value.borrow_mut().push("update".to_string()));
        assert_eq!(take(&ptr), "");
        app.update();
        assert_eq!(take(&ptr), "update");
        app.update();
        app.update();
        assert_eq!(take(&ptr), "update, update");
    }

    #[test]
    fn app_resource() {
        let mut app = App::new();

        app.insert_resource(TestA(1));
        assert_eq!(app.resource::<TestA>(), &TestA(1));

        app.insert_resource(TestB(2));
        assert_eq!(app.resource::<TestA>(), &TestA(1));
        assert_eq!(app.resource::<TestB>(), &TestB(2));
    }

    #[test]
    fn eval() {
        let mut app = App::new();

        app.insert_resource(TestA(1));
        assert_eq!(app.eval(|r: Res<TestA>| r.clone()), TestA(1));

        app.insert_resource(TestB(2));
        assert_eq!(app.eval(|r: Res<TestA>| r.clone()), TestA(1));
        assert_eq!(app.eval(|r: Res<TestB>| r.clone()), TestB(2));
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestA(u32);

    #[derive(Debug, Clone, PartialEq)]
    struct TestB(u32);

    fn take(ptr: &Rc<RefCell<Vec<String>>>) -> String {
        ptr.borrow_mut().drain(..).collect::<Vec<String>>().join(", ")
    }

    fn test_system() {
        println!("hello");
    }

}