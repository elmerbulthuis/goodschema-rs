use super::*;
use crate::{schemas, utils::namer::Namer};
use std::collections::HashMap;

impl From<&schemas::intermediate_a::SchemaJson> for TypeArena {
    /**
     * creates a type arena from the intermediate model. Basically this creates the types from that
     * model that can later be used to generate the actual rust types.
     */
    fn from(intermediate_document: &schemas::intermediate_a::SchemaJson) -> Self {
        // first, generate names for every node
        let mut namer = Namer::new("");
        for (node_id, _node) in intermediate_document.nodes.iter() {
            namer.register_path(node_id.to_string(), node_id);
        }
        let name_map = namer.get_names();

        // and now lets fill the arena!
        let mut arena = Self::new();
        for (node_id, node) in intermediate_document.nodes.iter() {
            // get the name for this node
            let type_name = name_map.get(node_id).unwrap();

            // we'll fill these later
            let mut simple_type_names = Vec::new();
            let mut one_of_type_names = Vec::new();
            let mut any_of_type_names = Vec::new();
            let mut all_of_type_names = Vec::new();

            // we'll fill these later
            let mut validators = Vec::new();
            let mut property = None;
            let mut properties = HashMap::new();
            let mut item = None;
            let mut items = Vec::new();

            // first we fill all simple types
            for type_node in node.types.iter() {
                match type_node {
                    schemas::intermediate_a::TypeUnion::NeverType(_type_node) => {
                        let simple_type_type = TypeEnum::Never;
                        let simple_type_name = format!("{}{}", type_name, "Never");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                    }
                    schemas::intermediate_a::TypeUnion::AnyType(_type_node) => {
                        let simple_type_type = TypeEnum::Any;
                        let simple_type_name = format!("{}{}", type_name, "Any");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                    }
                    schemas::intermediate_a::TypeUnion::NullType(_type_node) => {
                        let simple_type_type = TypeEnum::Null;
                        let simple_type_name = format!("{}{}", type_name, "Null");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                    }
                    schemas::intermediate_a::TypeUnion::BooleanType(_type_node) => {
                        let simple_type_type = TypeEnum::Boolean;
                        let simple_type_name = format!("{}{}", type_name, "Boolean");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
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
                            let simple_type_type = TypeEnum::Integer;
                            let simple_type_name = format!("{}{}", type_name, "Integer");
                            simple_type_names.push(simple_type_name.clone());
                            let simple_type_model = TypeModel {
                                r#type: simple_type_type,
                                validators: Default::default(),
                                property: Default::default(),
                                properties: Default::default(),
                                item: Default::default(),
                                items: Default::default(),
                            };
                            assert!(arena
                                .models
                                .insert(simple_type_name, simple_type_model)
                                .is_none());
                            validators.push(ValidatorEnum::Integer(IntegerValidator {}));
                        } else {
                            let simple_type = TypeEnum::Number;
                            let simple_type_name = format!("{}{}", type_name, "Number");
                            simple_type_names.push(simple_type_name.clone());
                            let simple_type_model = TypeModel {
                                r#type: simple_type,
                                validators: Default::default(),
                                property: Default::default(),
                                properties: Default::default(),
                                item: Default::default(),
                                items: Default::default(),
                            };
                            assert!(arena
                                .models
                                .insert(simple_type_name, simple_type_model)
                                .is_none());
                            validators.push(ValidatorEnum::Number(NumberValidator {}));
                        }
                    }
                    schemas::intermediate_a::TypeUnion::StringType(_type_node) => {
                        let simple_type_type = TypeEnum::String;
                        let simple_type_name = format!("{}{}", type_name, "String");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                        validators.push(ValidatorEnum::String(StringValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
                        let simple_type_type = TypeEnum::Tuple;
                        let simple_type_name = format!("{}{}", type_name, "Tuple");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                        // todo generate a type for every item in the union
                        items = type_node
                            .item_type_node_ids
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .cloned()
                            .collect();
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
                        let simple_type_type = TypeEnum::Array;
                        let simple_type_name = format!("{}{}", type_name, "Array");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                        // todo generate a type for the item
                        item = type_node
                            .item_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .cloned();
                        validators.push(ValidatorEnum::Array(ArrayValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
                        let simple_type_type = TypeEnum::Object;
                        let simple_type_name = format!("{}{}", type_name, "Object");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                        // todo generate a type for every property
                        properties = type_node
                            .property_type_node_ids
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|(key, node_id)| (key, node_id.as_ref()))
                            .map(|(key, node_id)| (key, name_map.get(node_id).unwrap()))
                            .map(|(key, node_name)| (key.clone(), node_name.clone()))
                            .collect();
                        validators.push(ValidatorEnum::Map(MapValidator {}));
                    }
                    schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                        let simple_type_type = TypeEnum::Map;
                        let simple_type_name = format!("{}{}", type_name, "Map");
                        simple_type_names.push(simple_type_name.clone());
                        let simple_type_model = TypeModel {
                            r#type: simple_type_type,
                            validators: Default::default(),
                            property: Default::default(),
                            properties: Default::default(),
                            item: Default::default(),
                            items: Default::default(),
                        };
                        assert!(arena
                            .models
                            .insert(simple_type_name, simple_type_model)
                            .is_none());
                        // todo generate a type for the property key and the property value
                        let key_type_name = format!("{}{}", type_name, "PropertyKey");
                        property = type_node
                            .property_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|node_name| (key_type_name, node_name.clone()));

                        validators.push(ValidatorEnum::Map(MapValidator {}));
                    }
                };
            }

            // then we fill the compounds
            for compound_node in node.compounds.iter() {
                match compound_node {
                    schemas::intermediate_a::CompoundUnion::OneOfCompound(compound_node) => {
                        one_of_type_names.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .cloned()
                                .collect(),
                        );
                    }
                    schemas::intermediate_a::CompoundUnion::AnyOfCompound(compound_node) => {
                        any_of_type_names.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .cloned()
                                .collect(),
                        );
                    }
                    schemas::intermediate_a::CompoundUnion::AllOfCompound(compound_node) => {
                        all_of_type_names.append(
                            &mut compound_node
                                .type_node_ids
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|node_id| node_id.as_ref())
                                .map(|node_id| name_map.get(node_id).unwrap())
                                .cloned()
                                .collect(),
                        );
                    }
                }
            }

            let mut sub_type_names = Vec::new();

            if !simple_type_names.is_empty() {
                let sub_type_name = format!("{}{}", type_name, "Type");
                sub_type_names.push(sub_type_name.clone());
                let sub_type_model = TypeModel {
                    r#type: TypeEnum::OneOf(simple_type_names),
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_name, sub_type_model).is_none());
            }

            if !all_of_type_names.is_empty() {
                let sub_type_name = format!("{}{}", type_name, "AllOf");
                sub_type_names.push(sub_type_name.clone());
                let sub_type_model = TypeModel {
                    r#type: TypeEnum::AllOf(all_of_type_names),
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_name, sub_type_model).is_none());
            }

            if !any_of_type_names.is_empty() {
                let sub_type_name = format!("{}{}", type_name, "AnyOf");
                sub_type_names.push(sub_type_name.clone());
                let sub_type_model = TypeModel {
                    r#type: TypeEnum::AnyOf(any_of_type_names),
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_name, sub_type_model).is_none());
            }

            if !one_of_type_names.is_empty() {
                let sub_type_name = format!("{}{}", type_name, "OneOf");
                sub_type_names.push(sub_type_name.clone());
                let sub_type_model = TypeModel {
                    r#type: TypeEnum::OneOf(one_of_type_names),
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_name, sub_type_model).is_none());
            }

            // get the super node name
            let super_type_name = node
                .super_node_id
                .as_ref()
                .map(|super_node_id| super_node_id.as_ref())
                .map(|super_node_id| name_map.get(super_node_id).unwrap())
                .cloned();
            if let Some(super_type_name) = super_type_name {
                sub_type_names.push(super_type_name)
            }

            // in the end the type will always be an allof type
            let type_type = TypeEnum::AllOf(sub_type_names);
            let type_model = TypeModel {
                r#type: type_type,
                validators,
                property,
                properties,
                item,
                items,
            };
            assert!(arena.models.insert(type_name.clone(), type_model).is_none());
        }
        arena
    }
}
