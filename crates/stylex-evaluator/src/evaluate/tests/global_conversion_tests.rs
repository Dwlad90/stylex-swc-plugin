//! `String`, `Number`, `Object` and `Array` over a value the engine never sees.
//!
//! All four are native JavaScript functions and fold by being called, which is
//! what the [applied global suite](super::applied_global_tests) asserts. What
//! is left for this compiler to answer is the argument the bridge cannot carry:
//! the folded function map is not a JavaScript value at all, so a call over one
//! is handed back unfolded and the conversion is applied here instead.
//!
//! Upstream folds every one of these, so a build that refused what upstream
//! converts would name a class the other build defines. The two coercions that
//! do refuse -- a value with no string form and one with no numeric form --
//! refuse in both compilers, and the sentence has to name the global the author
//! wrote rather than the shape this compiler holds.

use super::source_evaluation::*;
use stylex_ast::ast::convertors::convert_atom_to_string;
use stylex_constants::constants::evaluation_errors::uncoercible_value;
use stylex_state::evaluate_result_value::EvaluateResultValue;

/// The refusal a conversion gives, so a case can say which global it names.
#[track_caller]
fn assert_refuses_naming(source: &str, global: &str) {
  assert_refused_with(
    &evaluated_against_a_function_fold(source),
    source,
    &uncoercible_value(global),
  );
}

/// The value a conversion answered, for the two that answer rather than refuse.
#[track_caller]
fn folded_value(source: &str) -> EvaluateResultValue {
  folded_value_of(evaluated_against_a_function_fold(source), source)
}

/// Asserts a conversion over the fold answers `NaN`.
#[track_caller]
fn assert_conversion_is_nan(source: &str) {
  assert_result_folds_to_nan(evaluated_against_a_function_fold(source), source);
}

/// A single function of the fold has no string form -- its only string would be
/// its source text, and this compiler keeps none -- so the string conversion
/// refuses. The sentence names `String`, which is what the author wrote, and
/// not the shape this compiler was holding.
#[test]
fn the_string_conversion_refuses_a_value_with_no_string_form() {
  assert_refuses_naming("String(own)", "String");
}

/// The numeric conversion answers rather than refuses: `Number` of a function
/// is `NaN` in the language, and `NaN` flows into the declaration exactly as it
/// does upstream.
#[test]
fn the_number_conversion_answers_not_a_number_for_a_function() {
  assert_conversion_is_nan("Number(own)");
}

/// The namespace is an object where one of its functions is a function, and
/// that is what both coercions turn on: `import * as stylex` binds an object
/// whose properties happen to be functions, so the object default answers where
/// the function refuses. The reference implementation reads it the same way.
#[test]
fn the_namespace_coerces_as_the_object_it_is() {
  match folded_value("String(sx)") {
    EvaluateResultValue::Expr(expr) => assert_eq!(
      expr
        .as_lit()
        .and_then(|lit| lit.as_str())
        .map(|text| convert_atom_to_string(&text.value)),
      Some(String::from("[object Object]"))
    ),
    other => panic!("expected the object default, got {:?}", other),
  }

  assert_conversion_is_nan("Number(sx)");
}

/// `Object` of an object is that object, so the fold is handed straight back --
/// which is what keeps a member read off the result resolving to the same thing
/// the bare name resolves to.
#[test]
fn the_object_conversion_hands_a_function_fold_back() {
  assert!(
    matches!(
      folded_value("Object(sx)"),
      EvaluateResultValue::FunctionConfigMap(_)
    ),
    "expected the fold itself back"
  );
}

/// `Array` has no surplus argument: every argument is an element, so the whole
/// list is read and a call over one value answers a one-element array.
#[test]
fn the_array_conversion_reads_every_argument_as_an_element() {
  match folded_value("Array(sx)") {
    EvaluateResultValue::Vec(items) => assert_eq!(items.len(), 1),
    other => panic!("expected a one-element array, got {:?}", other),
  }

  match folded_value("Array(sx, sx)") {
    EvaluateResultValue::Vec(items) => assert_eq!(items.len(), 2),
    other => panic!("expected a two-element array, got {:?}", other),
  }
}

/// `String`, `Number` and `Object` read the first argument and ignore the rest,
/// as they do in the language: the surplus changes neither the answer nor the
/// refusal.
#[test]
fn the_surplus_arguments_of_the_other_three_are_ignored() {
  assert_refuses_naming("String(own, 'ignored')", "String");
  assert_conversion_is_nan("Number(own, 'ignored')");
  assert!(
    matches!(
      folded_value("Object(sx, 'ignored')"),
      EvaluateResultValue::FunctionConfigMap(_)
    ),
    "expected the fold itself back"
  );
}

/// A global that is not one of the four names no conversion, so the call falls
/// through to the dispatch below rather than being converted -- `Math` is not a
/// function at all.
#[test]
fn a_global_that_names_no_conversion_is_not_converted() {
  let result = evaluated_against_a_function_fold("Math(sx)");

  assert!(!result.confident, "expected `Math(sx)` to refuse");
  assert_ne!(
    result.reason.as_deref(),
    Some(uncoercible_value("Math").as_str()),
    "`Math` is not a conversion and must not be refused as one"
  );
}
