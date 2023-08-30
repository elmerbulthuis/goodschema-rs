use super::TypeKey;

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
    OneOf(Vec<TypeKey>),
    AnyOf(Vec<TypeKey>),
    AllOf(Vec<TypeKey>),
}
