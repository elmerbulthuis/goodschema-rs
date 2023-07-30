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
    nodes: HashMap<String, SchemaNode>,
}

pub struct Document {
    document_node: Schema,
    document_url: Url,
}

impl Document {
    pub fn new(given_url: Url, document_node: Value) -> Self {
        let document_node = serde_json::from_value(document_node).unwrap();
        Self {
            document_url: given_url,
            document_node,
        }
    }
}

impl super::Document for Document {
    fn document_url(&self) -> &Url {
        &self.document_url
    }

    fn get_node_urls(&self) -> Vec<Url> {
        self.document_node
            .nodes
            .keys()
            .map(|key| key.parse().unwrap())
            .collect()
    }
}
