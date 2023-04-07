use super::{loader::Loader, meta::MetaSchemaId};
use crate::schemas;
use std::{collections::HashMap, fs::File};
use url::Url;

#[derive(Default)]
pub struct Manager<'a> {
    loaders: HashMap<MetaSchemaId, Box<dyn Loader + 'a>>,
    retrieval_root_node_map: HashMap<Url, Url>,
    root_node_retrieval_map: HashMap<Url, Url>,
    root_node_meta_schema_id_map: HashMap<Url, MetaSchemaId>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Self {
        let mut manager = Self::default();

        manager.add_loader(
            MetaSchemaId::Draft202012,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new()),
        );
        manager.add_loader(
            MetaSchemaId::Draft201909,
            Box::new(schemas::draft_2019_09::loader::LoaderImpl::new()),
        );
        manager.add_loader(
            MetaSchemaId::Draft07,
            Box::new(schemas::draft_07::loader::LoaderImpl::new()),
        );
        manager.add_loader(
            MetaSchemaId::Draft06,
            Box::new(schemas::draft_06::loader::LoaderImpl::new()),
        );
        manager.add_loader(
            MetaSchemaId::Draft04,
            Box::new(schemas::draft_04::loader::LoaderImpl::new()),
        );

        manager
    }

    pub fn load_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<(), &'static str> {
        let meta_schema_id = self.discover_meta_schema_id(&node, default_meta_schema_id);

        let loader = self.loaders.get_mut(&meta_schema_id).unwrap();

        let node_url = loader.get_root_node_url(&node, node_url)?;

        loader.load_root_node(node, &node_url)?;

        Ok(())
    }

    pub fn load_from_url(
        &mut self,
        node_url: &Url,
        retrieval_url: &Url,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<(), &'static str> {
        if self.retrieval_root_node_map.contains_key(retrieval_url) {
            return Ok(());
        }

        let root_node = Self::fetch_json_from_url(retrieval_url)?;

        let meta_schema_id = self.discover_meta_schema_id(&root_node, default_meta_schema_id);

        let loader = self.loaders.get_mut(&meta_schema_id).unwrap();

        let node_url = loader.get_root_node_url(&root_node, node_url)?;

        self.retrieval_root_node_map
            .insert(retrieval_url.clone(), node_url.clone());
        self.root_node_retrieval_map
            .insert(node_url.clone(), retrieval_url.clone());
        self.root_node_meta_schema_id_map
            .insert(node_url.clone(), meta_schema_id);

        for (sub_node_url, sub_retrieval_url) in
            loader.get_sub_node_urls(&root_node, &node_url, retrieval_url)?
        {
            self.load_from_url(&sub_node_url, &sub_retrieval_url, meta_schema_id)?;
        }

        self.load_root_node(root_node, &node_url, default_meta_schema_id)?;

        Ok(())
    }

    pub fn add_loader(&mut self, meta_schema_id: MetaSchemaId, loader: Box<dyn Loader + 'a>) {
        self.loaders.insert(meta_schema_id, loader);
    }

    fn discover_meta_schema_id(
        &self,
        node: &serde_json::Value,
        default_meta_schema_id: MetaSchemaId,
    ) -> MetaSchemaId {
        for (schema_id, loader) in self.loaders.iter() {
            if loader.is_schema_root_node(node) {
                return *schema_id;
            }
        }

        default_meta_schema_id
    }

    fn fetch_json_from_url(url: &Url) -> Result<serde_json::Value, &'static str> {
        match url.scheme() {
            "file" => {
                let path = url.path();
                let reader = File::open(path).or(Err("error reading file"))?;

                let value: serde_json::Value =
                    serde_json::from_reader(reader).or(Err("error deserializing file content"))?;

                Ok(value)
            }
            _ => Err("not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_manager() {
        let _manager = Manager::new();
    }
}
