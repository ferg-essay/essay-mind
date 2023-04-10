use std::{
    any::{TypeId, type_name}, 
    borrow::Cow, 
    collections::HashMap
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct TypeIndex(usize);

impl TypeIndex {
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

pub struct TypeMeta {
    id: TypeIndex,
    name: Cow<'static, str>,
    type_id: Option<TypeId>,
}

impl TypeMeta {
    fn new<T:'static>(id: usize) -> Self {
        Self {
            id: TypeIndex(id),
            name: Cow::Borrowed(type_name::<T>()),
            type_id: Some(TypeId::of::<T>()),
        }
    }

    pub fn id(&self) -> TypeIndex {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }
}

impl std::fmt::Debug for TypeMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeInfo")
        .field("id", &self.id)
        .field("name", &self.name)
        .field("type_id", &self.type_id)
        .finish()
    }
}

#[derive(Debug, Default)]
pub struct TypeMetas {
    infos: Vec<TypeMeta>,
    type_to_id: HashMap<TypeId, usize>,
}

impl TypeMetas {
    pub fn new() -> Self {
        Self {
            .. Default::default()
        }
    }
    pub fn add_type<T:'static>(&mut self) -> TypeIndex {
        let type_id = TypeId::of::<T>();

        let id = self.type_to_id.entry(type_id)
            .or_insert_with(|| {
            let id = self.infos.len();

            self.infos.push(TypeMeta::new::<T>(id));

            id
        });

        TypeIndex(*id)
    }

    pub fn len(&self) -> usize {
        self.infos.len()
    }

    pub fn get_info(&self, id: TypeIndex) -> Option<&TypeMeta> {
        self.infos.get(id.0)
    }

    pub fn get_name(&self, id: TypeIndex) -> Option<&str> {
        self.get_info(id).map(|info| info.name())
    }

    pub fn get_id_by_type(&self, type_id: TypeId) -> Option<TypeIndex> {
        self.type_to_id.get(&type_id).map(|index| TypeIndex(*index))
    }

    pub fn get_id<T:'static>(&self) -> Option<TypeIndex> {
        self.get_id_by_type(TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::TypeMetas;

    #[test]
    fn empty_info_set() {
        let info = TypeMetas::new();
    
        assert_eq!(info.len(), 0);
    }
    
    #[test]
    fn add_info() {
        let mut info = TypeMetas::new();
    
        assert_eq!(info.len(), 0);
    
        let id = info.add_type::<TestA>();
        assert_eq!(id.index(), 0);
        assert_eq!(info.len(), 1);
        assert!(info.get_id_by_type(TypeId::of::<TestA>()).is_none());
        assert!(info.get_id::<TestA>().is_none());
    
        assert_eq!(info.get_name(id).expect("id"), "essay_ecs::type_info::tests::TestA");
        assert_eq!(info.get_id_by_type(TypeId::of::<TestA>()).expect("id"), id);
        assert!(info.get_id_by_type(TypeId::of::<TestB>()).is_none());
        assert_eq!(info.get_id::<TestA>().expect("id"), id);
    
        let id_b = info.add_type::<TestB>();
        assert_eq!(id_b.index(), 1);
        assert_eq!(info.len(), 2);
        assert_eq!(info.get_id_by_type(TypeId::of::<TestB>()).expect("id"), id_b);
        assert_eq!(info.get_id::<TestB>().expect("id"), id_b);
    
        let id_a2 = info.add_type::<TestA>();
        assert_eq!(id.index(), 0);
        assert_eq!(info.len(), 2);
        assert_eq!(id, id_a2);
    }
    
    struct TestA;
    
    struct TestB;
}