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
//!
//! That count belongs to the receiver rather than to the method reading it, so
//! it is taken off the receiver's own value and the method is asked only where
//! its callback is not the ordinary once-per-element one — see
//! [`element_parameter_of`]. A method nobody wrote down is therefore measured
//! like every other, which is the whole of why there is no list of them here.

use swc_core::{
  atoms::Atom,
  ecma::ast::{
    ArrayLit, BinExpr, BinaryOp, Expr, ExprOrSpread, KeyValueProp, Lit, Prop, PropName,
    PropOrSpread,
  },
};

use stylex_ast::ast::convertors::{atom_utf16_length, is_js_undefined};
use stylex_constants::constants::evaluation_errors::{
  amplification_inside_a_callback, amplified_entries_too_large, amplified_length_too_large,
  unbounded_amplified_length,
};
use stylex_js::coercions::to_js_number;
use stylex_utils::number::to_js_string;

use super::guard::{Bounds, Callback, Reader, Walk, without_parens};
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

/// The lengths an array may have, as the language's own range.
///
/// A number outside it declares no length at all: `Array(2 ** 32)` is a
/// `RangeError` rather than an array, raised before anything is allocated. Held
/// as a range of floats because that is what the number being checked is, and
/// comparing before any cast keeps a value the cast would saturate from reading
/// as a length.
const VALID_ARRAY_LENGTHS: std::ops::Range<f64> = 0.0..4_294_967_296.0;

/// How wide `undefined` renders under the language's own `ToString`.
///
/// Two spellings reach it — the identifier an author writes and the hole they
/// leave — and both have to answer the same number, because they are the same
/// value. One name so they cannot drift apart.
///
/// A join renders the value as nothing instead, so this is the wider of the two
/// readings. That is the direction a ceiling is safe to be told: reading a width
/// short admits a call nothing bounded, where reading it long only refuses
/// sooner. `null` beside it is read the same way, for the same reason.
const UNDEFINED_WIDTH: u64 = "undefined".len() as u64;

/// How wide one element of a string iterated by code point can be.
///
/// Two UTF-16 units, because a code point outside the basic plane is a surrogate
/// pair and `Array.from` hands the mapper both halves as one element.
const SURROGATE_PAIR_WIDTH: u64 = 2;

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
      Some(ExprOrSpread { expr, .. }) => self.count_bound(expr).ok_or_else(unreadable)?,
    };

    // A repeat of one is the receiver itself and a repeat of none is nothing, so
    // neither builds a character the receiver had not already been paid for --
    // and asking for its length would refuse a call that amplifies nothing.
    if method == "repeat" && count <= 1 {
      return Ok(());
    }

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
  /// hands the method no function to run.
  ///
  /// The receiver is read rather than the argument, because a callback's body runs
  /// once per element of the receiver the call was written on — and by the time
  /// this is asked that receiver has already been admitted, so the read is one the
  /// fold was going to pay for anyway.
  ///
  /// **The method is asked only where its callback is not the ordinary one.** The
  /// count belongs to the receiver, so a method the guard has never heard of is
  /// measured like every other rather than refused for being absent from a list —
  /// which is what [`element_parameter_of`] is for, and the whole of the
  /// difference between this and the ten-name table it replaced.
  pub(super) fn admitted_callback(
    &mut self,
    method: &Atom,
    receiver: &Expr,
    args: &[ExprOrSpread],
  ) -> Option<Callback> {
    if !hands_over_a_function(args) {
      return None;
    }

    let Some(element_at) = element_parameter_of(method) else {
      return Some(Callback::default());
    };

    // A callback the guard admits but could not measure is still a callback, so it
    // is `Some` holding the unmeasured default rather than `None`: what tells the
    // two apart is whether an arrow in this position is a body that runs at all.
    Some(
      self
        .measured_receiver(receiver, element_at)
        .unwrap_or_default(),
    )
  }

  /// The same for the mapper `Array.from` takes, whose elements come from the
  /// call's first argument rather than from a receiver.
  ///
  /// The mapper is the second argument, so a call handing `from` nothing else
  /// runs no callback at all and is left alone.
  ///
  /// **The ordering here is not the receiver's.** A receiver is admitted before
  /// its count is taken; an argument is walked *after*, so the read below happens
  /// in front of the source's own bounds. It costs nothing extra all the same,
  /// because [`length_property`] has already resolved the very same expression
  /// one step earlier — the entry ceiling is compared against a declared length
  /// before the engine runs, and that comparison is what does the resolving. So
  /// this adds a memo hit rather than a walk, and the exposure that is there is
  /// the entry rule's own and older than this reading.
  pub(super) fn admitted_mapper(&mut self, args: &[ExprOrSpread]) -> Option<Callback> {
    let [source, mapper @ ..] = args else {
      return None;
    };

    if !hands_over_a_function(mapper) {
      return None;
    }

    Some(self.measured_source(&source.expr).unwrap_or_default())
  }

  /// What a callback over this receiver would repeat and hold, or `None` where the
  /// guard cannot read the receiver at all.
  ///
  /// One reading answers all three, so the count, the width and the largest index
  /// come off the same measurement of the same value and cannot come to disagree.
  fn measured_receiver(&mut self, receiver: &Expr, element_at: usize) -> Option<Callback> {
    let depth = self.guard.depth;

    // The evaluator answers an array either as a list of its own or as the literal
    // it was written as, and both are one array here — the same two shapes the
    // inward conversion reads, for the same reason.
    let (elements, element) = match &countable_value_of(receiver, self.reader)? {
      EvaluateResultValue::Vec(items) => (
        items.len(),
        Bounds {
          characters: greatest_of(items.iter().map(|item| rendered_characters(item, depth))),
          magnitude: greatest_of(items.iter().map(number_held_by)),
        },
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
          Bounds {
            characters: greatest_of(elems.iter().map(|elem| rendered_element(elem, depth))),
            magnitude: greatest_of(elems.iter().map(number_written_as)),
          },
        )
      },
      _ => return None,
    };

    Some(self.callback_over(elements as u64, element, element_at))
  }

  /// The same for a value `Array.from` iterates, which is an array or one of two
  /// shapes only this call counts as one.
  ///
  /// A **string** is iterated by code point, so its UTF-16 length is at least the
  /// count and each element is one or two of those units wide. An **array-like**
  /// declares a length it does not hold, and every element the mapper is handed is
  /// `undefined` — the same length [`EntryAmplifier::From`] is bounded by one step
  /// earlier, read here for the count it also settles.
  fn measured_source(&mut self, source: &Expr) -> Option<Callback> {
    if let Some(measured) = self.measured_receiver(source, 0) {
      return Some(measured);
    }

    match countable_value_of(source, self.reader)? {
      EvaluateResultValue::Expr(Expr::Lit(Lit::Str(text))) => Some(self.callback_over(
        atom_utf16_length(&text.value) as u64,
        Bounds {
          characters: Some(SURROGATE_PAIR_WIDTH),
          magnitude: None,
        },
        0,
      )),
      resolved => match declared_length_of(&resolved) {
        Declared::Length(length) => Some(self.callback_over(
          length,
          Bounds {
            characters: Some(UNDEFINED_WIDTH),
            magnitude: None,
          },
          0,
        )),
        _ => None,
      },
    }
  }

  /// One measurement of a receiver as what the callback over it may do.
  ///
  /// The index is settled by the same count as the repeats — the largest one a
  /// receiver of `elements` has is one below its length — so it is read here
  /// rather than at each of the two places a count is taken.
  fn callback_over(&self, elements: u64, element: Bounds, element_at: usize) -> Callback {
    let last = elements.saturating_sub(1);

    Callback {
      repeats: self.guard.repeats.per_element(elements),
      element,
      index: Bounds {
        characters: Some(to_js_string(last as f64).len() as u64),
        magnitude: Some(last),
      },
      element_at,
    }
  }

  /// The largest count an amplifying call can ask for, or `None` where the guard
  /// cannot read one.
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
  ///
  /// A count the module cannot resolve is the one a callback writes: `n` is an
  /// element of a receiver the call around the body measured, and `i + 1` is that
  /// receiver's index one step along. Neither has a value here — the engine binds
  /// both — but each has a *ceiling*, taken off the same measurement the repeats
  /// were, and a ceiling is all a bound ever needed.
  fn count_bound(&mut self, expr: &Expr) -> Option<u64> {
    if let Expr::Lit(_) = expr {
      return count_of(to_js_number(expr)?);
    }

    match self.reader.resolve(expr) {
      Some(value) => count_of(to_js_number(&as_expr(&value)?)?),
      None => self.numeric_bound(expr),
    }
  }

  /// The largest number `expr` can come to, or `None` where the guard cannot see
  /// that it is a number at all.
  ///
  /// Narrower than [`Walk::count_bound`] on purpose, and the narrowing is what
  /// makes the arithmetic sound. Every leaf here is a value the guard has seen is
  /// a number and is not below zero, so `+` really is addition rather than
  /// concatenation and both operations carry a bound on their operands through to
  /// a bound on their result. A leaf that is anything else stops the reading,
  /// which costs a fold rather than admitting one nothing measured.
  fn numeric_bound(&mut self, expr: &Expr) -> Option<u64> {
    match without_parens(expr) {
      Expr::Lit(_) => number_of(expr),
      // A name the callback binds is the element the receiver was measured for,
      // or that element's index.
      Expr::Ident(ident) if self.guard.scope.binds(&ident.sym) => {
        self.guard.scope.bounds_of(&ident.sym)?.magnitude
      },
      Expr::Bin(BinExpr {
        op: op @ (BinaryOp::Add | BinaryOp::Mul),
        left,
        right,
        ..
      }) => {
        let left = self.numeric_bound(left)?;
        let right = self.numeric_bound(right)?;

        Some(match op {
          BinaryOp::Add => left.saturating_add(right),
          _ => left.saturating_mul(right),
        })
      },
      // A name the module holds, which is a leaf like a written number once the
      // evaluator has answered for it.
      other => number_of(&as_expr(&self.reader.resolve(other)?)?),
    }
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
        return self.guard.scope.bounds_of(&ident.sym)?.characters;
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

/// The value `expr` holds, read for how many elements it has rather than for how
/// long it is.
///
/// A **call** is resolved here where [`module_value_of`] refuses one, and the
/// difference is what each reading would cost. A length is read off a receiver
/// whose own answer is bounded per link, so resolving a call there is what would
/// let two allowed lengths multiply into one that is neither —
/// `"x".repeat(1000000).repeat(1000000)` builds the very string the bound exists
/// to prevent. A count is taken off a receiver the guard has already admitted,
/// so whatever it resolves to is inside both ceilings and the fold is about to
/// build it anyway.
///
/// That second half is [`Walk::admit_call`]'s ordering to keep — the count is read
/// where the receiver has just been admitted — and the two readings are named
/// apart so a later edit cannot swap one for the other without meeting this
/// sentence.
fn countable_value_of(expr: &Expr, reader: &mut Reader) -> Option<EvaluateResultValue> {
  reader.resolve(expr)
}

/// Where a method hands its callback an element of the receiver, or `None` where
/// the language does not run that callback once per element at all.
///
/// **Measuring is the default**, which is the whole of this rule: the count comes
/// off the receiver, so it is the same count whatever the method does with it,
/// and a method nobody wrote down folds rather than refusing. What is named here
/// is only the two families whose arithmetic differs:
///
/// - A **comparator** runs once per comparison, which for any sort worth using is
///   more often than the array is long. Nothing here counts comparisons, so the
///   callback is admitted with no count and a body that amplifies inside one
///   refuses — the answer every callback used to get, kept for the one shape that
///   still earns it.
/// - **`reduce`** and **`reduceRight`** hand the accumulator first and the element
///   second, so a width read off the receiver belongs to the second parameter.
///
/// Nothing else in the language runs a callback more often than its receiver is
/// long. A method added to it that did would be measured short here, and the
/// growth would then be refused on the way out of the engine instead of in front
/// of it — later and slower, but refused.
fn element_parameter_of(method: &Atom) -> Option<usize> {
  match &**method {
    "sort" | "toSorted" => None,
    "reduce" | "reduceRight" => Some(1),
    _ => Some(0),
  }
}

/// Whether any of these arguments could be a function the call runs.
///
/// A syntax check and deliberately a loose one: it decides whether measuring the
/// receiver is worth a read, not whether the call takes a callback. An arrow and
/// a function expression are written in place, and a name is the third spelling —
/// [`Walk::admit_a_named_function`](super::guard) is what makes one reach a body.
/// Everything else is an argument no method calls, and a call handing a method
/// only those is left unmeasured because there is nothing there to price.
fn hands_over_a_function(args: &[ExprOrSpread]) -> bool {
  args.iter().any(|arg| {
    matches!(
      without_parens(&arg.expr),
      Expr::Arrow(_) | Expr::Fn(_) | Expr::Ident(_)
    )
  })
}

/// One written array element's rendered width.
///
/// A hole is `undefined`, so its width is that value's and not nothing: a
/// callback handed the element renders it as the name. A spread stands for a
/// count the source does not state, so it has no width. Read from one place
/// because both the receiver's own elements and a nested array's are the same
/// question.
///
/// No input reaches the hole arm today. The evaluator refuses an array carrying
/// one at any depth, so nothing this resolves can hold a hole — see
/// `tests/array_hole_tests.rs`, which is the rule rather than an accident of
/// ordering. It answers the value's width all the same, because the reading a
/// dead arm holds is what the branch would be admitting on the day that rule is
/// relaxed, and nothing would then flag it. Nothing renders as zero characters,
/// which is what it used to claim: a width read short admits a call no ceiling
/// bounded, where reading it long only refuses sooner.
fn rendered_element(elem: &Option<ExprOrSpread>, depth: Depth) -> Option<u64> {
  match elem {
    Some(ExprOrSpread { spread: None, expr }) => rendered_expr(expr, depth),
    Some(_) => None,
    None => Some(UNDEFINED_WIDTH),
  }
}

/// The largest of a receiver's elements read one way, or `None` where one of them
/// could not be read at all.
///
/// Any one unreadable element gives up on all of them, because which element a
/// callback's parameter will hold is not something the guard chooses. Both units
/// an element is read in — the characters it renders to and the number it is —
/// are bounded by the same reasoning, so both come through here.
fn greatest_of(mut readings: impl Iterator<Item = Option<u64>>) -> Option<u64> {
  readings.try_fold(0, |greatest, reading| Some(greatest.max(reading?)))
}

/// The number one resolved element *is*, or `None` where the guard cannot see
/// that it is a number.
///
/// Written out as a number is the whole of what counts. A string that coerces to
/// one is left unread on purpose: `'2' + 1` is `'21'` and not `3`, so a bound
/// taken off a value the guard only knows the *coercion* of is no bound once the
/// source adds to it.
fn number_held_by(value: &EvaluateResultValue) -> Option<u64> {
  match value {
    EvaluateResultValue::Expr(expr) => number_of(expr),
    _ => None,
  }
}

/// The same for one element as the source wrote it.
///
/// A hole is `undefined`, whose `ToNumber` is `NaN` and which no arithmetic
/// recovers a number from, so it reads as no number rather than as zero.
fn number_written_as(elem: &Option<ExprOrSpread>) -> Option<u64> {
  match elem {
    Some(ExprOrSpread { spread: None, expr }) => number_of(expr),
    _ => None,
  }
}

/// One written number as the largest count it can stand for, or `None` where it
/// is not a number, or is one no count can be taken from.
///
/// Negative is refused rather than clamped, which is what makes a sum or a
/// product of two of these a bound at all: both operations are only monotone over
/// values that are not below zero, and `(-5) * (-5)` is the reading that would
/// otherwise come to twenty-five against a bound of nothing.
///
/// Rounded **up**, where [`count_of`] truncates, and the difference is what each
/// number is for. A count is truncated because that is what the language does to
/// it and the reading is the call's own. This one is arithmetic's input, and the
/// truncation happens to the result rather than to the parts: `0.9 * 2000000` is
/// one million eight hundred thousand characters, which a bound of `0 * 2000000`
/// admits and a bound of `1 * 2000000` refuses.
fn number_of(expr: &Expr) -> Option<u64> {
  let Expr::Lit(Lit::Num(number)) = without_parens(expr) else {
    return None;
  };

  match number.value >= 0.0 && number.value.is_finite() {
    true => Some(number.value.ceil() as u64),
    false => None,
  }
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
    // hold like any other element.
    Expr::Ident(ident) if is_js_undefined(ident) => Some(UNDEFINED_WIDTH),
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

  match reader.resolve(expr) {
    Some(resolved) => declared_length_of(&resolved),
    None => Declared::Unreadable,
  }
}

/// The same read off a value the evaluator has already answered.
///
/// Split from [`length_property`] because the count a mapper repeats asks the
/// same question of the same object, one argument along — see
/// [`Walk::measured_source`].
fn declared_length_of(resolved: &EvaluateResultValue) -> Declared {
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
