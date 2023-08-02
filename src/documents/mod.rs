use url::Url;

use crate::schemas;

pub mod intermediate_a;

pub mod context;
pub mod factory;

pub trait Document {
    fn document_url(&self) -> &Url;
    fn get_node_urls(&self) -> Vec<Url>;
    fn get_intermediate_node_pairs(
        &self,
    ) -> Box<dyn Iterator<Item = (&str, &schemas::intermediate_a::SchemaNode)> + '_>;
}
