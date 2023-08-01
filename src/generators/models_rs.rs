use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::HashMap;

use crate::schemas::intermediate_a::{self, CompoundEnum, SchemaNode, TypeEnum};

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
            if node.types.len() + node.compounds.len() == 1 {
                for node_type in &node.types {
                    let model_type_name = self.get_model_type_name(node_id, node_type)?;
                    let model_type_identifier = format_ident!("r#{}", model_type_name);
                    tokens.append_all(quote! {
                        pub type #model_identifier = #model_type_identifier;
                    });
                }

                for node_compound in &node.compounds {
                    let model_compound_name =
                        self.get_model_compound_name(node_id, node_compound)?;
                    let model_compound_identifier = format_ident!("r#{}", model_compound_name);
                    tokens.append_all(quote! {
                        pub type #model_identifier = #model_compound_identifier;
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
                            let member_name = self.to_member_name(property_name);
                            let member_identifier = format_ident!("r#{}", member_name);

                            let property_type_name = self.get_model_name(property_type_node_id)?;
                            let property_type_identifier =
                                format_ident!("r#{}", property_type_name);

                            if type_node.required_properties.contains(&member_name) {
                                property_tokens.append_all(quote! {
                                    #[serde(rename = #property_name)]
                                    pub #member_identifier: #property_type_identifier,
                                })
                            } else {
                                property_tokens.append_all(quote! {
                                    #[serde(rename = #property_name)]
                                    pub #member_identifier: Option<#property_type_identifier>,
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

            for node_compound in &node.compounds {
                let model_compound_name = self.get_model_compound_name(node_id, node_compound)?;
                let model_compound_identifier = format_ident!("r#{}", model_compound_name);

                match node_compound {
                    CompoundEnum::OneOf(compound_node) => {
                        let mut enum_tokens = quote! {};

                        for type_node_id in &compound_node.type_node_ids {
                            let type_name = self.get_model_name(type_node_id)?;
                            let type_identifier = format_ident!("r#{}", type_name);
                            enum_tokens.append_all(quote! {
                                #type_identifier(#type_identifier),
                            });
                        }

                        tokens.append_all(quote! {
                            #[derive(serde::Serialize, serde::Deserialize, Debug)]
                            #[serde(untagged)]
                            pub enum #model_compound_identifier {
                                #enum_tokens
                            }
                        });
                    }
                    CompoundEnum::AnyOf(compound_node) => {
                        let mut property_tokens = quote! {};

                        for type_node_id in &compound_node.type_node_ids {
                            let type_name = self.get_model_name(type_node_id)?;
                            let type_identifier = format_ident!("r#{}", type_name);

                            let member_name = self.to_member_name(&type_name);
                            let member_identifier = format_ident!("r#{}", member_name);

                            property_tokens.append_all(quote! {
                                #[serde(flatten)]
                                #member_identifier: Option<#type_identifier>,
                            });
                        }

                        tokens.append_all(quote! {
                            #[derive(serde::Serialize, serde::Deserialize, Debug)]
                            pub struct #model_compound_identifier{
                                #property_tokens
                            }
                        });

                        for type_node_id in &compound_node.type_node_ids {
                            let type_name = self.get_model_name(type_node_id)?;
                            let type_identifier = format_ident!("r#{}", type_name);

                            let member_name = self.to_member_name(&type_name);
                            let member_identifier = format_ident!("r#{}", member_name);

                            tokens.append_all(quote!{
                                impl TryFrom<#model_compound_identifier> for #type_identifier {
                                    type Error = ();

                                    fn try_from(value: #model_compound_identifier) -> Result<Self, Self::Error> {
                                        value.#member_identifier.ok_or(())
                                    }
                                }
                            });
                        }
                    }
                    CompoundEnum::AllOf(_) => todo!(),
                }
            }
        }

        Ok(tokens)
    }

    fn get_model_name(&self, node_id: &str) -> Result<String, &'static str> {
        let model_name = self.names.get(node_id).ok_or("could not find name")?;
        let model_name = model_name.to_pascal_case();

        Ok(model_name.to_string())
    }

    fn get_model_type_name(
        &self,
        node_id: &str,
        node_type: &intermediate_a::TypeEnum,
    ) -> Result<String, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let type_name = self.to_type_name(node_type);
        let model_type_name = format!("{}_{}", model_name, type_name);
        let model_type_name = model_type_name.to_pascal_case();

        Ok(model_type_name)
    }

    fn get_model_compound_name(
        &self,
        node_id: &str,
        node_compound: &intermediate_a::CompoundEnum,
    ) -> Result<String, &'static str> {
        let model_name = self.get_model_name(node_id)?;
        let compound_name = self.to_compound_name(node_compound);
        let model_compound_name = format!("{}_{}", model_name, compound_name);
        let model_compound_name = model_compound_name.to_pascal_case();

        Ok(model_compound_name)
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

    fn to_compound_name(&self, node_compound: &intermediate_a::CompoundEnum) -> &'static str {
        match node_compound {
            CompoundEnum::OneOf(_) => "OneOf",
            CompoundEnum::AnyOf(_) => "AnyOf",
            CompoundEnum::AllOf(_) => "AllOf",
        }
    }

    fn to_member_name(&self, property_name: &str) -> String {
        property_name.to_snake_case()
    }
}
