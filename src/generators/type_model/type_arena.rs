use std::collections::HashMap;

use super::{TypeEnum, TypeKey};

#[derive(Debug, Default)]
pub struct TypeArena {
    type_enums: HashMap<TypeKey, TypeEnum>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_type_enum(&self, type_key: TypeKey) -> Option<&TypeEnum> {
        self.type_enums.get(&type_key)
    }

    pub fn add_type_by_union(&mut self, type_key: TypeKey, other_type_enum: TypeEnum) {
        let type_enum = self.type_enums.remove(&type_key).unwrap_or(TypeEnum::Never);

        let new_type_key = TypeKey::new();
        let other_type_key = TypeKey::new();

        let new_type_enum = type_enum;
        let type_enum = TypeEnum::Union([new_type_key, other_type_key].into());

        assert!(self.type_enums.insert(type_key, type_enum).is_none());
        assert!(self
            .type_enums
            .insert(other_type_key, other_type_enum)
            .is_none());
        assert!(self
            .type_enums
            .insert(new_type_key, new_type_enum)
            .is_none());
    }

    pub fn add_type_by_intersection(&mut self, type_key: TypeKey, other_type_enum: TypeEnum) {
        let type_enum = self.type_enums.remove(&type_key).unwrap_or(TypeEnum::Any);

        let new_type_key = TypeKey::new();
        let other_type_key = TypeKey::new();

        let new_type_enum = type_enum;
        let type_enum = TypeEnum::Intersection([new_type_key, other_type_key].into());

        assert!(self.type_enums.insert(type_key, type_enum).is_none());
        assert!(self
            .type_enums
            .insert(other_type_key, other_type_enum)
            .is_none());
        assert!(self
            .type_enums
            .insert(new_type_key, new_type_enum)
            .is_none());
    }
}
