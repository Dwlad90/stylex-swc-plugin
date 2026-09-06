//! The string an object key names, whichever way it was written.
//!
//! `evaluate_obj_key` is what the transform above asks for the *name* of a
//! property, and every spelling a key can have has to answer the same string
//! the language names the property with -- a key read one way here and another
//! where the object itself is folded is how one property becomes two rules.
//!
//! Asked directly rather than through a folded object, because this is the
//! entry point the transform calls and its refusals are answers rather than
//! deopts recorded on a state.

use super::source_evaluation::*;
use crate::evaluate::evaluate_obj_key;
use stylex_constants::constants::messages::{EXPRESSION_IS_NOT_A_STRING, ILLEGAL_PROP_VALUE};
use stylex_state::{
  evaluate_result_value::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
};
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{DUMMY_SP, GLOBALS, Globals},
  ecma::ast::{ComputedPropName, Expr, IdentName, KeyValueProp, PropName},
};

use stylex_ast::ast::convertors::{convert_atom_to_string, create_number_expr, create_string_expr};

/// The key `name` names, or the refusal it answered.
fn key_of(name: PropName) -> Result<String, Option<String>> {
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut state = StateManager::new(StyleXOptions::default());
    let result = evaluate_obj_key(
      &KeyValueProp {
        key: name,
        value: Box::new(create_number_expr(1.0)),
      },
      &mut state,
      &FunctionMap::default(),
    );

    match (result.confident, result.value) {
      (true, Some(EvaluateResultValue::Expr(Expr::Lit(literal)))) => match literal.as_str() {
        Some(text) => Ok(convert_atom_to_string(&text.value)),
        None => panic!("expected a string key, got {:?}", literal),
      },
      (true, other) => panic!("expected a string key, got {:?}", other),
      (false, _) => Err(result.reason),
    }
  })
}

fn computed(expr: Expr) -> PropName {
  PropName::Computed(ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(expr),
  })
}

/// The three spellings that name a key without evaluating anything: an
/// identifier, a string, and a number. The number is rendered as JavaScript
/// spells it rather than as Rust does.
#[test]
fn a_written_key_names_itself() {
  assert_eq!(
    key_of(PropName::Ident(IdentName::new("color".into(), DUMMY_SP))),
    Ok(String::from("color"))
  );
  assert_eq!(
    key_of(PropName::Str("font-size".into())),
    Ok(String::from("font-size"))
  );
  assert_eq!(
    key_of(PropName::Num(1e21.into())),
    Ok(String::from("1e+21"))
  );
}

/// A computed key is the string its expression folds to.
#[test]
fn a_computed_key_is_what_it_folds_to() {
  assert_eq!(
    key_of(computed(create_string_expr("color"))),
    Ok(String::from("color"))
  );
  assert_eq!(
    key_of(computed(create_number_expr(2.0))),
    Ok(String::from("2"))
  );
}

/// A computed key whose expression resolves to nothing carries that
/// expression's own refusal, so the sentence names what the author wrote.
#[test]
fn a_computed_key_that_resolves_to_nothing_carries_its_own_refusal() {
  let refusal = key_of(computed(parse_expr("unknownName")));

  assert!(
    matches!(&refusal, Err(Some(reason)) if reason.contains("not defined")),
    "expected the name's own refusal, got {:?}",
    refusal
  );
}

/// A computed key that folded to a value with no expression form is a key this
/// does not read -- an ordinary refusal rather than a broken invariant.
#[test]
fn a_computed_key_with_no_expression_form_refuses() {
  assert_eq!(
    key_of(computed(parse_expr("[1, 2]"))),
    Err(Some(ILLEGAL_PROP_VALUE.to_string()))
  );
}

/// A computed key is the string its expression names, so an expression with no
/// string form names no key and the object refuses. An object literal is such
/// an expression: it evaluates, and then has no key spelling to give.
#[test]
fn a_computed_key_with_no_string_form_refuses() {
  assert_deopt_reason_contains("({ [{}]: 'x' })", EXPRESSION_IS_NOT_A_STRING);
}
