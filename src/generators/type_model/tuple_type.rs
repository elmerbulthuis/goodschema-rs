use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType {
    items: Vec<TypeKey>,
}
impl TupleType {
    pub fn new(items: Vec<TypeKey>) -> Self {
        Self { items }
    }

    pub fn get_item(&self) -> &Vec<TypeKey> {
        &self.items
    }
}
