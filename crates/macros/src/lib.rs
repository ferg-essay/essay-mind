mod label;
use proc_macro::TokenStream;

extern crate proc_macro;
extern crate syn;
extern crate quote;

#[proc_macro_derive(StriatumLabel, attributes(component))]
pub fn derive_striatum_label(input: TokenStream) -> TokenStream {
    label::derive_striatum_label(input)
}
