use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType {
    item: TypeKey,
}
impl ArrayType {
    pub fn new(item: TypeKey) -> Self {
        Self { item }
    }

    pub fn get_item(&self) -> &TypeKey {
        &self.item
    }
}
impl From<TypeKey> for ArrayType {
    fn from(value: TypeKey) -> Self {
        Self::new(value)
    }
}
