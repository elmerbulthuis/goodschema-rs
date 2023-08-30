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
    Union(Vec<TypeKey>),
    Intersection(Vec<TypeKey>),
}
