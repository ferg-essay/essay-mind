use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

pub fn derive_component(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl crate::mind_ecs::Component for #name {

        }
    })
}