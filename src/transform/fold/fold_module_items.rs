use swc_core::{
    common::{comments::Comments, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Decl, Expr, ExprStmt, Ident, ImportDecl, ImportDefaultSpecifier,
            ImportSpecifier, MemberExpr, MemberProp, ModuleDecl, ModuleItem, Stmt, Str,
        },
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        consts::DEFAULT_INJECT_PATH,
        enums::{InjectedStylesDeclarationType, ModuleCycle},
        structures::MetaData,
        utils::{
            expr_or_spread_number_expression_creator, expr_or_spread_string_expression_creator,
            get_pat_as_string,
        },
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        match self.cycle {
            ModuleCycle::Skip => return module_items,
            ModuleCycle::Initializing => {
                let module_items = module_items.fold_children_with(self);

                if self.declaration.is_none() {
                    self.cycle = ModuleCycle::Skip;

                    return module_items;
                }

                // module_items.retain(|module_item| {
                //     if let ModuleItem::ModuleDecl(import_decl) = module_item {
                //         if let ModuleDecl::Import(import) = import_decl {
                //             let src_value = import.src.value.to_string();

                //             if self.package_name.as_str() != &src_value {
                //                 return true;
                //             }

                //             let specifier = import.specifiers.first();

                //             if let Some(spec) = specifier {
                //                 match spec {
                //                     ImportSpecifier::Namespace(_) => {
                //                         return false;
                //                     }
                //                     ImportSpecifier::Named(_) => {
                //                         return true;
                //                     }
                //                     ImportSpecifier::Default(_) => {
                //                         return false;
                //                     }
                //                 }
                //             }
                //         }
                //     }

                //     true
                // });

                // TODO: Inject comment to css extraction

                module_items
            }
            ModuleCycle::Processing => module_items.fold_children_with(self),
            ModuleCycle::InjectClassName => module_items.fold_children_with(self),
            ModuleCycle::InjectStyles => {
                let mut styles_item_idx = 0;
                let mut styles_item_target_idx: Option<usize> = Option::None;

                let mut injected_styles_export: Option<InjectedStylesDeclarationType> =
                    Option::None;

                for module_item in module_items.clone().into_iter() {
                    match &module_item {
                        ModuleItem::ModuleDecl(import_decl) => match &import_decl {
                            ModuleDecl::ExportDecl(export_decl) => match &export_decl.decl {
                                Decl::Var(var_decl) => {
                                    for var_declarator in &var_decl.decls {
                                        let decl_name = self.get_props_desclaration_as_string();
                                        let var_declarator_name =
                                            get_pat_as_string(&var_declarator.name);

                                        if decl_name.eq(&var_declarator_name) {
                                            styles_item_target_idx = Option::Some(styles_item_idx);
                                            injected_styles_export = Option::Some(InjectedStylesDeclarationType::NamedDeclarationExport);
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            },
                            ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                                match export_default_expr.expr.as_ref() {
                                    Expr::Object(_) => {
                                        if self.props_declaration.is_none() {
                                            styles_item_target_idx = Option::Some(styles_item_idx);
                                            injected_styles_export = Option::Some(
                                            InjectedStylesDeclarationType::NamedPropertyOrDefaultExport,
                                        );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        ModuleItem::Stmt(stmt) => match &stmt {
                            Stmt::Decl(expr) => match expr {
                                Decl::Var(decl_var) => {
                                    for decl in &decl_var.decls {
                                        let decl_name = self.get_props_desclaration_as_string();
                                        let var_declarator_name = get_pat_as_string(&decl.name);

                                        if decl_name.eq(&var_declarator_name) {
                                            styles_item_target_idx = Option::Some(styles_item_idx);
                                            injected_styles_export = Option::Some(
                                                InjectedStylesDeclarationType::NamedPropertyOrDefaultExport,
                                            );
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                    }

                    styles_item_idx += 1;
                }

                let module_items: Vec<ModuleItem> = match styles_item_target_idx {
                    Some(styles_item_target_idx) => {
                        let module_item_start_slice = &module_items[0..styles_item_target_idx];
                        let module_items_end_slice = &module_items[styles_item_target_idx..];
                        let mut module_items_middle_vec: Vec<ModuleItem> = vec![];
                        let mut module_item_pre_start_vec: Vec<ModuleItem> = vec![];

                        for MetaData(_, styles, priority) in &self.css_output {
                            let stylex_inject_args = vec![
                                expr_or_spread_string_expression_creator(styles.ltr.clone()),
                                expr_or_spread_number_expression_creator(priority.clone().into()),
                            ];

                            let stylex = Expr::Ident(Ident::new("stylex".into(), DUMMY_SP));
                            let inject_src = "inject";

                            match &injected_styles_export {
                                Some(injected_styles_export) => match &injected_styles_export {
                                    InjectedStylesDeclarationType::NamedDeclarationExport => {
                                        let stylex_member = Expr::Member(MemberExpr {
                                            span: DUMMY_SP,
                                            obj: Box::new(stylex),
                                            prop: MemberProp::Ident(Ident {
                                                optional: false,
                                                span: DUMMY_SP,
                                                sym: inject_src.into(),
                                            }),
                                        });

                                        let stylex_call_expr = CallExpr {
                                            span: DUMMY_SP,
                                            type_args: Option::None,
                                            callee: Callee::Expr(Box::new(stylex_member.clone())),
                                            args: stylex_inject_args,
                                        };

                                        let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                                            span: DUMMY_SP,
                                            expr: Box::new(Expr::Call(stylex_call_expr)),
                                        }));

                                        module_items_middle_vec.push(module);
                                    }
                                    InjectedStylesDeclarationType::NamedPropertyOrDefaultExport => {
                                        let inject_import_stmt = ModuleItem::ModuleDecl(
                                            ModuleDecl::Import(ImportDecl {
                                                span: DUMMY_SP,
                                                specifiers: vec![ImportSpecifier::Default(
                                                    ImportDefaultSpecifier {
                                                        span: DUMMY_SP,
                                                        local: Ident::new(
                                                            format!("_{}", inject_src).into(),
                                                            DUMMY_SP,
                                                        ),
                                                    },
                                                )],
                                                src: Box::new(Str {
                                                    span: DUMMY_SP,
                                                    raw: Option::None,
                                                    value: DEFAULT_INJECT_PATH.into(),
                                                }),
                                                type_only: false,
                                                asserts: Option::None,
                                            }),
                                        );
                                        let _inject = Expr::Ident(Ident::new(
                                            format!("_{}", inject_src).into(),
                                            DUMMY_SP,
                                        ));

                                        let stylex_call_expr = CallExpr {
                                            span: DUMMY_SP,
                                            type_args: Option::None,
                                            callee: Callee::Expr(Box::new(_inject.clone())),
                                            args: stylex_inject_args,
                                        };

                                        let stylex_call = Expr::Call(stylex_call_expr);

                                        let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                                            span: DUMMY_SP,
                                            expr: Box::new(stylex_call),
                                        }));

                                        module_items_middle_vec.push(module);

                                        module_item_pre_start_vec.push(inject_import_stmt)
                                    }
                                },
                                None => {}
                            }
                        }
                        let mut module_items: Vec<ModuleItem> = vec![];

                        module_items.extend_from_slice(&&module_item_pre_start_vec[..]);
                        module_items.extend_from_slice(&module_item_start_slice);
                        module_items.extend_from_slice(&module_items_middle_vec[..]);
                        module_items.extend_from_slice(&module_items_end_slice);

                        module_items
                    }
                    None => module_items,
                };

                module_items.fold_children_with(self)
            }
            ModuleCycle::Cleaning => {
                // We need it twice for a clear dead code after declaration transforms
                let mut module_items = module_items.clone().fold_children_with(self);

                // We remove `Stmt::Empty` from the statement list.
                // This is optional, but it's required if you don't want extra `;` in output.
                module_items.retain(|s| !matches!(s, ModuleItem::Stmt(Stmt::Empty(..))));

                module_items.fold_children_with(self)
            }
        }
    }
}
