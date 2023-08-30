use super::ModelsRsGenerator;
use crate::{schemas, selectors::document::TypeEnum};
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};

impl<'a> ModelsRsGenerator<'a> {
    pub(super) fn generate_new_type_token_stream(
        type_name: &str,
        interior_name: &str,
        type_identifier: &Ident,
        interior_identifier: &Ident,
        validation_tokens: TokenStream,
        derive_tokens: TokenStream,
        boxed: bool,
    ) -> TokenStream {
        let mut tokens = quote! {};

        let interior_tokens = if boxed {
            quote! { Box<#interior_identifier> }
        } else {
            quote! { #interior_identifier }
        };

        let new_interior_tokens = if boxed {
            quote! { Box::new(interior) }
        } else {
            quote! { interior }
        };

        let from_tokens = if boxed {
            quote! { *value.0 }
        } else {
            quote! { value.0 }
        };

        tokens.append_all(quote! {
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, #derive_tokens)]
            #[serde(try_from = #interior_name)]
            pub struct #type_identifier(#interior_tokens);
        });

        tokens.append_all(quote! {
            impl #type_identifier {
                pub fn new(interior: #interior_identifier) -> Result<Self, ValidationError> {
                    let instance = Self(#new_interior_tokens);
                    if instance.validate() {
                        Ok(instance)
                    } else {
                        Err(ValidationError::new(#type_name))
                    }
                }

                fn validate(&self) -> bool {
                    #validation_tokens
                }
            }
        });

        tokens.append_all(quote! {
            impl TryFrom<#interior_identifier> for #type_identifier {
                type Error = ValidationError;

                fn try_from(interior: #interior_identifier) -> Result<Self, Self::Error> {
                    Self::new(interior)
                }
            }
        });

        tokens.append_all(quote! {
            impl From<#type_identifier> for #interior_identifier {
                fn from(value: #type_identifier) -> Self {
                    #from_tokens
                }
            }
        });

        tokens
    }

    pub(super) fn generate_new_type_ref_token_stream(
        type_identifier: &Ident,
        interior_identifier: &Ident,
    ) -> TokenStream {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            impl AsRef<#interior_identifier> for #type_identifier {
                fn as_ref(&self) -> &#interior_identifier {
                    &self.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::borrow::Borrow<#interior_identifier> for #type_identifier {
                fn borrow(&self) -> &#interior_identifier {
                    self.as_ref()
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #type_identifier {
                type Target = #interior_identifier;

                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        });

        tokens
    }

    pub(super) fn generate_new_type_string_ref_token_stream(
        type_identifier: &Ident,
    ) -> TokenStream {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            impl std::str::FromStr for #type_identifier {
                type Err = ValidationError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new(s.to_string())
                }
            }
        });

        tokens.append_all(quote! {
            impl ToString for #type_identifier {
                fn to_string(&self) -> String {
                    self.0.to_string()
                }
            }
        });

        tokens.append_all(quote! {
            impl AsRef<str> for #type_identifier {
                fn as_ref(&self) -> &str {
                    self.0.as_str()
                }
            }
        });

        tokens.append_all(quote! {
            impl std::borrow::Borrow<str> for #type_identifier {
                fn borrow(&self) -> &str {
                    self.as_ref()
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #type_identifier {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        });

        tokens
    }

    fn get_name(&self, node_id: &str) -> Result<&str, &'static str> {
        self.names
            .get(node_id)
            .map(|v| v.as_str())
            .ok_or("name not found")
    }

    pub(super) fn get_model_name(&self, node_id: &str) -> Result<String, &'static str> {
        let model_name = self.get_name(node_id)?;
        let model_name = model_name.to_pascal_case();

        Ok(model_name.to_string())
    }

    pub(super) fn get_model_type_name(
        &self,
        node_id: &str,
        type_enum: &TypeEnum,
    ) -> Result<String, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let type_name = self.to_type_name(type_enum);
        let model_type_name = format!("{}_{}", model_name, type_name);
        let model_type_name = model_type_name.to_pascal_case();

        Ok(model_type_name)
    }

    pub(super) fn get_model_compound_name(
        &self,
        node_id: &str,
        node_compound: &schemas::intermediate_a::CompoundUnion,
    ) -> Result<String, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let compound_name = self.to_compound_name(node_compound);
        let model_compound_name = format!("{}_{}", model_name, compound_name);
        let model_compound_name = model_compound_name.to_pascal_case();

        Ok(model_compound_name)
    }

    pub(super) fn to_type_name(&self, type_enum: &TypeEnum) -> &'static str {
        match type_enum {
            // never
            TypeEnum::Never => "Never",
            // any
            TypeEnum::Any => "Any",
            // null
            TypeEnum::Null => "Null",
            // boolean
            TypeEnum::Boolean => "Boolean",
            // number
            TypeEnum::Integer => "Integer",
            // number
            TypeEnum::Number => "Number",
            // string
            TypeEnum::String => "String",
            // tuple
            TypeEnum::Tuple => "Tuple",
            // array
            TypeEnum::Array => "Array",
            // interface
            TypeEnum::Object => "Object",
            // record
            TypeEnum::Record => "Record",
        }
    }

    pub(super) fn to_compound_name(
        &self,
        node_compound: &schemas::intermediate_a::CompoundUnion,
    ) -> &'static str {
        match node_compound {
            schemas::intermediate_a::CompoundUnion::OneOfCompound(_) => "OneOf",
            schemas::intermediate_a::CompoundUnion::AnyOfCompound(_) => "AnyOf",
            schemas::intermediate_a::CompoundUnion::AllOfCompound(_) => "AllOf",
        }
    }

    pub(super) fn to_member_name(&self, property_name: &str) -> String {
        property_name.to_snake_case()
    }
}
