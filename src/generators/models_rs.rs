use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::HashMap;

use crate::schemas::intermediate_a::{self, SchemaNode, TypeEnum};

pub struct ModelsRsGenerator<'a> {
    intermediate_data: &'a intermediate_a::Schema,
    names: &'a HashMap<String, String>,
}

impl<'a> ModelsRsGenerator<'a> {
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

        for (node_id, node) in &self.intermediate_data.nodes {
            tokens.append_all(self.generate_model_token_stream(node_id, node));
        }

        Ok(tokens)
    }

    fn generate_model_token_stream(
        &self,
        node_id: &str,
        node: &SchemaNode,
    ) -> Result<TokenStream, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let model_identifier = format_ident!("r#{}", model_name);

        let mut tokens = quote! {};

        if let Some(super_node_id) = &node.super_node_id {
            let super_model_name = self.get_model_name(super_node_id)?;
            let super_model_identifier = format_ident!("r#{}", super_model_name);
            tokens.append_all(quote! {
                pub type #model_identifier = #super_model_identifier;
            });
        } else {
            if node.types.len() == 1 {
                for node_type in &node.types {
                    let model_type_name = self.get_model_type_name(node_id, node_type)?;
                    let model_type_identifier = format_ident!("r#{}", model_type_name);
                    tokens.append_all(quote! {
                        pub type #model_identifier = #model_type_identifier;
                    });
                }
            } else {
                let mut enum_tokens = quote! {};

                for node_type in &node.types {
                    let type_name = self.to_type_name(node_type);
                    let type_identifier = format_ident!("r#{}", type_name);
                    let model_type_name = self.get_model_type_name(node_id, node_type)?;
                    let model_type_identifier = format_ident!("r#{}", model_type_name);
                    enum_tokens.append_all(quote! {
                        #type_identifier(#model_type_identifier),
                    });
                }

                tokens.append_all(quote! {
                    #[derive(serde::Serialize, serde::Deserialize, Debug)]
                    #[serde(untagged)]
                    pub enum #model_identifier {
                        #enum_tokens
                    }
                });
            }

            for node_type in &node.types {
                let model_type_name = self.get_model_type_name(node_id, node_type)?;
                let model_type_identifier = format_ident!("r#{}", model_type_name);

                match node_type {
                    TypeEnum::Null(_) => {
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = ();
                        });
                    }
                    TypeEnum::Any(_) => {
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = std::any::Any;
                        });
                    }
                    TypeEnum::Never(_) => todo!(),
                    TypeEnum::Boolean(_) => {
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = bool;
                        });
                    }
                    TypeEnum::Number(_) => {
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = i64;
                        });
                    }
                    TypeEnum::String(_) => {
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = String;
                        });
                    }
                    TypeEnum::Tuple(type_node) => {
                        let mut tuple_tokens = quote! {};

                        for item_type_node_id in &type_node.item_type_node_ids {
                            let item_type_name = self.get_model_name(item_type_node_id)?;
                            let item_type_identifier = format_ident!("r#{}", item_type_name);
                            tuple_tokens.append_all(quote! {
                                #item_type_identifier,
                            });
                        }

                        tokens.append_all(quote! {
                            pub type #model_type_identifier = (#tuple_tokens);
                        });
                    }
                    TypeEnum::Array(type_node) => {
                        let item_type_name = self.get_model_name(&type_node.item_type_node_id)?;
                        let item_type_identifier = format_ident!("r#{}", item_type_name);
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = Vec<#item_type_identifier>;
                        });
                    }
                    TypeEnum::Interface(type_node) => {
                        let mut property_tokens = quote! {};

                        for (property_name, property_type_node_id) in
                            &type_node.property_type_node_ids
                        {
                            let property_name = self.to_property_name(property_name);
                            let property_identifier = format_ident!("r#{}", property_name);

                            let property_type_name = self.get_model_name(property_type_node_id)?;
                            let property_type_identifier =
                                format_ident!("r#{}", property_type_name);

                            if type_node.required_properties.contains(&property_name) {
                                property_tokens.append_all(quote! {
                                    pub #property_identifier: #property_type_identifier,
                                })
                            } else {
                                property_tokens.append_all(quote! {
                                    pub #property_identifier: Option<#property_type_identifier>,
                                })
                            }
                        }

                        tokens.append_all(quote! {
                            #[derive(serde::Serialize, serde::Deserialize, Debug)]
                            pub struct #model_type_identifier {
                                #property_tokens
                            }
                        });
                    }
                    TypeEnum::Record(type_node) => {
                        let property_type_name =
                            self.get_model_name(&type_node.property_type_node_id)?;
                        let property_type_identifier = format_ident!("r#{}", property_type_name);
                        tokens.append_all(quote! {
                            pub type #model_type_identifier = HashMap<String, #property_type_identifier>;
                        });
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn get_model_name(&self, node_id: &str) -> Result<String, &'static str> {
        let model_name = self.names.get(node_id).ok_or("could not find name")?;

        Ok(model_name.to_string())
    }

    fn get_model_type_name(
        &self,
        node_id: &str,
        node_type: &intermediate_a::TypeEnum,
    ) -> Result<String, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let type_name = self.to_type_name(node_type);
        let model_type_name = format!("{}{}", model_name, type_name);
        Ok(model_type_name)
    }

    fn to_type_name(&self, node_type: &intermediate_a::TypeEnum) -> &'static str {
        match node_type {
            TypeEnum::Null(_) => "Null",
            TypeEnum::Any(_) => "Any",
            TypeEnum::Never(_) => "Never",
            TypeEnum::Boolean(_) => "Boolean",
            TypeEnum::Number(_) => "Number",
            TypeEnum::String(_) => "String",
            TypeEnum::Tuple(_) => "Tuple",
            TypeEnum::Array(_) => "Array",
            TypeEnum::Interface(_) => "Interface",
            TypeEnum::Record(_) => "Record",
        }
    }

    fn to_property_name(&self, property_name: &str) -> String {
        property_name.to_string()
    }
}
