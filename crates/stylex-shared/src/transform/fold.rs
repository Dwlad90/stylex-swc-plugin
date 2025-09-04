mod fold_computed_prop_name_impl;
mod fold_decl;
mod fold_export_decl;
mod fold_export_default_expr;
mod fold_expr;
// mod fold_fn_declarator;
mod fold_ident;
mod fold_import_decl;
mod fold_jsx_attr_or_spread;
mod fold_member_expression;
mod fold_member_prop;
mod fold_module;
mod fold_module_items;
mod fold_prop_name;
mod fold_stmt;
mod fold_stmts;
mod fold_var_declarator;
mod fold_var_declarators;

use super::StyleXTransform;
use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{
      ComputedPropName, Decl, ExportDecl, ExportDefaultExpr, Expr, Ident, ImportDecl, MemberExpr,
      MemberProp, Module, ModuleItem, PropName, Stmt, VarDeclarator,
    },
    visit::{Fold, noop_fold_type},
  },
};

impl<C> Fold for StyleXTransform<C>
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

  // fn fold_fn_decl(&mut self, fn_decl: FnDecl) -> FnDecl {
  //   self.fold_fn_declarator_impl(fn_decl)
  // }

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

  fn fold_prop_name(&mut self, prop_name: PropName) -> PropName {
    self.fold_prop_name_impl(prop_name)
  }

  fn fold_member_prop(&mut self, member_prop: MemberProp) -> MemberProp {
    self.fold_member_prop_impl(member_prop)
  }

  fn fold_member_expr(&mut self, member_expresseion: MemberExpr) -> MemberExpr {
    self.fold_member_expr_impl(member_expresseion)
  }

  fn fold_computed_prop_name(&mut self, computed_prop_name: ComputedPropName) -> ComputedPropName {
    self.fold_computed_prop_name_impl(computed_prop_name)
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

  fn fold_decl(&mut self, decl: Decl) -> Decl {
    self.fold_decl_impl(decl)
  }

  fn fold_jsx_attr_or_spreads(
    &mut self,
    jsx_attrs: Vec<swc_core::ecma::ast::JSXAttrOrSpread>,
  ) -> Vec<swc_core::ecma::ast::JSXAttrOrSpread> {
    self.fold_jsx_attr_or_spreads(jsx_attrs)
  }
}
