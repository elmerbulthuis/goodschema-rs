use super::*;
use crate::schemas;
use std::{
    cell::RefCell,
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
        let mut required = Vec::new();

        for type_node in node.types.iter() {
            match type_node {
                schemas::intermediate_a::TypeUnion::NeverType(_type_node) => {
                    let simple_type_key = TypeKey::new();
                    simple_type_keys.push(simple_type_key);
                    let simple_type_model = TypeModel {
                        node_id: None,
                        name: Some("Never".to_string()),
                        r#type: TypeEnum::Never,
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                    if let Some(required_properties) = &type_node.required_properties {
                        required.append(
                            &mut required_properties
                                .iter()
                                .map(|property| property.to_string())
                                .collect(),
                        )
                    }
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
            required,
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

fn merge_all_of_types(arena: &mut TypeArena) -> usize {
    let mut count = 0;

    let keys: Vec<_> = arena.models.keys().cloned().collect();
    for type_key in keys {
        let mut type_model = arena.models.remove(&type_key).unwrap();

        if let TypeEnum::AllOf(compound_type_keys) = &type_model.r#type {
            count += 1;

            let compound_models: HashMap<_, _> = compound_type_keys
                .iter()
                .map(|key| (*key, arena.models.get(key).unwrap().clone()))
                .collect();

            let compound_model_all_of_type_keys: HashMap<_, _> = compound_models
                .iter()
                .filter_map(|(key, model)| {
                    if let TypeEnum::AllOf(type_keys) = &model.r#type {
                        Some((*key, type_keys.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            if !compound_model_all_of_type_keys.is_empty() {
                type_model.r#type = TypeEnum::AllOf(
                    empty()
                        .chain(compound_model_all_of_type_keys.values().flatten())
                        .chain(
                            compound_type_keys
                                .iter()
                                .filter(|key| !compound_model_all_of_type_keys.contains_key(key)),
                        )
                        .cloned()
                        .collect(),
                );
            }
        }

        arena.models.insert(type_key, type_model.clone());
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arena = TypeArena::new();

        let key_1 = TypeKey::new();
        let key_2 = TypeKey::new();
        let key_3 = TypeKey::new();
        let key_4 = TypeKey::new();
        let key_5 = TypeKey::new();

        let model_1 = TypeModel {
            r#type: TypeEnum::String,
            ..Default::default()
        };
        let model_2 = TypeModel {
            r#type: TypeEnum::String,
            ..Default::default()
        };
        let model_3 = TypeModel {
            r#type: TypeEnum::AllOf([key_1, key_2].into()),
            ..Default::default()
        };
        let model_4 = TypeModel {
            r#type: TypeEnum::String,
            ..Default::default()
        };
        let model_5 = TypeModel {
            r#type: TypeEnum::AllOf([key_3, key_4].into()),
            ..Default::default()
        };

        assert!(arena.models.insert(key_1, model_1).is_none());
        assert!(arena.models.insert(key_2, model_2).is_none());
        assert!(arena.models.insert(key_3, model_3).is_none());
        assert!(arena.models.insert(key_4, model_4).is_none());
        assert!(arena.models.insert(key_5, model_5).is_none());

        merge_all_of_types(&mut arena);

        let model_5 = arena.models.get(&key_5).unwrap();

        assert_eq!(
            model_5,
            &TypeModel {
                r#type: TypeEnum::AllOf([key_1, key_2, key_4].into()),
                ..Default::default()
            }
        );

        println!("{:?}", arena);
    }
}
