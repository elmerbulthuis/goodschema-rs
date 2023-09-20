mod compounds;
mod helpers;
mod types;

use crate::{
    models::type_arena::TypeArena, schemas, selectors::document::DocumentSelectors,
    selectors::document::TypeEnum, selectors::node::NodeSelectors,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::HashMap;

pub struct ModelsRsGenerator<'a> {
    intermediate_data: &'a schemas::intermediate_a::SchemaJson,
    names: &'a HashMap<String, String>,
    arena: TypeArena,
}

impl<'a> ModelsRsGenerator<'a> {
    pub fn new(
        intermediate_data: &'a schemas::intermediate_a::SchemaJson,
        names: &'a HashMap<String, String>,
    ) -> Self {
        let arena = TypeArena::from(intermediate_data);
        Self {
            intermediate_data,
            names,
            arena,
        }
    }
}

impl<'a> ModelsRsGenerator<'a> {
    pub fn generate_file_token_stream(&self) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
            pub struct ValidationError {
                r#type: &'static str,
            }
        });

        tokens.append_all(quote! {
            impl ValidationError {
                pub fn new(r#type: &'static str) -> Self {
                    Self { r#type }
                }
            }
        });

        tokens.append_all(quote! {
            impl std::fmt::Display for ValidationError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "validation error for type {}", self.r#type)
                }
            }

        });

        for (node_id, node) in self.intermediate_data.nodes.iter() {
            if node.select_is_empty() && node.super_node_id.is_some() {
                continue;
            }

            tokens.append_all(self.generate_model_token_stream(node_id, node));
        }

        Ok(tokens)
    }

    fn generate_model_token_stream(
        &self,
        node_id: &str,
        node: &schemas::intermediate_a::Node,
    ) -> Result<TokenStream, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let model_identifier = format_ident!("r#{}", model_name);

        let mut tokens = quote! {};

        let type_enums = self.intermediate_data.select_type_enums(node_id);

        if type_enums.len() + node.compounds.len() == 0 {
            tokens.append_all(self.generate_any_token_stream(&model_name))
        }

        if type_enums.len() + node.compounds.len() > 1 {
            let mut enum_tokens = quote! {};

            for node_type in &type_enums {
                let type_name = self.to_type_name(node_type);
                let type_identifier = format_ident!("r#{}", type_name);
                let model_type_name = self.get_model_type_name(node_id, node_type)?;
                let model_type_identifier = format_ident!("r#{}", model_type_name);
                enum_tokens.append_all(quote! {
                    #type_identifier(#model_type_identifier),
                });
            }

            tokens.append_all(quote! {
                #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
                #[serde(untagged)]
                pub enum #model_identifier {
                    #enum_tokens
                }
            });
        }

        for type_enum in &type_enums {
            let model_type_name = if type_enums.len() + node.compounds.len() > 1 {
                self.get_model_type_name(node_id, type_enum)?
            } else {
                model_name.clone()
            };

            match type_enum {
                // never
                TypeEnum::Never => {
                    tokens.append_all(self.generate_never_token_stream(&model_type_name))
                }
                // any
                TypeEnum::Any => {
                    tokens.append_all(self.generate_any_token_stream(&model_type_name))
                }
                // null
                TypeEnum::Null => {
                    tokens.append_all(self.generate_null_token_stream(&model_type_name))
                }
                // boolean
                TypeEnum::Boolean => {
                    tokens
                        .append_all(self.generate_boolean_token_stream(&model_type_name, node_id));
                }
                // integer
                TypeEnum::Integer => {
                    tokens
                        .append_all(self.generate_integer_token_stream(&model_type_name, node_id));
                }
                // number
                TypeEnum::Number => {
                    tokens.append_all(self.generate_number_token_stream(&model_type_name, node_id));
                }
                // string
                TypeEnum::String => {
                    tokens.append_all(self.generate_string_token_stream(&model_type_name, node_id));
                }
                // tuple
                TypeEnum::Tuple => {
                    tokens.append_all(self.generate_tuple_token_stream(&model_type_name, node_id)?);
                }
                // array
                TypeEnum::Array => {
                    tokens.append_all(self.generate_array_token_stream(&model_type_name, node_id)?);
                }
                // object
                TypeEnum::Object => {
                    tokens
                        .append_all(self.generate_object_token_stream(&model_type_name, node_id)?);
                }
                // map
                TypeEnum::Map => {
                    tokens.append_all(self.generate_map_token_stream(&model_type_name, node_id)?);
                }
            }
        }

        for node_compound in node.compounds.iter() {
            let model_compound_name = if type_enums.len() + node.compounds.len() > 1 {
                self.get_model_compound_name(node_id, node_compound)?
            } else {
                model_name.clone()
            };

            match node_compound {
                // one-of
                schemas::intermediate_a::CompoundUnion::OneOfCompound(compound_node) => {
                    tokens.append_all(
                        self.generate_one_of_token_stream(&model_compound_name, compound_node)?,
                    );
                }
                // any-of
                schemas::intermediate_a::CompoundUnion::AnyOfCompound(compound_node) => {
                    tokens.append_all(
                        self.generate_any_of_token_stream(&model_compound_name, compound_node)?,
                    );
                }
                // all-of
                schemas::intermediate_a::CompoundUnion::AllOfCompound(compound_node) => {
                    tokens.append_all(
                        self.generate_all_of_token_stream(&model_compound_name, compound_node)?,
                    );
                }
            }
        }

        Ok(tokens)
    }
}
