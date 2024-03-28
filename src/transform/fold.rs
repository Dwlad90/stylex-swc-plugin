mod fold_export_default_expr;
mod fold_expr;
mod fold_ident;
mod fold_import_decl;
mod fold_member_expression;
mod fold_module;
mod fold_module_items;
mod fold_stmt;
mod fold_stmts;
mod fold_var_declarator;
mod fold_var_declarators;
mod fold_export_decl;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{
            ExportDecl, ExportDefaultExpr, Expr, Ident, ImportDecl, MemberExpr, Module, ModuleItem,
            Stmt, VarDeclarator,
        },
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

    fn fold_var_declarators(&mut self, var_declarators: Vec<VarDeclarator>) -> Vec<VarDeclarator> {
        self.fold_var_declarators_impl(var_declarators)
    }

    fn fold_var_declarator(&mut self, var_declarator: VarDeclarator) -> VarDeclarator {
        self.fold_var_declarator_impl(var_declarator)
    }

    fn fold_export_decl(&mut self, export_decl: ExportDecl) -> ExportDecl {
        self.fold_export_decl_impl(export_decl)
    }

    fn fold_stmts(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        self.fold_stmts_impl(stmts)
    }

    fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
        self.fold_stmt_impl(stmt)
    }

    fn fold_expr(&mut self, expr: Expr) -> Expr {
        self.fold_expr_impl(expr)
    }

    fn fold_member_expr(&mut self, member_expresseion: MemberExpr) -> MemberExpr {
        self.fold_member_expr_impl(member_expresseion)
    }

    fn fold_export_default_expr(
        &mut self,
        export_default_expr: ExportDefaultExpr,
    ) -> ExportDefaultExpr {
        self.fold_export_default_expr_impl(export_default_expr)
    }

    fn fold_ident(&mut self, ident: Ident) -> Ident {
        self.fold_ident_impl(ident)
    }
}
