use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::Field;
#[allow(unused_imports)]
use syn::{Data, DataStruct, Fields, Path, Token, TypePath, Visibility};

use crate::field_info::FieldInfo;

pub fn gen(input: syn::DeriveInput) -> TokenStream2 {
    let src = input.clone();
    let vis = src.vis;
    let (g_impl, g_ty, g_where) = src.generics.split_for_impl();

    let name = src.ident;
    let builder_name = format_ident!("{}Builder", name);
    let builder_name = quote! {#builder_name};

    let fields_fn = self::get_fields(&src.data);

    let ast = quote! {
         // builder strust
        #[allow(dead_code)]
         #vis struct #builder_name #g_ty #g_where{
            inner:#name #g_ty,
         }

        // builder impl
        #[allow(dead_code)]
        impl #g_impl #builder_name #g_ty #g_where {

            #fields_fn

            #[allow(dead_code)]
            pub fn build(&self)->#name #g_ty{
                self.inner.clone()
            }
        }

        // origin struct add fn builder()
        #[allow(dead_code)]
        impl #g_impl #name #g_ty #g_where {
            #[allow(dead_code)]
            #vis fn builder()->#builder_name #g_ty{
                #builder_name{
                    inner: #name::default(),
                }
            }
        }
    };

    // let ast =
    // quote! {
    //     pub fn trace(){
    //        println!("-----------{}-----------",stringify!(#ast));
    //     }
    // }

    ast
}

fn get_fields(data: &Data) -> TokenStream2 {
    let action = "RBuilder";
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields,
        _ => abort_call_site!("#[derive(RGet)] is only defined for structs, not for enums!"),
    };

    let lines = fields.named.iter().map(|f| self::gen_fn(action, f));

    quote! {
       #( #lines )*
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

    // gen rset
    let mut v = quote! {v};
    if is_opt {
        v = quote! {Some(v)};
    }

    quote! {
      #[allow(dead_code)]
      #vis fn #name (&mut self,v:#ty)->&mut Self{
        self.inner.#name = #v;
        self
      }
    }
}
