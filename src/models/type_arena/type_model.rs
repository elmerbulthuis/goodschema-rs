use std::collections::HashMap;

use super::*;

#[derive(Debug, Default)]
pub struct TypeModel {
    pub super_type_key: Option<TypeKey>,
    pub types: Vec<TypeEnum>,
    pub properties: HashMap<String, TypeKey>,
}
