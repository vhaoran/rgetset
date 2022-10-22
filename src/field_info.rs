use crate::_common::{get_type_smart, is_attr_sub_type};
use syn::{Field, Type, Visibility};

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
    is_attr_sub_type(&field.attrs, action, vec!["skip"])
}
