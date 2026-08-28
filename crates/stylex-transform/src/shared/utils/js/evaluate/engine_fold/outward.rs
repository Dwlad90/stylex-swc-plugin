//! What the engine answered, read back as the evaluator's own value.
//!
//! The evaluator's type rather than a bare syntax node, so a folded value
//! reaches every place a value the author wrote reaches. The walk out is bounded
//! for the reason the walk in is: a value the guard would refuse as too deep on
//! the way in can still be built by a loop the engine ran, and the stack this
//! recurses on was claimed for so many levels and no more.

use boa_engine::{Context, JsObject, JsValue, js_string, property::PropertyKey};
use swc_core::{
  atoms::Atom,
  common::DUMMY_SP,
  ecma::ast::{Expr, Lit, Null},
};

use stylex_ast::ast::factories::{create_ident_key_value_prop, create_object_lit};
use stylex_constants::constants::evaluation_errors::{
  array_length_too_large, folded_string_too_large, object_size_too_large, unfoldable_fold_result,
};
use stylex_js::coercions;

use super::super::helpers::js_undefined;
use super::as_expr;
use super::engine::read;
use super::{Ceilings, Decline, Depth};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue, utils::common::order_own_keys,
};

/// What the outward bridge carries as it converts a value back: the method
/// whose answer it is reading, so a refusal can name it, and how much nesting
/// is left before the value is refused as too deep.
///
/// The method is carried because the engine's own sentence does not always
/// name it — `"abc".unsupported()` throws `not a callable function`, which
/// tells an author nothing the code frame has not already shown them.
#[derive(Clone, Copy)]
pub(super) struct Outward<'a> {
  pub(super) method: &'a Atom,
  pub(super) depth: Depth,
  pub(super) ceilings: Ceilings,
}

impl Outward<'_> {
  /// The bridge one level in.
  ///
  /// A value nested deeper than the guard admits on the way in can still be
  /// built on the way out, by a loop the engine ran rather than by syntax the
  /// author wrote. Bounded for the reason the input is bounded, and against the
  /// same ceiling: the conversion recurses, and the stack it recurses on was
  /// claimed for that many levels and no more.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }
}

/// Converts an engine value into the evaluator's own value type, or declines
/// with the rule that refused it.
///
/// The evaluator's type rather than a bare syntax node, so a folded value
/// reaches every place a value the author wrote reaches: an array answers the
/// `Vec` an array literal answers, and an object the `Expr::Object` an object
/// literal answers. Answering a syntax node instead is what left a folded array
/// and an evaluated one in two dispatch arms that disagree about which methods
/// they carry.
pub(super) fn to_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  // The one value with no literal at all, so it crosses back as the name the
  // language spells it with. An array hole is the same value and arrives here
  // the same way, as does a member read that found nothing and a callback that
  // returned nothing. Answering it rather than refusing is what leaves the
  // style-value check to be the one an author hears from, on both compilers --
  // a refusal here would refuse the whole array rather than the holes in it.
  if value.is_undefined() {
    return Ok(js_undefined());
  }

  // Read through the accessors rather than matching variants: the engine's
  // value is nan-boxed by default and an enum only under a feature, and both
  // answer these.
  if let Some(number) = value.as_number() {
    // Spelled rather than written straight into a `Number` node: `NaN` and the
    // infinities have no numeric literal, so the emitter would write `0 / 0` and
    // a numeral no author wrote. A class name is a hash of the declaration text,
    // so the spelling is the value.
    return Ok(EvaluateResultValue::Expr(coercions::js_number_expr(number)));
  }

  let literal = if let Some(truth) = value.as_boolean() {
    Lit::Bool(truth.into())
  } else if value.is_null() {
    Lit::Null(Null { span: DUMMY_SP })
  } else if let Some(string) = value.as_string() {
    // The bound on an amplifying argument bounds what one written call may be
    // asked to build; this bounds what actually came back, whatever produced
    // it. The array arm below has had such a bound from the start, and a string
    // is the other shape a fold can return at size.
    if string.len() as u64 > outward.ceilings.characters {
      return Err(Decline::rule(folded_string_too_large(
        outward.ceilings.characters,
      )));
    }

    // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
    // unpaired surrogate cannot survive this step. Substituting the replacement
    // character keeps the declaration text identical to what the reference
    // implementation writes to disk, and diverges only in the class name.
    Lit::Str(string.to_std_string_lossy().into())
  } else {
    return to_object_value(value, engine, outward);
  };

  Ok(EvaluateResultValue::Expr(Expr::Lit(literal)))
}

/// The half of [`to_value`] that needs the engine to read the value back:
/// arrays and plain objects, and the refusal for everything else.
fn to_object_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  // `typeof` names the kind rather than a word list of this module's own: the
  // engine already answers this question, exhaustively, and its answer is the
  // one an author would use for the value they wrote.
  let Some(object) = value.as_object() else {
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
  };

  let inner = outward.descend()?;

  if object.is_array() {
    let length = read_length(&object, engine, outward)?;
    let mut items = Vec::with_capacity(length as usize);

    for index in 0..length {
      let element = read(outward.method, || object.get(index, engine))?;

      items.push(to_value(&element, engine, inner)?);
    }

    return Ok(EvaluateResultValue::Vec(items));
  }

  if !object.is_ordinary() {
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
  }

  let keys = read(outward.method, || object.own_property_keys(engine))?;

  if keys.len() as u64 > outward.ceilings.entries {
    return Err(Decline::rule(object_size_too_large(
      outward.ceilings.entries,
    )));
  }

  let mut props = Vec::with_capacity(keys.len());

  for key in keys {
    // A symbol key has no spelling in an object literal, so an object carrying
    // one cannot be written back out whole — and writing it out partly would
    // fold a value the source does not describe. `PropertyKey` has three
    // variants and all three are answered, as the crate's error policy asks.
    let name = match &key {
      PropertyKey::String(string) => string.to_std_string_lossy(),
      PropertyKey::Index(index) => index.get().to_string(),
      PropertyKey::Symbol(_) => {
        return Err(Decline::rule(unfoldable_fold_result(
          "object with a symbol key",
        )));
      },
    };

    let element = read(outward.method, || object.get(key.clone(), engine))?;
    let expr = as_property_value(to_value(&element, engine, inner)?)?;

    props.push(create_ident_key_value_prop(&name, expr));
  }

  // Ordered by the same rule an object the author wrote is ordered by, rather
  // than by trusting two implementations of own-key order to agree.
  Ok(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
    order_own_keys(props),
  ))))
}

/// One folded value as the expression an object property carries.
///
///
/// An array is the one case that has to be rebuilt rather than moved: a `Vec`
/// is the shape the evaluator wants at the top of a value, and an object
/// literal wants a nested array literal in the same position. Rebuilt by the
/// evaluator's own conversion rather than by a second copy of it here, so a
/// folded property and an evaluated one cannot come to disagree about what an
/// array element may be.
fn as_property_value(value: EvaluateResultValue) -> Result<Expr, Decline> {
  // Every arm of `to_value` answers one of the two shapes `as_expr` reads, so
  // nothing else is reachable by construction — and a refusal is answered rather
  // than a panic if that ever stops holding.
  as_expr(&value)
    .ok_or_else(|| Decline::rule(unfoldable_fold_result("value of an unreadable kind")))
}

/// An array's `length`, bounded: the count the conversion loop below reads.
///
/// The two ways it can fail say different things, because they are different
/// faults. A length past the entry ceiling is the bound, and names it. A
/// `length` that is not a count at all — not a number, or negative — is not the
/// bound and must not claim to be; it is a value the bridge cannot read, and is
/// refused as one.
fn read_length(object: &JsObject, engine: &mut Context, outward: Outward) -> Result<u64, Decline> {
  let length = read(outward.method, || object.get(js_string!("length"), engine))?;

  let Some(length) = length.as_number().filter(|length| *length >= 0.0) else {
    return Err(Decline::rule(unfoldable_fold_result(
      "array with no readable length",
    )));
  };

  if length > outward.ceilings.entries as f64 {
    return Err(Decline::rule(array_length_too_large(
      outward.ceilings.entries,
    )));
  }

  Ok(length as u64)
}
