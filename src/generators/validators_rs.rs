use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::HashMap;
use url::Url;

use crate::schemas::intermediate_a::{self, SchemaNode};

pub struct ValidatorsRsGenerator<'a> {
    intermediate_data: &'a intermediate_a::Schema,
    names: &'a HashMap<String, String>,
}

impl<'a> ValidatorsRsGenerator<'a> {
    pub fn new(
        intermediate_data: &'a intermediate_a::Schema,
        names: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            intermediate_data,
            names,
        }
    }

    pub fn generate_file_token_stream(&self) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            use super::models;
        });

        for (node_id, node) in self.intermediate_data.nodes.iter() {
            tokens.append_all(self.generate_model_token_stream(node_id.as_str(), node));
        }

        Ok(tokens)
    }

    fn generate_model_token_stream(
        &self,
        node_id: &str,
        node: &SchemaNode,
    ) -> Result<TokenStream, &'static str> {
        let node_name = self.names.get(node_id).ok_or("could not find name")?;

        // let validator_name = node_name.join(" ");
        // let validator_name = format!("validate_{}", validator_name);
        // let validator_name = to_snake_case(&validator_name);
        // let validator_name = format_ident!("r#{}", validator_name);

        // let model_name = node_name.join(" ");
        // let model_name = to_class_case(&model_name);
        // let model_name = format_ident!("r#{}", model_name);

        let mut tokens = quote! {};

        // tokens.append_all(quote! {

        //     pub fn #validator_name(model: &models::#model_name) -> bool {
        //         todo!();
        //     }

        // });

        Ok(tokens)
    }
}
