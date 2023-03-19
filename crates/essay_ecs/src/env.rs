use crate::type_info::{TypeInfo, TypeInfos};

pub struct Env {
    components: TypeInfos,
    entities: Vec<String>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            components: TypeInfos::new(),
            entities: Vec::new(),
        }
    }
}