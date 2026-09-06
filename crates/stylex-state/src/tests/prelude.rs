//! Builders every test module in this crate needs.
//!
//! A declarator over an identifier name is what a test hands the state to
//! record a binding, and four modules asked for one. They built it four times,
//! which was unavoidable while they sat in different crates and is not now that
//! they are siblings.

use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{BindingIdent, Expr, Pat, VarDeclarator},
};

use stylex_ast::ast::factories::create_ident;

/// One declarator over the initializer handed in. No case cares about the name
/// pattern beyond it being an identifier, which is also the only shape
/// [`crate::state_manager::StateManager::declaration_of`] answers for.
pub(super) fn make_var_declarator(name: &str, init: Expr) -> VarDeclarator {
  declarator(name, Some(Box::new(init)))
}

/// The same declarator with no initializer -- `let x;`, which the state records
/// as a binding that holds no value yet.
pub(super) fn make_var_declarator_no_init(name: &str) -> VarDeclarator {
  declarator(name, None)
}

fn declarator(name: &str, init: Option<Box<Expr>>) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent {
      id: create_ident(name),
      type_ann: None,
    }),
    init,
    definite: false,
  }
}
