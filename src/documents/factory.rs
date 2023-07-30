use super::Document;
use serde_json::Value;
use url::Url;

pub struct Initializer<'l> {
    pub retrieval_url: &'l Url,
    pub given_url: &'l Url,
    pub antecedent_url: Option<&'l Url>,
    pub document_node: Value,
}

pub type Factory = Box<dyn Fn(Initializer) -> Box<dyn Document>>;
