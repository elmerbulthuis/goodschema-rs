use crate::schemas;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeArena {
    type_references: HashMap<String, TypeKey>,
    type_models: HashMap<TypeKey, TypeModel>,
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
            assert!(arena
                .type_references
                .insert(node_id.clone(), type_key)
                .is_none());
        }

        for (node_id, node) in intermediate_document.nodes.iter() {
            let type_key = *arena.type_references.get(node_id).unwrap();
            let type_model = Default::default();
            assert!(arena.type_models.insert(type_key, type_model).is_none());
        }

        arena
    }
}
