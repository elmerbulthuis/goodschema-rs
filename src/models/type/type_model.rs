use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct TypeModel {
    pub name: Option<String>,
    pub super_type_key: Option<TypeKey>,
    pub r#type: TypeEnum,
    pub validators: Vec<ValidatorEnum>,
    pub property: Option<TypeKey>,
    pub properties: HashMap<String, TypeKey>,
    pub item: Option<TypeKey>,
    pub items: Vec<TypeKey>,
}
