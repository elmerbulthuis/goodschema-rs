use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct TypeModel {
    pub r#type: TypeEnum,
    pub validators: Vec<ValidatorEnum>,
    pub property: Option<(String, String)>,
    pub properties: HashMap<String, String>,
    pub item: Option<String>,
    pub items: Vec<String>,
}

impl TypeArena {
    pub fn get_model(&self, type_name: &str) -> Option<&TypeModel> {
        self.models.get(type_name)
    }
}
