//! How much a call would build, in the two units a fold spends.
//!
//! Both rules here are arithmetic rather than syntax: work out what the call
//! would come to — characters of string, or array elements — and refuse when
//! that is past the ceiling. So a count may be written out, named or computed,
//! and what stays refused is a length nothing here can read.
//!
//! A body inside a callback runs once per element of the receiver the call
//! around it was written on, so every bound below is multiplied by that count.
//! A receiver nothing counted leaves the product unbounded, which is the one
//! refusal both rules share.

use swc_core::{
  atoms::Atom,
  ecma::ast::{ArrayLit, Expr, ExprOrSpread, KeyValueProp, Lit, Prop, PropName, PropOrSpread},
};

use stylex_ast::ast::convertors::{atom_utf16_length, is_js_undefined};
use stylex_constants::constants::evaluation_errors::{
  amplification_inside_a_callback, amplified_entries_too_large, amplified_length_too_large,
  unbounded_amplified_length,
};
use stylex_js::coercions::to_js_number;
use stylex_utils::number::to_js_string;

use super::guard::{Callback, Reader, Walk, without_parens};
use super::{Decline, Depth, as_expr, lists};
use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;

/// Methods whose result *string* length is set by an argument, and so are the
/// only ones a single small argument can blow up into text.
///
/// Set by an argument is what makes them answerable *here*, in front of the
/// engine: the guard reads arguments. A method whose result length comes from its
/// receiver adds nothing to that receiver's own length, so it is bounded by
/// whatever set it — which is one call earlier and is [`EntryAmplifier`].
/// `Array(n).fill(0)` is `n` read from the other end, and `fill` itself is
/// innocent.
const LENGTH_AMPLIFYING_METHODS: [&str; 3] = ["repeat", "padStart", "padEnd"];

/// Methods whose callback the language evaluates at most once per element of the
/// receiver, and hands the element to as its first parameter.
///
/// Both halves are what a bound needs. *At most once per element* is what makes
/// the receiver's element count a factor the two amplification rules can
/// multiply by; *first parameter* is what makes the element's width the width of
/// the name a body reads it through.
///
/// A name not listed here leaves a callback unmeasured, which is the refusal
/// every callback used to get — so the list is safe by default and grows only
/// where both halves were checked. `sort` is left out because a comparator runs
/// more often than its array is long, and `reduce` and `reduceRight` with it
/// because the element they hand a callback is its second parameter, so a width
/// read off the receiver would name the accumulator.
const PER_ELEMENT_METHODS: [&str; 10] = [
  "map",
  "flatMap",
  "filter",
  "forEach",
  "some",
  "every",
  "find",
  "findIndex",
  "findLast",
  "findLastIndex",
];

/// The lengths an array may have, as the language's own range.
///
/// A number outside it declares no length at all: `Array(2 ** 32)` is a
/// `RangeError` rather than an array, raised before anything is allocated. Held
/// as a range of floats because that is what the number being checked is, and
/// comparing before any cast keeps a value the cast would saturate from reading
/// as a length.
const VALID_ARRAY_LENGTHS: std::ops::Range<f64> = 0.0..4_294_967_296.0;

impl Walk<'_, '_> {
  /// Whether a length-amplifying call is bounded well enough to evaluate.
  ///
  /// The rule is arithmetic rather than syntax: work out how long a string the
  /// call would build, and refuse when that is past the ceiling. So a count may be
  /// written out, named, or computed — `'x'.repeat(n)` and `'x'.repeat(2 * 2)` are
  /// bounded by reading them, exactly as `'x'.repeat(4)` is — and what stays
  /// refused is a length that cannot be read at all.
  ///
  /// `repeat` multiplies its receiver, so the receiver's own length is half of the
  /// product and a receiver whose length cannot be read leaves it unbounded. A
  /// **call** is the receiver deliberately left unread: its answer is bounded per
  /// link, and multiplying two allowed lengths is exactly how
  /// `"x".repeat(1000000).repeat(1000000)` reaches a length neither of them is.
  /// That is the rule that used to be spelled as "the receiver must not be a
  /// call", and the product is what it was standing in for — a name holding a
  /// bounded string is a receiver it never covered.
  ///
  /// `padStart` and `padEnd` build to their count whatever the receiver holds, so
  /// the count alone bounds them and a chain through one cannot multiply.
  ///
  /// A call inside a callback is bounded by the product rather than refused: the
  /// body runs once per element of the receiver the call around it was written on,
  /// so the bound the source states is one factor and that element count is the
  /// other. A receiver nothing counted leaves the product unbounded, and that is
  /// the remainder the blanket refusal was standing in for all along.
  ///
  /// Reading a length costs a fold that folded before nothing at all. The
  /// name check above answers first, so every call to a method that is not one of
  /// the three pays one scan of three names, exactly as it did. For the three, a
  /// count and a receiver written out are matched as syntax and nothing is
  /// evaluated. The only resolution this rule adds is for a *named* count or
  /// receiver — a call that used to refuse outright — where the read is memoised
  /// and the walk below would make it a moment later anyway.
  pub(super) fn admit_amplification(
    &mut self,
    method: &Atom,
    receiver: &Expr,
    args: &[ExprOrSpread],
  ) -> Result<(), Decline> {
    if !lists(&LENGTH_AMPLIFYING_METHODS, method) {
      return Ok(());
    }

    let ceiling = self.guard.ceilings.characters;
    let unreadable = || Decline::rule(unbounded_amplified_length(method, ceiling));

    let count = match args.first() {
      // `"x".padStart()` amplifies nothing, so there is no length to bound.
      None => return Ok(()),
      // A spread is a count that is not one argument, and the guard refuses a
      // spread everywhere else too.
      Some(ExprOrSpread {
        spread: Some(_), ..
      }) => return Err(unreadable()),
      Some(ExprOrSpread { expr, .. }) => {
        resolved_count(expr, self.reader).ok_or_else(unreadable)?
      },
    };

    // Saturating because the product exists to be refused on, and a wrapped one
    // would admit.
    let per_evaluation = match method == "repeat" {
      true => self
        .receiver_length(receiver)
        .ok_or_else(unreadable)?
        .saturating_mul(count),
      false => count,
    };

    // A read bound bounds one evaluation, and a callback body runs once per element
    // of the receiver the call around it was written on. Where that count was read
    // the product is the bound; where it was not, the same written bound would be
    // multiplied by a number the source never states — `"x".repeat(999999)
    // .split("").map(() => "y".repeat(999999))` is two calls, each inside the
    // bound, building a terabyte between them.
    let repeats = self.guard.repeats.counted("string", method)?;

    if per_evaluation.saturating_mul(repeats) > ceiling {
      return Err(Decline::rule(amplified_length_too_large(
        method,
        count,
        per_evaluation,
        repeats,
        ceiling,
      )));
    }

    Ok(())
  }

  /// What a callback passed to this call would repeat, or `None` where the call
  /// takes no callback the guard counts.
  ///
  /// The receiver is read rather than the argument, because a callback's body runs
  /// once per element of the receiver the call was written on — and by the time
  /// this is asked that receiver has already been admitted, so the read is one the
  /// fold was going to pay for anyway.
  pub(super) fn admitted_callback(&mut self, method: &Atom, receiver: &Expr) -> Option<Callback> {
    if !lists(&PER_ELEMENT_METHODS, method) {
      return None;
    }

    // A callback the guard admits but could not measure is still a callback, so it
    // is `Some` holding the unmeasured default rather than `None`: what tells the
    // two apart is whether an arrow in this position is a body that runs at all.
    Some(self.measured_receiver(receiver).unwrap_or_default())
  }

  /// What a callback over this receiver would repeat and hold, or `None` where the
  /// guard cannot read the receiver at all.
  ///
  /// One reading answers both, so the count and the width come off the same
  /// measurement of the same value and cannot come to disagree.
  fn measured_receiver(&mut self, receiver: &Expr) -> Option<Callback> {
    let depth = self.guard.depth;

    // The evaluator answers an array either as a list of its own or as the literal
    // it was written as, and both are one array here — the same two shapes the
    // inward conversion reads, for the same reason.
    let (elements, characters) = match &module_value_of(receiver, self.reader)? {
      EvaluateResultValue::Vec(items) => (
        items.len(),
        widest_of(items.iter().map(|item| rendered_characters(item, depth))),
      ),
      EvaluateResultValue::Expr(Expr::Array(ArrayLit { elems, .. })) => {
        // A spread stands for however many elements its operand holds, so the
        // written length is not the count — and a count read short is the one
        // reading that would admit a call nothing bounded. The literal arm is
        // answered by written length alone, so a spread has to leave it.
        if elems.iter().flatten().any(|elem| elem.spread.is_some()) {
          return None;
        }

        (
          elems.len(),
          widest_of(elems.iter().map(|elem| rendered_element(elem, depth))),
        )
      },
      _ => return None,
    };

    Some(Callback {
      repeats: self.guard.repeats.per_element(elements as u64),
      characters,
    })
  }

  /// The receiver's own length in UTF-16 code units, or `None` where the guard
  /// cannot read it.
  ///
  /// A string written into the source is measured where it stands; anything else
  /// is resolved, so a name holding a string is a receiver like the literal it was
  /// given the name of. A **call** is refused rather than resolved, which is the
  /// half of this rule that keeps per-link bounds from multiplying across a chain.
  ///
  /// A name a callback binds is neither: the module cannot resolve it, and what it
  /// holds is an element of a receiver the call around the callback measured — so
  /// that element's width is the length, and a name nothing measured has none.
  fn receiver_length(&mut self, receiver: &Expr) -> Option<u64> {
    let text = match without_parens(receiver) {
      Expr::Lit(Lit::Str(text)) => text.value.clone(),
      // A name the callback binds is answered from the element it was handed, and
      // this arm is what makes `['a','b'].map(x => x.repeat(3))` fold at all: the
      // module has no value for `x`, so without it there is no length to read.
      // Asked before the resolution rather than left to it — see
      // [`module_value_of`] for why the module could not answer for it anyway.
      Expr::Ident(ident) if self.guard.scope.binds(&ident.sym) => {
        return self.guard.scope.characters_of(&ident.sym);
      },
      _ => match module_value_of(receiver, self.reader)? {
        EvaluateResultValue::Expr(Expr::Lit(Lit::Str(text))) => text.value,
        _ => return None,
      },
    };

    Some(atom_utf16_length(&text) as u64)
  }

  /// Whether a call declaring an array length is bounded well enough to evaluate.
  ///
  /// The arithmetic [`Walk::admit_amplification`] does, in the other unit and read
  /// off a declaration rather than off a receiver: work out how many elements the
  /// call would build, and refuse when that is past the entry ceiling. It is that
  /// same ceiling which refuses the array on the way *out*, so this changes when
  /// the answer arrives rather than what it is — and when is the whole of the
  /// difference between a refusal that costs half a minute and one that costs
  /// nothing.
  ///
  /// Outside a callback, a length it cannot read is admitted rather than refused —
  /// which is where this parts company with an amplifying method's count. A count
  /// that cannot be read leaves a product nothing bounds; a length that cannot be
  /// read means the argument is a value the guard carried inward, which both
  /// ceilings already bounded. Inside a callback that reasoning does not hold, and
  /// [`Declared::Unreadable`] is why the read answers three things rather than two.
  ///
  /// The unreadable rule is therefore asked *after* the length rather than in front
  /// of it, unlike the string one: a call that declares no length is
  /// `Array('a', 'b')`, whose elements the source wrote out, and refusing that
  /// inside a callback would take away a fold nothing threatens.
  pub(super) fn admit_entry_amplification(
    &mut self,
    amplifier: EntryAmplifier,
    args: &[ExprOrSpread],
  ) -> Result<(), Decline> {
    let declared = amplifier.declared(args, self.reader);

    // A length the guard could not read is refused inside a callback, and only
    // there: that is where the declaration arrives through a parameter, so nothing
    // in front of the engine sees it. `[{length: 100000000}].map(x =>
    // Array.from(x).length)` folded in sixty-eight seconds when this told the two
    // apart by reading nothing.
    //
    // Refused whatever the element count came to, because it is the *length* that
    // is unreadable rather than the repeats: a receiver of one element still
    // declares an array of a hundred million.
    if self.guard.scope.inside_a_callback() && matches!(declared, Declared::Unreadable) {
      return Err(Decline::rule(amplification_inside_a_callback(
        "array",
        amplifier.name(),
      )));
    }

    let Declared::Length(declared) = declared else {
      return Ok(());
    };

    let ceiling = self.guard.ceilings.entries;

    // A callback body runs once per element of the receiver the call around it was
    // written on, so a length written into one declares that many arrays rather
    // than one. Where the receiver was never counted the product is unbounded, and
    // `['a', 'b'].map(x => Array(9999).fill(x))` is one bounded length multiplied
    // by a number the source never states.
    let repeats = self.guard.repeats.counted("array", amplifier.name())?;

    match declared.saturating_mul(repeats) > ceiling {
      true => Err(Decline::rule(amplified_entries_too_large(
        amplifier.name(),
        declared,
        repeats,
        ceiling,
      ))),
      false => Ok(()),
    }
  }
}

/// The value `expr` holds *in the module*, or `None` where the module has none to
/// answer with.
///
/// The one home for what both speculative reads below share: the paren unwrapping
/// and the **call** they each refuse, whose answer is bounded per link so that
/// reading it is what would let two allowed counts multiply into one that is
/// neither.
///
/// **Why a name a callback binds cannot come back from here**, which is what
/// makes reading a receiver inside a callback safe at all. A module
/// `const parts = ['q']` beside `big.map(parts => parts.map(…))` spells one name
/// two ways, and answering the first where the call is made on the second would
/// count one evaluation against ten thousand. It cannot happen: the evaluator
/// resolves a reference through `StateManager::declaration_of`, which is keyed by
/// the full SWC `Id` — the symbol *and* its `SyntaxContext` — so the parameter
/// and the module binding are different keys and the parameter's has no
/// initializer. The resolver, not a check here, is what holds that; this is the
/// place that depends on it, so it is the place that says so.
fn module_value_of(expr: &Expr, reader: &mut Reader) -> Option<EvaluateResultValue> {
  match without_parens(expr) {
    Expr::Call(_) => None,
    _ => reader.resolve(expr),
  }
}

/// One written array element's rendered width.
///
/// A hole renders to nothing, which is what the language's own join does with it;
/// a spread stands for a count the source does not state, so it has no width.
/// Read from one place because both the receiver's own elements and a nested
/// array's are the same question.
fn rendered_element(elem: &Option<ExprOrSpread>, depth: Depth) -> Option<u64> {
  match elem {
    Some(ExprOrSpread { spread: None, expr }) => rendered_expr(expr, depth),
    Some(_) => None,
    None => Some(0),
  }
}

/// The widest of a receiver's elements, or `None` where one of them renders to a
/// width the guard could not read.
///
/// Any one unreadable element gives up on all of them, because which element a
/// callback's parameter will hold is not something the guard chooses.
fn widest_of(mut widths: impl Iterator<Item = Option<u64>>) -> Option<u64> {
  widths.try_fold(0, |widest, width| Some(widest.max(width?)))
}

/// How many characters one resolved value renders to under the language's own
/// `ToString`, or `None` where the guard cannot read it.
///
/// An object is one of those: it renders to `[object Object]` whatever it holds,
/// and treating that as a width would put a number in front of the engine that
/// says nothing about the value. A refusal is the honest answer.
fn rendered_characters(value: &EvaluateResultValue, depth: Depth) -> Option<u64> {
  match value {
    EvaluateResultValue::Expr(expr) => rendered_expr(expr, depth),
    EvaluateResultValue::Vec(items) => {
      let inner = depth.descend().ok()?;

      joined(
        items.len(),
        items.iter().map(|item| rendered_characters(item, inner)),
      )
    },
    _ => None,
  }
}

/// The same for a value the evaluator answered as the expression it was written
/// as.
fn rendered_expr(expr: &Expr, depth: Depth) -> Option<u64> {
  let inner = depth.descend().ok()?;

  match expr {
    Expr::Lit(Lit::Str(text)) => Some(atom_utf16_length(&text.value) as u64),
    // Read through the conversion every other number-to-string in this compiler
    // uses, so the width is the one the engine will actually build.
    Expr::Lit(Lit::Num(number)) => Some(to_js_string(number.value).len() as u64),
    Expr::Lit(Lit::Bool(truth)) => Some(match truth.value {
      true => "true".len() as u64,
      false => "false".len() as u64,
    }),
    Expr::Lit(Lit::Null(_)) => Some("null".len() as u64),
    // The value the grammar has no literal for, which a callback parameter can
    // hold like any other element. Its `ToString` is the name itself, and that
    // is the width to read: a join renders it as nothing instead, so this is
    // the wider of the two readings and therefore the one a ceiling is safe to
    // be told.
    Expr::Ident(ident) if is_js_undefined(ident) => Some("undefined".len() as u64),
    Expr::Array(ArrayLit { elems, .. }) => joined(
      elems.len(),
      elems.iter().map(|elem| rendered_element(elem, inner)),
    ),
    _ => None,
  }
}

/// What a list of rendered widths comes to once the language joins them with a
/// comma, or `None` where one of them could not be read.
fn joined(count: usize, mut widths: impl Iterator<Item = Option<u64>>) -> Option<u64> {
  let separators = count.saturating_sub(1) as u64;

  widths.try_fold(separators, |total, width| {
    Some(total.saturating_add(width?))
  })
}

/// The count an amplifying call asks for, or `None` where the argument is not a
/// number this guard can read.
///
/// A literal is answered where it stands, because that is the common spelling
/// and reading it costs nothing. Anything else is resolved through the
/// evaluator, which is a [speculative read](../../../../../CONTEXT.md) like
/// every other the guard makes — and one the fold would pay for anyway, since a
/// call it admits evaluates the same argument a moment later.
///
/// Whatever it resolves to then goes through the compiler's own `ToNumber`,
/// because that is what the language does to it: `'x'.repeat('3')` repeats three
/// times and `'x'.repeat('lots')` repeats none. Reading the count any other way
/// would refuse an input the reference compiler folds, and bound a call by a
/// number the engine is not going to use.
fn resolved_count(expr: &Expr, reader: &mut Reader) -> Option<u64> {
  let resolved = match expr {
    Expr::Lit(_) => return count_of(to_js_number(expr)?),
    _ => as_expr(&reader.resolve(expr)?)?,
  };

  count_of(to_js_number(&resolved)?)
}

/// One resolved number as the bound it puts on what a call will build.
///
/// Truncated toward zero and floored there, which is what the language's own
/// `ToIntegerOrInfinity` does to it — so a fractional or negative count is
/// bounded exactly as the engine will read it, and the `RangeError` a negative
/// one really produces is left to the language to say, in its own words. A count
/// that is infinite is not a bound at all; `NaN` is zero, because `f64::max`
/// answers zero for one and so does the language.
fn count_of(value: f64) -> Option<u64> {
  match value.is_infinite() {
    true => None,
    false => Some(value.trunc().max(0.0) as u64),
  }
}

/// The two calls whose result is one array element per unit of a length an
/// argument declares.
///
/// `Array(n)` is why this exists. The array it makes is *sparse*, so it looks
/// free and is: nothing is allocated until a later call in the chain fills,
/// copies, sorts or joins it, and by then the length is the engine's rather than
/// the guard's. The refusal still arrives — the entry ceiling reads the answer on
/// the way out — but it arrives after half a minute of work rather than before
/// it, which is the failure the ceilings were put in to prevent. Bounding the
/// declaration bounds every call that would go on to materialise one.
///
/// `Array.from(x)` is the same length read one property along, off `x`'s own. It
/// is here for `{ length: n }`, the object that declares a length without
/// holding it; a string or an array handed to `from` was either written out or
/// carried inward, and both of those the ceilings already bounded.
///
/// Every other name was measured against the same question — what is the result
/// length a function of? — and left out for the answer:
///
/// - `fill`, `copyWithin`, `reverse` and `sort` answer their receiver's own
///   length, so they add nothing to it.
/// - `slice`, `splice`, `concat` and `flat` answer a length no larger than the
///   elements their receiver and their arguments already hold.
/// - `map`, `filter` and `join` answer one element, or one element's text, per
///   element of a receiver.
///
/// Each of those is a length something already paid for, so the two below are the
/// whole of what a fold can be asked to build for free.
#[derive(Clone, Copy)]
pub(super) enum EntryAmplifier {
  /// `Array(n)`, whose length is its single numeric argument.
  Constructor,
  /// `Array.from(x)`, whose length is the one `x` declares.
  From,
}

impl EntryAmplifier {
  /// The amplifier a call on an unshadowed global names, or `None` where the call
  /// declares no length.
  ///
  /// `method` is the static read off the global, and `None` where the global is
  /// applied as a function. One recogniser rather than a test at each of the two
  /// call sites, so the names are written once and the two sites cannot come to
  /// disagree about which spellings this rule owns.
  pub(super) fn named(global: &Atom, method: Option<&Atom>) -> Option<Self> {
    match (&**global, method.map(|method| &**method)) {
      ("Array", None) => Some(Self::Constructor),
      ("Array", Some("from")) => Some(Self::From),
      _ => None,
    }
  }

  /// How the call is spelled, which is what a refusal names.
  fn name(self) -> &'static str {
    match self {
      Self::Constructor => "Array",
      Self::From => "Array.from",
    }
  }

  /// What the guard could read about the length the call declares.
  fn declared(self, args: &[ExprOrSpread], reader: &mut Reader) -> Declared {
    match self {
      Self::Constructor => constructor_length(args, reader),
      Self::From => length_property(args.first(), reader),
    }
  }
}

/// What the guard could read about the length a call declares.
///
/// Three answers rather than two, because the third is a rule of its own. Outside
/// a callback an unreadable length is the same as no length: the argument that
/// would say is a value the guard carried inward, and both ceilings already
/// bounded that. Inside one it is the dangerous case — `[{ length: 100000000 }]
/// .map(x => Array.from(x).length)` reaches the declaration through a parameter
/// the guard cannot resolve, and folded in sixty-eight seconds when this told the
/// two apart by returning `None` for both.
enum Declared {
  /// The elements the call will build.
  Length(u64),
  /// The call declares no length: its arguments are elements the source wrote
  /// out, or an array-like holding what it says it holds.
  Nothing,
  /// The argument that would say is not one the guard can read.
  Unreadable,
}

/// What `Array(n)` declares about the length of its array.
///
/// `Array` declares a length only when it is handed exactly one argument and
/// that argument *is* a number: `Array(3)` is three holes, where `Array('3')` is
/// one element holding a string and `Array('a', 'b')` is two elements. So the
/// number is read as the language reads it rather than through `ToNumber`, which
/// is where this parts company with [`resolved_count`] — `'x'.repeat('3')`
/// repeats three times and `Array('3')` does not.
///
/// A number that is not a valid array length — fractional, negative, `NaN`,
/// infinite, or `2 ** 32` and up — declares nothing either, and is left to the
/// language: `Array` answers each of them with a `RangeError` before allocating,
/// so a ceiling in front of it would replace the accurate sentence with a
/// misleading one.
fn constructor_length(args: &[ExprOrSpread], reader: &mut Reader) -> Declared {
  // More than one argument, or none, is elements the source wrote out. So is a
  // spread, which is refused where it is written in any case.
  let [ExprOrSpread { spread: None, expr }] = args else {
    return Declared::Nothing;
  };

  let number = match expr.as_ref() {
    Expr::Lit(Lit::Num(number)) => number.value,
    _ => match reader.resolve(expr) {
      Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(number)))) => number.value,
      // A value that resolved and is not a number is the single element it will
      // become; one that did not resolve is the length nobody can see.
      Some(_) => return Declared::Nothing,
      None => return Declared::Unreadable,
    },
  };

  match valid_array_length(number) {
    Some(length) => Declared::Length(length),
    None => Declared::Nothing,
  }
}

/// What an `Array.from` argument declares about the length it will build.
///
/// `{ length: n }` is a length declared without being held, which is what
/// `Array(n)` is bounded for one call earlier. The argument is *resolved* rather
/// than read as syntax, so a name holding the object and a spread that builds one
/// are the object they come to — the same reading the rest of this guard makes,
/// and the reason `{ ...{ length: n } }` is not a way round the bound.
///
/// The last `length` property wins, because that is the one the object ends up
/// with.
fn length_property(arg: Option<&ExprOrSpread>, reader: &mut Reader) -> Declared {
  // `Array.from()` with nothing to iterate throws, and a spread is refused where
  // it is written.
  let Some(ExprOrSpread { spread: None, expr }) = arg else {
    return Declared::Nothing;
  };

  let resolved = match reader.resolve(expr) {
    Some(resolved) => resolved,
    None => return Declared::Unreadable,
  };

  // Anything that is not an object holds what its length says — a string or an
  // array — and the ceilings bounded it where it was written or carried.
  let EvaluateResultValue::Expr(Expr::Object(object)) = resolved else {
    return Declared::Nothing;
  };

  let length = object.props.iter().rev().find_map(|prop| match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::KeyValue(KeyValueProp { key, value }) if is_a_length_key(key) => Some(value),
      _ => None,
    },
    PropOrSpread::Spread(_) => None,
  });

  // An object with no own `length` is the empty array, and one whose length the
  // language will not accept is a throw it raises itself. Both declare nothing
  // this guard has to bound. `trunc` is `ToLength`'s own truncation, which is
  // what makes `{ length: 1.9 }` a length of one rather than none.
  match length
    .and_then(|length| to_js_number(length))
    .and_then(|number| valid_array_length(number.trunc()))
  {
    Some(length) => Declared::Length(length),
    None => Declared::Nothing,
  }
}

/// Whether a property name is the `length` an array-like declares.
///
/// Both spellings of the one key, because `{ 'length': n }` declares what
/// `{ length: n }` does. A computed key is not read: the evaluator answers a
/// resolved object, whose keys are settled by the time this sees them.
fn is_a_length_key(key: &PropName) -> bool {
  match key {
    PropName::Ident(name) => name.sym == "length",
    PropName::Str(name) => name.value.as_str() == Some("length"),
    _ => false,
  }
}

/// One number as the array length the language would make of it, or `None` where
/// the language rejects it instead.
///
/// Shared by both readers, because the range is the language's rather than this
/// guard's: a length outside it raises a `RangeError` before anything is
/// allocated — `Array` from its argument, `Array.from` from `ArrayCreate` — so
/// falling through to that costs nothing and says more than a ceiling could.
/// `Array.from({ length: Infinity })` is the case that makes it worth sharing:
/// bounded here it would name `2 ** 53 - 1`, a number the language never reaches
/// because it refuses the length first.
///
/// The two readers differ only in how they arrive at the number — `Array(n)`
/// takes its argument as written, and an array-like's `length` comes through
/// `ToLength` — which is why the range is checked here and the coercion is not.
fn valid_array_length(number: f64) -> Option<u64> {
  match VALID_ARRAY_LENGTHS.contains(&number) && number.fract() == 0.0 {
    true => Some(number as u64),
    false => None,
  }
}
