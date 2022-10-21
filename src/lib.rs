extern crate proc_macro;

use proc_macro::TokenStream;
mod _common;
mod field_info;
mod r_builder;
mod r_get_set;

#[proc_macro_derive(RGet, attributes(skip))]
pub fn get_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_get_set::gen_get_set("RGet", src))
}

#[proc_macro_derive(RSet, attributes(skip))]
pub fn set_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_get_set::gen_get_set("RSet", src))
}

#[proc_macro_derive(RBuilder, attributes(skip))]
pub fn builder_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_builder::gen(src))
}
