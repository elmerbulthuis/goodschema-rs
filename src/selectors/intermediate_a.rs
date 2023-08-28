use super::{Selectors, TypeEnum};
use crate::schemas;
use std::collections::HashSet;

impl Selectors for schemas::intermediate_a::SchemaJson {
    fn select_types(&self, node_id: &str) -> HashSet<TypeEnum> {
        let node = self.nodes.get(node_id).unwrap();

        let mut type_enums: HashSet<_> = node
            .types
            .iter()
            .map(|type_node| type_node.into())
            .collect();

        if let Some(super_node_id) = &node.super_node_id {
            let super_type_enums = self.select_types(super_node_id);

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

        let mut options: HashSet<_> = node
            .types
            .iter()
            .flat_map(string_options_from_type_node)
            .collect();

        if let Some(super_node_id) = &node.super_node_id {
            let super_type_options = self.select_string_options(super_node_id);

            options = options.intersection(&super_type_options).cloned().collect();
        }

        options
    }
}

fn string_options_from_type_node(
    type_node: &schemas::intermediate_a::TypeUnionOneOf,
) -> HashSet<&str> {
    match type_node {
        // string
        schemas::intermediate_a::TypeUnion::OneOf5(type_node) => {
            if let Some(options) = type_node.options {
                options.iter().map(|option| option.as_ref()).collect()
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

impl From<&schemas::intermediate_a::TypeUnionOneOf> for TypeEnum {
    fn from(type_node: &schemas::intermediate_a::TypeUnionOneOf) -> Self {
        match type_node {
            // null
            schemas::intermediate_a::TypeUnion::TypeUnionOneOf0(_) => TypeEnum::Null,
            // any
            schemas::intermediate_a::TypeUnion::TypeUnionOneOf1(_) => TypeEnum::Any,
            // never
            schemas::intermediate_a::TypeUnion::TypeUnionOneOf2(_) => TypeEnum::Never,
            // boolean
            schemas::intermediate_a::TypeUnion::OneOf3(_) => TypeEnum::Boolean,
            // number
            schemas::intermediate_a::TypeUnion::OneOf4(_) => TypeEnum::Number,
            // string
            schemas::intermediate_a::TypeUnion::OneOf5(_) => TypeEnum::String,
            // tuple
            schemas::intermediate_a::TypeUnion::OneOf6(_) => TypeEnum::Tuple,
            // array
            schemas::intermediate_a::TypeUnion::OneOf7(_) => TypeEnum::Array,
            // interface
            schemas::intermediate_a::TypeUnion::OneOf8(_) => TypeEnum::Object,
            // record
            schemas::intermediate_a::TypeUnion::OneOf9(_) => TypeEnum::Record,
        }
    }
}
