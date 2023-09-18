mod array_validator;
mod boolean_validator;
mod integer_validator;
mod intermediate;
mod map_validator;
mod number_validator;
mod string_validator;
mod type_enum;
mod type_model;
mod validator_enum;

pub use array_validator::*;
pub use boolean_validator::*;
pub use integer_validator::*;
pub use intermediate::*;
pub use map_validator::*;
pub use number_validator::*;
pub use string_validator::*;
pub use type_enum::*;
pub use type_model::*;
pub use validator_enum::*;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeArena {
    pub(super) models: HashMap<String, TypeModel>,
}

impl TypeArena {
    pub fn new() -> Self {
        Default::default()
    }
}
