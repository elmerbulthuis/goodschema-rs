use super::{DocumentSelectors, TypeEnum};
use crate::{schemas, selectors::node::NodeSelectors};
use std::{
    collections::{HashMap, HashSet},
    iter::empty,
};

impl DocumentSelectors for schemas::intermediate_a::SchemaJson {
    fn select_type_enums(&self, node_id: &str) -> HashSet<TypeEnum> {
        let node = self.nodes.get(node_id).unwrap();

        let mut type_enums: HashSet<_> = node
            .types
            .iter()
            .map(|type_node| type_node.into())
            .collect();

        if let Some(super_node_id) = &node.super_node_id {
            let super_type_enums = self.select_type_enums(super_node_id);

            type_enums = type_enums
                .intersection(&super_type_enums)
                .cloned()
                .collect();
        }

        // for node_compound in &node.compounds {
        //     match node_compound {
        //         // one-of
        //         schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf0(compound_node) => {
        //             if let Some(type_node_ids) = &compound_node.type_node_ids {
        //                 for type_node_id in type_node_ids {
        //                     let type_node_enums = self.select_types(type_node_id);

        //                     type_enums =
        //                         type_enums.intersection(&type_node_enums).cloned().collect();
        //                 }
        //             }
        //         }
        //         // any-of
        //         schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf1(compound_node) => {
        //             if let Some(type_node_ids) = &compound_node.type_node_ids {
        //                 for type_node_id in type_node_ids {
        //                     let type_node_enums = self.select_types(type_node_id);

        //                     type_enums =
        //                         type_enums.intersection(&type_node_enums).cloned().collect();
        //                 }
        //             }
        //         }
        //         // all-of
        //         schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf2(compound_node) => {
        //             if let Some(type_node_ids) = &compound_node.type_node_ids {
        //                 for type_node_id in type_node_ids {
        //                     let type_node_enums = self.select_types(type_node_id);

        //                     type_enums =
        //                         type_enums.intersection(&type_node_enums).cloned().collect();
        //                 }
        //             }
        //         }
        //     }
        // }

        type_enums
    }

    fn select_string_options(&self, node_id: &str) -> HashSet<&str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut options: Vec<_> = node
            .types
            .iter()
            .map(string_options_from_type_node)
            .filter(|options| !options.is_empty())
            .collect();
        assert!(options.len() <= 1);
        let mut options = options.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_type_options = self.select_string_options(super_node_id);

            options = options.intersection(&super_type_options).cloned().collect();
        }

        options
    }

    fn select_tuple_item_type_node_ids(&self, node_id: &str) -> Vec<&str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut item_type_node_ids: Vec<_> = node
            .types
            .iter()
            .map(tuple_item_type_node_ids_from_type_node)
            .filter(|item_type_node_ids| !item_type_node_ids.is_empty())
            .collect();
        assert!(item_type_node_ids.len() <= 1);
        let mut item_type_node_ids = item_type_node_ids.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_item_type_node_ids = self.select_tuple_item_type_node_ids(super_node_id);

            assert!(item_type_node_ids.is_empty() || super_item_type_node_ids.is_empty());

            if item_type_node_ids.is_empty() {
                item_type_node_ids = super_item_type_node_ids
            }
        }

        item_type_node_ids
    }

    fn select_array_item_type_node_id(&self, node_id: &str) -> Option<&str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut item_type_node_id: Vec<_> = node
            .types
            .iter()
            .map(array_item_type_node_id_from_type_node)
            .filter(|item_type_node_id| item_type_node_id.is_some())
            .collect();
        assert!(item_type_node_id.len() <= 1);
        let mut item_type_node_id = item_type_node_id.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_item_type_node_id = self.select_array_item_type_node_id(super_node_id);

            assert!(item_type_node_id.is_none() || super_item_type_node_id.is_none());

            if item_type_node_id.is_none() {
                item_type_node_id = super_item_type_node_id
            }
        }

        item_type_node_id
    }

    fn select_object_property_type_node_ids(
        &self,
        node_id: &str,
    ) -> std::collections::HashMap<&str, &str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut property_type_node_ids: Vec<_> = node
            .types
            .iter()
            .map(object_property_type_node_ids_from_type_node)
            .filter(|property_type_node_ids| !property_type_node_ids.is_empty())
            .collect();
        assert!(property_type_node_ids.len() <= 1);
        let mut property_type_node_ids = property_type_node_ids.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_item_type_node_ids = self.select_object_property_type_node_ids(super_node_id);

            property_type_node_ids = empty()
                .chain(property_type_node_ids)
                .chain(super_item_type_node_ids)
                .collect();
        }

        property_type_node_ids
    }

    fn select_map_property_type_node_id(&self, node_id: &str) -> Option<&str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut property_type_node_id: Vec<_> = node
            .types
            .iter()
            .map(record_property_type_node_id_from_type_node)
            .filter(|property_type_node_id| property_type_node_id.is_some())
            .collect();
        assert!(property_type_node_id.len() <= 1);
        let mut property_type_node_id = property_type_node_id.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_property_type_node_id = self.select_map_property_type_node_id(super_node_id);

            assert!(property_type_node_id.is_none() || super_property_type_node_id.is_none());

            if property_type_node_id.is_none() {
                property_type_node_id = super_property_type_node_id
            }
        }

        property_type_node_id
    }

    fn select_object_required_properties(&self, node_id: &str) -> HashSet<&str> {
        let node = self.nodes.get(node_id).unwrap();

        let mut required_properties: Vec<_> = node
            .types
            .iter()
            .map(object_required_properties_from_type_node)
            .filter(|required_properties| !required_properties.is_empty())
            .collect();
        assert!(required_properties.len() <= 1);
        let mut required_properties = required_properties.pop().unwrap_or_default();

        if let Some(super_node_id) = &node.super_node_id {
            let super_required_properties = self.select_object_required_properties(super_node_id);

            required_properties = empty()
                .chain(required_properties)
                .chain(super_required_properties)
                .collect();
        }

        required_properties
    }

    fn select_non_empty<'l>(&'l self, node_id: &'l str) -> &'l str {
        let node = self.nodes.get(node_id).unwrap();

        if node.select_is_empty() {
            if let Some(super_node_id) = &node.super_node_id {
                return self.select_non_empty(super_node_id);
            }
        }

        node_id
    }
}

impl From<&schemas::intermediate_a::TypeUnion> for TypeEnum {
    fn from(type_node: &schemas::intermediate_a::TypeUnion) -> Self {
        match type_node {
            // null
            schemas::intermediate_a::TypeUnion::NullType(_) => TypeEnum::Null,
            // any
            schemas::intermediate_a::TypeUnion::AnyType(_) => TypeEnum::Any,
            // never
            schemas::intermediate_a::TypeUnion::NeverType(_) => TypeEnum::Never,
            // boolean
            schemas::intermediate_a::TypeUnion::BooleanType(_) => TypeEnum::Boolean,
            // number
            schemas::intermediate_a::TypeUnion::DefsNumberType(type_node) => {
                if let Some(number_type) = &type_node.number_type {
                    match number_type.as_ref() {
                        "integer" => TypeEnum::Integer,
                        "float" => TypeEnum::Number,
                        &_ => unreachable!(),
                    }
                } else {
                    TypeEnum::Number
                }
            }
            // string
            schemas::intermediate_a::TypeUnion::StringType(_) => TypeEnum::String,
            // tuple
            schemas::intermediate_a::TypeUnion::TupleType(_) => TypeEnum::Tuple,
            // array
            schemas::intermediate_a::TypeUnion::ArrayType(_) => TypeEnum::Array,
            // interface
            schemas::intermediate_a::TypeUnion::InterfaceType(_) => TypeEnum::Object,
            // record
            schemas::intermediate_a::TypeUnion::RecordType(_) => TypeEnum::Map,
        }
    }
}

fn string_options_from_type_node(type_node: &schemas::intermediate_a::TypeUnion) -> HashSet<&str> {
    match type_node {
        // string
        schemas::intermediate_a::TypeUnion::StringType(type_node) => {
            if let Some(options) = &type_node.options {
                options.iter().map(|option| option.as_ref()).collect()
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn tuple_item_type_node_ids_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnion,
) -> Vec<&str> {
    match type_node {
        // tuple
        schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
            if let Some(node_ids) = &type_node.item_type_node_ids {
                node_ids.iter().map(|node_id| node_id.as_ref()).collect()
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn array_item_type_node_id_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnion,
) -> Option<&str> {
    match type_node {
        // array
        schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
            if let Some(node_id) = &type_node.item_type_node_id {
                Some(node_id.as_ref())
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn object_property_type_node_ids_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnion,
) -> HashMap<&str, &str> {
    match type_node {
        // interface
        schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
            if let Some(node_ids) = &type_node.property_type_node_ids {
                node_ids
                    .iter()
                    .map(|(name, node_id)| (name.as_ref(), node_id.as_ref()))
                    .collect()
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn record_property_type_node_id_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnion,
) -> Option<&str> {
    match type_node {
        // record
        schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
            if let Some(node_id) = &type_node.property_type_node_id {
                Some(node_id.as_ref())
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn object_required_properties_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnion,
) -> HashSet<&str> {
    match type_node {
        // interface
        schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
            if let Some(properties) = &type_node.required_properties {
                properties.iter().map(|name| name.as_ref()).collect()
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}
