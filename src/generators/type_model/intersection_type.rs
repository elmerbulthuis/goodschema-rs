use super::TypeKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntersectionType {
    types: Vec<TypeKey>,
}
impl IntersectionType {
    pub fn new(types: Vec<TypeKey>) -> Self {
        assert!(types.len() >= 2);

        Self { types }
    }

    pub fn get_types(&self) -> &Vec<TypeKey> {
        &self.types
    }
}
impl<T> From<T> for IntersectionType
where
    T: IntoIterator<Item = TypeKey>,
{
    fn from(value: T) -> Self {
        Self::new(value.into_iter().collect())
    }
}
