//! The ECMAScript coercions, over the expressions the evaluator has already
//! reduced to values.
//!
//! Each function answers what the language says the coercion produces, and
//! nothing about where the value came from. `None` means the value has no
//! compile-time form of that type — the caller deopts rather than inventing
//! one.

use stylex_constants::constants::messages::INVALID_UTF8;
use stylex_macros::stylex_panic;
use stylex_utils::number;
use swc_core::ecma::ast::{Expr, Lit};

/// What `ToString` produces for any ordinary object: the `Object.prototype`
/// default, which no value reaching the evaluator overrides.
pub const OBJECT_TO_STRING: &str = "[object Object]";

/// ECMA-262 `ToString`, over an already-evaluated expression.
///
/// Returns `None` for values with no compile-time string form — a function,
/// whose `ToString` is its source text, which this evaluator does not retain.
pub fn to_js_string(expr: &Expr) -> Option<String> {
  match expr {
    Expr::Lit(Lit::Str(strng)) => Some(match strng.value.as_str() {
      Some(value) => value.to_string(),
      None => stylex_panic!("{}", INVALID_UTF8),
    }),
    Expr::Lit(Lit::Num(num)) => Some(number::to_js_string(num.value)),
    Expr::Lit(Lit::Bool(bool_lit)) => Some(bool_lit.value.to_string()),
    Expr::Lit(Lit::Null(_)) => Some("null".to_string()),
    // `undefined`, `NaN` and `Infinity` survive evaluation as the global
    // identifiers they were written as; nothing else can, because a binding in
    // scope would have been inlined.
    Expr::Ident(ident) => match ident.sym.as_ref() {
      "undefined" => Some("undefined".to_string()),
      "NaN" => Some(number::to_js_string(f64::NAN)),
      "Infinity" => Some(number::to_js_string(f64::INFINITY)),
      _ => None,
    },
    Expr::Array(array) => {
      let mut parts = Vec::with_capacity(array.elems.len());

      for elem in &array.elems {
        parts.push(match elem {
          // A hole joins as nothing, the same as the `null` and `undefined`
          // that can occupy the slot.
          None => String::new(),
          Some(elem) if elem.spread.is_some() => return None,
          Some(elem) => js_array_element_to_string(&elem.expr)?,
        });
      }

      Some(parts.join(","))
    },
    Expr::Object(_) => Some(OBJECT_TO_STRING.to_string()),
    _ => None,
  }
}

/// Whether `Array.prototype.join` renders this element as nothing rather than
/// as its `ToString`. Exported because the evaluator's own array
/// representation joins by the same rule.
pub fn joins_as_empty(expr: &Expr) -> bool {
  match expr {
    Expr::Lit(Lit::Null(_)) => true,
    Expr::Ident(ident) => ident.sym == *"undefined",
    _ => false,
  }
}

fn js_array_element_to_string(expr: &Expr) -> Option<String> {
  if joins_as_empty(expr) {
    return Some(String::new());
  }

  to_js_string(expr)
}

#[cfg(test)]
#[path = "tests/coercions_tests.rs"]
mod tests;
