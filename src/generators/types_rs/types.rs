use super::ModelsRsGenerator;
use crate::selectors::document::DocumentSelectors;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

impl<'a> ModelsRsGenerator<'a> {
    pub(super) fn generate_never_token_stream(&self, model_type_name: &str) -> TokenStream {
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

    pub(super) fn generate_any_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = &format_ident!("r#{}", model_type_name);

        tokens.append_all(quote! {
            pub type #model_type_identifier = serde_json::Value;
        });

        tokens
    }

    pub(super) fn generate_null_token_stream(&self, model_type_name: &str) -> TokenStream {
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

    pub(super) fn generate_boolean_token_stream(
        &self,
        model_type_name: &str,
        _node_id: &str,
    ) -> TokenStream {
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

    pub(super) fn generate_integer_token_stream(
        &self,
        model_type_name: &str,
        _node_id: &str,
    ) -> TokenStream {
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

    pub(super) fn generate_number_token_stream(
        &self,
        model_type_name: &str,
        _node_id: &str,
    ) -> TokenStream {
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

    pub(super) fn generate_string_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> TokenStream {
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

    pub(super) fn generate_tuple_token_stream(
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

    pub(super) fn generate_array_token_stream(
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

    pub(super) fn generate_object_token_stream(
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

    pub(super) fn generate_record_token_stream(
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
}
