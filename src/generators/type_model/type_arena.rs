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
        /*
         * remove the enum so we can work with it
         */
        let type_enum = self.type_enums.remove(&type_key).unwrap_or(TypeEnum::Never);

        /*
         * if both types are the same re-insert
         */
        if type_enum == other_type_enum {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }

        /*
         * If one of the types is never, insert the other
         */
        if type_enum == TypeEnum::Never {
            assert!(self.type_enums.insert(type_key, other_type_enum).is_none());
            return;
        }
        if other_type_enum == TypeEnum::Never {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }

        /*
         * If one of the types is any, insert that type (any)
         */
        if type_enum == TypeEnum::Any {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }
        if other_type_enum == TypeEnum::Any {
            assert!(self.type_enums.insert(type_key, other_type_enum).is_none());
            return;
        }

        {
            /*
             * create a union if not able to flatten
             */
            let union_type_key = type_key;
            let type_key = TypeKey::new();
            let other_type_key = TypeKey::new();

            let union_type_enum = TypeEnum::Union([type_key, other_type_key].into());

            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            assert!(self
                .type_enums
                .insert(other_type_key, other_type_enum)
                .is_none());
            assert!(self
                .type_enums
                .insert(union_type_key, union_type_enum)
                .is_none());
        }
    }

    pub fn add_type_by_intersection(&mut self, type_key: TypeKey, other_type_enum: TypeEnum) {
        /*
         * remove the enum so we can work with it
         */
        let type_enum = self.type_enums.remove(&type_key).unwrap_or(TypeEnum::Any);

        /*
         * if both types are the same then re-insert
         */
        if type_enum == other_type_enum {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }

        /*
         * If one of the types is never, insert that type (never)
         *
         */
        if type_enum == TypeEnum::Never {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }
        if other_type_enum == TypeEnum::Never {
            assert!(self.type_enums.insert(type_key, other_type_enum).is_none());
            return;
        }

        /*
         * If one of the types is any, return the other type
         *
         */
        if type_enum == TypeEnum::Any {
            assert!(self.type_enums.insert(type_key, other_type_enum).is_none());
            return;
        }
        if other_type_enum == TypeEnum::Any {
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
            return;
        }

        {
            /*
             * if we cannot flatten the intersection, then make a never type
             */

            let type_enum = TypeEnum::Never;
            assert!(self.type_enums.insert(type_key, type_enum).is_none());
        }
    }
}
