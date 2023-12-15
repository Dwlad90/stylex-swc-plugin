use swc_core::{
    common::{comments::Comments, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Decl, Expr, ExprStmt, Ident, MemberExpr, MemberProp, ModuleDecl,
            ModuleItem, Stmt,
        },
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        enums::ModuleCycle,
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
                let mut styles_item_target_idx = 0;

                for module_item in module_items.clone().into_iter() {
                    if let ModuleItem::ModuleDecl(import_decl) = module_item {
                        match &import_decl {
                            ModuleDecl::ExportDecl(export_decl) => match &export_decl.decl {
                                Decl::Var(var_decl) => {
                                    for var_declarator in &var_decl.decls {
                                        let decl_name = self.get_props_desclaration_as_string();
                                        let var_declarator_name =
                                            get_pat_as_string(&var_declarator.name);

                                        if decl_name.eq(&var_declarator_name) {
                                            styles_item_target_idx = styles_item_idx;
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }

                    styles_item_idx += 1;
                }

                let a = &module_items[0..styles_item_target_idx];
                let b = &module_items[styles_item_target_idx..];
                let mut c: Vec<ModuleItem> = vec![].to_vec();

                for MetaData(_, styles, priority) in &self.css_output {
                    let stylex = Expr::Ident(Ident::new("stylex".into(), DUMMY_SP));

                    let stylex_member = Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(stylex),
                        prop: MemberProp::Ident(Ident {
                            optional: false,
                            span: DUMMY_SP,
                            sym: "inject".into(),
                        }),
                    });

                    let stylex_call_expr = CallExpr {
                        span: DUMMY_SP,
                        type_args: Option::None,
                        callee: Callee::Expr(Box::new(stylex_member.clone())),
                        args: vec![
                            expr_or_spread_string_expression_creator(styles.ltr.clone()),
                            expr_or_spread_number_expression_creator(priority.clone().into()),
                        ],
                    };

                    let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Call(stylex_call_expr)),
                    }));

                    c.push(module);
                }

                let mut module_items: Vec<ModuleItem> = vec![];

                a.iter().for_each(|item| module_items.push(item.clone()));
                c.iter().for_each(|item| module_items.push(item.clone()));
                b.iter().for_each(|item| module_items.push(item.clone()));

                // module_items.splice(
                //     0..0,
                //     ModuleItem::Stmt(),
                // );

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
