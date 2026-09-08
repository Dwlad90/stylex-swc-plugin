//! Builders every test module in this crate needs.
//!
//! A declarator over an identifier name is what a test hands the state to
//! record a binding, and four modules asked for one. They built it four times,
//! which was unavoidable while they sat in different crates and is not now that
//! they are siblings. The same holds for the handful of builders beside it: an
//! empty state, the two expression shapes a style value takes, an identifier at
//! a named scope, and the path of a fixture package.

use std::{env, path::PathBuf};

use path_clean::PathClean;
use swc_core::{
  common::{DUMMY_SP, SyntaxContext},
  ecma::ast::{BindingIdent, Expr, Ident, Lit, Pat, Str, VarDeclarator},
};

use stylex_ast::ast::factories::{create_ident, create_object_lit};
use stylex_structures::stylex_state_options::StyleXStateOptions;

use crate::state_manager::StateManager;

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

/// A state for a project that sets no option, which is what a case asking about
/// the state itself rather than about the project wants.
pub(super) fn test_state() -> StateManager {
  StateManager::for_test(None, StyleXStateOptions::default())
}

/// An identifier at a given scope. Zero is the scope every identifier the parser
/// produces before the resolver runs, so it doubles as "the scope does not
/// matter to this case"; anything else is a different scope.
///
/// `SyntaxContext::from_u32` rather than `apply_mark`, which would need
/// `GLOBALS` installed for what is only "some other scope than that one".
pub(super) fn ident_at(name: &str, ctxt: u32) -> Ident {
  Ident {
    span: DUMMY_SP,
    sym: name.into(),
    optional: false,
    ctxt: SyntaxContext::from_u32(ctxt),
  }
}

/// The same identifier at the parser's own scope, for the cases a scope never
/// enters.
pub(super) fn ident(name: &str) -> Ident {
  ident_at(name, 0)
}

/// A string literal, which is what a style value most often is.
pub(super) fn string_expr(value: &str) -> Expr {
  Expr::Lit(Lit::Str(Str {
    span: DUMMY_SP,
    value: value.into(),
    raw: None,
  }))
}

/// An empty object literal, standing for the namespace a compiled `create` call
/// leaves behind. No case reads into it.
pub(super) fn object_expr() -> Expr {
  Expr::Object(create_object_lit(vec![]))
}

/// The path of a fixture package under this crate's own test tree.
pub(super) fn fixture_path(name: &str) -> PathBuf {
  match env::current_dir() {
    Ok(dir) => dir.join("src/tests/fixtures").join(name).clean(),
    Err(error) => panic!("the working directory could not be read: {error}"),
  }
}
