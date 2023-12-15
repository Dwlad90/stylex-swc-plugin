use std::path::PathBuf;

use swc_core::{
    common::{comments::Comments, FileName},
    ecma::ast::{CallExpr, Callee, Expr, Id, MemberProp},
};

use crate::{
    shared::{enums::ModuleCycle, structures::MetaData, utils::extract_filename_from_path},
    StylexConfig, StylexConfigParams,
};

mod css_map;
mod fold;

pub struct ModuleTransformVisitor<C>
where
    C: Comments,
{
    comments: C,
    declaration: Option<Id>,
    cycle: ModuleCycle,
    package_name: String,
    file_name: String,
    props_declaration: Option<Id>,
    css_output: Vec<MetaData>,
    config: StylexConfig,
}

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn new(comments: C, file_name: FileName, config: StylexConfigParams) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(file_name),
            props_declaration: Option::None,
            css_output: vec![],
            config: config.into(),
        }
    }

    pub fn new_test_classname(comments: C, config: Option<StylexConfigParams>) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(FileName::Real(PathBuf::from("app/page.tsx"))),
            props_declaration: Option::None,
            css_output: vec![],
            config: config.unwrap_or(StylexConfigParams::default()).into(),
        }
    }
    pub fn new_test_styles(comments: C, config: Option<StylexConfigParams>) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(FileName::Real(PathBuf::from("app/page.tsx"))),
            props_declaration: Option::None,
            css_output: vec![],
            config: match &config {
                Some(config) => config.clone().into(),
                None => {
                    let mut config = StylexConfig::default();

                    config.runtime_injection = true;

                    config
                }
            },
        }
    }

    pub(crate) fn process_declaration(&mut self, call_expr: &CallExpr) -> Option<(Id, String)> {
        match &mut call_expr.callee.clone() {
            Callee::Expr(callee) => match callee.as_ref() {
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        let ident_id = ident.to_id();

                        if self.declaration.clone().unwrap_or_default().eq(&ident_id) {
                            match member.prop.clone() {
                                MemberProp::Ident(ident) => {
                                    return Option::Some((
                                        ident_id.clone(),
                                        format!("{}", ident.sym),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        Option::None
    }
}
