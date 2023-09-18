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
