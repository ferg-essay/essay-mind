use std::marker::PhantomData;

use crate::{store::prelude::{Table, RowRef}, type_meta::TypeMetas};

pub struct Resources<'w> {
    types: TypeMetas,
    table: Table<'w>,
    //resources: Vec<RowMeta>,
}

impl<'w> Resources<'w> {
    pub fn new() -> Self {
        Self {
            types: TypeMetas::new(),
            table: Table::new(),
            //resources: Vec::new(),
        }
    }

    pub fn set<T:'static>(&mut self, value: T) -> RowRef<T> {
        let entity_ref = self.create_ref::<T>();

        self.set_ref(&entity_ref, value);

        entity_ref
    }

    pub fn set_ref<T:'static>(&mut self, entity_ref: &RowRef<T>, value: T) {
        self.table.replace(entity_ref, value);
    }

    pub fn get_by_type<T:'static>(&mut self) -> Option<&T> {
        let type_id = self.types.add_type::<T>();

        let en_ref = self.table.create_ref::<T>(type_id.index() as u32);

        //self.table.get(&self.create_ref::<T>())
        self.table.get(&en_ref)
    }

    pub fn get_mut_by_type<T:'static>(&mut self) -> Option<&mut T> {
        let type_id = self.types.add_type::<T>();

        let en_ref = self.table.create_ref::<T>(type_id.index() as u32);

        //self.table.get(&self.create_ref::<T>())
        self.table.get_mut(&en_ref)
    }

    pub fn create_ref<T:'static>(&mut self) -> RowRef<T> {
        let type_id = self.types.add_type::<T>();

        self.table.create_ref::<T>(type_id.index() as u32)
    }

    pub fn get<T:'static>(&self, entity: &RowRef<T>) -> Option<&T> {
        self.table.get(entity)
    }

    pub fn get_mut<T:'static>(&mut self, entity: &RowRef<T>) -> Option<&mut T> {
        self.table.get_mut(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::Resources;

    #[test]
    fn set_get() {
        let mut resources = Resources::new();

        let res_a = resources.set(TestA(1));
        assert_eq!(resources.get(&res_a), Some(&TestA(1)));

        let res_b = resources.set(TestB(2));
        assert_eq!(resources.get(&res_b), Some(&TestB(2)));

        let res_a2 = resources.set(TestA(3));
        assert_eq!(resources.get(&res_a2), Some(&TestA(3)));
        assert_eq!(resources.get(&res_a), Some(&TestA(3)));
    }

    #[derive(PartialEq, Debug)]
    struct TestA(u32);

    #[derive(PartialEq, Debug)]
    struct TestB(u32);
}
