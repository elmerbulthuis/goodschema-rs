use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapType {
    key: TypeKey,
    value: TypeKey,
}
impl MapType {
    pub fn new(key: TypeKey, value: TypeKey) -> Self {
        Self { key, value }
    }

    pub fn get_key(&self) -> &TypeKey {
        &self.key
    }

    pub fn get_value(&self) -> &TypeKey {
        &self.value
    }
}
