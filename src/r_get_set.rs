use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::Field;
#[allow(unused_imports)]
use syn::{Data, DataStruct, Fields, Path, Token, TypePath, Visibility};

use crate::field_info::FieldInfo;

pub fn gen_get_set(action: &str, input: syn::DeriveInput) -> TokenStream2 {
    let src = input.clone();

    let name = src.ident;
    let (g_impl, g_ty, g_where) = src.generics.split_for_impl();
    let fields_fn = self::get_fields(action, &src.data);

    quote! {
         impl #g_impl #name #g_ty #g_where {
            #fields_fn
        }
    }
}

fn get_fields(action: &str, data: &Data) -> TokenStream2 {
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields,
        _ => abort_call_site!("#[derive(RGetter)] is only defined for structs, not for enums!"),
    };

    let recurse = fields.named.iter().map(|f| self::gen_fn(action, f));

    quote! {
       #( #recurse )*
    }
}

fn gen_fn(action: &str, src: &Field) -> TokenStream2 {
    let info = FieldInfo::parse(action, src);
    let name = info.data.ident;
    let vis = info.data.vis;
    let ty = info.data.ty;
    let is_opt = info.data.is_opt;
    let skip = info.data.skip;
    if skip {
        return quote!();
    }

    //

    //

    match action {
        "rgetter" => {
            let mut suffix = quote!();
            if is_opt {
                suffix = quote! {.unwrap_or(Default::default())}
            };

            quote! {
                #[allow(dead_code)]
                 #vis fn #name(&self)->#ty{
                     self.#name.clone() #suffix
                 }
            }
        }
        "rsetter" => {
            let mut v = quote! {v};
            if is_opt {
                v = quote! {Some(v)};
            }

            let fn_name = format_ident!("set_{}", name.clone().unwrap());
            let fn_name = quote! {#fn_name};

            quote! {
              #[allow(dead_code)]
              #vis fn #fn_name (&mut self,v:#ty)->&mut Self{
                self.#name = #v;
                self
              }
            }
        }
        _ => {
            quote!()
        }
    }
    // gen rset
}
