use crate::{entity::{prelude::{Table, ViewIterator, View, Insert, PtrCell}}};

use super::resource::Resources;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tick(u64);

pub struct World<'w> {
    ptr: PtrCell<'w,WorldInner<'w>>,
}

impl<'w> World<'w> {
    pub fn new() -> Self {
        Self {
            ptr: PtrCell::new(WorldInner::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.ptr.deref().table.len()
    }

    pub fn spawn<T:Insert>(&mut self, value: T) {
        self.ptr.deref_mut().table.push::<T>(value);
    }

    pub(crate) fn query<T:View>(&self) -> ViewIterator<'_,'w,T> {
        self.ptr.deref_mut().table.iter_view::<T>()
    }

    pub fn eval<'a,T:View,F>(&self, fun: &mut F)
        where F: FnMut(T) + FnMut(<T as View>::Item<'w>)
    {
        for arg in self.ptr.deref_mut().table.iter_view::<T>() {
            fun(arg);
        }
    }

    pub fn ticks(&self) -> Tick {
        self.ptr.deref().tick
    }

    pub fn next_tick(&mut self) -> Tick {
        self.ptr.deref_mut().next_tick()
    }

    pub fn add_resource<T:'static>(&mut self, value: T) {
        // self.ptr.deref_mut().resources.set(value);
    }
    
    pub fn get_resource<T:'static>(&self) -> Option<&T> {
        self.ptr.deref_mut().resources.get_by_type()
    }
    
    pub fn get_resource_mut<T:'static>(&self) -> Option<&mut T> {
        self.ptr.deref_mut().resources.get_mut_by_type()
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
    use essay_ecs_macros::Component;

    use super::World;

    #[test]
    fn spawn() {
        let mut world = World::new();
        assert_eq!(world.len(), 0);

        world.spawn(TestA(1));
        assert_eq!(world.len(), 1);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        world.spawn(TestB(10000));
        assert_eq!(world.len(), 2);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000)");

        world.spawn(TestB(100));
        assert_eq!(world.len(), 3);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| t.0 += 1);
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[derive(Component, Debug)]
    struct TestA(u32);

    #[derive(Component, Debug)]
    struct TestB(u16);
}