extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod _common;
mod field_info;
mod r_builder;
mod r_get_set;

#[proc_macro_derive(RGetter, attributes(rgetter))]
#[proc_macro_error]
pub fn get_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_get_set::gen_get_set("rgetter", src))
}

#[proc_macro_derive(RSetter, attributes(rsetter))]
#[proc_macro_error]
pub fn set_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_get_set::gen_get_set("rsetter", src))
}

#[proc_macro_derive(RBuilder, attributes(rbuilder))]
#[proc_macro_error]
pub fn builder_fn(input: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::from(r_builder::gen(src))
}
