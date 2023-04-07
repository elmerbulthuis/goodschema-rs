use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use std::collections::HashMap;
use url::Url;

#[derive(Default)]
pub struct LoaderImpl {
    root_node_map: HashMap<Url, serde_json::Value>,
}

impl LoaderImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Loader for LoaderImpl {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool {
        if let Some(schema) = node.select_schema() {
            return schema == META_SCHEMA_ID;
        }

        false
    }

    fn load_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
    ) -> Result<(), &'static str> {
        if self.root_node_map.insert(node_url.clone(), node).is_some() {
            return Err("root_node already present");
        }

        Ok(())
    }

    fn get_sub_node_urls(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        let node_url = self.get_root_node_url(node, node_url)?;
        let mut result = Vec::new();

        for node_ref in node
            .select_all_sub_nodes("")
            .iter()
            .filter_map(|(_sub_pointer, sub_node)| sub_node.select_ref())
        {
            let node_ref_url = node_url
                .join(node_ref)
                .map_err(|_error| "could not build node_ref_url")?;
            let mut retrieval_ref_url = retrieval_url
                .join(node_ref)
                .map_err(|_error| "could not build retrieval_ref_url")?;
            retrieval_ref_url.set_fragment(None);

            result.push((node_ref_url, retrieval_ref_url));
        }

        Ok(result)
    }

    fn get_root_node_url(
        &self,
        node: &serde_json::Value,
        default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        let node_url;

        let node_id = node.select_id();
        if let Some(node_id) = node_id {
            node_url = node_id.parse().map_err(|_error| "could not parse id")?;
        } else {
            node_url = default_node_url.clone();
        }

        Ok(node_url)
    }
}
