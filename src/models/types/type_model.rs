use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeModel {
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub r#type: TypeEnum,
    pub validators: Vec<ValidatorEnum>,
    pub property: Option<(TypeKey, TypeKey)>,
    pub properties: HashMap<String, TypeKey>,
    pub item: Option<TypeKey>,
    pub items: Vec<TypeKey>,
    pub required: Vec<String>,
}

impl TypeModel {
    pub fn is_alias_for(&self) -> Option<TypeKey> {
        if !self.validators.is_empty() {
            return None;
        }

        if self.property.is_some() {
            return None;
        }

        if !self.properties.is_empty() {
            return None;
        }

        if self.item.is_some() {
            return None;
        }

        if !self.items.is_empty() {
            return None;
        }

        match &self.r#type {
            TypeEnum::OneOf(type_keys) if type_keys.len() == 1 => type_keys.get(0).cloned(),
            TypeEnum::AnyOf(type_keys) if type_keys.len() == 1 => type_keys.get(0).cloned(),
            TypeEnum::AllOf(type_keys) if type_keys.len() == 1 => type_keys.get(0).cloned(),
            _ => None,
        }
    }
}
