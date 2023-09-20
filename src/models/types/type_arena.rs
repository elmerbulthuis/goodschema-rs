use super::*;
use crate::schemas;
use std::{
    collections::{HashMap, HashSet},
    iter::empty,
};

#[derive(Debug, Default)]
pub struct TypeArena {
    pub(super) models: HashMap<TypeKey, TypeModel>,
    pub(super) node_id_to_type_key: HashMap<String, TypeKey>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    fn resolve_node_id(&self, node_id: impl AsRef<str>) -> TypeKey {
        *self.node_id_to_type_key.get(node_id.as_ref()).unwrap()
    }
}

impl From<&schemas::intermediate_a::SchemaJson> for TypeArena {
    fn from(intermediate_document: &schemas::intermediate_a::SchemaJson) -> Self {
        let mut arena = Self::new();

        // first we add all types to the arena from the intermediate document
        add_intermediate_to_arena(&mut arena, intermediate_document);

        // and then we flatten em until there is nothing more to flatten!
        while flatten_types(&mut arena) > 0 {}

        arena
    }
}

// Add all types to arena from intermediate document
fn add_intermediate_to_arena(
    arena: &mut TypeArena,
    intermediate_document: &schemas::intermediate_a::SchemaJson,
) {
    // create type keys for all nodes so we can later lookup the type key for every node
    for (node_id, _node) in intermediate_document.nodes.iter() {
        let type_key = TypeKey::new();
        assert!(arena
            .node_id_to_type_key
            .insert(node_id.clone(), type_key)
            .is_none());
    }

    // and now walk through the nodes and add the types they create
    for (node_id, node) in intermediate_document.nodes.iter() {
        let node_type_key = arena.resolve_node_id(node_id);

        let mut simple_type_keys = Vec::new();
        let mut one_of_type_keys = Vec::new();
        let mut any_of_type_keys = Vec::new();
        let mut all_of_type_keys = Vec::new();

        let mut node_type_validators = Vec::new();
        let mut property = None;
        let mut properties = HashMap::new();
        let mut item = None;
        let mut items = Vec::new();

        for type_node in node.types.iter() {
            match type_node {
                schemas::intermediate_a::TypeUnion::NeverType(_type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Never".to_string()),
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
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Any".to_string()),
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
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Null".to_string()),
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
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Boolean".to_string()),
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

                    node_type_validators.push(ValidatorEnum::Boolean(BooleanValidator {}));
                }
                schemas::intermediate_a::TypeUnion::DefsNumberType(type_node) => {
                    let number_type = if let Some(number_type) = &type_node.number_type {
                        number_type.as_ref()
                    } else {
                        "float"
                    };
                    if number_type == "integer" {
                        let simple_type_key = TypeKey::new();
                        simple_type_keys.push(simple_type_key);
                        let simple_type_model = TypeModel {
                            node_id: None,
                            name: Some("Integer".to_string()),
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

                        node_type_validators.push(ValidatorEnum::Integer(IntegerValidator {}));
                    } else {
                        let simple_type_key = TypeKey::new();
                        simple_type_keys.push(simple_type_key);
                        let simple_type_model = TypeModel {
                            node_id: None,
                            name: Some("Number".to_string()),
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

                        node_type_validators.push(ValidatorEnum::Number(NumberValidator {}));
                    }
                }
                schemas::intermediate_a::TypeUnion::StringType(_type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("String".to_string()),
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

                    node_type_validators.push(ValidatorEnum::String(StringValidator {}));
                }
                schemas::intermediate_a::TypeUnion::TupleType(type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Tuple".to_string()),
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
                        .map(|node_id| arena.resolve_node_id(node_id))
                        .collect();

                    node_type_validators.push(ValidatorEnum::Array(ArrayValidator {}));
                }
                schemas::intermediate_a::TypeUnion::ArrayType(type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Array".to_string()),
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
                        .map(|node_id| arena.resolve_node_id(node_id));

                    node_type_validators.push(ValidatorEnum::Array(ArrayValidator {}));
                }
                schemas::intermediate_a::TypeUnion::InterfaceType(type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Object".to_string()),
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
                        .map(|(key, node_id)| (key.to_string(), arena.resolve_node_id(node_id)))
                        .collect();
                    node_type_validators.push(ValidatorEnum::Map(MapValidator {}));
                }
                schemas::intermediate_a::TypeUnion::RecordType(type_node) => {
                    let property_key_type_validators = Vec::new();
                    node_type_validators.push(ValidatorEnum::String(StringValidator {}));

                    let property_key_type_key = TypeKey::new();
                    let property_key_type_model = TypeModel {
                        node_id: None,
                        name: Some("PropertyKey".to_string()),
                        r#type: TypeEnum::String,
                        validators: property_key_type_validators,
                        property: Default::default(),
                        properties: Default::default(),
                        item: Default::default(),
                        items: Default::default(),
                    };
                    assert!(arena
                        .models
                        .insert(property_key_type_key, property_key_type_model)
                        .is_none());

                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Map".to_string()),
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
                        .map(|node_id| arena.resolve_node_id(node_id))
                        .map(|type_key| (property_key_type_key, type_key));

                    node_type_validators.push(ValidatorEnum::Map(MapValidator {}));
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
                            .map(|node_id| arena.resolve_node_id(node_id))
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
                            .map(|node_id| arena.resolve_node_id(node_id))
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
                            .map(|node_id| arena.resolve_node_id(node_id))
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
                node_id: None,
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
                node_id: None,
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
                node_id: None,
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

        let super_type_key = node
            .super_node_id
            .as_ref()
            .map(|super_node_id| arena.resolve_node_id(super_node_id));

        if let Some(super_type_key) = super_type_key {
            node_type_keys.push(super_type_key);
        }

        let node_type_type = if node_type_keys.is_empty() {
            TypeEnum::Unknown
        } else {
            // in the end the type will be all of, because all of the types need to validate
            TypeEnum::AllOf(node_type_keys)
        };
        let node_type_model = TypeModel {
            node_id: Some(node_id.to_string()),
            name: None,
            r#type: node_type_type,
            validators: node_type_validators,
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
}

fn flatten_types(arena: &mut TypeArena) -> usize {
    let mut count = 0;

    count += merge_all_of_types(arena);

    count
}

/**
 * Merge all of types into something else than all of
 */
fn merge_all_of_types(arena: &TypeArena) -> usize {
    let mut count = 0;
    for (type_key, model) in arena.models.iter() {
        if let TypeEnum::AllOf(all_of_type_keys) = &model.r#type {
            count += 1;
            let all_of_models: Vec<_> = all_of_type_keys
                .iter()
                .map(|type_key| arena.models.get(type_key).unwrap())
                .cloned()
                .collect();
            let all_of_model_types: HashSet<_> = all_of_models
                .iter()
                .map(|model| &model.r#type)
                .cloned()
                .collect();

            let merged_type_validators = all_of_models
                .iter()
                .flat_map(|model| model.validators.iter())
                .cloned()
                .collect();

            let merged_property_vec: Vec<_> = all_of_models
                .iter()
                .filter_map(|model| model.property)
                .collect();
            let merged_property = if merged_property_vec.len() > 1 {
                let property_key_type_key = TypeKey::new();
                let property_value_type_key = TypeKey::new();

                // todo create all of type here

                Some((property_key_type_key, property_value_type_key))
            } else {
                merged_property_vec.first().cloned()
            };

            // todo
            let merged_properties = HashMap::new();

            // todo
            let merged_item = None;

            // todo
            let merged_items = Vec::new();

            let merged_type_type = {
                if all_of_model_types.len() > 1 {
                    let all_of_model_types: HashSet<_> = all_of_model_types
                        .into_iter()
                        .filter(|r#type| r#type != &TypeEnum::Unknown)
                        .filter(|r#type| r#type != &TypeEnum::Any)
                        .collect();

                    if all_of_model_types.len() > 1 {
                        if all_of_model_types.contains(&TypeEnum::Never) {
                            TypeEnum::Never
                        } else {
                            // ok lets get to merging!

                            let all_of_type_keys: HashSet<_> = all_of_model_types
                                .iter()
                                .flat_map(|r#type| {
                                    if let TypeEnum::AllOf(type_keys) = r#type {
                                        type_keys.iter()
                                    } else {
                                        Default::default()
                                    }
                                })
                                .cloned()
                                .collect();

                            let any_of_type_keys: HashSet<_> = all_of_model_types
                                .iter()
                                .flat_map(|r#type| {
                                    if let TypeEnum::AnyOf(type_keys) = r#type {
                                        type_keys.iter()
                                    } else {
                                        Default::default()
                                    }
                                })
                                .cloned()
                                .collect();

                            let one_of_type_keys: HashSet<_> = all_of_model_types
                                .iter()
                                .flat_map(|r#type| {
                                    if let TypeEnum::OneOf(type_keys) = r#type {
                                        type_keys.iter()
                                    } else {
                                        Default::default()
                                    }
                                })
                                .cloned()
                                .collect();

                            if all_of_type_keys.is_empty() {
                                if any_of_type_keys.is_empty() {
                                    if one_of_type_keys.is_empty() {
                                        // all of, any of, one of are empty
                                        todo!()
                                    } else {
                                        // only one not empty
                                        todo!()
                                    }
                                } else if one_of_type_keys.is_empty() {
                                    // only any of not empty
                                    todo!()
                                } else {
                                    // only all of is empty
                                    todo!()
                                }
                            } else if any_of_type_keys.is_empty() {
                                if one_of_type_keys.is_empty() {
                                    // only all of not empty
                                    TypeEnum::AllOf(all_of_type_keys.iter().cloned().collect())
                                } else {
                                    // only any of is empty
                                    todo!()
                                }
                            } else if one_of_type_keys.is_empty() {
                                // only one of is empty
                                todo!()
                            } else {
                                // all of, any of, one of not empty
                                todo!()
                            }
                        }
                    } else {
                        all_of_model_types
                            .iter()
                            .next()
                            .cloned()
                            .unwrap_or_default()
                    }
                } else {
                    all_of_model_types
                        .iter()
                        .next()
                        .cloned()
                        .unwrap_or_default()
                }
            };

            let _merged_type_model = TypeModel {
                node_id: None,
                name: None,
                r#type: merged_type_type,
                validators: merged_type_validators,
                property: merged_property,
                properties: merged_properties,
                item: merged_item,
                items: merged_items,
            };
        }
    }

    count
}
