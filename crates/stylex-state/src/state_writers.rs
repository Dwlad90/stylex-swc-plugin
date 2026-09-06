//! Recording what a module's top level declares into the state manager.
//!
//! The visitors walk a module once and hand what they pass here. Every function
//! in this module writes to the state manager and returns nothing; nothing here
//! decides what a declaration means, only that the state has to remember it.

use swc_core::ecma::ast::{Decl, Expr, Module, ModuleDecl, ModuleItem, Stmt, VarDeclarator};

use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::top_level_expression::TopLevelExpression;

use crate::state_manager::StateManager;

/// Records every top-level expression the module declares, so a later phase can
/// find a call by the name it was bound to instead of walking the module again.
pub fn fill_top_level_expressions(module: &Module, state: &mut StateManager) {
  module.body.iter().for_each(|item| match item {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
      if let Decl::Var(decl_var) = &export_decl.decl {
        for decl in &decl_var.decls {
          record_top_level_declarator(state, TopLevelExpressionKind::NamedExport, decl);
        }
      }
    },
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_decl)) => {
      match export_decl.expr.as_paren() {
        Some(paren) => {
          state.push_top_level_expression(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            paren.expr.as_ref().clone(),
            None,
          ));
        },
        _ => {
          state.push_top_level_expression(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            export_decl.expr.as_ref().clone(),
            None,
          ));
        },
      }
    },
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) => {
      for decl in &var.decls {
        record_top_level_declarator(state, TopLevelExpressionKind::Stmt, decl);
      }
    },
    _ => {},
  });
}

/// Record one declarator of a top-level variable declaration, `kind` telling
/// an exported one from a plain statement.
///
/// A declarator bound to a pattern rather than a name declares no single name
/// to record, so it contributes no top-level expression — `export const { a } =
/// expr;` is ordinary JavaScript, and an API that does require a name reports
/// that itself, against the call the author wrote. Its position is still worth
/// keeping: nothing else marks the call as program level.
fn record_top_level_declarator(
  state: &mut StateManager,
  kind: TopLevelExpressionKind,
  decl: &VarDeclarator,
) {
  let Some(decl_init) = decl.init.as_ref() else {
    return;
  };

  match decl.name.as_ident() {
    Some(ident) => {
      state.push_top_level_expression(TopLevelExpression(
        kind,
        decl_init.as_ref().clone(),
        Some(ident.sym.clone()),
      ));

      fill_state_declarations(state, decl);
    },
    None => {
      if let Expr::Call(call) = decl_init.as_ref()
        && !call.span.is_dummy()
      {
        state.pattern_bound_top_level_calls.insert(call.span);
      }
    },
  }
}

/// Records one declarator, unless the state already holds it.
pub fn fill_state_declarations(state: &mut StateManager, decl: &VarDeclarator) {
  if !state.holds_declaration(decl) {
    state.push_declaration(decl.clone());
  }
}
