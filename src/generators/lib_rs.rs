use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub fn generate_file_token_stream() -> Result<TokenStream, &'static str> {
    let mut tokens = quote! {};

    tokens.append_all(quote! {
        pub mod models;
        pub mod validators;
    });

    Ok(tokens)
}
