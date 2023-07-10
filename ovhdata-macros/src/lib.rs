extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintObjectCompletely)]
pub fn ensure_secret(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name of the structure
    let struct_name = &ast.ident;

    // Generate the implementation of the `EnsureSecret` trait
    let expanded = quote! {
        impl EnsureSecret<#struct_name> for #struct_name {
            fn hide_secrets(&self) -> #struct_name {
                self.clone()
            }
        }
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(expanded)
}
