//! Object keys that a number or an index-like string spells.
//!
//! Two readings of a key decide what the compiler emits, and both are settled
//! by JavaScript rather than by Rust.
//!
//! A number key names a string property, and the string is the one JavaScript
//! spells: `1e21` names `1e+21`, not the twenty-two digit run Rust's `f64`
//! formatting prints. The name feeds the class-name hash, so the spelling is
//! observable in the output and not only in the emitted object.
//!
//! An index-like key decides enumeration order, hence declaration order, hence
//! which of two rules at equal specificity wins. Only the canonical decimal
//! spelling of an integer counts: `0` is an index and `+0` is an ordinary
//! string key, because JavaScript reads no leading sign where Rust's integer
//! parser does.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0`
//! under the parity harness's options and agrees with it.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── The spelling of a number key ────────────────────────────────────

// The namespace is named `1e+21`, which is what `String(1e21)` gives.
stylex_test!(
  a_number_namespace_key_takes_the_javascript_spelling,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ 1e21: { color: 'red' } });
  "#
);

// Written as a computed key, the same number names the same namespace.
stylex_test!(
  a_computed_number_namespace_key_takes_the_javascript_spelling,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ [1e21]: { color: 'red' } });
  "#
);

// A small magnitude takes an exponent too, and keeps its sign there.
stylex_test!(
  a_small_number_key_keeps_its_exponent,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ 1e-7: { color: 'red' } });
  "#
);

// An integer small enough to have no exponent is unaffected, which is why no
// fixture caught the two spellings parting.
stylex_test!(
  an_integer_key_spells_the_same_either_way,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ 42: { color: 'red' } });
  "#
);

// ── The order an index-like key takes ───────────────────────────────

// `0` is an index and is enumerated first; `+0` is an ordinary string key and
// keeps its place among the others. So the declarations reach the stylesheet
// in the order `0`, `+0`, `color`, and not in the order they are written.
stylex_test!(
  an_index_key_is_declared_before_a_signed_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { '+0': 'red', 0: 'blue', color: 'green' },
    });
  "#
);
