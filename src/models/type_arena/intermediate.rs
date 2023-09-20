use crate::schemas;

use super::*;
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

        for (node_id, _node) in intermediate_document.nodes.iter() {
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

                match type_node {
                    schemas::intermediate_a::TypeUnion::NeverType(_type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Never".to_owned()),
                            r#type: TypeEnum::Never,
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
                    schemas::intermediate_a::TypeUnion::AnyType(_type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Any".to_owned()),
                            r#type: TypeEnum::Any,
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
                    schemas::intermediate_a::TypeUnion::NullType(_type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Null".to_owned()),
                            r#type: TypeEnum::Null,
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
                    schemas::intermediate_a::TypeUnion::BooleanType(_type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Boolean".to_owned()),
                            r#type: TypeEnum::Boolean,
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

                        validators.push(ValidatorEnum::Boolean(BooleanValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::DefsNumberType(type_node) => {
                        let number_type = if let Some(number_type) = &type_node.number_type {
                            number_type.as_ref()
                        } else {
                            "float"
                        };
                        if number_type == "integer" {
                            let simple_type_model = TypeModel {
                                name: Some("Integer".to_owned()),
                                r#type: TypeEnum::Integer,
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

                            validators.push(ValidatorEnum::Integer(IntegerValidator {}));
                        } else {
                            let simple_type_model = TypeModel {
                                name: Some("Number".to_owned()),
                                r#type: TypeEnum::Number,
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

                            validators.push(ValidatorEnum::Number(NumberValidator {}));
                        }
                    }
                    schemas::intermediate_a::TypeUnion::StringType(_type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("String".to_owned()),
                            r#type: TypeEnum::String,
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

                        validators.push(ValidatorEnum::String(StringValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Tuple".to_owned()),
                            r#type: TypeEnum::Tuple,
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
                    }
                    schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Array".to_owned()),
                            r#type: TypeEnum::Array,
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

                        item = type_node
                            .item_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|type_name| *type_keys.get(type_name).unwrap());

                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Object".to_owned()),
                            r#type: TypeEnum::Object,
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
                    }
                    schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                        let simple_type_model = TypeModel {
                            name: Some("Map".to_owned()),
                            r#type: TypeEnum::Map,
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

                        property = type_node
                            .property_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|type_name| *type_keys.get(type_name).unwrap())
                            .map(|type_key| (TypeKey::new(), type_key));

                        validators.push(ValidatorEnum::Map(MapValidator {}));
                    }
                };
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

            let mut node_type_keys = Vec::new();

            if !simple_type_keys.is_empty() {
                let simple_type_key = TypeKey::new();
                node_type_keys.push(simple_type_key);
                let simple_type_type = TypeEnum::OneOf(simple_type_keys);
                let sub_type_model = TypeModel {
                    name: Some("Type".to_string()),
                    r#type: simple_type_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena
                    .models
                    .insert(simple_type_key, sub_type_model)
                    .is_none());
            }

            if !one_of_type_keys.is_empty() {
                let one_of_type_key = TypeKey::new();
                one_of_type_keys.push(one_of_type_key);
                let one_of_type_type = TypeEnum::OneOf(one_of_type_keys);
                let one_of_type_model = TypeModel {
                    name: Some("OneOf".to_string()),
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
                    name: Some("AnyOf".to_string()),
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

            if let Some(super_type_key) = super_type_key {
                node_type_keys.push(super_type_key);
            }

            let node_type_type = if node_type_keys.is_empty() {
                TypeEnum::Unknown
            } else {
                TypeEnum::AllOf(node_type_keys)
            };
            let node_type_model = TypeModel {
                name: Some(node_type_name.to_string()),
                r#type: node_type_type,
                validators,
                property,
                properties,
                item,
                items,
            };
            assert!(arena
                .models
                .insert(node_type_key, node_type_model)
                .is_none());
        }

        arena
    }
}
