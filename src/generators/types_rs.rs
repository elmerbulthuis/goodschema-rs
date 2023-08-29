use crate::{
    schemas, selectors::document::DocumentSelectors, selectors::document::TypeEnum,
    selectors::node::NodeSelectors,
};
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::HashMap;

pub struct ModelsRsGenerator<'a> {
    intermediate_data: &'a schemas::intermediate_a::SchemaJson,
    names: &'a HashMap<String, String>,
}

impl<'a> ModelsRsGenerator<'a> {
    pub fn new(
        intermediate_data: &'a schemas::intermediate_a::SchemaJson,
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
                // record
                TypeEnum::Record => {
                    tokens
                        .append_all(self.generate_record_token_stream(&model_type_name, node_id)?);
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

    fn generate_never_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        tokens.append_all(quote! {
            pub type #model_interior_identifier = ();
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {false},
            quote! {Copy, PartialEq, PartialOrd, Eq, Ord},
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        tokens
    }

    fn generate_any_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = &format_ident!("r#{}", model_type_name);

        tokens.append_all(quote! {
            pub type #model_type_identifier = serde_json::Value;
        });

        tokens
    }

    fn generate_null_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        tokens.append_all(quote! {
            pub type #model_interior_identifier = ();
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {true},
            quote! {Copy, PartialEq, PartialOrd, Eq, Ord},
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        tokens
    }

    fn generate_boolean_token_stream(&self, model_type_name: &str, _node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        tokens.append_all(quote! {
            pub type #model_interior_identifier = bool;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {true},
            quote! {Copy, PartialEq, PartialOrd, Eq, Ord},
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        tokens
    }

    fn generate_integer_token_stream(&self, model_type_name: &str, _node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        tokens.append_all(quote! {
            pub type #model_interior_identifier = i64;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {true},
            quote! {Copy, PartialEq, PartialOrd, Eq, Ord},
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        tokens
    }

    fn generate_number_token_stream(&self, model_type_name: &str, _node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let mut validation_tokens = quote! {};

        validation_tokens.append_all(quote! {
            true
        });

        tokens.append_all(quote! {
            pub type #model_interior_identifier = f64;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {true},
            quote! {Copy, PartialEq, PartialOrd},
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        tokens
    }

    fn generate_string_token_stream(&self, model_type_name: &str, node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let mut validation_tokens = quote! {};

        let options = self.intermediate_data.select_string_options(node_id);
        if !options.is_empty() {
            let mut test_tokens = quote! {};
            for (index, option) in options.iter().enumerate() {
                if index > 0 {
                    test_tokens.append_all(quote! { && })
                }
                test_tokens.append_all(quote! {self.as_ref() != #option})
            }

            validation_tokens.append_all(quote! {
                if #test_tokens {
                    return false;
                }
            })
        }

        validation_tokens.append_all(quote! {
            true
        });

        tokens.append_all(quote! {
            pub type #model_interior_identifier = String;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            validation_tokens,
            quote! {PartialEq, PartialOrd, Eq, Ord},
            false,
        ));

        tokens.append_all(Self::generate_new_type_string_ref_token_stream(
            model_type_identifier,
        ));

        tokens
    }

    fn generate_tuple_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let mut tuple_tokens = quote! {};

        let item_type_node_ids = self
            .intermediate_data
            .select_tuple_item_type_node_ids(node_id)
            .into_iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id));

        for item_type_node_id in item_type_node_ids {
            let item_type_name = self.get_model_name(item_type_node_id)?;
            let item_type_identifier = format_ident!("r#{}", item_type_name);
            tuple_tokens.append_all(quote! {
                #item_type_identifier,
            });
        }

        tokens.append_all(quote! {
            pub type #model_interior_identifier = (#tuple_tokens);
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! { true },
            quote! { PartialEq },
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        Ok(tokens)
    }

    fn generate_array_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let item_type_node_id = self
            .intermediate_data
            .select_array_item_type_node_id(node_id)
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
            .ok_or("item type not set")?;
        let item_type_name = self.get_model_name(item_type_node_id)?;
        let item_type_identifier = format_ident!("r#{}", item_type_name);
        tokens.append_all(quote! {
            pub type #model_interior_identifier = Vec<#item_type_identifier>;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! { true},
            quote! { PartialEq },
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        Ok(tokens)
    }

    fn generate_object_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let mut property_tokens = quote! {};

        let property_type_node_ids = self
            .intermediate_data
            .select_object_property_type_node_ids(node_id)
            .into_iter()
            .map(|(property, node_id)| {
                (property, self.intermediate_data.select_non_empty(node_id))
            });

        let required_properties = self
            .intermediate_data
            .select_object_required_properties(node_id);

        for (property_name, property_type_node_id) in property_type_node_ids {
            let member_name = self.to_member_name(property_name);
            let member_identifier = format_ident!("r#{}", member_name);

            let property_type_name = self.get_model_name(property_type_node_id)?;
            let property_type_identifier = format_ident!("r#{}", property_type_name);

            if required_properties.contains(member_name.as_str()) {
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
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
            pub struct #model_interior_identifier {
                #property_tokens
            }
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! { true},
            quote! { PartialEq },
            true,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        Ok(tokens)
    }

    fn generate_record_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_interior_name = &format!("{}Interior", model_type_name);

        let model_type_identifier = &format_ident!("r#{}", model_type_name);
        let model_interior_identifier = &format_ident!("r#{}", model_interior_name);

        let property_type_node_id = self
            .intermediate_data
            .select_record_property_type_node_id(node_id)
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
            .ok_or("item type not set")?;
        let property_type_name = self.get_model_name(property_type_node_id)?;
        let property_type_identifier = format_ident!("r#{}", property_type_name);

        tokens.append_all( quote!{
            pub type #model_interior_identifier = std::collections::HashMap<String, #property_type_identifier>;
        });

        tokens.append_all(Self::generate_new_type_token_stream(
            model_type_name,
            model_interior_name,
            model_type_identifier,
            model_interior_identifier,
            quote! {true},
            quote! {PartialEq },
            false,
        ));

        tokens.append_all(Self::generate_new_type_ref_token_stream(
            model_type_identifier,
            model_interior_identifier,
        ));

        Ok(tokens)
    }

    fn generate_one_of_token_stream(
        &self,
        model_compound_name: &str,
        compound_node: &schemas::intermediate_a::OneOfCompound,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut enum_tokens = quote! {};

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);
            enum_tokens.append_all(quote! {
                #type_identifier(#type_identifier),
            });
        }

        tokens.append_all(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
            #[serde(untagged)]
            pub enum #model_compound_identifier {
                #enum_tokens
            }
        });

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            tokens.append_all(quote! {
                impl TryFrom<#model_compound_identifier> for #type_identifier {
                    type Error = ();

                    fn try_from(value: #model_compound_identifier) -> Result<Self, Self::Error> {
                        match value {
                            #model_compound_identifier::#type_identifier(value) => Ok(value),
                            _ => Err(()),
                        }
                    }
                }
            });
        }

        Ok(tokens)
    }

    fn generate_any_of_token_stream(
        &self,
        model_compound_name: &str,
        compound_node: &schemas::intermediate_a::AnyOfCompound,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};
        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut property_tokens = quote! {};

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
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
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
            pub struct #model_compound_identifier{
                #property_tokens
            }
        });

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            let member_name = self.to_member_name(&type_name);
            let member_identifier = format_ident!("r#{}", member_name);

            tokens.append_all(quote! {
                impl TryFrom<#model_compound_identifier> for #type_identifier {
                    type Error = ();

                    fn try_from(value: #model_compound_identifier) -> Result<Self, Self::Error> {
                        value.#member_identifier.ok_or(())
                    }
                }
            });
        }

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            let member_name = self.to_member_name(&type_name);
            let member_identifier = format_ident!("r#{}", member_name);

            tokens.append_all(quote! {
                impl AsRef<Option<#type_identifier>> for #model_compound_identifier {
                    fn as_ref(&self) -> &Option<#type_identifier> {
                        &self.#member_identifier
                    }
                }
            });
        }

        Ok(tokens)
    }

    fn generate_all_of_token_stream(
        &self,
        model_compound_name: &str,
        compound_node: &schemas::intermediate_a::AllOfCompound,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};
        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut property_tokens = quote! {};

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            let member_name = self.to_member_name(&type_name);
            let member_identifier = format_ident!("r#{}", member_name);

            property_tokens.append_all(quote! {
                #[serde(flatten)]
                #member_identifier: #type_identifier,
            });
        }

        tokens.append_all(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
            pub struct #model_compound_identifier{
                #property_tokens
            }
        });

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            let member_name = self.to_member_name(&type_name);
            let member_identifier = format_ident!("r#{}", member_name);

            tokens.append_all(quote! {
                impl From<#model_compound_identifier> for #type_identifier {
                    fn from(value: #model_compound_identifier) -> Self {
                        value.#member_identifier
                    }
                }
            });
        }

        for type_node_id in compound_node
            .type_node_ids
            .as_ref()
            .unwrap()
            .iter()
            .map(|node_id| self.intermediate_data.select_non_empty(node_id))
        {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);

            let member_name = self.to_member_name(&type_name);
            let member_identifier = format_ident!("r#{}", member_name);

            tokens.append_all(quote! {
                impl AsRef<#type_identifier> for #model_compound_identifier {
                    fn as_ref(&self) -> &#type_identifier {
                        &self.#member_identifier
                    }
                }
            });
        }

        Ok(tokens)
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_new_type_token_stream(
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

    fn generate_new_type_ref_token_stream(
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
            impl std::ops::Deref for #type_identifier {
                type Target = #interior_identifier;

                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        });

        tokens
    }

    fn generate_new_type_string_ref_token_stream(type_identifier: &Ident) -> TokenStream {
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

    fn get_model_name(&self, node_id: &str) -> Result<String, &'static str> {
        let model_name = self.get_name(node_id)?;
        let model_name = model_name.to_pascal_case();

        Ok(model_name.to_string())
    }

    fn get_model_type_name(
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

    fn get_model_compound_name(
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

    fn to_type_name(&self, type_enum: &TypeEnum) -> &'static str {
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

    fn to_compound_name(
        &self,
        node_compound: &schemas::intermediate_a::CompoundUnion,
    ) -> &'static str {
        match node_compound {
            schemas::intermediate_a::CompoundUnion::OneOfCompound(_) => "OneOf",
            schemas::intermediate_a::CompoundUnion::AnyOfCompound(_) => "AnyOf",
            schemas::intermediate_a::CompoundUnion::AllOfCompound(_) => "AllOf",
        }
    }

    fn to_member_name(&self, property_name: &str) -> String {
        property_name.to_snake_case()
    }
}
