use super::*;
use crate::{schemas, selectors::document::DocumentSelectors, selectors::node::NodeSelectors};
use std::collections::HashMap;

/**
 * We store all type models in the type arena so we can easily generate types from them.
 *
 */
impl TypeArena {
    pub fn new_from_intermediate_document(
        intermediate_document: &schemas::intermediate_a::SchemaJson,
        name_map: &HashMap<String, String>,
    ) -> Self {
        let mut arena = Self::new();
        let mut type_keys = HashMap::new();

        for (node_id, node) in intermediate_document.nodes.iter() {
            let type_key = TypeKey::new();
            let type_name = name_map.get(node_id).unwrap();
            assert!(type_keys.insert(type_name.clone(), type_key).is_none());
        }

        for (node_id, node) in intermediate_document.nodes.iter() {
            let node_type_name = name_map.get(node_id).unwrap();
            let node_type_key = *type_keys.get(node_type_name).unwrap();

            let super_type_name = node
                .super_node_id
                .as_ref()
                .map(|super_node_id| super_node_id.as_ref())
                .map(|super_node_id| name_map.get(super_node_id).unwrap());
            let super_type_key =
                super_type_name.map(|super_type_name| *type_keys.get(super_type_name).unwrap());

            let mut simple_type_keys = Vec::new();
            let mut one_of_type_keys = Vec::new();
            let mut any_of_type_keys = Vec::new();
            let mut all_of_type_keys = Vec::new();

            let mut validators = Vec::new();
            let mut property = None;
            let mut properties = HashMap::new();
            let mut item = None;
            let mut items = Vec::new();

            for type_node in node.types.iter() {
                let simple_type_key = TypeKey::new();
                simple_type_keys.push(simple_type_key);

                let simple_type_type = match type_node {
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
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|type_name| *type_keys.get(type_name).unwrap())
                            .collect();
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                        TypeEnum::Tuple
                    }
                    schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
                        item = type_node
                            .item_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|type_name| *type_keys.get(type_name).unwrap());
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                        TypeEnum::Array
                    }
                    schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
                        properties = type_node
                            .property_type_node_ids
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|(key, node_id)| (key, node_id.as_ref()))
                            .map(|(key, node_id)| (key, name_map.get(node_id).unwrap()))
                            .map(|(key, type_name)| {
                                (key.clone(), *type_keys.get(type_name).unwrap())
                            })
                            .collect();
                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Object
                    }
                    schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                        property = type_node
                            .property_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|type_name| *type_keys.get(type_name).unwrap())
                            .map(|type_key| (TypeKey::new(), type_key));

                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Map
                    }
                };
                let simple_type_model = TypeModel {
                    name: Some(format!("{}", simple_type_type)),
                    r#type: simple_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena
                    .models
                    .insert(simple_type_key, simple_type_model)
                    .is_none());
            }

            for compound_node in node.compounds.iter() {
                match compound_node {
                    schemas::intermediate_a::CompoundUnion::OneOfCompound(compound_node) => {
                        one_of_type_keys.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .map(|type_name| *type_keys.get(type_name).unwrap())
                                .collect(),
                        );
                    }
                    schemas::intermediate_a::CompoundUnion::AnyOfCompound(compound_node) => {
                        any_of_type_keys.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .map(|type_name| *type_keys.get(type_name).unwrap())
                                .collect(),
                        );
                    }
                    schemas::intermediate_a::CompoundUnion::AllOfCompound(compound_node) => {
                        all_of_type_keys.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .map(|type_name| *type_keys.get(type_name).unwrap())
                                .collect(),
                        );
                    }
                }
            }

            let mut type_keys = Vec::new();

            if !simple_type_keys.is_empty() {
                let sub_type_key = TypeKey::new();
                simple_type_keys.push(sub_type_key);
                let sub_type_type = TypeEnum::OneOf(simple_type_keys);
                let sub_type_model = TypeModel {
                    name: None,
                    r#type: sub_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_key, sub_type_model).is_none());
            }

            if !one_of_type_keys.is_empty() {
                let one_of_type_key = TypeKey::new();
                one_of_type_keys.push(one_of_type_key);
                let one_of_type_type = TypeEnum::OneOf(one_of_type_keys);
                let one_of_type_model = TypeModel {
                    name: Some(format!("{}{}", node_type_name, one_of_type_type)),
                    r#type: one_of_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena
                    .models
                    .insert(one_of_type_key, one_of_type_model)
                    .is_none());
            }

            if !any_of_type_keys.is_empty() {
                let any_of_type_key = TypeKey::new();
                all_of_type_keys.push(any_of_type_key);
                let any_of_type_type = TypeEnum::AnyOf(any_of_type_keys);
                let any_of_type_model = TypeModel {
                    name: Some(format!("{}{}", node_type_name, any_of_type_type)),
                    r#type: any_of_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena
                    .models
                    .insert(any_of_type_key, any_of_type_model)
                    .is_none());
            }

            let type_type = TypeEnum::AllOf(type_keys);
            let type_model = TypeModel {
                name: Some(node_type_name.clone()),
                r#type: type_type,
                validators,
                property,
                properties,
                item,
                items,
            };
            assert!(arena.models.insert(node_type_key, type_model).is_none());
        }

        arena
    }
}
