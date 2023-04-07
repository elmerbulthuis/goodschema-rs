use url::Url;

pub trait Loader {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool;

    fn load_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
    ) -> Result<(), &'static str>;

    fn get_sub_node_urls(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str>;

    fn get_root_node_url(
        &self,
        node: &serde_json::Value,
        default_node_url: &Url,
    ) -> Result<Url, &'static str>;
}
