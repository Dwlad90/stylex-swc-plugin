use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Ident, Lit, MemberProp};

use crate::shared::{
    structures::{flat_compiled_styles::FlatCompiledStylesValue, state_manager::StateManager},
    utils::common::get_string_val_from_lit,
};

#[derive(Debug, Clone)]
pub(crate) enum StyleObject {
    Style(IndexMap<String, FlatCompiledStylesValue>),
    Nullable,
    Other,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedArg {
    StyleObject(StyleObject, Ident),
    ConditionalStyle((Expr, StyleObject, StyleObject, Ident)),
}

pub(crate) fn parse_nullable_style(node: &Expr, state: &StateManager) -> StyleObject {
    match node {
        Expr::Lit(lit) => match lit {
            Lit::Null(_) => StyleObject::Nullable,
            _ => StyleObject::Other,
        },
        Expr::Ident(ident) => {
            if ident.sym == "undefined" {
                StyleObject::Nullable
            } else {
                StyleObject::Other
            }
        }
        Expr::Member(member) => {
            let mut obj_name: Option<String> = Option::None;
            let mut prop_name: Option<String> = Option::None;

            if let Some(obj_ident) = member.obj.as_ident() {
                if state
                    .style_map
                    .contains_key(&obj_ident.sym.as_str().to_string())
                {
                    match &member.prop {
                        MemberProp::Ident(prop_ident) => {
                            obj_name = Option::Some(obj_ident.clone().sym.as_str().to_string());
                            prop_name = Option::Some(prop_ident.clone().sym.as_str().to_string());
                        }
                        MemberProp::Computed(computed) => {
                            if let Some(lit) = computed.expr.as_lit() {
                                obj_name = Option::Some(obj_ident.clone().sym.as_str().to_string());
                                prop_name = Option::Some(get_string_val_from_lit(lit));
                            }
                        }
                        MemberProp::PrivateName(_) => {}
                    }
                }
            }

            if let Some(obj_name) = obj_name {
                if let Some(prop_name) = prop_name {
                    let style = state.style_map.get(&obj_name);

                    if let Some(style) = style {
                        let style_value = style.get(&prop_name);

                        if let Some(style_value) = style_value {
                            return StyleObject::Style(style_value.clone());
                        }
                    }
                }
            }
            StyleObject::Other
        }
        _ => StyleObject::Other,
    }
}
