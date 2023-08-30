use crate::schemas;

use super::*;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
pub struct TypeArena {
    models: HashMap<TypeKey, TypeModel>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_model(&self, type_key: &TypeKey) -> Option<&TypeModel> {
        self.models.get(type_key)
    }

    pub fn new_from_intermediate_document(
        intermediate_document: &schemas::intermediate_a::SchemaJson,
    ) -> Self {
        let mut arena = Self::new();
        let mut names = HashMap::new();

        for node_id in intermediate_document.nodes.keys() {
            let type_key = TypeKey::new();
            assert!(names.insert(node_id.clone(), type_key).is_none());
        }

        for (node_id, node) in intermediate_document.nodes.iter() {
            let super_type_key = node
                .super_node_id
                .as_ref()
                .map(|super_node_id| *names.get(super_node_id.as_ref()).unwrap());
            let type_key = *names.get(node_id).unwrap();
            let mut sub_type_keys = Vec::new();

            let mut validators = Vec::new();
            let mut property = None;
            let mut properties = HashMap::new();
            let mut item = None;
            let mut items = Vec::new();

            for type_node in node.types.iter() {
                let sub_type_key = TypeKey::new();
                sub_type_keys.push(sub_type_key);

                let sub_type_type = match type_node {
                    schemas::intermediate_a::TypeUnion::NeverType(_type_node) => TypeEnum::Never,
                    schemas::intermediate_a::TypeUnion::AnyType(_type_node) => TypeEnum::Any,
                    schemas::intermediate_a::TypeUnion::NullType(_type_node) => TypeEnum::Null,
                    schemas::intermediate_a::TypeUnion::BooleanType(_type_node) => {
                        validators.push(ValidatorEnum::Boolean(BooleanValidator {}));
                        TypeEnum::Boolean
                    }
                    schemas::intermediate_a::TypeUnion::DefsNumberType(type_node) => {
                        let number_type = if let Some(number_type) = &type_node.number_type {
                            number_type.as_ref()
                        } else {
                            "float"
                        };
                        if number_type == "integer" {
                            validators.push(ValidatorEnum::Integer(IntegerValidator {}));
                            TypeEnum::Integer
                        } else {
                            validators.push(ValidatorEnum::Number(NumberValidator {}));
                            TypeEnum::Number
                        }
                    }
                    schemas::intermediate_a::TypeUnion::StringType(_type_node) => {
                        validators.push(ValidatorEnum::String(StringValidator {}));
                        TypeEnum::String
                    }
                    schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
                        items = type_node
                            .item_type_node_ids
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|node_id| *names.get(node_id.as_ref()).unwrap())
                            .collect();
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                        TypeEnum::Tuple
                    }
                    schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
                        item = type_node
                            .item_type_node_id
                            .as_ref()
                            .map(|node_id| *names.get(node_id.as_ref()).unwrap());
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                        TypeEnum::Array
                    }
                    schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
                        properties = type_node
                            .property_type_node_ids
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|(name, node_id)| {
                                (name.clone(), *names.get(node_id.as_ref()).unwrap())
                            })
                            .collect();
                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Object
                    }
                    schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                        property = type_node
                            .property_type_node_id
                            .as_ref()
                            .map(|node_id| *names.get(node_id.as_ref()).unwrap());

                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Map
                    }
                };
                let sub_type_model = TypeModel {
                    name: None,
                    super_type_key: Some(type_key),
                    r#type: sub_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_key, sub_type_model).is_none());
            }

            let type_model = TypeModel {
                name: Some(node_id.clone()),
                super_type_key,
                r#type: TypeEnum::Union(sub_type_keys),
                validators,
                property,
                properties,
                item,
                items,
            };
            assert!(arena.models.insert(type_key, type_model).is_none());
        }

        arena
    }
}
