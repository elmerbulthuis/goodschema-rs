use super::{ArrayType, IntersectionType, ObjectType, RecordType, TupleType, TypeKey, UnionType};

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
    Record(RecordType),
    Union(UnionType),
    Intersection(IntersectionType),
    Alias(TypeKey),
}
