use super::TypeKey;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectType {
    properties: HashMap<String, TypeKey>,
}
impl ObjectType {
    pub fn new(properties: HashMap<String, TypeKey>) -> Self {
        Self { properties }
    }

    pub fn get_properties(&self) -> &HashMap<String, TypeKey> {
        &self.properties
    }
}
