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
impl<T> From<T> for UnionType
where
    T: IntoIterator<Item = TypeKey>,
{
    fn from(value: T) -> Self {
        Self::new(value.into_iter().collect())
    }
}
