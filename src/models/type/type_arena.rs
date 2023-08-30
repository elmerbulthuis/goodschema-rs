use crate::schemas;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeArena {
    names: HashMap<String, TypeKey>,
    models: HashMap<TypeKey, TypeModel>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from_intermediate_document(
        intermediate_document: &schemas::intermediate_a::SchemaJson,
    ) -> Self {
        let mut arena = Self::new();

        for node_id in intermediate_document.nodes.keys() {
            let type_key = TypeKey::new();
            assert!(arena.names.insert(node_id.clone(), type_key).is_none());
        }

        for (node_id, node) in intermediate_document.nodes.iter() {
            let type_key = *arena.names.get(node_id).unwrap();

            let type_model = TypeModel {
                super_type_key: None,
                r#type: TypeEnum::Any,
                properties: HashMap::new(),
                validators: Vec::new(),
            };
            assert!(arena.models.insert(type_key, type_model).is_none());
        }

        arena
    }
}
