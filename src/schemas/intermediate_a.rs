use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaNode {
    super_node_id: Option<String>,
    deprecated: bool,
    title: String,
    description: String,
    examples: Vec<Value>,
    types: Vec<Value>,
    compounds: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub nodes: HashMap<String, SchemaNode>,
}
