use crate::_common::get_type_smart;
use syn::{Attribute, Field, Meta, NestedMeta, Type, Visibility};

#[derive(Clone, Debug)]
pub struct FieldInfo {
    pub action: String,
    pub data: Info,
}

impl FieldInfo {
    pub fn parse(action: &str, src: &Field) -> Self {
        let info = Info::parse(action, src);
        Self {
            action: action.to_string(),
            data: info,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub vis: Visibility,
    pub ident: Option<proc_macro2::Ident>,
    pub ty: Type,
    pub is_opt: bool,
    pub skip: bool,
}

impl Info {
    pub fn parse(action: &str, src: &Field) -> Self {
        let (tp, is_opt) = get_type_smart(&src.ty);

        let mut r = Self {
            vis: src.vis.clone(),
            ident: src.ident.clone(),
            ty: tp,
            is_opt,
            skip: false,
        };
        r.skip = self::is_skip(action, src);
        r
    }
} //impl

fn is_skip(action: &str, field: &Field) -> bool {
    field
        .attrs
        .iter()
        .map(|f: &Attribute| {
            match f.parse_meta() {
                Ok(Meta::List(l)) => {
                    if l.path.is_ident(action) {
                        l.nested
                            .iter()
                            .map(|x: &NestedMeta| match x {
                                NestedMeta::Meta(Meta::Path(p)) => p.is_ident("skip"),
                                _ => false,
                            })
                            .filter(|&b| b)
                            .count()
                            > 0
                    } else {
                        false
                    }
                }
                _ => false,
            } //match
        })
        .filter(|&x| x)
        .count()
        > 0_usize
}
