use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaNode {
    super_node_id: Option<String>,
    deprecated: bool,
    title: String,
    description: String,
    examples: Vec<Value>,
    types: Vec<TypeEnum>,
    compounds: Vec<CompoundEnum>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub nodes: HashMap<String, SchemaNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeEnum {
    //
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompoundEnum {
    //
}
