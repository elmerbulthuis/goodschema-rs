use std::collections::HashSet;

pub mod intermediate_a;

pub trait Selectors {
    fn select_types(&self, node_id: &str) -> HashSet<TypeEnum>;
    fn select_string_options(&self, node_id: &str) -> HashSet<&str>;
}

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
    Record,
}
