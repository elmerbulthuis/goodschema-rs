use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionType {
    types: Vec<TypeKey>,
}
impl UnionType {
    pub fn new(types: Vec<TypeKey>) -> Self {
        assert!(types.len() >= 2);

        Self { types }
    }

    pub fn get_types(&self) -> &Vec<TypeKey> {
        &self.types
    }
}
