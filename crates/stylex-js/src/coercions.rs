//! The ECMAScript coercions, over the expressions the evaluator has already
//! reduced to values.
//!
//! Each function answers what the language says the coercion produces, and
//! nothing about where the value came from. `None` means the value has no
//! compile-time form of that type — the caller deopts rather than inventing
//! one.

use stylex_utils::number;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{
  BigIntValue, BlockStmtOrExpr, Expr, Ident, Lit, Number, ObjectLit, Prop, PropName, PropOrSpread,
  UnaryOp,
};

/// What `ToString` produces for an object that still takes the
/// `Object.prototype` default.
///
/// Not every object does, which is why [`keeps_default_primitive`] is asked
/// before this is answered.
pub const OBJECT_TO_STRING: &str = "[object Object]";

/// The two methods `OrdinaryToPrimitive` asks an object for, which an object
/// may own rather than inherit and so take its conversion away from the
/// `Object.prototype` default.
const TO_STRING: &str = "toString";
const VALUE_OF: &str = "valueOf";

/// What a function contributes to the string `ToNumber` works from.
///
/// A function's real `ToString` is its source text, which this evaluator does
/// not retain — but `ToNumber` needs only that the text is *not* a numeric
/// literal, which every function's source text also is. So the stand-in yields
/// the number the source would have, and a function inside an array stops
/// making the whole array's number unknowable.
const FUNCTION_TO_NUMBER: &str = "function";

/// How a coercion renders a function it walks over — the one arm on which
/// `ToString` and `ToNumber` part company.
///
/// `ToString` proper has no answer, because a function's string is its source
/// text and this evaluator retains none. `ToNumber` reaches its number
/// *through* that string and needs only that the text is not a numeric literal,
/// so [`FUNCTION_TO_NUMBER`] serves it exactly.
///
/// Carried as a type rather than as the stand-in text itself, so a walker
/// threading it through a nested value says which coercion it is performing
/// rather than which string it happens to substitute.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum FunctionForm {
  /// Refuse, leaving the caller to deopt rather than invent a source text.
  Refuse,
  /// Stand in with text that is not a numeric literal, so a function inside an
  /// array stops making the whole array's number unknowable.
  NotANumber,
}

impl FunctionForm {
  /// The string a function renders as under this form. Exported because the
  /// evaluator's own function values never take the shape of a function
  /// expression and so reach the same decision by their own route.
  pub fn render(self) -> Option<String> {
    match self {
      FunctionForm::Refuse => None,
      FunctionForm::NotANumber => Some(FUNCTION_TO_NUMBER.to_string()),
    }
  }
}

/// `Array.prototype.join(',')` over elements each rendered by `render`, which
/// answers `None` for an element with no string form and so refuses the whole
/// join. Exported because the evaluator's own array representation joins by the
/// same rule as an array literal's, and the two must not drift.
pub fn join_js_elements<T>(
  elements: impl IntoIterator<Item = T>,
  render: impl FnMut(T) -> Option<String>,
) -> Option<String> {
  elements
    .into_iter()
    .map(render)
    .collect::<Option<Vec<_>>>()
    .map(|parts| parts.join(","))
}

/// A value the language spells as an identifier rather than as a literal, and
/// which therefore reaches a coercion still written as the name it was authored
/// with.
///
/// These three are the only identifiers that can: anything the module bound
/// would have been inlined by the evaluator before a coercion ever saw it, so
/// an identifier still standing here is one of the globals below or a value the
/// coercion cannot read. Every coercion asks this rather than matching the
/// names itself, so the set is decided once.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SurvivingGlobal {
  Undefined,
  NaN,
  Infinity,
}

fn surviving_global(ident: &Ident) -> Option<SurvivingGlobal> {
  match ident.sym.as_ref() {
    "undefined" => Some(SurvivingGlobal::Undefined),
    "NaN" => Some(SurvivingGlobal::NaN),
    "Infinity" => Some(SurvivingGlobal::Infinity),
    _ => None,
  }
}

/// Whether `ident` names one of the three globals above — the only values the
/// language spells as an identifier rather than as a literal.
///
/// Exported for the callers that need the *set* rather than a coercion of it:
/// the evaluator's reference-resolution chain, which decides whether such a
/// name is the global or a binding that took it over, and its object coercion,
/// which answers that all three carry no own properties. They ask here for the
/// same reason the coercions do — so the set is written down once and a fourth
/// name would be added in one place.
pub fn is_global_spelled_as_an_identifier(ident: &Ident) -> bool {
  surviving_global(ident).is_some()
}

/// The value one of the three globals *is*, written the way the language would
/// write it if it could.
///
/// `NaN` and `Infinity` are numbers that the grammar has no literal for, so
/// they are authored as identifiers and reach the evaluator as identifiers —
/// but a consumer that asks what a value *is* rather than what it coerces to
/// has to be told a number, or it reads the name as an unresolved reference and
/// refuses. `undefined` has no other spelling, so it answers itself and the
/// caller is no worse off than before.
///
/// The distinction matters exactly where a consumer inspects the expression's
/// shape instead of coercing it. Style-value validation is the one that does:
/// it admits a number and refuses an identifier, so `height: [NaN, '2px']`
/// refused an array the reference implementation accepts — while `height:
/// [0/0, '2px']`, the same value reached by arithmetic, folded and agreed.
///
/// `None` for every other name, so a caller can use this as the set as well.
pub fn global_identifier_to_value(ident: &Ident) -> Option<Expr> {
  match surviving_global(ident)? {
    // No literal spells it, so the name stands. Every coercion above reads the
    // identifier form, so nothing downstream is worse off for it.
    SurvivingGlobal::Undefined => Some(Expr::Ident(ident.clone())),
    SurvivingGlobal::NaN => Some(js_number_expr(f64::NAN)),
    SurvivingGlobal::Infinity => Some(js_number_expr(f64::INFINITY)),
  }
}

/// A number as the expression that writes it, carrying its authored text where
/// the grammar has no literal for it.
///
/// `NaN` and the two infinities have none: asked to write a `Number` node
/// holding `NaN`, the emitter falls back to `0 / 0`, and an infinity to a
/// numeral no author wrote. Both evaluate to the right value, so this is about
/// the text rather than the semantics -- but the text is what a reader diffs,
/// what the reference implementation prints, and what a class name is a hash of.
///
/// Every finite number spells itself, so this is safe to reach for wherever a
/// number becomes an expression rather than being asked about first.
///
/// A negative infinity spells itself with the minus sign in the text. That is a
/// unary expression rather than a numeric literal, and it is the right text all
/// the same: nothing re-parses the raw form, every reader of the node takes its
/// value, and the alternative is the numeral the emitter invents.
pub fn js_number_expr(value: f64) -> Expr {
  let raw = if value.is_nan() {
    Some("NaN")
  } else if value == f64::INFINITY {
    Some("Infinity")
  } else if value == f64::NEG_INFINITY {
    Some("-Infinity")
  } else {
    None
  };

  Expr::Lit(Lit::Num(Number {
    span: DUMMY_SP,
    value,
    raw: raw.map(Into::into),
  }))
}

/// ECMA-262 `ToString`, over an already-evaluated expression.
///
/// Returns `None` for values with no compile-time string form — a function,
/// whose `ToString` is its source text, which this evaluator does not retain.
pub fn to_js_string(expr: &Expr) -> Option<String> {
  to_js_string_with(expr, FunctionForm::Refuse)
}

/// `ToString` under a chosen [`FunctionForm`]. Exported because the evaluator's
/// own value representation walks the same values and has to walk them the same
/// way.
pub fn to_js_string_with(expr: &Expr, function_form: FunctionForm) -> Option<String> {
  match expr {
    // A string that is not valid UTF-8 holds a lone surrogate, which Rust has
    // no `str` for. Refusing hands the caller the same deopt every other
    // unreadable value gets, which names the property the value sits on --
    // where panicking here would report the coercion's own source location and
    // lose that key path.
    Expr::Lit(Lit::Str(strng)) => strng.value.as_str().map(ToString::to_string),
    Expr::Lit(Lit::Num(num)) => Some(number::to_js_string(num.value)),
    Expr::Lit(Lit::Bool(bool_lit)) => Some(bool_lit.value.to_string()),
    Expr::Lit(Lit::Null(_)) => Some("null".to_string()),
    // A big integer renders as its digits with no `n` suffix, which is the one
    // place its string and its source text part company.
    Expr::Lit(Lit::BigInt(big_int)) => Some(format!("{}", big_int.value)),
    // A regular expression is the one object whose `ToString` is not the
    // `Object.prototype` default: it answers its own source text, which unlike
    // a function's the evaluator does retain.
    Expr::Lit(Lit::Regex(regex)) => Some(format!("/{}/{}", regex.exp, regex.flags)),
    Expr::Ident(ident) => match surviving_global(ident)? {
      SurvivingGlobal::Undefined => Some("undefined".to_string()),
      SurvivingGlobal::NaN => Some(number::to_js_string(f64::NAN)),
      SurvivingGlobal::Infinity => Some(number::to_js_string(f64::INFINITY)),
    },
    Expr::Array(array) => join_js_elements(&array.elems, |elem| match elem {
      // A hole joins as nothing, the same as the `null` and `undefined` that
      // can occupy the slot.
      None => Some(String::new()),
      Some(elem) if elem.spread.is_some() => None,
      Some(elem) => js_array_element_to_string(&elem.expr, function_form),
    }),
    // An object converts through the method pair a string prefers: its own
    // `toString` where it has one, and the `Object.prototype` default where it
    // does not.
    Expr::Object(object) => match object_to_primitive(object, ToPrimitiveHint::String)? {
      ObjectPrimitive::Default => Some(OBJECT_TO_STRING.to_string()),
      ObjectPrimitive::Returned(returned) => to_js_string_with(returned, function_form),
    },
    Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_) => function_form.render(),
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
    // An object converts through the method pair a number prefers, which is
    // the reverse of the string one: an own `valueOf` answers ahead of an own
    // `toString`, so `Number({ valueOf: () => 2, toString: () => '1' })` is
    // `2`. An array owns neither and so still reaches its join below.
    Expr::Object(object) => match object_to_primitive(object, ToPrimitiveHint::Number)? {
      ObjectPrimitive::Default => Some(string_to_js_number(OBJECT_TO_STRING)),
      ObjectPrimitive::Returned(returned) => to_js_number(returned),
    },
    // Everything else takes `ToNumber` of its primitive value, which for a
    // string is itself and for an array is its join.
    _ => to_js_string_with(expr, FunctionForm::NotANumber).map(|strng| string_to_js_number(&strng)),
  }
}

/// ECMA-262 `ToBoolean`, over an already-evaluated expression.
///
/// Refuses on strictly less than the other two coercions: every object is
/// truthy whatever it holds, so an object this crate cannot read a *string* out
/// of still has a boolean. `ToBoolean` is the one coercion that never reaches
/// `ToPrimitive`, which is why an own `toString` does not enter into it —
/// `Boolean({ toString: () => '' })` is `true`.
///
/// `None` is an expression whose kind cannot be read at all, so the caller
/// deopts rather than guessing which side of the falsy list it falls on.
pub fn to_js_boolean(expr: &Expr) -> Option<bool> {
  match expr {
    // A string that is not valid UTF-8 holds a lone surrogate, and so is not
    // the empty string — the one string that is falsy. Unlike `ToString`, this
    // never has to read the text.
    Expr::Lit(Lit::Str(strng)) => Some(!strng.value.is_empty()),
    // Both zeroes and `NaN` are the falsy numbers. `-0.0 != 0.0` is already
    // `false`, but `NaN != 0.0` is `true` — every comparison against `NaN` is
    // false *except* the inequality, so it has to be named rather than left to
    // fall out of the arithmetic.
    Expr::Lit(Lit::Num(num)) => Some(num.value != 0.0 && !num.value.is_nan()),
    Expr::Lit(Lit::Bool(bool_lit)) => Some(bool_lit.value),
    Expr::Lit(Lit::Null(_)) => Some(false),
    // `0n` is falsy the way `0` is, and is the only big integer that is.
    Expr::Lit(Lit::BigInt(big_int)) => Some(*big_int.value != BigIntValue::from(0u8)),
    Expr::Ident(ident) => match surviving_global(ident)? {
      SurvivingGlobal::Undefined => Some(false),
      SurvivingGlobal::NaN => Some(false),
      SurvivingGlobal::Infinity => Some(true),
    },
    // Every object is truthy — the empty ones and the empty regular expression
    // among them, since only primitives appear on the falsy list.
    Expr::Object(_) | Expr::Array(_) | Expr::Lit(Lit::Regex(_)) => Some(true),
    Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_) => Some(true),
    // `void x` is `undefined` and so falsy, whatever it was applied to. Named
    // here as well as in [`is_nullish`] so the two answer the same value: a
    // `??` that folds on it and a `||` that refused would disagree about a
    // value the language does not.
    Expr::Unary(unary) if unary.op == UnaryOp::Void => Some(false),
    _ => None,
  }
}

/// Whether an expression is nullish — the values `??` takes its right side for.
///
/// A plain question about the expression rather than a coercion, and answered
/// as a plain `false` for anything else: a value this crate cannot read is not
/// nullish, because the two spellings of nullish are both syntax it can always
/// recognise.
pub fn is_nullish(expr: &Expr) -> bool {
  match expr {
    Expr::Lit(Lit::Null(_)) => true,
    Expr::Ident(ident) => surviving_global(ident) == Some(SurvivingGlobal::Undefined),
    // `void x` is `undefined` whatever `x` is, which is the third spelling the
    // reference implementation recognises alongside the other two.
    Expr::Unary(unary) => unary.op == UnaryOp::Void,
    _ => false,
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
    Expr::Ident(ident) => surviving_global(ident) == Some(SurvivingGlobal::Undefined),
    _ => false,
  }
}

/// `ToInt32` over a number, the coercion the bitwise operators apply to their
/// operands before operating on them.
///
/// Truncates toward zero, then wraps into the signed 32-bit range -- so `~` and
/// friends see the same operand JavaScript gives them, and a value past 2^31
/// wraps rather than growing. `~[4294967296]` is `-1` and not `-4294967297`,
/// which is what a 64-bit negation answers.
///
/// Total, because `ToInt32` is: a `NaN`, an infinity and a zero of either sign
/// all answer `0`, as the specification says, rather than refusing.
pub fn to_int32(value: f64) -> i32 {
  const WRAP: f64 = 4_294_967_296.0;
  const SIGN_BOUNDARY: f64 = 2_147_483_648.0;

  if !value.is_finite() || value == 0.0 {
    return 0;
  }

  let wrapped = value.trunc().rem_euclid(WRAP);

  if wrapped >= SIGN_BOUNDARY {
    (wrapped - WRAP) as i32
  } else {
    wrapped as i32
  }
}

/// What kind of object `ToObject` answers with over a value.
///
/// Reported rather than carried out, and now only as coarsely as its one caller
/// asks: `typeof` tells a function from everything else and nothing else does.
/// `Object(x)` is folded by the engine, which answers with a real object rather
/// than a name for one, so the outcomes it needed apart — a fresh empty object,
/// the value itself, a boxed primitive — are the language's business again.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ObjectCoercion {
  /// A function, which is an object whose `typeof` is nonetheless `function`.
  Function,
  /// Every other object `ToObject` can answer with — one the value already is,
  /// a wrapper around a primitive, or the fresh object the nullish values take.
  Object,
}

/// ECMA-262 `ToObject`, reported as which kind of object it answers with rather
/// than as a value.
///
/// `None` is a value whose kind cannot be read off the expression, so the
/// caller deopts instead of guessing.
pub fn to_object(expr: &Expr) -> Option<ObjectCoercion> {
  match expr {
    Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_) => Some(ObjectCoercion::Function),
    // Every remaining readable value is an object or boxes into one: the two
    // nullish spellings take a fresh one, an array, an object and a regular
    // expression already are one, and a primitive is wrapped in one.
    Expr::Ident(ident) => surviving_global(ident).map(|_| ObjectCoercion::Object),
    Expr::Object(_) | Expr::Array(_) | Expr::Lit(_) => Some(ObjectCoercion::Object),
    _ => None,
  }
}

/// Whether an object literal's primitive conversion is still the
/// `Object.prototype` default, and so is [`OBJECT_TO_STRING`].
///
/// An own `toString` or `valueOf` replaces that default and `Symbol.toPrimitive`
/// precedes it, so an object carrying any of them coerces to a value this crate
/// cannot compute -- `String({ toString: () => 'red' })` is `red`, not
/// `[object Object]`. Answering the default for one of those would put a
/// confidently wrong value in the stylesheet, which is the one outcome a
/// refused fold exists to prevent.
/// ECMA-262 `OrdinaryToPrimitive` over an object literal: the value the first
/// of the object's two conversion methods to answer a primitive returns.
///
/// `None` is an object this crate cannot convert -- one whose keys it cannot
/// name, one whose own method is not a form it can apply, and one JavaScript
/// itself refuses with `Cannot convert object to primitive value`. Each is a
/// refusal rather than the default, because the default would be a value no
/// runtime produces.
fn object_to_primitive(object: &ObjectLit, hint: ToPrimitiveHint) -> Option<ObjectPrimitive<'_>> {
  // A spread contributes keys this crate cannot name, and a computed key is
  // how `Symbol.toPrimitive` is spelled -- which precedes both methods below.
  if !object.props.iter().all(readable_key) {
    return None;
  }

  for name in hint.method_order() {
    match own_conversion_method(object, name) {
      // The object does not override this one, so `Object.prototype`'s
      // applies: its `toString` answers the default text, while its `valueOf`
      // answers the object itself, which is not a primitive and is passed over.
      None if name == TO_STRING => break,
      None => continue,
      Some(returned) => return Some(ObjectPrimitive::Returned(returned?)),
    }
  }

  // Both orders end in `toString`, so the loop leaves off at the default rather
  // than at a refusal however it is left.
  Some(ObjectPrimitive::Default)
}

/// Which primitive a conversion prefers, and so which of the two methods an
/// object is asked for first. `String(x)` prefers a string and `Number(x)` a
/// number, which is the whole of the difference between them here.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum ToPrimitiveHint {
  String,
  Number,
}

impl ToPrimitiveHint {
  fn method_order(self) -> [&'static str; 2] {
    match self {
      ToPrimitiveHint::String => [TO_STRING, VALUE_OF],
      ToPrimitiveHint::Number => [VALUE_OF, TO_STRING],
    }
  }
}

/// What an object literal converts to under [`object_to_primitive`].
enum ObjectPrimitive<'a> {
  /// The `Object.prototype` pair, whose primitive is [`OBJECT_TO_STRING`].
  Default,
  /// The expression an own conversion method returns.
  Returned(&'a Expr),
}

/// The body of an own `name` method the coercion can apply, as
/// `Some(Some(body))`.
///
/// `None` is an object that does not own `name` at all, and `Some(None)` one
/// that owns it in a form this crate cannot apply -- a method shorthand, a
/// getter, a parameterised or block-bodied arrow, or a value that is not
/// callable, which JavaScript answers with a `TypeError` rather than a value.
/// The two are told apart because only the first falls through to the other
/// method.
fn own_conversion_method<'a>(object: &'a ObjectLit, name: &str) -> Option<Option<&'a Expr>> {
  let prop = object.props.iter().find_map(|prop| match prop {
    PropOrSpread::Prop(prop) if prop_name(prop) == Some(name) => Some(prop.as_ref()),
    _ => None,
  })?;

  let Prop::KeyValue(key_value) = prop else {
    return Some(None);
  };

  let Expr::Arrow(arrow) = key_value.value.as_ref() else {
    return Some(None);
  };

  // A conversion method is called with no arguments, so a parameter would only
  // ever bind `undefined` -- but a default initialiser on one makes the body
  // depend on it, which is more than this crate reads. A block body is more
  // than it reads either.
  if !arrow.params.is_empty() {
    return Some(None);
  }

  let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
    return Some(None);
  };

  // A method answering an object has not answered a primitive. JavaScript
  // moves on to the other method, whose `Object.prototype` version answers the
  // object again and ends in a `TypeError` -- so there is no value to fold.
  if matches!(
    body.as_ref(),
    Expr::Object(_) | Expr::Array(_) | Expr::Arrow(_) | Expr::Fn(_) | Expr::Class(_)
  ) {
    return Some(None);
  }

  Some(Some(body.as_ref()))
}

fn readable_key(prop: &PropOrSpread) -> bool {
  match prop {
    PropOrSpread::Spread(_) => false,
    PropOrSpread::Prop(prop) => !matches!(prop_key(prop), Some(PropName::Computed(_))),
  }
}

fn prop_key(prop: &Prop) -> Option<&PropName> {
  match prop {
    Prop::Shorthand(_) | Prop::Assign(_) => None,
    Prop::KeyValue(key_value) => Some(&key_value.key),
    Prop::Getter(getter) => Some(&getter.key),
    Prop::Setter(setter) => Some(&setter.key),
    Prop::Method(method) => Some(&method.key),
  }
}

fn prop_name(prop: &Prop) -> Option<&str> {
  match prop {
    Prop::Shorthand(ident) => Some(ident.sym.as_ref()),
    // Only reachable inside a destructuring pattern, never in an evaluated
    // value, but named rather than defaulted so a later reader is not left to
    // decide.
    Prop::Assign(assign) => Some(assign.key.sym.as_ref()),
    // The two arms above are the whole of what [`prop_key`] has no key for, so
    // what is left always carries one.
    _ => prop_key(prop).and_then(|key| match key {
      PropName::Ident(ident) => Some(ident.sym.as_ref()),
      // A key that is not valid UTF-8 holds a lone surrogate, which neither
      // ASCII name spells.
      PropName::Str(strng) => strng.value.as_str(),
      PropName::Num(_) | PropName::BigInt(_) | PropName::Computed(_) => None,
    }),
  }
}

fn js_array_element_to_string(expr: &Expr, function_form: FunctionForm) -> Option<String> {
  if joins_as_empty(expr) {
    return Some(String::new());
  }

  to_js_string_with(expr, function_form)
}

#[cfg(test)]
#[path = "tests/coercions_tests.rs"]
mod tests;
