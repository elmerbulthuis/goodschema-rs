use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct TypeModel {
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub r#type: TypeEnum,
    pub validators: Vec<ValidatorEnum>,
    pub property: Option<(TypeKey, TypeKey)>,
    pub properties: HashMap<String, TypeKey>,
    pub item: Option<TypeKey>,
    pub items: Vec<TypeKey>,
}

impl TypeArena {
    pub fn get_model(&self, type_key: &TypeKey) -> Option<&TypeModel> {
        self.models.get(type_key)
    }
}
