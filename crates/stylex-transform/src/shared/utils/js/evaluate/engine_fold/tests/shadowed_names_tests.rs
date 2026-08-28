//! Which bindings shadow one of the globals the fold owns, in each of the two
//! positions a global is written in.
//!
//! The two rules go opposite ways, so a single table would hide the one thing
//! worth reading here: every binding shadows a **callee**, and only a declarator
//! shadows a **receiver**. Both directions are asserted on every shape, because a
//! rule that answers the same way in both positions is a rule that has drifted
//! back into one.

use super::*;

use std::rc::Rc;

use stylex_ast::ast::convertors::create_string_expr;
use swc_core::common::{DUMMY_SP, SyntaxContext};
use swc_core::ecma::ast::{BindingIdent, ParenExpr, Pat, VarDeclarator};

/// The syntax context the module's own bindings and references share.
const MODULE_CONTEXT: SyntaxContext = SyntaxContext::empty();

/// A context no binding in the module has, for the case where some unrelated
/// scope happens to bind the same spelling.
fn another_scope() -> SyntaxContext {
  SyntaxContext::from_u32(1)
}

fn ident_in(name: &str, ctxt: SyntaxContext) -> Ident {
  Ident {
    span: DUMMY_SP,
    sym: name.into(),
    optional: false,
    ctxt,
  }
}

/// The name as it is read at the use site.
fn reference(name: &str) -> Expr {
  Expr::Ident(ident_in(name, MODULE_CONTEXT))
}

fn declarator(name: &str, init: Expr) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent {
      id: ident_in(name, MODULE_CONTEXT),
      type_ann: None,
    }),
    init: Some(Box::new(init)),
    definite: false,
  }
}

/// A module that declares nothing.
fn a_module() -> StateManager {
  StateManager::default()
}

/// The module with `name` bound the way a `function` or an import binds it:
/// recorded as a binding, with no declarator to read a value from.
fn binding_only(name: &str) -> StateManager {
  let mut state = a_module();

  Rc::make_mut(&mut state.declared_bindings).insert(ident_in(name, MODULE_CONTEXT).to_id());

  state
}

/// The module with `name` declared as a `const` holding `init`, which is both a
/// binding and a declarator — as the pre-scan records it.
fn declared_as_a_const(name: &str, init: Expr) -> StateManager {
  let mut state = binding_only(name);

  state.push_declaration(declarator(name, init));

  state
}

#[track_caller]
fn assert_applied_global(expr: &Expr, state: &StateManager, expected: Option<&str>) {
  assert_eq!(
    unshadowed_applied_global(expr, state).map(|name| &**name),
    expected
  );
}

#[track_caller]
fn assert_receiver_global(expr: &Expr, state: &StateManager, expected: Option<&str>) {
  assert_eq!(
    unshadowed_receiver_global(expr, state).map(|name| &**name),
    expected
  );
}

// ──────────────────────────────────────────────
// A name nothing declared is the global in both
// ──────────────────────────────────────────────

#[test]
fn an_undeclared_name_is_the_global_in_both_positions() {
  let state = a_module();

  for name in ["String", "Number", "Object", "Array", "Math"] {
    assert_applied_global(&reference(name), &state, Some(name));
    assert_receiver_global(&reference(name), &state, Some(name));
  }
}

// A binding some unrelated scope holds is not this reference's, because the `Id`
// carries the context — which is what keeps a dynamic style's parameter from
// deciding for the module around it.
#[test]
fn a_binding_in_another_scope_shadows_neither_position() {
  let mut state = a_module();
  Rc::make_mut(&mut state.declared_bindings).insert(ident_in("String", another_scope()).to_id());

  assert_applied_global(&reference("String"), &state, Some("String"));
  assert_receiver_global(&reference("String"), &state, Some("String"));
}

// ──────────────────────────────────────────────
// The two rules parting company
// ──────────────────────────────────────────────

// A `function` or an import binds the name without leaving a declarator to read.
// The callee is the author's own function; the receiver still names the global,
// since the printed source is all a static needs.
#[test]
fn a_binding_without_a_declarator_shadows_the_callee_alone() {
  let state = binding_only("Math");

  assert_applied_global(&reference("Math"), &state, None);
  assert_receiver_global(&reference("Math"), &state, Some("Math"));
}

// A declarator holds a value the static could have been meant to read, so it
// shadows both.
#[test]
fn a_declarator_shadows_both_positions() {
  let state = declared_as_a_const("String", create_string_expr("abc"));

  assert_applied_global(&reference("String"), &state, None);
  assert_receiver_global(&reference("String"), &state, None);
}

// ──────────────────────────────────────────────
// The shapes neither rule owns
// ──────────────────────────────────────────────

// Parentheses change nothing about which name is written, in either position.
#[test]
fn parentheses_are_read_through_however_many_deep() {
  let state = a_module();
  let wrapped = Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(reference("Math")),
    })),
  });

  assert_applied_global(&wrapped, &state, Some("Math"));
  assert_receiver_global(&wrapped, &state, Some("Math"));
}

// A name that is not one of the globals the fold owns is not one either rule
// answers for, declared or not.
#[test]
fn a_name_outside_the_owned_set_is_no_global() {
  let state = a_module();

  assert_applied_global(&reference("Reflect"), &state, None);
  assert_receiver_global(&reference("Reflect"), &state, None);
}

// Neither rule reads an expression that is not a bare name, so nothing below a
// member or a call is mistaken for a global.
#[test]
fn an_expression_that_is_not_a_name_is_no_global() {
  let state = a_module();
  let literal = create_string_expr("String");

  assert_applied_global(&literal, &state, None);
  assert_receiver_global(&literal, &state, None);
}
