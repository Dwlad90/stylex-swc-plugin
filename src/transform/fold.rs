use serde::de;
use swc_core::{
    common::{
        comments::{Comment, CommentKind, Comments},
        Span, DUMMY_SP,
    },
    ecma::{
        ast::{
            BindingIdent, CallExpr, Callee, Expr, Ident, ImportDecl, ImportSpecifier, Module,
            ModuleDecl, ModuleItem, Pat, Stmt, VarDecl, VarDeclarator,
        },
        visit::{noop_fold_type, Fold, FoldWith},
    },
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> Fold for ModuleTransformVisitor<C>
where
    C: Comments,
{
    noop_fold_type!();

    // Collect import modules that indicator if this file need to be transformed
    fn fold_import_decl(&mut self, import_decl: ImportDecl) -> ImportDecl {
        if self.cycle == ModuleCycle::Skip {
            return import_decl;
        }

        if self.cycle == ModuleCycle::Processing {
            if import_decl.type_only {
                return import_decl;
            }

            let src = &import_decl.src;
            let declaration = &src.value;

            if declaration.eq(self.package_name.as_str()) {
                for specifier in &import_decl.specifiers {
                    match &specifier {
                        ImportSpecifier::Default(import_specifier) => {
                            self.declaration = Some(import_specifier.local.to_id());
                        }
                        ImportSpecifier::Namespace(import_specifier) => {
                            self.declaration = Some(import_specifier.local.to_id());
                        }
                        _ => panic!("Must be default import"),
                    };
                }
            }

            if self.declaration.is_none() {
                import_decl
            } else {
                import_decl.fold_children_with(self)
            }
        } else {
            import_decl
        }
    }

    fn fold_module(&mut self, module: Module) -> Module {
        let module = module.clone().fold_children_with(self);

        if self.declaration.is_some() {
            self.cycle = ModuleCycle::Cleaning;

            let module = module.fold_children_with(self);

            self.comments.add_leading_comments(
                module.span.lo,
                vec![Comment {
                    kind: CommentKind::Block,
                    text: format!(
                        "__stylex_metadata_start__{}__stylex_metadata_end__",
                        serde_json::to_string(&self.css_output).unwrap()
                    )
                    .into(),
                    span: module.span,
                }],
            );

            module.fold_children_with(self)
        } else {
            self.cycle = ModuleCycle::Skip;
            module
        }
    }

    fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        match self.cycle {
            ModuleCycle::Skip => return module_items,
            ModuleCycle::Processing => {
                let mut module_items = module_items.fold_children_with(self);

                if self.declaration.is_none() {
                    self.cycle = ModuleCycle::Skip;

                    return module_items;
                }

                module_items.retain(|module_item| {
                    if let ModuleItem::ModuleDecl(import_decl) = module_item {
                        if let ModuleDecl::Import(import) = import_decl {
                            let src_value = import.src.value.to_string();

                            if self.package_name.as_str() != &src_value {
                                return true;
                            }

                            let specifier = import.specifiers.first();

                            if let Some(spec) = specifier {
                                match spec {
                                    ImportSpecifier::Namespace(_) => {
                                        return false;
                                    }
                                    ImportSpecifier::Named(_) => {
                                        return true;
                                    }
                                    ImportSpecifier::Default(_) => {
                                        return false;
                                    }
                                }
                            }
                        }
                    }

                    true
                });

                // TODO: Inject comment to css extraction

                module_items
            }
            ModuleCycle::Cleaning => {
                // We need it twice for a clear dead code after declaration transforms
                let mut module_items = module_items.clone().fold_children_with(self);

                // We remove `Stmt::Empty` from the statement list.
                // This is optional, but it's required if you don't want extra `;` in output.
                module_items.retain(|s| !matches!(s, ModuleItem::Stmt(Stmt::Empty(..))));

                module_items
            }
        }
    }

    fn fold_var_declarator(&mut self, mut var_declarator: VarDeclarator) -> VarDeclarator {
        // Get the declarations from the VarDecl struct
        // let var_declarator_id = var_declarator.clone().name.as_ident().unwrap().to_id();
        // let stylex_var_declarator = self.declaration.clone().unwrap();

        if &var_declarator.init.is_some() == &true {
            match &*var_declarator.init.clone().unwrap() {
                Expr::Call(call) => {
                    let declaration_tuple = self.process_declaration(&call);

                    match &declaration_tuple {
                        Some(declaration) => {
                            let (declaration, member) = declaration;

                            if declaration.eq(&self.declaration.clone().unwrap())
                                && member == "create"
                            {
                                self.props_declaration =
                                    var_declarator.name.as_ident().map(|ident| ident.to_id());

                                var_declarator.name = Pat::Ident(BindingIdent {
                                    id: Ident {
                                        span: DUMMY_SP,
                                        optional: false,
                                        sym: "_stylex$props".into(),
                                    },
                                    type_ann: None,
                                })
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }

        // Call the fold_children_with method on the VarDecl struct
        var_declarator.fold_children_with(self)
    }

    fn fold_expr(&mut self, mut expr: Expr) -> Expr {
        if self.cycle == ModuleCycle::Skip {
            return expr;
        }

        if self.cycle == ModuleCycle::Processing {
            match &mut expr {
                Expr::Call(ex) => {
                    let declaration = self.process_declaration(&ex);

                    if declaration.is_some() {
                        let value = self.target_call_expression_to_css_map_expr(&ex);

                        match value {
                            Some(value) => {
                                return value;
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        expr.fold_children_with(self)
    }
}
