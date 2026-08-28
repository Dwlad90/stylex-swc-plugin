//! What the engine answered, read back as the evaluator's own value.
//!
//! The evaluator's type rather than a bare syntax node, so a folded value
//! reaches every place a value the author wrote reaches. The walk out is bounded
//! for the reason the walk in is: a value the guard would refuse as too deep on
//! the way in can still be built by a loop the engine ran, and the stack this
//! recurses on was claimed for so many levels and no more.
//!
//! Bounded in size for a reason the guard could not have covered either. The
//! engine aliases, so one array referenced ten thousand times costs it one
//! array; this side copies, so the same answer is a hundred million syntax
//! nodes. That cost is spent here and nowhere else, which is why the counting is
//! here and runs as values are produced.

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
use stylex_utils::string::utf16_length;

use super::super::growable_stack::grown_per_level;
use super::super::helpers::js_undefined;
use super::as_expr;
use super::engine::read;
use super::{Ceilings, Decline, Depth, Totals};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue, utils::common::order_own_keys,
};

/// What the outward bridge carries as it converts a value back: the method whose
/// answer it is reading, so a refusal can name it, and how much of the two
/// allocation ceilings the answer has spent so far.
///
/// The method is carried because the engine's own sentence does not always
/// name it — `"abc".unsupported()` throws `not a callable function`, which
/// tells an author nothing the code frame has not already shown them.
///
/// The totals are carried for the reason the inward ones are, and are the
/// inward ones' opposite number: what is about to be built is the whole answer
/// rather than one value of it. Nesting is the parameter of the walk rather than
/// a field, because it is the one budget that is spent going down and given back
/// coming up.
pub(super) struct Outward<'a> {
  pub(super) method: &'a Atom,
  totals: Totals,
}

impl<'a> Outward<'a> {
  /// A conversion of `method`'s answer, with nothing spent yet.
  pub(super) fn new(method: &'a Atom, ceilings: Ceilings) -> Self {
    Self {
      method,
      totals: Totals::new(ceilings),
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
  pub(super) fn value(
    &mut self,
    value: &JsValue,
    engine: &mut Context,
    depth: Depth,
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
      // it — and counts it against every other string of the same answer, since
      // an array of a thousand strings is the same text a thousand times over.
      self
        .totals
        .count_characters(string.len() as u64)
        .map_err(|ceiling| Decline::rule(folded_string_too_large(ceiling)))?;

      // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
      // unpaired surrogate cannot survive this step. Substituting the replacement
      // character keeps the declaration text identical to what the reference
      // implementation writes to disk, and diverges only in the class name.
      Lit::Str(string.to_std_string_lossy().into())
    } else {
      return self.object_value(value, engine, depth);
    };

    Ok(EvaluateResultValue::Expr(Expr::Lit(literal)))
  }

  /// The half of [`value`](Outward::value) that needs the engine to read the
  /// value back: arrays and plain objects, and the refusal for everything else.
  fn object_value(
    &mut self,
    value: &JsValue,
    engine: &mut Context,
    depth: Depth,
  ) -> Result<EvaluateResultValue, Decline> {
    // Room for the next level asked for at this one, as every walk this module
    // owns does — the engine builds by looping, so what comes back can be
    // nested deeper than anything the guard admitted. See `growable_stack`.
    grown_per_level(|| self.nested_object(value, engine, depth))
  }

  /// One object or array, on the room [`object_value`](Outward::object_value)
  /// asked for, and reached only through it — a direct call would descend on no
  /// room at all.
  fn nested_object(
    &mut self,
    value: &JsValue,
    engine: &mut Context,
    depth: Depth,
  ) -> Result<EvaluateResultValue, Decline> {
    // `typeof` names the kind rather than a word list of this module's own: the
    // engine already answers this question, exhaustively, and its answer is the
    // one an author would use for the value they wrote.
    let Some(object) = value.as_object() else {
      return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
    };

    // A value nested deeper than the guard admits on the way in can still be
    // built on the way out, by a loop the engine ran rather than by syntax the
    // author wrote. Bounded for the reason the input is bounded, and against the
    // same ceiling: the conversion recurses, and the stack it recurses on was
    // claimed for that many levels and no more.
    let inner = depth.descend()?;

    if object.is_array() {
      let length = self.length_of(&object, engine)?;
      let mut items = Vec::with_capacity(length as usize);

      for index in 0..length {
        let element = read(self.method, || object.get(index, engine))?;

        items.push(self.value(&element, engine, inner)?);
      }

      return Ok(EvaluateResultValue::Vec(items));
    }

    if !object.is_ordinary() {
      return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
    }

    let keys = read(self.method, || object.own_property_keys(engine))?;

    self
      .totals
      .count_entries(keys.len() as u64)
      .map_err(|ceiling| Decline::rule(object_size_too_large(ceiling)))?;

    let mut props = Vec::with_capacity(keys.len());

    for key in keys {
      // A symbol key has no spelling in an object literal, so an object carrying
      // one cannot be written back out whole — and writing it out partly would
      // fold a value the source does not describe. `PropertyKey` has three
      // variants and all three are answered, as the crate's error policy asks.
      let name = match &key {
        PropertyKey::String(string) => string.to_std_string_lossy(),
        PropertyKey::Index(index) => index.get().to_string(),
        // Unreachable from any module an author can write, and answered rather
        // than asserted for the reason every other refusal here is. Reaching it
        // needs a symbol, and the only spellings that produce one are a `Symbol`
        // the guard does not admit as a global and a computed key it refuses
        // outright — in a callback body as much as at the top of an expression,
        // since a free name there resolves through the same walk.
        PropertyKey::Symbol(_) => {
          return Err(Decline::rule(unfoldable_fold_result(
            "object with a symbol key",
          )));
        },
      };

      // A key is text the answer holds as surely as a value is, and an object of
      // few enormous keys is the shape that says so. Counted against the same
      // total the strings are, exactly as a key crossing the other way is.
      self
        .totals
        .count_characters(utf16_length(&name) as u64)
        .map_err(|ceiling| Decline::rule(folded_string_too_large(ceiling)))?;

      let element = read(self.method, || object.get(key.clone(), engine))?;
      let expr = as_property_value(self.value(&element, engine, inner)?)?;

      props.push(create_ident_key_value_prop(&name, expr));
    }

    // Ordered by the same rule an object the author wrote is ordered by, rather
    // than by trusting two implementations of own-key order to agree. The engine
    // answers its own keys in that order already, so this re-states it rather
    // than changing it — which is exactly why it is here: the day the two stop
    // agreeing is the day a declaration silently changes which rule wins.
    Ok(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
      order_own_keys(props),
    ))))
  }

  /// An array's `length`, bounded: the count the conversion loop reads.
  ///
  /// The two ways it can fail say different things, because they are different
  /// faults. A length past what is left of the entry budget is the bound, and
  /// names it. A `length` that is not a count at all — not a number, or negative
  /// — is not the bound and must not claim to be; it is a value the bridge cannot
  /// read, and is refused as one.
  ///
  /// Counted before the elements are read, so an array past the bound refuses
  /// without first converting a single element of it.
  fn length_of(&mut self, object: &JsObject, engine: &mut Context) -> Result<u64, Decline> {
    let length = read(self.method, || object.get(js_string!("length"), engine))?;

    let Some(length) = length.as_number().filter(|length| *length >= 0.0) else {
      return Err(Decline::rule(unfoldable_fold_result(
        "array with no readable length",
      )));
    };

    // Saturating at the cast, which is what a float past `u64` does here: the
    // total it feeds is refused on rather than allocated from, so a saturated
    // reading refuses exactly as the true one would.
    self
      .totals
      .count_entries(length as u64)
      .map_err(|ceiling| Decline::rule(array_length_too_large(ceiling)))?;

    Ok(length as u64)
  }
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
  // Every arm of `Outward::value` answers one of the two shapes `as_expr` reads,
  // so nothing else is reachable by construction — and a refusal is answered
  // rather than a panic if that ever stops holding.
  as_expr(&value)
    .ok_or_else(|| Decline::rule(unfoldable_fold_result("value of an unreadable kind")))
}
