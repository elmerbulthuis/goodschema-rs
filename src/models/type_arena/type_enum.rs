use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeEnum {
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
    OneOf(Vec<String>),
    AnyOf(Vec<String>),
    AllOf(Vec<String>),
}

impl Display for TypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeEnum::Never => write!(f, "Never"),
            TypeEnum::Any => write!(f, "Any"),
            TypeEnum::Null => write!(f, "Null"),
            TypeEnum::Boolean => write!(f, "Boolean"),
            TypeEnum::Integer => write!(f, "Integer"),
            TypeEnum::Number => write!(f, "Number"),
            TypeEnum::String => write!(f, "String"),
            TypeEnum::Tuple => write!(f, "Tuple"),
            TypeEnum::Array => write!(f, "Array"),
            TypeEnum::Object => write!(f, "Object"),
            TypeEnum::Map => write!(f, "Map"),
            TypeEnum::OneOf(_) => write!(f, "OneOf"),
            TypeEnum::AnyOf(_) => write!(f, "AnyOf"),
            TypeEnum::AllOf(_) => write!(f, "AllOf"),
        }
    }
}
