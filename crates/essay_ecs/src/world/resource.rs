use std::{collections::HashMap, any::TypeId, cell::UnsafeCell};

use crate::{entity::prelude::{Table}, type_meta::TypeMetas};

use super::cell::Ptr;

struct IsResource;

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd)]
pub struct ResourceId(usize);

struct Resource<'r> {
    id: ResourceId,
    value: Ptr<'r>,
}

pub struct Resources<'r> {
    resource_map: HashMap<TypeId,ResourceId>,
    resources: Vec<Resource<'r>>,
    types: TypeMetas,
    table: Table<'r>,
    //resources: Vec<RowMeta>,
}

impl ResourceId {
    fn new(index: usize) -> Self {
        ResourceId(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

impl<'r> Resource<'r> {
    fn new<T>(id: ResourceId, value: T) -> Self {
        Resource {
            id: id,
            value: Ptr::new(value),
        }
    }

    unsafe fn deref<T>(&self) -> &'r T {
        self.value.deref()
    }

    unsafe fn deref_mut<T>(&self) -> &'r mut T {
        self.value.deref_mut()
    }
}

impl<'w> Resources<'w> {
    pub fn new() -> Self {
        Self {
            resource_map: HashMap::new(),
            resources: Vec::new(),
            types: TypeMetas::new(),
            table: Table::new(),
            //resources: Vec::new(),
        }
    }

    pub fn set<T:'static>(&mut self, value: T) {
        let id = ResourceId::new(self.resources.len());
        let type_id = TypeId::of::<T>();

        let id = *self.resource_map.entry(type_id).or_insert(id);

        if id.index() == self.resources.len() {
            self.resources.push(Resource::new(id, value));
        } else {
            // TODO: drop
            self.resources[id.index()] = Resource::new(id, value);
        }
    }

    pub fn get<T:'static>(&mut self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        let id = self.resource_map.get(&type_id)?;

        unsafe { Some(self.resources[id.index()].deref()) }
    }

    pub fn get_mut<T:'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        let id = self.resource_map.get(&type_id)?;

        unsafe { Some(self.resources[id.index()].deref_mut()) }
    }
}

#[cfg(test)]
mod tests {
    use super::Resources;

    #[test]
    fn set_get() {
        let mut resources = Resources::new();

        assert_eq!(resources.get::<TestB>(), None);
        assert_eq!(resources.get_mut::<TestB>(), None);

        resources.set(TestA(1));
        assert_eq!(resources.get::<TestA>(), Some(&TestA(1)));
        assert_eq!(resources.get_mut::<TestA>(), Some(&mut TestA(1)));
        assert_eq!(resources.get::<TestB>(), None);
        assert_eq!(resources.get_mut::<TestB>(), None);

        resources.get_mut::<TestA>().unwrap().0 += 1;

        assert_eq!(resources.get::<TestA>(), Some(&TestA(2)));
        assert_eq!(resources.get_mut::<TestA>(), Some(&mut TestA(2)));
        assert_eq!(resources.get::<TestB>(), None);
        assert_eq!(resources.get_mut::<TestB>(), None);

        resources.set(TestA(1000));
        assert_eq!(resources.get::<TestA>(), Some(&TestA(1000)));
        assert_eq!(resources.get_mut::<TestA>(), Some(&mut TestA(1000)));
        assert_eq!(resources.get::<TestB>(), None);
        assert_eq!(resources.get_mut::<TestB>(), None);

        resources.set(TestB(1001));
        assert_eq!(resources.get::<TestA>(), Some(&TestA(1000)));
        assert_eq!(resources.get_mut::<TestA>(), Some(&mut TestA(1000)));
        assert_eq!(resources.get::<TestB>(), Some(&TestB(1000)));
        assert_eq!(resources.get_mut::<TestB>(), Some(&mut TestB(1000)));
    }

    #[derive(PartialEq, Debug)]
    struct TestA(u32);

    #[derive(PartialEq, Debug)]
    struct TestB(u32);
}
