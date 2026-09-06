//! What a fold owes the code underneath it when it declines.
//!
//! A shape the bridge cannot carry belongs to the dispatch below, and the
//! dispatch answers for it as it always did. Two places had started answering
//! for it themselves, by failing the build:
//!
//! An **argument with no expression form** — a function, most of all — bound
//! nothing and was refused. The language binds nothing there too, and leaves the
//! parameter unbound, so a body that never reads that parameter has everything it
//! needs. Refusing failed a module that folds on both compilers.
//!
//! A **rule decided before candidacy**: the allocation bound is arithmetic on
//! resolved values, and it answered before the guard had said whether the
//! receiver resolves at all. A call the fold was never going to claim therefore
//! reported a ceiling instead of declining, so an author read about a limit where
//! the reason was a name that does not resolve.
//!
//! Every folded output below was measured against `@stylexjs/babel-plugin@0.19.0`
//! and carries the class name it produced. Where a case refuses on both
//! compilers, the sentence is this one's — upstream answers
//! `Unsupported expression: CallExpression` for the same input.

use crate::utils::transform::{assert_folds, assert_refuses, fold_module as fold};

// ──────────────────────────────────────────────
// An argument with no expression form
// ──────────────────────────────────────────────

/// The row the ticket is about: an arrow handed to a function that never looks
/// at it.
#[test]
fn an_arrow_argument_a_body_ignores_folds() {
  assert_folds(
    "const first = (fn) => 'red';",
    "color: first(() => 1),",
    ".x1e2nbdu{color:red}",
  );
}

/// The unbindable argument in a position other than the first, so which
/// parameter went unbound is read from the call rather than assumed.
#[test]
fn an_arrow_argument_after_a_bound_one_folds() {
  assert_folds(
    "const pick = (a, fn) => a;",
    "color: pick('red', () => 1),",
    ".x1e2nbdu{color:red}",
  );
}

/// The bound argument is still read, which is the half a blanket refusal used to
/// take with it: one argument having no form says nothing about the others.
#[test]
fn a_bound_argument_beside_an_unbound_one_is_still_read() {
  assert_folds(
    "const both = (a, fn) => a + '!';",
    "content: both('a', () => 1),",
    ".x1bt3ucs{content:\"a!\"}",
  );

  assert_folds(
    "const both = (fn, s) => s.toUpperCase();",
    "content: both(() => 1, 'ab'),",
    ".xpf4ll6{content:\"AB\"}",
  );
}

/// The function reached through a name rather than written in place. It has no
/// expression form either way, so the parameter is unbound for the same reason.
#[test]
fn a_named_arrow_argument_a_body_ignores_folds() {
  assert_folds(
    "const other = (x) => x; const first = (fn) => 'red';",
    "color: first(other),",
    ".x1e2nbdu{color:red}",
  );
}

/// The same call one level inside an expression the engine claimed, so the fold
/// and the dispatch below agree about it rather than one folding what the other
/// refuses.
#[test]
fn an_arrow_argument_inside_a_claimed_fold_folds() {
  assert_folds(
    "const first = (fn) => 'red';",
    "content: ['a', 'b'].map(() => first(() => 1)).join('-'),",
    ".x5p5xap{content:\"red-red\"}",
  );
}

/// An object argument, which does have an expression form and bound before this
/// change too — pinned so the arm that stopped refusing cannot stop binding.
#[test]
fn an_argument_that_writes_itself_down_still_binds() {
  assert_folds(
    "const first = (o) => o.a;",
    "content: first({ a: 'red' }),",
    ".x1f1gxam{content:\"red\"}",
  );
}

// ──────────────────────────────────────────────
// The body that does read the unbound parameter
// ──────────────────────────────────────────────

/// A body reading the parameter has nothing to answer with, and the refusal
/// names the argument — the thing an author can go and change. Upstream folds
/// this to the function's own source text, which this compiler does not retain.
#[test]
fn a_body_reading_an_unbound_parameter_names_the_argument() {
  assert_refuses(
    "const inner = (y) => y + '!';",
    "content: inner(() => 1),",
    "Function argument must be a static expression.",
  );
}

/// The same, with the function reached through a name.
#[test]
fn a_body_reading_a_named_function_argument_names_the_argument() {
  assert_refuses(
    "const inner = (y) => y + '!';",
    "content: inner(inner),",
    "Function argument must be a static expression.",
  );
}

/// A body that answered nothing with *every* argument bound has no argument to
/// blame, so the sentence names the body instead. Here the body hands back a
/// name that resolves to nothing.
#[test]
fn a_body_that_answers_nothing_with_every_argument_bound_names_the_body() {
  assert_refuses(
    "const paint = (c) => c + nope;",
    "color: paint('red'),",
    "The function's body has no compile-time value.",
  );
}

/// The limit of that reading, pinned rather than left to be discovered: the same
/// body, in a call that *also* passes an argument with no form, is named for the
/// argument. Both sentences are true of this call and the argument's is the less
/// useful one — the position that applies a closure cannot count the parameters
/// it declares, so an argument nobody has a parameter for reads like one that
/// does. Upstream names `nope`.
#[test]
fn a_body_failing_for_its_own_reason_beside_an_unbound_argument_names_the_argument() {
  assert_refuses(
    "const paint = (c) => c + nope;",
    "color: paint('red', () => 1),",
    "Function argument must be a static expression.",
  );
}

// ──────────────────────────────────────────────
// Candidacy before the rules that read a value
// ──────────────────────────────────────────────

/// The receiver does not resolve, so the call is not the fold's to price. Both
/// spellings answer the resolution rather than a ceiling — and the pair is the
/// point: `padStart` reads its bound off an argument and always declined here,
/// where `repeat` reads it off the receiver and used to report a limit for a
/// receiver nothing had claimed.
#[test]
fn an_unresolved_receiver_answers_the_resolution_rather_than_a_ceiling() {
  for body in [
    "content: nope.repeat(3),",
    "content: nope.padStart(4, '0'),",
    "content: nope.padEnd(4, '0'),",
  ] {
    assert_refuses("", body, "Referenced constant is not defined.");
  }
}

/// A receiver the module *does* hold folds, so moving the bound behind candidacy
/// did not move the fold with it.
#[test]
fn a_resolved_receiver_still_folds_its_repeat() {
  assert_folds(
    "const s = 'ab';",
    "content: s.repeat(3),",
    ".x5ryvnc{content:\"ababab\"}",
  );
}

/// And the ceiling still fires where the receiver is one the fold claimed, which
/// is the half that must not have moved: the rule is later, not gone.
#[test]
fn a_claimed_receiver_past_the_ceiling_still_names_both_numbers() {
  assert_refuses(
    "",
    "content: 'x'.repeat(200000000),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// The two syntax-only refusals stay in front of everything, which is what makes
/// them free: neither reads a value, so neither can refuse a call the fold was
/// never going to claim.
#[test]
fn the_syntax_only_refusals_still_answer_first() {
  assert_refuses(
    "",
    "content: 'i'.toLocaleUpperCase('tr'),",
    "Cannot fold 'toLocaleUpperCase' at compile time.",
  );

  assert_refuses(
    "",
    "content: (1.5).toFixed(1),",
    "Cannot call 'toFixed' on a number literal.",
  );
}

// ──────────────────────────────────────────────
// Spec story 28, which none of this may disturb
// ──────────────────────────────────────────────

/// A refusal inside a dynamic style function still leaves the call for the
/// runtime rather than failing the build — including the two shapes this ticket
/// moved. A dynamic style's own parameter is exactly a receiver nothing resolves.
///
/// Both halves are asserted, because either alone passes on almost any dynamic
/// output: the declaration compiles to a custom property, *and* the call the fold
/// declined survives into the runtime function as the author wrote it.
#[test]
fn a_refusal_inside_a_dynamic_style_still_leaves_the_call_for_the_runtime() {
  // The second string is the same call as the printer spells it, which is how the
  // survival is read back out of the module.
  for (body, printed) in [
    ("label.repeat(2)", "label.repeat(2)"),
    ("inner(() => 1)", "inner(()=>1)"),
  ] {
    let output = fold(&format!(
      r#"
        import * as stylex from '@stylexjs/stylex';
        const inner = (y) => y + '!';
        export const styles = stylex.create({{
          base: (label) => ({{ content: {} }}),
        }});
      "#,
      body
    ));

    assert!(
      output.contains("{content:var(--"),
      "expected `{}` to compile to a custom property, got:\n{}",
      body,
      output
    );
    assert!(
      output.contains(printed),
      "expected `{}` to survive into the runtime function, got:\n{}",
      body,
      output
    );
  }
}

// ──────────────────────────────────────────────
// The edges of both changes
// ──────────────────────────────────────────────

/// Every argument unbindable, on a body that reads none of them. The check is
/// per argument rather than a single verdict on the call.
#[test]
fn a_call_of_nothing_but_unbindable_arguments_folds() {
  assert_folds(
    "const none = (a, b, c, d) => 'red';",
    "color: none(() => 1, () => 2, () => 3, () => 4),",
    ".x1e2nbdu{color:red}",
  );
}

/// More arguments than parameters, and fewer: an argument nobody has a parameter
/// for binds nothing either, and a parameter nobody passed an argument to is the
/// same unbound parameter by another route.
#[test]
fn arguments_and_parameters_that_do_not_line_up_still_fold() {
  assert_folds(
    "const one = (a) => a + '!';",
    "content: one('x', () => 1, () => 2),",
    ".xae5ks1{content:\"x!\"}",
  );

  assert_folds(
    "const two = (a, b) => 'red';",
    "color: two(() => 1),",
    ".x1e2nbdu{color:red}",
  );
}

/// A thousand unbindable arguments, so the reading is linear in what was written
/// rather than quadratic in it.
#[test]
fn a_thousand_unbindable_arguments_fold() {
  let arguments = (0..1000)
    .map(|index| format!("() => {}", index))
    .collect::<Vec<_>>()
    .join(", ");

  assert_folds(
    "const first = (fn) => 'red';",
    &format!("color: first({}),", arguments),
    ".x1e2nbdu{color:red}",
  );
}

/// An unbindable argument beside a receiver the fold claims, so the two halves of
/// this ticket meet in one declaration: the argument costs the call nothing, and
/// the method on the answer folds.
#[test]
fn an_unbindable_argument_beside_a_folded_receiver() {
  assert_folds(
    "const label = (fn) => 'ab'; const text = label(() => 1);",
    "content: text.repeat(3),",
    ".x5ryvnc{content:\"ababab\"}",
  );
}

/// The ceiling still bounds that receiver, which is the arithmetic the reordering
/// had to keep: the rule is later, not gone.
#[test]
fn a_folded_receiver_from_an_unbindable_argument_is_still_bounded() {
  assert_refuses(
    "const label = (fn) => 'ab'; const text = label(() => 1);",
    "content: text.repeat(200000000),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// The same receiver written as the call itself is refused, and deliberately: a
/// call's own length is bounded per link, so reading it is what would let two
/// allowed lengths multiply into one that is neither.
///
/// Not a candidacy question — the walk *admits* a call receiver, and refuses on
/// the length it will not read — so moving the bound behind candidacy neither
/// caused this nor could fix it. Measured on this branch before the reordering
/// and after: the same sentence both times. Upstream folds it to `ababab`, having
/// no ceiling to bound; the divergence is that ceiling, and it is spec story 24
/// rather than this ticket's.
#[test]
fn a_call_as_the_receiver_of_a_repeat_stays_unread() {
  assert_refuses(
    "const label = (fn) => 'ab';",
    "content: label(() => 1).repeat(3),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// An unresolved receiver inside a callback body, where the count a callback
/// repeats is what the amplification rule reads. Candidacy still answers first,
/// so the sentence is the surrounding method's rather than a bound on a receiver
/// nothing claimed.
#[test]
fn an_unresolved_receiver_inside_a_callback_answers_the_surrounding_method() {
  assert_refuses(
    "",
    "content: ['x'].map(() => nope.repeat(3)).join(''),",
    "Cannot fold 'map' at compile time.",
  );
}

/// A chain of unbindable arguments two hundred deep, each call folding on a body
/// that ignores its own. Nothing here grows with depth, and a build that stopped
/// answering would say so by refusing rather than by running out of stack.
#[test]
fn two_hundred_nested_calls_with_unbindable_arguments_fold() {
  let mut value = String::from("'red'");

  for _ in 0..200 {
    value = format!("first(() => {})", value);
  }

  assert_folds(
    "const first = (fn) => 'red';",
    &format!("color: {},", value),
    ".x1e2nbdu{color:red}",
  );
}
