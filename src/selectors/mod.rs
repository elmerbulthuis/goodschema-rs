use std::collections::{HashMap, HashSet};

pub mod intermediate_a;

pub trait Selectors {
    fn select_types(&self, node_id: &str) -> HashSet<TypeEnum>;
    fn select_string_options(&self, node_id: &str) -> HashSet<&str>;
    fn select_tuple_item_type_node_ids(&self, node_id: &str) -> Vec<&str>;
    fn select_array_item_type_node_id(&self, node_id: &str) -> Option<&str>;
    fn select_object_property_type_node_ids(&self, node_id: &str) -> HashMap<&str, &str>;
    fn select_record_property_type_node_id(&self, node_id: &str) -> Option<&str>;
    fn select_object_required_properties(&self, node_id: &str) -> HashSet<&str>;
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
