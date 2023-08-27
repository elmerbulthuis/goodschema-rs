use super::{ArrayType, ObjectType, RecordType, TupleType, UnionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeEnum {
    Unknown,
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
}
