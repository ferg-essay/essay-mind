use std::{
    any::{TypeId, type_name}, 
    borrow::Cow, 
    collections::HashMap
};

pub trait Component {
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct TypeIndex(usize);

impl TypeIndex {
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

pub struct TypeInfo {
    id: TypeIndex,
    name: Cow<'static, str>,
    type_id: Option<TypeId>,
}

impl TypeInfo {
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

impl std::fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeInfo")
        .field("id", &self.id)
        .field("name", &self.name)
        .field("type_id", &self.type_id)
        .finish()
    }
}

#[derive(Debug, Default)]
pub struct TypeInfos {
    infos: Vec<TypeInfo>,
    type_to_id: HashMap<TypeId, usize>,
}

impl TypeInfos {
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

            self.infos.push(TypeInfo::new::<T>(id));

            id
        });

        TypeIndex(*id)
    }

    pub fn len(&self) -> usize {
        self.infos.len()
    }

    pub fn get_info(&self, id: TypeIndex) -> Option<&TypeInfo> {
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
