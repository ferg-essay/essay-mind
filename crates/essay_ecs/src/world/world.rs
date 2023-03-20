use std::{mem, ptr::NonNull};

use super::{type_info::{TypeInfo, TypeInfos, TypeIndex}, ptr::{Ptr, PtrMut, PtrOwn}};

pub struct World<'w> {
    components: TypeInfos,
    entities: Vec<Entity>,
    table: Table<'w>,
}

pub struct Entity {
    type_id: TypeIndex, 
}

impl<'e> World<'e> {
    pub fn new() -> Self {
        Self {
            components: TypeInfos::new(),
            entities: Vec::new(),
            table: Table::new(),
        }
    }

    pub fn spawn<T:'static>(&mut self, value: T) {
        let type_id = self.components.add_type::<T>();

        self.entities.push(Entity { type_id });
        self.table.push(Row::new(value));
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn eval<T:'static,F>(&mut self, fun: &mut F)
        where F: FnMut(&mut T)
    {
        let type_id = self.components.add_type::<T>();

        for (row, entity) in &mut self.table.rows.iter_mut().zip(&self.entities) {
            if entity.type_id == type_id {
                unsafe { fun(row.ptr.as_mut().deref_mut()); }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn spawn() {
        let mut env = World::new();
        assert_eq!(env.len(), 0);

        env.spawn(TestA(1));
        assert_eq!(env.len(), 1);

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        env.spawn(TestB(10000));
        assert_eq!(env.len(), 2);

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000)");

        env.spawn(TestB(100));
        assert_eq!(env.len(), 3);

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        let mut values = Vec::<String>::new();
        env.eval(&mut|t: &mut TestB| t.0 += 1);
        env.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);
}