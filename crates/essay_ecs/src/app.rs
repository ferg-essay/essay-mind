use crate::{
    system::prelude::{IntoSystem, Schedule}, 
    world::prelude::{World}, entity::{prelude::{Insert, EntityId}},
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tick(u64);

pub struct App {
    schedule: Schedule,
    world: World<'static>,
}

impl Tick {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            schedule: Schedule::new(),
            world: World::new(),
        };
        
        app.add_resource(Tick(0));

        app
    }

    pub fn add_system<M>(&mut self, into_system: impl IntoSystem<(), M>) -> &mut Self
    {
        self.schedule.push(Box::new(IntoSystem::into_system(
            into_system,
            &mut self.world,
        )));

        self
    }

    pub fn spawn<T:Insert>(&mut self, value: T) -> EntityId {
        self.world.spawn(value)
    }

    pub fn add_resource<T:'static>(&mut self, value: T) {
        self.world.add_resource(value);
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

    pub fn update(&mut self) -> &mut Self {
        self.world.resource_mut::<Tick>().0 += 1;
        self.schedule.update(&self.world);
        self
    }
}
/*
pub trait IntoSystem<M> {
    fn to_system(&self) -> Box<dyn System>;
}
 */

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::prelude::*;

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

        app.add_resource(TestA(1));
        assert_eq!(app.resource::<TestA>(), &TestA(1));

        app.add_resource(TestB(2));
        assert_eq!(app.resource::<TestA>(), &TestA(1));
        assert_eq!(app.resource::<TestB>(), &TestB(2));
    }

    #[derive(Debug, PartialEq)]
    struct TestA(u32);

    #[derive(Debug, PartialEq)]
    struct TestB(u32);

    fn take(ptr: &Rc<RefCell<Vec<String>>>) -> String {
        ptr.borrow_mut().drain(..).collect::<Vec<String>>().join(", ")
    }

    fn test_system() {
        println!("hello");
    }

}