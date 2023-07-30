use url::Url;

pub mod intermediate_a;

pub mod context;
pub mod factory;

pub trait Document {
    fn document_url(&self) -> &Url;
    fn get_node_urls(&self) -> Vec<Url>;
}
