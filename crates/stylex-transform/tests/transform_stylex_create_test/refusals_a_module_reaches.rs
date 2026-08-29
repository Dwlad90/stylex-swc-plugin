//! A module for every sentence this compiler can print.
//!
//! A diagnostic nobody can reach is worse than no diagnostic: it reads as
//! covered, it survives refactors that should have deleted it, and its wording
//! is never checked against what an author actually sees. So each refusal below
//! is reached the way an author reaches it -- a module goes in, and the sentence
//! comes out -- rather than by asserting the shape of a constant.
//!
//! Two refusals are deliberately absent, because no module reaches them: the
//! argument-count guard in the conversions behind the fold and their `ToObject`
//! refusal are invariant breaks, and each names its invariant where it is
//! written rather than being reached here by a contrived input.
//!
//! Every case was measured against `@stylexjs/babel-plugin` 0.19.0 and says
//! which way the two compilers went. Message text is not a parity obligation
//! (`ADR 0008`); what is asserted here is that the sentence exists and says what
//! it means.

use crate::utils::{
  prelude::*,
  transform::{assert_refuses, assert_refuses_under, base_style_module, stringify_js},
};

/// A binding holding an arrow the fold cannot carry, so a call written around it
/// is handed back and the dispatch below answers.
const AN_UNCARRIABLE_VALUE: &str = "const helpers = { identity: (x) => x };";

/// A theme import, the other shape a fold hands back.
const A_THEME_IMPORT: &str = "import { colors } from 'colors.stylex.js';";

// ──────────────────────────────────────────────
// A value no conversion can coerce
// ──────────────────────────────────────────────

/// A function read off an object, passed to `String`. It is the plainest shape
/// that reaches the coercion with nothing to coerce: a function's only string
/// form is its source text, and this compiler keeps none.
///
/// The reference compiler folds it -- to the *source of its own evaluator's*
/// closure, which is neither the author's function nor a declaration anybody
/// wants -- so both compilers are wrong to fold and only one of them does.
#[test]
fn a_function_reaching_the_string_conversion_names_the_callee() {
  assert_refuses(
    AN_UNCARRIABLE_VALUE,
    "color: String(helpers.identity)",
    "Cannot coerce this value at compile time.\nOnly static values can be passed to String().",
  );
}

/// The same refusal reached through a StyleX function read as a value. This is
/// the injected function map's single-entry shape crossing the bridge inward:
/// the bridge does not carry it, so the call is handed back rather than refused,
/// and the conversion below the fold is what answers.
#[test]
fn a_stylex_function_read_as_a_value_reaches_the_same_refusal() {
  assert_refuses(
    "",
    "color: String(stylex.when)",
    "Only static values can be passed to String().",
  );
}

/// And through a callback: an arrow written inline is admitted by the guard on
/// its own, so the namespace is passed beside it to hand the whole call back.
/// What arrives at the conversion is then the evaluator's own callback rather
/// than the arrow's syntax -- the third and last of the shapes the bridge hands
/// back.
#[test]
fn a_callback_reaching_the_string_conversion_reaches_the_same_refusal() {
  assert_refuses(
    "",
    "color: String((x) => x, stylex)",
    "Only static values can be passed to String().",
  );
}

/// `Number` has a refusal of its own, and it is not a function that reaches it:
/// every function has a number, `NaN`. What has neither is a string holding an
/// unpaired surrogate, which no compile-time text can hold -- so the conversion
/// naming itself is what says which of the two arguments the author has to
/// change.
#[test]
fn a_string_with_no_number_names_the_number_conversion() {
  assert_refuses(
    AN_UNCARRIABLE_VALUE,
    r"color: Number('\uD800', helpers.identity)",
    "Cannot coerce this value at compile time.\nOnly static values can be passed to Number().",
  );
}

// ──────────────────────────────────────────────
// A property the dispatch cannot determine
// ──────────────────────────────────────────────

/// A method called on the namespace object a conversion handed back. The
/// namespace is this compiler's own function map rather than a JavaScript
/// object, so it carries no methods for the call to find.
///
/// The reference compiler folds it to `[object Object]`, having a plain object
/// where this compiler has a map of Rust functions.
#[test]
fn a_method_on_a_value_with_no_methods_says_so() {
  assert_refuses(
    "",
    "color: Object(stylex).toString()",
    "Unexpected error:\nCould not determine the property being accessed.",
  );
}

/// The same lookup written as a computed key, which is a separate arm and was
/// separately unreached. A theme reference is the receiver an author actually
/// writes this on.
#[test]
fn a_computed_method_on_a_value_with_no_methods_says_so() {
  assert_refuses(
    A_THEME_IMPORT,
    "content: colors['toString']()",
    "Unexpected error:\nCould not determine the property being accessed.",
  );
}

// ──────────────────────────────────────────────
// A global standing where a value belongs
// ──────────────────────────────────────────────

/// A global handed to a method as its callback. The bridge carries values and a
/// global is not one, so the guard names it here rather than letting the
/// dispatch below report a constant nothing declared.
///
/// The reference compiler refuses the input too, in a sentence about the call
/// rather than about the name. The whole surface is in
/// `globals_written_as_a_value`; this is the sentence's registration.
#[test]
fn a_global_written_as_a_value_names_itself() {
  assert_refuses(
    "",
    "fontFamily: ['Arial', false].filter(Boolean).join(', ')",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// A value too wide to carry into a fold
// ──────────────────────────────────────────────

/// The entry ceiling, reached by a named array. Entries and characters are two
/// costs rather than one -- ten empty strings hold no text and are still ten
/// values to build -- so this refusal counts what the other one weighs, and says
/// so in its second line.
///
/// Written against a lowered ceiling because the shipped default is ten
/// thousand: a case that reached it by writing ten thousand elements out would
/// be measuring the parser. The reference compiler has no such ceiling and folds
/// the module.
#[test]
fn a_named_array_past_the_entry_ceiling_counts_what_it_holds() {
  assert_refuses_under(
    "const palette = ['a','b','c','d','e','f','g','h','i','j'];",
    "content: palette.join('-')",
    concat!(
      "Cannot carry the value of 'palette' into a fold.\n",
      "At most 8 elements and properties are supported."
    ),
    |module| {
      stringify_js(module, ts_syntax(), |tr| {
        theme_import_transform_with(tr.comments.clone(), |builder| {
          builder.with_max_folded_entries(8)
        })
      })
    },
  );
}

// ──────────────────────────────────────────────
// A read that escapes the value it is written on
// ──────────────────────────────────────────────

/// `s.constructor` is `String`, whose own `constructor` compiles source text
/// into a function -- so the read is refused whether or not a call sits on the
/// end of it. It used to be refused only with a call, and a bare read answered
/// `Unexpected error:` one property later, which names the syntax rather than
/// the rule.
///
/// The sentence names the property that escaped rather than the one after it,
/// so a chain is not reported for its last link.
///
/// The reference compiler folds this one, to `"String"`. The divergence is the
/// escaping-property rule and is argued where the rule is written: refusing
/// costs the answer to a read no stylesheet needs, and folding it hands a build
/// a route to the language's own function graph.
#[test]
fn an_escaping_read_names_the_rule_rather_than_the_property_after_it() {
  let reads = [
    ("s.constructor.name", "constructor"),
    ("s['constructor'].name", "constructor"),
    ("s.trim.bind", "bind"),
  ];

  for (read, property) in reads {
    assert_refuses(
      "const s = 'abc';",
      &format!("content: {}", read),
      &format!(
        concat!(
          "Cannot fold a read of '{}' at compile time.\n",
          "It leads off the value that was written and onto the language's own ",
          "function graph."
        ),
        property
      ),
    );
  }
}

// ──────────────────────────────────────────────
// A length the language itself refuses
// ──────────────────────────────────────────────

/// `Array(-1)` is a `RangeError` in every JavaScript engine, so the engine's own
/// sentence is what an author reads, under this compiler's naming of the call
/// that produced it. Deliberately the engine's words and not a rule of this
/// compiler's: the two ceilings this module owns are about what a fold may
/// allocate, and a negative length is not one of them -- it is the language
/// refusing, and a sentence of our own would claim a rule that does not exist.
///
/// The reference compiler refuses it too, in the engine's words without the
/// call: `Invalid array length`.
#[test]
fn a_negative_array_length_carries_the_engine_s_own_range_error() {
  assert_refuses(
    "",
    "color: Array(-1)",
    "Cannot fold 'Array' at compile time.\nRangeError: invalid array length",
  );
}

/// The same length one link further down a chain, so the refusal is about the
/// length rather than about where the call sat.
#[test]
fn a_negative_array_length_refuses_under_a_chained_call() {
  assert_refuses(
    "",
    "color: Array(-1).join('-')",
    "RangeError: invalid array length",
  );
}

// ──────────────────────────────────────────────
// The edges of the shapes above
// ──────────────────────────────────────────────

/// A conversion that *can* answer for one of these values still answers: the
/// refusals above are about the coercion rather than about the hand-back, and a
/// rule that fired on the hand-back itself would take those folds away.
///
/// One per hand-back shape the file names, so each is observed answering as well
/// as refusing: the namespace map, the single injected config, and the callback.
/// Measured on the reference compiler: `[object Object]` for the namespace and
/// `NaN` for the two functions.
#[test]
fn a_hand_back_the_conversion_can_answer_still_folds() {
  let folds = [
    ("color: String(stylex)", "color:[object Object]"),
    ("color: Number(stylex.when)", "color:NaN"),
    ("color: Number((x) => x, stylex)", "color:NaN"),
  ];

  for (body, rule) in folds {
    let output = crate::utils::transform::fold_module(&base_style_module("", body));

    assert!(
      output.contains(rule),
      "expected `{}` to emit `{}`, got:\n{}",
      body,
      rule,
      output
    );
  }
}

/// An escaping read inside a dynamic style leaves the value to the runtime
/// rather than failing the build, which is where every refusal lands and is not
/// something this rule changes.
#[test]
fn an_escaping_read_inside_a_dynamic_style_reaches_the_runtime() {
  let output = crate::utils::transform::fold_module(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (label) => ({ content: label.constructor.name }),
      });
    "#,
  );

  assert!(
    output.contains("var(--"),
    "expected the read to be left to the runtime, got:\n{}",
    output
  );
}
