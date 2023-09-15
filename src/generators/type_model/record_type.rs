use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    key: TypeKey,
    value: TypeKey,
}
impl RecordType {
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
