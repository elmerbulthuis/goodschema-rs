use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct TypeModel {
    pub id: Option<String>,
    pub super_type_key: Option<TypeKey>,
    pub r#type: TypeEnum,
    pub validators: Vec<ValidatorEnum>,
    pub property: Option<TypeKey>,
    pub properties: HashMap<String, TypeKey>,
    pub item: Option<TypeKey>,
    pub items: Vec<TypeKey>,
}

impl TypeArena {
    pub fn get_model(&self, type_key: &TypeKey) -> Option<&TypeModel> {
        self.models.get(type_key)
    }
}
