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
            let mut sub_type_names = Vec::new();
            let mut one_of_type_names = Vec::new();
            let mut any_of_type_names = Vec::new();
            let mut all_of_type_names = Vec::new();

            // we'll fill these later
            let mut validators = Vec::new();
            let mut property = None;
            let mut properties = HashMap::new();
            let mut item = None;
            let mut items = Vec::new();

            for type_node in node.types.iter() {
                let sub_type = match type_node {
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
                            .cloned()
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
                            .cloned();
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
                            .map(|(key, node_name)| (key.clone(), node_name.clone()))
                            .collect();
                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Object
                    }
                    schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                        let key_type_name = format!("{}{}", type_name, "Property");
                        property = type_node
                            .property_type_node_id
                            .as_ref()
                            .map(|node_id| node_id.as_ref())
                            .map(|node_id| name_map.get(node_id).unwrap())
                            .map(|node_name| (key_type_name, node_name.clone()));

                        validators.push(ValidatorEnum::Map(MapValidator {}));
                        TypeEnum::Map
                    }
                };
                let sub_type_name = format!("{}{}", type_name, sub_type);
                sub_type_names.push(sub_type_name.clone());
                let sub_type_model = TypeModel {
                    super_type_name: Some(type_name.clone()),
                    r#type: sub_type,
                    validators: Default::default(),
                    property: Default::default(),
                    properties: Default::default(),
                    item: Default::default(),
                    items: Default::default(),
                };
                assert!(arena.models.insert(sub_type_name, sub_type_model).is_none());
            }

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

            // get the super node name
            let super_type_name = node
                .super_node_id
                .as_ref()
                .map(|super_node_id| super_node_id.as_ref())
                .map(|super_node_id| name_map.get(super_node_id).unwrap())
                .cloned();
            let type_type = TypeEnum::AllOf(all_of_type_names);
            let type_model = TypeModel {
                super_type_name,
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

/*
 * We store all type models in the type arena so we can easily generate types from them.
 *
 */
// impl TypeArena {
//     pub fn new_from_intermediate_document(
//         intermediate_document: &schemas::intermediate_a::SchemaJson,
//         name_map: &HashMap<String, String>,
//     ) -> Self {
//         let mut arena = Self::new();
//         let mut type_keys = HashMap::new();

//         for (node_id, node) in intermediate_document.nodes.iter() {
//             if node.select_is_empty() && node.super_node_id.is_some() {
//                 continue;
//             }

//             let type_key = TypeKey::new();
//             let type_name = name_map.get(node_id).unwrap();
//             assert!(type_keys.insert(type_name.clone(), type_key).is_none());
//         }

//         for (node_id, node) in intermediate_document.nodes.iter() {
//             if node.select_is_empty() && node.super_node_id.is_some() {
//                 continue;
//             }

//             let type_name = name_map.get(node_id).unwrap();
//             let type_key = *type_keys.get(type_name).unwrap();

//             let super_type_name = node
//                 .super_node_id
//                 .as_ref()
//                 .map(|super_node_id| super_node_id.as_ref())
//                 .map(|super_node_id| intermediate_document.select_non_empty(super_node_id))
//                 .map(|super_node_id| name_map.get(super_node_id).unwrap());
//             let super_type_key =
//                 super_type_name.map(|super_type_name| *type_keys.get(super_type_name).unwrap());

//             let mut sub_type_keys = Vec::new();
//             let mut one_of_type_keys = Vec::new();
//             let mut any_of_type_keys = Vec::new();
//             let mut all_of_type_keys = Vec::new();

//             let mut validators = Vec::new();
//             let mut property = None;
//             let mut properties = HashMap::new();
//             let mut item = None;
//             let mut items = Vec::new();

//             for type_node in node.types.iter() {
//                 let sub_type_key = TypeKey::new();
//                 sub_type_keys.push(sub_type_key);

//                 let sub_type = match type_node {
//                     schemas::intermediate_a::TypeUnion::NeverType(_type_node) => TypeEnum::Never,
//                     schemas::intermediate_a::TypeUnion::AnyType(_type_node) => TypeEnum::Any,
//                     schemas::intermediate_a::TypeUnion::NullType(_type_node) => TypeEnum::Null,
//                     schemas::intermediate_a::TypeUnion::BooleanType(_type_node) => {
//                         validators.push(ValidatorEnum::Boolean(BooleanValidator {}));
//                         TypeEnum::Boolean
//                     }
//                     schemas::intermediate_a::TypeUnion::DefsNumberType(type_node) => {
//                         let number_type = if let Some(number_type) = &type_node.number_type {
//                             number_type.as_ref()
//                         } else {
//                             "float"
//                         };
//                         if number_type == "integer" {
//                             validators.push(ValidatorEnum::Integer(IntegerValidator {}));
//                             TypeEnum::Integer
//                         } else {
//                             validators.push(ValidatorEnum::Number(NumberValidator {}));
//                             TypeEnum::Number
//                         }
//                     }
//                     schemas::intermediate_a::TypeUnion::StringType(_type_node) => {
//                         validators.push(ValidatorEnum::String(StringValidator {}));
//                         TypeEnum::String
//                     }
//                     schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
//                         items = type_node
//                             .item_type_node_ids
//                             .as_ref()
//                             .unwrap()
//                             .iter()
//                             .map(|node_id| node_id.as_ref())
//                             .map(|node_id| intermediate_document.select_non_empty(node_id))
//                             .map(|node_id| name_map.get(node_id).unwrap())
//                             .map(|type_name| *type_keys.get(type_name).unwrap())
//                             .collect();
//                         validators.push(ValidatorEnum::Array(ArrayValidator {}));
//                         TypeEnum::Tuple
//                     }
//                     schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
//                         item = type_node
//                             .item_type_node_id
//                             .as_ref()
//                             .map(|node_id| node_id.as_ref())
//                             .map(|node_id| intermediate_document.select_non_empty(node_id))
//                             .map(|node_id| name_map.get(node_id).unwrap())
//                             .map(|type_name| *type_keys.get(type_name).unwrap());
//                         validators.push(ValidatorEnum::Array(ArrayValidator {}));
//                         TypeEnum::Array
//                     }
//                     schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
//                         properties = type_node
//                             .property_type_node_ids
//                             .as_ref()
//                             .unwrap()
//                             .iter()
//                             .map(|(key, node_id)| (key, node_id.as_ref()))
//                             .map(|(key, node_id)| {
//                                 (key, intermediate_document.select_non_empty(node_id))
//                             })
//                             .map(|(key, node_id)| (key, name_map.get(node_id).unwrap()))
//                             .map(|(key, type_name)| {
//                                 (key.clone(), *type_keys.get(type_name).unwrap())
//                             })
//                             .collect();
//                         validators.push(ValidatorEnum::Map(MapValidator {}));
//                         TypeEnum::Object
//                     }
//                     schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
//                         property = type_node
//                             .property_type_node_id
//                             .as_ref()
//                             .map(|node_id| node_id.as_ref())
//                             .map(|node_id| intermediate_document.select_non_empty(node_id))
//                             .map(|node_id| name_map.get(node_id).unwrap())
//                             .map(|type_name| *type_keys.get(type_name).unwrap())
//                             .map(|type_key| (TypeKey::new(), type_key));

//                         validators.push(ValidatorEnum::Map(MapValidator {}));
//                         TypeEnum::Map
//                     }
//                 };
//                 let sub_type_model = TypeModel {
//                     name: Some(format!("{}{}", type_name, sub_type)),
//                     super_type_name: Some(type_key),
//                     r#type: sub_type,
//                     validators: Default::default(),
//                     property: Default::default(),
//                     properties: Default::default(),
//                     item: Default::default(),
//                     items: Default::default(),
//                 };
//                 assert!(arena.models.insert(sub_type_key, sub_type_model).is_none());
//             }

//             if !sub_type_keys.is_empty() {
//                 let sub_type_key = TypeKey::new();
//                 sub_type_keys.push(sub_type_key);
//                 let sub_type_type = TypeEnum::OneOf(sub_type_keys);
//                 let sub_type_model = TypeModel {
//                     name: None,
//                     super_type_name: Some(type_key),
//                     r#type: sub_type_type,
//                     validators: Default::default(),
//                     property: Default::default(),
//                     properties: Default::default(),
//                     item: Default::default(),
//                     items: Default::default(),
//                 };
//                 assert!(arena.models.insert(sub_type_key, sub_type_model).is_none());
//             }

//             for compound_node in node.compounds.iter() {
//                 match compound_node {
//                     schemas::intermediate_a::CompoundUnion::OneOfCompound(compound_node) => {
//                         one_of_type_keys.append(
//                             &mut compound_node
//                                 .type_node_ids
//                                 .as_ref()
//                                 .unwrap()
//                                 .iter()
//                                 .map(|node_id| node_id.as_ref())
//                                 .map(|node_id| intermediate_document.select_non_empty(node_id))
//                                 .map(|node_id| name_map.get(node_id).unwrap())
//                                 .map(|type_name| *type_keys.get(type_name).unwrap())
//                                 .collect(),
//                         );
//                     }
//                     schemas::intermediate_a::CompoundUnion::AnyOfCompound(compound_node) => {
//                         any_of_type_keys.append(
//                             &mut compound_node
//                                 .type_node_ids
//                                 .as_ref()
//                                 .unwrap()
//                                 .iter()
//                                 .map(|node_id| node_id.as_ref())
//                                 .map(|node_id| intermediate_document.select_non_empty(node_id))
//                                 .map(|node_id| name_map.get(node_id).unwrap())
//                                 .map(|type_name| *type_keys.get(type_name).unwrap())
//                                 .collect(),
//                         );
//                     }
//                     schemas::intermediate_a::CompoundUnion::AllOfCompound(compound_node) => {
//                         all_of_type_keys.append(
//                             &mut compound_node
//                                 .type_node_ids
//                                 .as_ref()
//                                 .unwrap()
//                                 .iter()
//                                 .map(|node_id| node_id.as_ref())
//                                 .map(|node_id| intermediate_document.select_non_empty(node_id))
//                                 .map(|node_id| name_map.get(node_id).unwrap())
//                                 .map(|type_name| *type_keys.get(type_name).unwrap())
//                                 .collect(),
//                         );
//                     }
//                 }
//             }

//             if !one_of_type_keys.is_empty() {
//                 let one_of_type_key = TypeKey::new();
//                 one_of_type_keys.push(one_of_type_key);
//                 let one_of_type_type = TypeEnum::OneOf(one_of_type_keys);
//                 let one_of_type_model = TypeModel {
//                     name: Some(format!("{}{}", type_name, one_of_type_type)),
//                     super_type_name: Some(type_key),
//                     r#type: one_of_type_type,
//                     validators: Default::default(),
//                     property: Default::default(),
//                     properties: Default::default(),
//                     item: Default::default(),
//                     items: Default::default(),
//                 };
//                 assert!(arena
//                     .models
//                     .insert(one_of_type_key, one_of_type_model)
//                     .is_none());
//             }

//             if !any_of_type_keys.is_empty() {
//                 let any_of_type_key = TypeKey::new();
//                 all_of_type_keys.push(any_of_type_key);
//                 let any_of_type_type = TypeEnum::AnyOf(any_of_type_keys);
//                 let any_of_type_model = TypeModel {
//                     name: Some(format!("{}{}", type_name, any_of_type_type)),
//                     super_type_name: Some(type_key),
//                     r#type: any_of_type_type,
//                     validators: Default::default(),
//                     property: Default::default(),
//                     properties: Default::default(),
//                     item: Default::default(),
//                     items: Default::default(),
//                 };
//                 assert!(arena
//                     .models
//                     .insert(any_of_type_key, any_of_type_model)
//                     .is_none());
//             }

//             let type_type = TypeEnum::AllOf(all_of_type_keys);
//             let type_model = TypeModel {
//                 name: Some(type_name.clone()),
//                 super_type_name: super_type_key,
//                 r#type: type_type,
//                 validators,
//                 property,
//                 properties,
//                 item,
//                 items,
//             };
//             assert!(arena.models.insert(type_key, type_model).is_none());
//         }

//         arena
//     }
// }
