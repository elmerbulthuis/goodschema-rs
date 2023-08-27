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
impl<T> From<T> for ObjectType
where
    T: IntoIterator<Item = (String, TypeKey)>,
{
    fn from(value: T) -> Self {
        Self::new(value.into_iter().collect())
    }
}
