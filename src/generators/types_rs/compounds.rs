use super::ModelsRsGenerator;
use crate::{schemas, selectors::document::DocumentSelectors};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

impl<'a> ModelsRsGenerator<'a> {
    pub(super) fn generate_one_of_token_stream(
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

    pub(super) fn generate_any_of_token_stream(
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

    pub(super) fn generate_all_of_token_stream(
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
}
