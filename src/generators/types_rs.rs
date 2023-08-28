use crate::selectors::{intermediate_a, TypeEnum};
use crate::{schemas, selectors::Selectors};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::collections::{HashMap, HashSet};

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
            #[derive(Debug)]
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

        for (node_id, node) in &self.intermediate_data.nodes {
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

        let types = self.intermediate_data.select_types(node_id);

        if let Some(super_node_id) = &node.super_node_id {
            let super_model_name = self.get_model_name(super_node_id)?;
            let super_model_identifier = format_ident!("r#{}", super_model_name);
            tokens.append_all(quote! {
                pub type #model_identifier = #super_model_identifier;
            });
        } else {
            if types.len() + node.compounds.len() == 1 {
                for type_enum in &types {
                    let model_type_name = self.get_model_type_name(node_id, type_enum)?;
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

                for node_type in &types {
                    let type_name = self.to_type_name(node_type);
                    let type_identifier = format_ident!("r#{}", type_name);
                    let model_type_name = self.get_model_type_name(node_id, node_type)?;
                    let model_type_identifier = format_ident!("r#{}", model_type_name);
                    enum_tokens.append_all(quote! {
                        #type_identifier(#model_type_identifier),
                    });
                }

                tokens.append_all(quote! {
                    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
                    #[serde(untagged)]
                    pub enum #model_identifier {
                        #enum_tokens
                    }
                });
            }

            for type_enum in &types {
                let model_type_name = self.get_model_type_name(node_id, type_enum)?;

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
                        tokens.append_all(
                            self.generate_boolean_token_stream(&model_type_name, &node_id),
                        );
                    }
                    // integer
                    TypeEnum::Integer => {
                        tokens.append_all(
                            self.generate_number_token_stream(&model_type_name, &node_id),
                        );
                    }
                    // number
                    TypeEnum::Number => {
                        tokens.append_all(
                            self.generate_number_token_stream(&model_type_name, &node_id),
                        );
                    }
                    // string
                    TypeEnum::String => {
                        tokens.append_all(
                            self.generate_string_token_stream(&model_type_name, &node_id),
                        );
                    }
                    // tuple
                    TypeEnum::Tuple => {
                        tokens.append_all(
                            self.generate_tuple_token_stream(&model_type_name, &node_id)?,
                        );
                    }
                    // array
                    TypeEnum::Array => {
                        tokens.append_all(
                            self.generate_array_token_stream(&model_type_name, &node_id)?,
                        );
                    }
                    // object
                    TypeEnum::Object => {
                        tokens.append_all(
                            self.generate_interface_token_stream(&model_type_name, &node_id)?,
                        );
                    }
                    // record
                    TypeEnum::Record => {
                        tokens.append_all(
                            self.generate_record_token_stream(&model_type_name, &node_id)?,
                        );
                    }
                }
            }

            for node_compound in &node.compounds {
                let model_compound_name = self.get_model_compound_name(node_id, node_compound)?;

                match node_compound {
                    // one-of
                    schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf0(compound_node) => {
                        tokens.append_all(
                            self.generate_one_of_token_stream(&model_compound_name, compound_node)?,
                        );
                    }
                    // any-of
                    schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf1(compound_node) => {
                        tokens.append_all(
                            self.generate_any_of_token_stream(&model_compound_name, compound_node)?,
                        );
                    }
                    // all-of
                    schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf2(compound_node) => {
                        tokens.append_all(
                            self.generate_all_of_token_stream(&model_compound_name, compound_node)?,
                        );
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn generate_null_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        tokens.append_all(quote! {
            pub type #model_type_identifier = ();
        });

        tokens
    }

    fn generate_any_token_stream(&self, model_type_name: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        tokens.append_all(quote! {
            pub type #model_type_identifier = serde_json::Value;
        });

        tokens
    }

    fn generate_never_token_stream(&self, _model_type_name: &str) -> TokenStream {
        todo!()
    }

    fn generate_boolean_token_stream(&self, model_type_name: &str, node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        tokens.append_all(quote!{
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[serde(try_from = "bool")]
            pub struct #model_type_identifier(bool);
        });

        tokens.append_all(quote! {
            impl #model_type_identifier {
                fn new(value: bool) -> Result<Self, ValidationError> {
                    let instance = Self(value);
                    if instance.validate() {
                        Ok(instance)
                    } else {
                        Err(ValidationError::new(#model_type_name))
                    }
                }

                pub fn validate(&self) -> bool {
                    true
                }
            }
        });

        tokens.append_all(quote! {
            impl TryFrom<bool> for #model_type_identifier {
                type Error = ValidationError;

                fn try_from(value: bool) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        });

        tokens.append_all(quote! {
            impl From<#model_type_identifier> for bool {
                fn from(value: #model_type_identifier) -> Self {
                    value.0
                }
            }
        });

        tokens.append_all(quote! {
            impl AsRef<bool> for #model_type_identifier {
                fn as_ref(&self) -> &bool {
                    &self.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #model_type_identifier {
                type Target = bool;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        });

        tokens
    }

    fn generate_number_token_stream(&self, model_type_name: &str, node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        tokens.append_all(quote!{
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[serde(try_from = "usize")]
            pub struct #model_type_identifier(usize);
        });

        tokens.append_all(quote! {
            impl #model_type_identifier {
                fn new(value: usize) -> Result<Self, ValidationError> {
                    let instance = Self(value);
                    if instance.validate() {
                        Ok(instance)
                    } else {
                        Err(ValidationError::new(#model_type_name))
                    }
                }

                pub fn validate(&self) -> bool {
                    if self.0 == 0 {
                        return false;
                    }

                    true
                }
            }
        });

        tokens.append_all(quote! {
            impl TryFrom<usize> for #model_type_identifier {
                type Error = ValidationError;

                fn try_from(value: usize) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        });

        tokens.append_all(quote! {
            impl From<#model_type_identifier> for usize {
                fn from(value: #model_type_identifier) -> Self {
                    value.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::str::FromStr for #model_type_identifier {
                type Err = ValidationError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let value = s
                        .parse()
                        .map_err(|_error| ValidationError::new(#model_type_name))?;
                    Self::new(value)
                }
            }
        });

        tokens.append_all(quote! {
            impl ToString for #model_type_identifier {
                fn to_string(&self) -> String {
                    self.0.to_string()
                }
            }
        });

        tokens.append_all(quote! {
            impl AsRef<usize> for #model_type_identifier {
                fn as_ref(&self) -> &usize {
                    &self.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #model_type_identifier {
                type Target = usize;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        });

        tokens
    }

    fn generate_string_token_stream(&self, model_type_name: &str, node_id: &str) -> TokenStream {
        let mut tokens = quote! {};

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

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        tokens.append_all(quote! {
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[serde(try_from = "String")]
            pub struct #model_type_identifier(String);
        });

        tokens.append_all(quote! {

            impl #model_type_identifier {
                fn new(value: String) -> Result<Self, ValidationError> {
                    let instance = Self(value);
                    if instance.validate() {
                        Ok(instance)
                    } else {
                        Err(ValidationError::new(#model_type_name))
                    }
                }

                fn validate(&self) -> bool {
                    #validation_tokens

                    true
                }
            }

        });

        tokens.append_all(quote! {
            impl TryFrom<String> for #model_type_identifier {
                type Error = ValidationError;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        });

        tokens.append_all(quote! {
            impl From<#model_type_identifier> for String {
                fn from(value: #model_type_identifier) -> Self {
                    value.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::str::FromStr for #model_type_identifier {
                type Err = ValidationError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new(s.to_string())
                }
            }
        });

        tokens.append_all(quote! {
            impl ToString for #model_type_identifier {
                fn to_string(&self) -> String {
                    self.0.to_string()
                }
            }
        });

        tokens.append_all(quote! {
            impl AsRef<str> for #model_type_identifier {
                fn as_ref(&self) -> &str {
                    self.0.as_str()
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #model_type_identifier {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    self.0.as_str()
                }
            }
        });

        tokens
    }

    fn generate_tuple_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        let mut tuple_tokens = quote! {};

        for item_type_node_id in type_node.item_type_node_ids.as_ref().unwrap() {
            let item_type_name = self.get_model_name(item_type_node_id)?;
            let item_type_identifier = format_ident!("r#{}", item_type_name);
            tuple_tokens.append_all(quote! {
                #item_type_identifier,
            });
        }

        tokens.append_all(quote! {
            pub type #model_type_identifier = (#tuple_tokens);
        });

        Ok(tokens)
    }

    fn generate_array_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_type_identifier = format_ident!("r#{}", model_type_name);

        let item_type_name = self.get_model_name(type_node.item_type_node_id.as_ref().unwrap())?;
        let item_type_identifier = format_ident!("r#{}", item_type_name);
        tokens.append_all(quote! {
            pub type #model_type_identifier = Vec<#item_type_identifier>;
        });

        Ok(tokens)
    }

    fn generate_interface_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_interior_name = format!("{}Interior", model_type_name);

        let model_type_identifier = format_ident!("r#{}", model_type_name);
        let model_interior_identifier = format_ident!("r#{}", model_interior_name);

        let mut property_tokens = quote! {};

        for (property_name, property_type_node_id) in
            type_node.property_type_node_ids.as_ref().unwrap()
        {
            let member_name = self.to_member_name(property_name);
            let member_identifier = format_ident!("r#{}", member_name);

            let property_type_name = self.get_model_name(property_type_node_id)?;
            let property_type_identifier = format_ident!("r#{}", property_type_name);

            let required_properties: HashSet<_> = type_node
                .required_properties
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.as_ref())
                .collect();

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
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
            pub struct #model_interior_identifier {
                #property_tokens
            }
        });

        tokens.append_all(quote! {
            #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
            #[serde(try_from = #model_interior_name)]
            pub struct #model_type_identifier(Box<#model_interior_identifier>);
        });

        tokens.append_all(quote! {
            impl #model_type_identifier {
                fn new(value: #model_interior_identifier) -> Result<Self, ValidationError> {
                    let instance = Self(Box::new(value));
                    if instance.validate() {
                        Ok(instance)
                    } else {
                        Err(ValidationError::new(#model_type_name))
                    }
                }

                pub fn validate(&self) -> bool {
                    true
                }
            }
        });

        tokens.append_all(quote! {
            impl TryFrom<#model_interior_identifier> for #model_type_identifier {
                type Error = ValidationError;

                fn try_from(value: #model_interior_identifier) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        });

        tokens.append_all(quote! {
            impl From<#model_type_identifier> for #model_interior_identifier {
                fn from(value: #model_type_identifier) -> Self {
                    *value.0
                }
            }
        });

        tokens.append_all(quote! {
            impl AsRef<#model_interior_identifier> for #model_type_identifier {
                fn as_ref(&self) -> &#model_interior_identifier {
                    &self.0
                }
            }
        });

        tokens.append_all(quote! {
            impl std::ops::Deref for #model_type_identifier {
                type Target = #model_interior_identifier;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        });

        Ok(tokens)
    }

    fn generate_record_token_stream(
        &self,
        model_type_name: &str,
        node_id: &str,
    ) -> Result<TokenStream, &'static str> {
        let model_type_identifier = format_ident!("r#{}", model_type_name);

        let property_type_name =
            self.get_model_name(type_node.property_type_node_id.as_ref().unwrap())?;
        let property_type_identifier = format_ident!("r#{}", property_type_name);
        let tokens = quote! {
            pub type #model_type_identifier = std::collections::HashMap<String, #property_type_identifier>;
        };

        Ok(tokens)
    }

    fn generate_one_of_token_stream(
        &self,
        model_compound_name: &str,
        compound_node: &schemas::intermediate_a::OneOfCompoundInterface,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut enum_tokens = quote! {};

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
            let type_name = self.get_model_name(type_node_id)?;
            let type_identifier = format_ident!("r#{}", type_name);
            enum_tokens.append_all(quote! {
                #type_identifier(#type_identifier),
            });
        }

        tokens.append_all(quote! {
            #[derive(serde::Serialize, serde::Deserialize,Clone, Debug, PartialEq, Eq)]
            #[serde(untagged)]
            pub enum #model_compound_identifier {
                #enum_tokens
            }
        });

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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
        compound_node: &schemas::intermediate_a::AnyOfCompoundInterface,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};
        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut property_tokens = quote! {};

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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
            #[derive(serde::Serialize, serde::Deserialize,Clone, Debug, PartialEq, Eq)]
            pub struct #model_compound_identifier{
                #property_tokens
            }
        });

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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
        compound_node: &schemas::intermediate_a::AllOfCompoundInterface,
    ) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};
        let model_compound_identifier = format_ident!("r#{}", model_compound_name);

        let mut property_tokens = quote! {};

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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
            #[derive(serde::Serialize, serde::Deserialize,Clone, Debug, PartialEq, Eq)]
            pub struct #model_compound_identifier{
                #property_tokens
            }
        });

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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

        for type_node_id in compound_node.type_node_ids.as_ref().unwrap() {
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
            schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf0(_) => "OneOf",
            schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf1(_) => "AnyOf",
            schemas::intermediate_a::CompoundUnion::CompoundUnionOneOf2(_) => "AllOf",
        }
    }

    fn to_member_name(&self, property_name: &str) -> String {
        property_name.to_snake_case()
    }
}
