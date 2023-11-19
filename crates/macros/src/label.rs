use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

pub fn derive_striatum_label(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl essay_mind::vertebrate::striatum::StriatumLabel for #name {
            fn box_clone(&self) -> Box<dyn essay_mind::vertebrate::striatum::StriatumLabel> {
                Box::new(Clone::clone(self))
            }
        }

        impl AsRef<dyn essay_mind::vertebrate::striatum::StriatumLabel> for #name {
            fn as_ref(&self) -> &dyn essay_mind::vertebrate::striatum::StriatumLabel {
                self
            }
        }
    })
}

pub fn derive_peptide_label(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl vertebrate::mid_peptides::Peptide for #name {
            fn box_clone(&self) -> Box<dyn vertebrate::mid_peptides::Peptide> {
                Box::new(Clone::clone(self))
            }
        }

        impl AsRef<dyn vertebrate::mid_peptides::Peptide> for #name {
            fn as_ref(&self) -> &dyn vertebrate::mid_peptides::Peptide {
                self
            }
        }
    })
}
