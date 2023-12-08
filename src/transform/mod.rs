use std::path::PathBuf;

use indexmap::IndexMap;
use swc_core::{
    common::{comments::Comments, FileName},
    ecma::{
        ast::{CallExpr, Callee, Expr, Id, MemberProp, Prop, Ident},
        utils::member_expr,
    },
};

use crate::shared::{enums::ModuleCycle, utils::extract_filename_from_path};

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
    props_declaration: Option<Id>
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
        }
    }

    pub fn process_declaration(&mut self, expr: &CallExpr) -> Option<Id> {
        match &mut expr.callee.clone() {
            Callee::Expr(callee) => match callee.as_ref() {
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        let ident_id = &ident.to_id();

                        if self.declaration.is_some() {
                            if self.declaration.clone().unwrap().eq(&ident_id) {
                                return Option::Some(ident_id.clone());
                            }
                        }

                        Option::None
                    }
                    _ => Option::None,
                },
                // Expr::Ident(ident) => {
                //     let ident_id = &ident.to_id();

                //     println!("process_declaration, ident_id: {:#?}", ident_id);
                //     println!(
                //         "process_declaration, self.declaration: {:#?}",
                //         self.declaration
                //     );
                //     if self.declaration.is_some() {
                //         if self.declaration.clone().unwrap().eq(&ident_id) {
                //             return Option::Some(ident_id.clone());
                //         }
                //     }

                //     Option::None
                // }
                _ => Option::None,
            },
            _ => Option::None,
        }
    }


}
