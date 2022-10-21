use proc_macro_error::abort_call_site;
use syn::PathArguments::AngleBracketed;
use syn::{AngleBracketedGenericArguments, GenericArgument, Type, TypePath};

/// result: (ty,is_option)
pub fn get_type_smart(tp: &Type) -> (Type, bool) {
    if self::is_opt_type(tp) {
        return (self::get_type_of_opt_inner(tp).unwrap(), true);
    }
    (tp.clone(), false)
}

pub fn get_type_of_opt_inner(tp: &Type) -> Option<Type> {
    match tp.clone() {
        Type::Path(TypePath { path, .. }) => {
            let opt = path.segments.first()?.clone();
            if opt.ident != "Option" {
                abort_call_site!("#[derive(RGet)] is only defined for structs, not for enums!");
            }

            match opt.arguments {
                AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                    match args.first()?.to_owned() {
                        GenericArgument::Type(tp) => Some(tp),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// if type is same as : Option<i64>,Option<String> .etc
pub fn is_opt_type(tp: &Type) -> bool {
    match tp {
        Type::Path(TypePath { path, .. }) => match path.segments.first() {
            Some(v) => v.ident == "Option",
            _ => false,
        },
        _ => false,
    }
}
