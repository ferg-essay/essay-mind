use proc_macro::TokenStream;

mod component;
extern crate proc_macro;
extern crate syn;
extern crate quote;



#[proc_macro_derive(SystemParam, attributes(ticker_param))]
pub fn derive_ticker_param(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    component::derive_component(input)
}