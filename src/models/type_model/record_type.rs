use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    property: TypeKey,
}
impl RecordType {
    pub fn new(property: TypeKey) -> Self {
        Self { property }
    }

    pub fn get_property(&self) -> TypeKey {
        self.property
    }
}
impl From<TypeKey> for RecordType {
    fn from(value: TypeKey) -> Self {
        Self::new(value)
    }
}
