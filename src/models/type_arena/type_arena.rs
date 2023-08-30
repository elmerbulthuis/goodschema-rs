use super::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeArena {
    type_enums: HashMap<TypeKey, TypeEnum>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_type_enum(&self, type_key: &TypeKey) -> Option<&TypeEnum> {
        self.type_enums.get(type_key)
    }
}
