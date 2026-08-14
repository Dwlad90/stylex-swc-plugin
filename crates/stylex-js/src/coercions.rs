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

/// What a function contributes to the string `ToNumber` works from.
///
/// A function's real `ToString` is its source text, which this evaluator does
/// not retain — but `ToNumber` needs only that the text is *not* a numeric
/// literal, which every function's source text also is. So the stand-in yields
/// the number the source would have, and a function inside an array stops
/// making the whole array's number unknowable.
pub const FUNCTION_TO_NUMBER: &str = "function";

/// ECMA-262 `ToString`, over an already-evaluated expression.
///
/// Returns `None` for values with no compile-time string form — a function,
/// whose `ToString` is its source text, which this evaluator does not retain.
pub fn to_js_string(expr: &Expr) -> Option<String> {
  to_js_string_of(expr, None)
}

/// `ToString` as `ToNumber` needs it, which is the same string except that a
/// function renders as [`FUNCTION_TO_NUMBER`] rather than refusing. Exported
/// because the evaluator's own array representation needs the same leniency
/// when it walks a nested value.
pub fn to_js_string_for_number(expr: &Expr) -> Option<String> {
  to_js_string_of(expr, Some(FUNCTION_TO_NUMBER))
}

fn to_js_string_of(expr: &Expr, function_form: Option<&str>) -> Option<String> {
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
          Some(elem) => js_array_element_to_string(&elem.expr, function_form)?,
        });
      }

      Some(parts.join(","))
    },
    Expr::Object(_) => Some(OBJECT_TO_STRING.to_string()),
    Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_) => function_form.map(str::to_string),
    _ => None,
  }
}

/// ECMA-262 `ToNumber`, over an already-evaluated expression.
///
/// Refuses on less than `to_js_string` does: a function has a number even
/// though it has no string, because [`FUNCTION_TO_NUMBER`] stands in for the
/// source text. `NaN` is a value, not a refusal — `Number('10px')` is `NaN` in
/// JavaScript and lands in the stylesheet as `NaN`.
pub fn to_js_number(expr: &Expr) -> Option<f64> {
  match expr {
    Expr::Lit(Lit::Num(num)) => Some(num.value),
    Expr::Lit(Lit::Bool(bool_lit)) => Some(if bool_lit.value { 1.0 } else { 0.0 }),
    // `null` is zero and `undefined` is `NaN` — the one place the two part
    // company, since `ToString` spells both out. `undefined` needs no arm of
    // its own: it stringifies to `"undefined"`, which is not a numeric
    // literal.
    Expr::Lit(Lit::Null(_)) => Some(0.0),
    // Everything else takes `ToNumber` of its primitive value, which for a
    // string is itself and for an object is its `ToString` — an array's join,
    // and `[object Object]` for anything else.
    _ => to_js_string_for_number(expr).map(|strng| string_to_js_number(&strng)),
  }
}

/// ECMA-262 `StringToNumber`: the value of the numeric literal a string
/// spells, or `NaN` if it spells anything else.
///
/// Not `f64::from_str`, which disagrees with the language in both directions.
/// It rejects the radix prefixes and the surrounding whitespace JavaScript
/// accepts, and accepts `inf` and `nan`, which JavaScript rejects — and each
/// of those disagreements would put a wrong value in a stylesheet rather than
/// fail a build.
pub fn string_to_js_number(value: &str) -> f64 {
  let literal = value.trim_matches(is_js_whitespace);

  if literal.is_empty() {
    return 0.0;
  }

  match non_decimal_digits(literal) {
    Some((radix, digits)) => digits_to_number(radix, digits),
    None => decimal_to_number(literal),
  }
}

/// Whether the language counts this as whitespace around a numeric literal.
///
/// Not `char::is_whitespace`, which follows Unicode rather than the language:
/// it admits U+0085, which JavaScript does not, and omits U+FEFF, which
/// JavaScript does.
fn is_js_whitespace(c: char) -> bool {
  // The tab family and the space, the two line terminators, and the rest of
  // the Unicode space separators.
  matches!(c, '\u{2000}'..='\u{200A}')
    || matches!(
      c,
      '\u{0009}'
        | '\u{000A}'
        | '\u{000B}'
        | '\u{000C}'
        | '\u{000D}'
        | '\u{0020}'
        | '\u{00A0}'
        | '\u{1680}'
        | '\u{2028}'
        | '\u{2029}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}'
        | '\u{FEFF}'
    )
}

/// The radix and digits of a `NonDecimalIntegerLiteral`, which takes no sign —
/// which is why it is recognised ahead of the signed decimal grammar, and why
/// `-0x1f` reaches that grammar and is not a number at all.
fn non_decimal_digits(literal: &str) -> Option<(u32, &str)> {
  let radix = match literal.get(..2)? {
    "0x" | "0X" => 16,
    "0o" | "0O" => 8,
    "0b" | "0B" => 2,
    _ => return None,
  };

  Some((radix, &literal[2..]))
}

fn digits_to_number(radix: u32, digits: &str) -> f64 {
  if digits.is_empty() {
    return f64::NAN;
  }

  let mut exact: Option<u128> = Some(0);
  let mut accumulated = 0.0_f64;

  for c in digits.chars() {
    let digit = match c.to_digit(radix) {
      Some(digit) => digit,
      None => return f64::NAN,
    };

    exact = exact.and_then(|value| {
      value
        .checked_mul(u128::from(radix))?
        .checked_add(u128::from(digit))
    });
    accumulated = accumulated * f64::from(radix) + f64::from(digit);
  }

  // The exact value rounds once, at the end, the way the language says. Past
  // 128 bits — a literal no stylesheet holds — the running total has already
  // rounded at each digit instead.
  match exact {
    Some(value) => value as f64,
    None => accumulated,
  }
}

fn decimal_to_number(literal: &str) -> f64 {
  let (negative, unsigned) = match literal.strip_prefix('-') {
    Some(rest) => (true, rest),
    None => (false, literal.strip_prefix('+').unwrap_or(literal)),
  };

  let magnitude = if unsigned == "Infinity" {
    // Spelled exactly this way and no other: `infinity` is not a number.
    f64::INFINITY
  } else if is_decimal_literal(unsigned) {
    // The fallback is unreachable — the grammar checked above is a subset of
    // the one Rust parses.
    unsigned.parse::<f64>().unwrap_or(f64::NAN)
  } else {
    f64::NAN
  };

  if negative { -magnitude } else { magnitude }
}

/// Whether the string is a `StrUnsignedDecimalLiteral` — digits with an
/// optional fractional part, an optional exponent, and nothing else. The
/// spellings Rust accepts and the language does not all fail here.
fn is_decimal_literal(value: &str) -> bool {
  let (mantissa, exponent) = match value.split_once(['e', 'E']) {
    Some((mantissa, exponent)) => (mantissa, Some(exponent)),
    None => (value, None),
  };

  if let Some(exponent) = exponent {
    let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);

    if !is_digits(digits) {
      return false;
    }
  }

  match mantissa.split_once('.') {
    // Either side of the point may be empty — `5.` and `.5` are both
    // literals — but not both, which would leave a bare `.`.
    Some((integral, fractional)) => {
      !(integral.is_empty() && fractional.is_empty())
        && is_digits_or_empty(integral)
        && is_digits_or_empty(fractional)
    },
    None => is_digits(mantissa),
  }
}

fn is_digits(value: &str) -> bool {
  !value.is_empty() && is_digits_or_empty(value)
}

fn is_digits_or_empty(value: &str) -> bool {
  value.bytes().all(|byte| byte.is_ascii_digit())
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

/// The number an expression *is*, as opposed to what it would coerce to.
///
/// `Array(3)` is a length where `Array('3')` is an element, so a caller that
/// has to tell a number from a numeric string asks this rather than
/// [`to_js_number`]. `NaN` and `Infinity` survive evaluation as the global
/// identifiers they were written as and are numbers all the same; `undefined`
/// arrives the same way and is not one.
pub fn js_number_value(expr: &Expr) -> Option<f64> {
  match expr {
    Expr::Lit(Lit::Num(num)) => Some(num.value),
    Expr::Ident(ident) => match ident.sym.as_ref() {
      "NaN" => Some(f64::NAN),
      "Infinity" => Some(f64::INFINITY),
      _ => None,
    },
    _ => None,
  }
}

/// ECMA-262 `ArrayCreate`'s length check: a length is an integer in
/// `0..2 ** 32`.
///
/// `None` is every count JavaScript answers with a `RangeError` — a fraction,
/// a negative, `NaN`, an infinity, or a value at or past the limit — for which
/// no array exists.
pub fn to_array_length(count: f64) -> Option<usize> {
  const LENGTH_LIMIT: f64 = 4_294_967_296.0;

  (count.is_finite() && count.fract() == 0.0 && (0.0..LENGTH_LIMIT).contains(&count))
    .then_some(count as usize)
}

/// Which outcome `ToObject` takes over a value.
///
/// Reported rather than carried out, because not every outcome produces a value
/// a caller can hold: naming the wrapper and the function keeps a boxed
/// primitive out of the caller's value type and lets it decide what a function
/// means where it sits.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ObjectCoercion {
  /// `null` and `undefined`, which `ToObject` answers with a fresh empty
  /// object rather than a wrapper around anything.
  EmptyObject,
  /// A value that already is an object — an array among them — which
  /// `ToObject` returns unchanged.
  Identity,
  /// A function, which is an object and which `ToObject` also returns
  /// unchanged. Told apart from [`ObjectCoercion::Identity`] because a caller
  /// that has to hold the result may have no representation for a function.
  Function,
  /// A primitive, which `ToObject` boxes in a wrapper object.
  Wrapper,
}

/// ECMA-262 `ToObject`, reported as which outcome it takes rather than as a
/// value.
///
/// `None` is a value whose kind cannot be read off the expression, so the
/// caller deopts instead of guessing which outcome applies.
pub fn to_object(expr: &Expr) -> Option<ObjectCoercion> {
  match expr {
    Expr::Lit(Lit::Null(_)) => Some(ObjectCoercion::EmptyObject),
    // `undefined` survives evaluation as the global identifier it was written
    // as, and so do `NaN` and `Infinity`, which are numbers and box like one.
    Expr::Ident(ident) => match ident.sym.as_ref() {
      "undefined" => Some(ObjectCoercion::EmptyObject),
      "NaN" | "Infinity" => Some(ObjectCoercion::Wrapper),
      _ => None,
    },
    Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_) => Some(ObjectCoercion::Function),
    // A regular expression is an object, so it passes through as one.
    Expr::Object(_) | Expr::Array(_) | Expr::Lit(Lit::Regex(_)) => Some(ObjectCoercion::Identity),
    Expr::Lit(_) => Some(ObjectCoercion::Wrapper),
    _ => None,
  }
}

fn js_array_element_to_string(expr: &Expr, function_form: Option<&str>) -> Option<String> {
  if joins_as_empty(expr) {
    return Some(String::new());
  }

  to_js_string_of(expr, function_form)
}

#[cfg(test)]
#[path = "tests/coercions_tests.rs"]
mod tests;
