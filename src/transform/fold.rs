mod fold_expr;
mod fold_import_decl;
mod fold_module;
mod fold_module_items;
mod fold_var_declarator;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{Expr, ImportDecl, Module, ModuleItem, VarDeclarator},
        visit::{noop_fold_type, Fold},
    },
};

use crate::ModuleTransformVisitor;

impl<C> Fold for ModuleTransformVisitor<C>
where
    C: Comments,
{
    noop_fold_type!();

    fn fold_module(&mut self, module: Module) -> Module {
        self.fold_module_impl(module)
    }

    fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        self.fold_module_items(module_items)
    }

    fn fold_import_decl(&mut self, import_decl: ImportDecl) -> ImportDecl {
        self.fold_import_decl_impl(import_decl)
    }

    fn fold_var_declarator(&mut self, var_declarator: VarDeclarator) -> VarDeclarator {
        self.fold_var_declarator_impl(var_declarator)
    }

    fn fold_expr(&mut self, expr: Expr) -> Expr {
        self.fold_expr_impl(expr)
    }
}
