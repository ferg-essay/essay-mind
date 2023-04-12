use crate::{entity::{prelude::{Table, QueryIterator, Query, Insert}, ptr::PtrCell}};

use super::resource::Resources;

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

    pub fn add_entity<T:Insert>(&mut self, value: T) {
        self.ptr.deref_mut().table.push::<T>(value);
    }

    pub(crate) fn query<T:Query>(&self) -> QueryIterator<'_,'w,T> {
        self.ptr.deref_mut().table.query::<T>()
    }

    pub fn eval<'a,T:Query,F>(&self, fun: &mut F)
        where F: FnMut(T)
    {
        todo!();
        /*
        for entity in self.ptr.deref_mut().entities.query::<T>() {
            fun(entity);
        }
        */
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
}

impl<'w> WorldInner<'w> {
    fn new() -> Self {
        Self {
            table: Table::new(),
            resources: Resources::new(),
        }
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

        world.add_entity(TestA(1));
        assert_eq!(world.len(), 1);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        /*
        world.add_entity(TestB(10000));
        assert_eq!(world.len(), 2);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000)");

        world.add_entity(TestB(100));
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
        */
    }

    #[derive(Component, Debug)]
    struct TestA(u32);

    #[derive(Component, Debug)]
    struct TestB(u16);
}