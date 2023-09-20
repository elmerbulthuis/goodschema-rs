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
        match self {
            TypeEnum::OneOf(_) => true,
            TypeEnum::AnyOf(_) => true,
            TypeEnum::AllOf(_) => true,
            TypeEnum::Alias(_) => true,
            _ => false,
        }
    }
}

impl Default for TypeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}
