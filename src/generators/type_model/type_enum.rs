use super::{ArrayType, IntersectionType, MapType, ObjectType, TupleType, TypeKey, UnionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeEnum {
    Never,
    Any,
    Null,
    Boolean,
    Number,
    String,
    Tuple(TupleType),
    Array(ArrayType),
    Object(ObjectType),
    Map(MapType),
    Union(UnionType),
    Intersection(IntersectionType),
    Alias(TypeKey),
}
