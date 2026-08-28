//! The conversions a global names, applied to a value the engine was not handed.
//!
//! `String(x)`, `Number(x)`, `Object(x)` and `Array(x)` are native JavaScript
//! functions, and the [fold](super::super::engine_fold) answers them by calling
//! them — which is why there is no table of conversions here standing in for the
//! language.
//!
//! What is here is the one set of arguments the engine can never be handed. Its
//! bridge carries JavaScript values, and this compiler has values of its own: a
//! resolved theme reference, the injected function map, the environment object.
//! None has a JavaScript form to cross as, so the fold hands the call back and
//! this answers it — with the same coercions `+` and an interpolation already
//! use, so a coerced token group cannot come to read one way here and another
//! there.
//!
//! Upstream folds every one of these, and a build that refused them would name a
//! class the other build defines.

use super::super::*;
use stylex_constants::constants::evaluation_errors::{STRING_CONVERSION, uncoercible_value};
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::CallExpr;

/// One of the four conversions, named by the global the author wrote.
#[derive(Clone, Copy)]
pub(super) enum Conversion {
  String,
  Number,
  Object,
  Array,
}

impl Conversion {
  /// The conversion `global` names, or `None` where the name is some other
  /// global — `Math` is not a function at all, and the fold has already said so.
  pub(super) fn named(global: &Atom) -> Option<Self> {
    match &**global {
      "String" => Some(Self::String),
      "Number" => Some(Self::Number),
      "Object" => Some(Self::Object),
      "Array" => Some(Self::Array),
      _ => None,
    }
  }

  /// The name the author wrote, for the sentence a refusal carries.
  fn name(self) -> &'static str {
    match self {
      Self::String => "String",
      Self::Number => "Number",
      Self::Object => "Object",
      Self::Array => "Array",
    }
  }

  /// The conversion's answer, or the refusal that names the callee.
  ///
  /// The fold owns every call to an unbound global, so this has to answer one
  /// way or the other: a call handed on from here would end at the catch-all's
  /// `Unsupported expression` with the reason lost.
  ///
  /// Surplus arguments are ignored and a missing one is the conversion's own
  /// empty answer, as in JavaScript — `String(1, 2)` is `"1"` and `String()` is
  /// `""`.
  pub(super) fn evaluate(
    self,
    call: &CallExpr,
    path: &Expr,
    state: &mut EvaluationState,
    traversal_state: &mut StateManager,
    fns: &FunctionMap,
  ) -> Option<EvaluateResultValue> {
    let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

    if !state.confident {
      return None;
    }

    // An argument that evaluated to nothing while staying confident was dropped
    // rather than deopted, so the remaining arguments no longer line up with
    // what was written. Refuse rather than convert a shifted argument list.
    if args.len() != call.args.len() {
      return deopt(path, state, &uncoercible_value(self.name()));
    }

    match args.first() {
      Some(argument) => self.of(argument, path, state, traversal_state),
      None => Some(self.of_nothing()),
    }
  }

  /// The conversion applied to the one argument that matters.
  fn of(
    self,
    argument: &EvaluateResultValue,
    path: &Expr,
    state: &mut EvaluationState,
    traversal_state: &mut StateManager,
  ) -> Option<EvaluateResultValue> {
    match self {
      Self::String => {
        // Grown through the shared buffer rather than collected, so an array
        // holding one of this compiler's values is still measured element by
        // element and refuses at the one that passes the ceiling.
        let mut text = GrownString::new(STRING_CONVERSION);

        match text.push_string_of(argument, || path.clone(), state, traversal_state) {
          Ok(()) => Some(EvaluateResultValue::Expr(create_string_expr(
            &text.into_text(),
          ))),
          // A value with no compile-time string at all — a function, whose only
          // string form is its source text and neither compiler has the
          // author's. The ceiling refusal already deopted and carries its own
          // sentence.
          Err(StringAppend::NoStringForm) => deopt(path, state, &uncoercible_value(self.name())),
          Err(StringAppend::TooLarge(sentence)) => deopt(path, state, &sentence),
        }
      },
      Self::Number => match evaluate_result_to_js_number(argument, traversal_state) {
        // `NaN` is not a refusal: it flows into the declaration exactly as it
        // does upstream, where `Number('10px')` writes `NaN` into the rule.
        Ok(number) => Some(EvaluateResultValue::Expr(create_number_expr(number))),
        Err(NumberRefusal::NoNumberForm) => deopt(path, state, &uncoercible_value(self.name())),
        Err(NumberRefusal::TooLarge) => deopt(
          path,
          state,
          &grown_string_too_large(
            NUMERIC_CONVERSION,
            traversal_state.character_ceiling() as u64,
          ),
        ),
      },
      // Every value that reaches here is an object upstream, and `Object` hands
      // an object back unchanged — which is what keeps a member read off the
      // result resolving to the same thing the bare group resolves to.
      Self::Object => match evaluate_result_to_js_object(argument) {
        Some(_) => Some(argument.clone()),
        None => deopt(path, state, &uncoercible_value(self.name())),
      },
      // A single argument that is not a number is an element rather than a
      // length, and none of the values reaching here is a number.
      Self::Array => Some(EvaluateResultValue::Vec(vec![argument.clone()])),
    }
  }

  /// The conversion applied to no arguments at all.
  ///
  /// Each is its own empty answer rather than the conversion of `undefined`:
  /// `String()` is the empty string where `String(undefined)` is `"undefined"`,
  /// and `Number()` is zero where `Number(undefined)` is `NaN`.
  fn of_nothing(self) -> EvaluateResultValue {
    match self {
      Self::String => EvaluateResultValue::Expr(create_string_expr("")),
      Self::Number => EvaluateResultValue::Expr(create_number_expr(0.0)),
      Self::Object => EvaluateResultValue::Expr(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![],
      })),
      Self::Array => EvaluateResultValue::Vec(vec![]),
    }
  }
}
