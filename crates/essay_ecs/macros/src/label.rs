use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

pub fn derive_schedule_label(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl crate::schedule::ScheduleLabel for #name {
            fn box_clone(&self) -> Box<dyn crate::schedule::ScheduleLabel> {
                Box::new(Clone::clone(self))
            }
        }
    })
}

pub fn derive_phase(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl crate::schedule::Phase for #name {
            fn box_clone(&self) -> Box<dyn crate::schedule::Phase> {
                Box::new(Clone::clone(self))
            }
        }
    })
}
