use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeEnum {
    Unknown,
    Never,
    Any,
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Tuple,
    Array,
    Object,
    Map,
    OneOf(Vec<TypeKey>),
    AnyOf(Vec<TypeKey>),
    AllOf(Vec<TypeKey>),
    Alias(TypeKey),
}

impl TypeEnum {
    pub fn is_compound(&self) -> bool {
        matches!(
            self,
            TypeEnum::OneOf(_) | TypeEnum::AnyOf(_) | TypeEnum::AllOf(_) | TypeEnum::Alias(_)
        )
    }
}

impl Default for TypeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}
