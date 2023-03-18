use std::{
    any::{TypeId, type_name}, 
    borrow::Cow, 
    collections::HashMap
};

pub trait Component: 'static {
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct ComponentId(usize);

impl ComponentId {
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

pub struct ComponentInfo {
    id: ComponentId,
    name: Cow<'static, str>,
    type_id: Option<TypeId>,
}

impl ComponentInfo {
    fn new<T: Component>(id: usize) -> Self {
        Self {
            id: ComponentId(id),
            name: Cow::Borrowed(type_name::<T>()),
            type_id: Some(TypeId::of::<T>()),
        }
    }

    pub fn id(&self) -> ComponentId {
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

impl std::fmt::Debug for ComponentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentInfo")
        .field("id", &self.id)
        .field("name", &self.name)
        .field("type_id", &self.type_id)
        .finish()
    }
}

#[derive(Debug, Default)]
pub struct ComponentInfos {
    infos: Vec<ComponentInfo>,
    type_to_id: HashMap<TypeId, usize>,
}

impl ComponentInfos {
    pub fn new() -> Self {
        Self {
            .. Default::default()
        }
    }
    pub fn add_info<T: Component>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();

        let id = self.type_to_id.entry(type_id)
            .or_insert_with(|| {
            let id = self.infos.len();

            self.infos.push(ComponentInfo::new::<T>(id));

            id
        });

        ComponentId(*id)
    }

    pub fn len(&self) -> usize {
        self.infos.len()
    }

    pub fn get_info(&self, id: ComponentId) -> Option<&ComponentInfo> {
        self.infos.get(id.0)
    }

    pub fn get_name(&self, id: ComponentId) -> Option<&str> {
        self.get_info(id).map(|info| info.name())
    }

    pub fn get_id_by_type(&self, type_id: TypeId) -> Option<ComponentId> {
        self.type_to_id.get(&type_id).map(|index| ComponentId(*index))
    }

    pub fn get_id<T: Component>(&self) -> Option<ComponentId> {
        self.get_id_by_type(TypeId::of::<T>())
    }
}
