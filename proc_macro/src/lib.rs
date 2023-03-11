use proc_macro::{Span, TokenStream};
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(ModelAttribute)]
pub fn derive_builder(input: TokenStream) -> TokenStream {

    let derive_input = parse_macro_input!(input as DeriveInput );

    derive_input.

    proc_macro::TokenStream::from(proc_macro2::TokenStream::from(input))
}