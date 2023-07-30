use serde_json::Value;
use url::Url;

pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

pub struct Document {
    document_node: Value,
    document_url: Url,
}

impl Document {
    pub fn new(given_url: Url, document_node: Value) -> Self {
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

    fn get_node_urls(&self) -> Vec<&Url> {
        vec![]
    }
}
