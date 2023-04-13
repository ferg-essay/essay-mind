use std::cell::UnsafeCell;

use crate::{entity::{prelude::{Table, ViewIterator, View, Insert, EntityId}}, prelude::{System, IntoSystem}};

use super::resource::Resources;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tick(u64);

pub struct World<'w> {
    ptr: UnsafeCell<WorldInner<'w>>,
}

impl<'w> World<'w> {
    pub fn new() -> Self {
        Self {
            ptr: UnsafeCell::new(WorldInner::new()),
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.ptr.get()).table.len() }
    }

    pub fn spawn<T:Insert>(&mut self, value: T) -> EntityId {
        self.ptr.get_mut().table.spawn::<T>(value)
    }

    pub fn get<T:'static>(&mut self, id: EntityId) -> Option<&T> {
        unsafe { (*self.ptr.get()).table.get::<T>(id) }
    }

    pub fn get_mut<T:'static>(&mut self, id: EntityId) -> Option<&mut T> {
        unsafe { (*self.ptr.get()).table.get_mut::<T>(id) }
    }

    pub fn view<V:View>(&self) -> ViewIterator<'_,'w,V> {
        unsafe { (*self.ptr.get()).table.iter_view::<V>() }
    }

    /*
    pub fn eval<'a,V:View,F>(&self, fun: &mut F)
        where F: FnMut(V) + FnMut(<V as View>::Item<'w>)
    {
        for arg in self.view::<V>() {
            fun(arg);
        }
    }
    */

    /*
    pub fn eval<'a,F>(&self, fun: &mut impl System)
    {
        fun.run(&self);
    }
    */

    pub fn eval<M>(&mut self, fun: impl IntoSystem<M>)
    {
        let mut system = IntoSystem::into_system(
            fun,
            self,
        );

        system.run(&self)
    }

    pub fn ticks(&self) -> Tick {
        unsafe { (*self.ptr.get()).tick }
    }

    pub fn next_tick(&mut self) -> Tick {
        self.ptr.get_mut().next_tick()
    }

    pub fn add_resource<T:'static>(&mut self, value: T) {
        self.ptr.get_mut().resources.set::<T>(value)
    }
    
    pub fn get_resource<T:'static>(&self) -> Option<&T> {
        unsafe { (*self.ptr.get()).resources.get::<T>() }
    }
    
    pub fn get_resource_mut<T:'static>(&self) -> Option<&mut T> {
        unsafe { (*self.ptr.get()).resources.get_mut::<T>() }
    }
}

pub struct WorldInner<'w> {
    table: Table<'w>,
    resources: Resources<'w>,

    tick: Tick,
}

impl<'w> WorldInner<'w> {
    fn new() -> Self {
        Self {
            table: Table::new(),
            resources: Resources::new(),
            tick: Tick(0),
        }
    }

    fn next_tick(&mut self) -> Tick {
        self.tick = Tick(self.tick.0 + 1);
        self.tick
    }
}

impl From<Tick> for u64 {
    fn from(value: Tick) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use essay_ecs_macros::Component;

    use super::World;

    #[test]
    fn spawn() {
        let mut world = World::new();
        assert_eq!(world.len(), 0);

        let id_a = world.spawn(TestA(1));
        assert_eq!(world.len(), 1);

        assert_eq!(world.get::<TestA>(id_a), Some(&TestA(1)));
        assert_eq!(world.get::<TestB>(id_a), None);

        let id_b = world.spawn(TestB(10000));
        assert_eq!(world.len(), 2);

        assert_eq!(world.get::<TestA>(id_a), Some(&TestA(1)));
        assert_eq!(world.get::<TestB>(id_b), Some(&TestB(10000)));

        assert_eq!(world.get::<TestA>(id_b), None);
        assert_eq!(world.get::<TestB>(id_a), None);

        let id_b2 = world.spawn(TestB(100));
        assert_eq!(world.len(), 3);

        assert_eq!(world.get::<TestA>(id_a), Some(&TestA(1)));
        assert_eq!(world.get::<TestA>(id_b), None);
        assert_eq!(world.get::<TestA>(id_b2), None);

        assert_eq!(world.get::<TestB>(id_b), Some(&TestB(10000)));
        assert_eq!(world.get::<TestB>(id_b2), Some(&TestB(100)));

        world.get_mut::<TestB>(id_b).unwrap().0 += 1;
        world.get_mut::<TestB>(id_b2).unwrap().0 += 1;

        assert_eq!(world.get::<TestB>(id_b), Some(&TestB(10001)));
        assert_eq!(world.get::<TestB>(id_b2), Some(&TestB(101)));
        assert_eq!(world.get::<TestA>(id_a), Some(&TestA(1)));
    }

    #[test]
    fn resource_set_get() {
        let mut world = World::new();

        assert_eq!(world.get_resource::<TestB>(), None);
        assert_eq!(world.get_resource_mut::<TestB>(), None);

        world.add_resource(TestA(1));
        assert_eq!(world.get_resource::<TestA>(), Some(&TestA(1)));
        assert_eq!(world.get_resource_mut::<TestA>(), Some(&mut TestA(1)));
        assert_eq!(world.get_resource::<TestB>(), None);
        assert_eq!(world.get_resource_mut::<TestB>(), None);

        world.get_resource_mut::<TestA>().unwrap().0 += 1;

        assert_eq!(world.get_resource::<TestA>(), Some(&TestA(2)));
        assert_eq!(world.get_resource_mut::<TestA>(), Some(&mut TestA(2)));
        assert_eq!(world.get_resource::<TestB>(), None);
        assert_eq!(world.get_resource_mut::<TestB>(), None);

        world.add_resource(TestA(1000));
        assert_eq!(world.get_resource::<TestA>(), Some(&TestA(1000)));
        assert_eq!(world.get_resource_mut::<TestA>(), Some(&mut TestA(1000)));
        assert_eq!(world.get_resource::<TestB>(), None);
        assert_eq!(world.get_resource_mut::<TestB>(), None);

        world.add_resource(TestB(1001));
        assert_eq!(world.get_resource::<TestA>(), Some(&TestA(1000)));
        assert_eq!(world.get_resource_mut::<TestA>(), Some(&mut TestA(1000)));
        assert_eq!(world.get_resource::<TestB>(), Some(&TestB(1001)));
        assert_eq!(world.get_resource_mut::<TestB>(), Some(&mut TestB(1001)));
    }

    #[test]
    fn eval() {
        let mut world = World::new();
        assert_eq!(world.len(), 0);

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        let ptr = values.clone();
        world.eval(move |a: &TestA| push(&ptr, format!("{:?}", a)));
        assert_eq!(take(&values), "");

        world.spawn(TestA(1001));
        let ptr = values.clone();
        world.eval(move |a: &TestA| push(&ptr, format!("{:?}", a)));
        assert_eq!(take(&values), "TestA(1001)");

        world.spawn(TestA(2002));
        let ptr = values.clone();
        world.eval(move |a: &TestA| push(&ptr, format!("{:?}", a)));
        assert_eq!(take(&values), "TestA(1001), TestA(2002)");

        world.eval(|a: &mut TestA| a.0 += 1);
        let ptr = values.clone();
        world.eval(move |a: &TestA| push(&ptr, format!("{:?}", a)));
        assert_eq!(take(&values), "TestA(1002), TestA(2003)");
    }

    #[test]
    fn eval_out() {
        let mut world = World::new();
        assert_eq!(world.len(), 0);

        let out = world.eval(|| { "result"; });
    }

    fn push(ptr: &Rc<RefCell<Vec<String>>>, value: String) {
        ptr.borrow_mut().push(value);
    }

    fn take(ptr: &Rc<RefCell<Vec<String>>>) -> String {
        ptr.borrow_mut().drain(..).collect::<Vec<String>>().join(", ")
    }

    #[derive(Component, Debug, PartialEq)]
    struct TestA(u32);

    #[derive(Component, Debug, PartialEq)]
    struct TestB(u16);
}