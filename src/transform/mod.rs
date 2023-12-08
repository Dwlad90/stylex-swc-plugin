use std::path::PathBuf;

use indexmap::IndexMap;
use swc_core::{
    common::{
        comments::{Comment, CommentKind, Comments},
        FileName,
    },
    ecma::{
        ast::{CallExpr, Callee, Expr, Id, Ident, MemberProp, Module, Prop},
        utils::member_expr,
    },
};

use crate::shared::{enums::ModuleCycle, structures::MetaData, utils::extract_filename_from_path};

mod css_map;
mod fold;

pub struct ModuleTransformVisitor<C>
where
    C: Comments,
{
    comments: C,
    pub declaration: Option<Id>,
    pub cycle: ModuleCycle,
    pub package_name: String,
    pub class_name_map: IndexMap<Prop, Prop>,
    file_name: String,
    props_declaration: Option<Id>,
    pub css_output: Vec<MetaData>,
}

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub fn new(comments: C, file_name: FileName) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Processing,
            package_name: "@stylexjs/stylex".to_string(),
            class_name_map: IndexMap::new(),
            file_name: extract_filename_from_path(file_name),
            props_declaration: Option::None,
            css_output: vec![],
        }
    }

    pub fn new_test(comments: C) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Processing,
            package_name: "@stylexjs/stylex".to_string(),
            class_name_map: IndexMap::new(),
            file_name: extract_filename_from_path(FileName::Real(PathBuf::from("app/page.tsx"))),
            props_declaration: Option::None,
            css_output: vec![],
        }
    }

    pub fn process_declaration(&mut self, expr: &CallExpr) -> Option<(Id, String)> {
        match &mut expr.callee.clone() {
            Callee::Expr(callee) => match callee.as_ref() {
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        let ident_id = &ident.to_id();

                        if self.declaration.is_some() {
                            if self.declaration.clone().unwrap().eq(&ident_id) {
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
