use crate::schemas::intermediate_a::Schema;
use serde_json::Value;
use url::Url;

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

    fn get_intermediate_node_pairs(
        &self,
    ) -> Box<dyn Iterator<Item = (&str, &crate::schemas::intermediate_a::SchemaNode)> + '_> {
        Box::new(
            self.document_node
                .nodes
                .iter()
                .map(|(k, v)| (k.as_str(), v)),
        )
    }
}
