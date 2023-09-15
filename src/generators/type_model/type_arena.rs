use std::collections::HashMap;

use super::{TypeEnum, TypeKey};

#[derive(Debug, Default)]
pub struct TypeArena {
    type_enums: HashMap<TypeKey, TypeEnum>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_type_enum(&self, type_key: TypeKey) -> Option<&TypeEnum> {
        self.type_enums.get(&type_key)
    }

    pub fn set_type_enum(&mut self, type_key: TypeKey, type_enum: TypeEnum) {
        assert!(self.type_enums.insert(type_key, type_enum).is_none());
    }
}
