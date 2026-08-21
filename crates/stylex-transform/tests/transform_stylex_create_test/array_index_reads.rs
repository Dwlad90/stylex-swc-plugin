//! Reading an element out of an array by index, where a style value belongs.
//!
//! `const FALLBACKS = ['1px']; create({ s: { height: FALLBACKS[0] } })` stopped
//! the build with `Unsupported index: 0` while the reference implementation
//! folded the element. An array literal a fold produced answered an index only
//! where it was written as a numeric literal; an array the evaluator holds as
//! its own value answered none at all.
//!
//! Both receivers read one now, on the language's terms: a canonical digit key
//! names a slot, any other key is an ordinary property name, and a slot past
//! the end is `undefined`. The unit-level sweep -- every key shape, every
//! receiver, the overflow and the non-ASCII digits -- is in the evaluator's own
//! `tests/array_index_tests.rs`. This file asks what the *compiler* emits, in
//! both style-value positions.
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

// ── The reported shape ──────────────────────────────────────────────

// `.x1n1r2ho{height:1px}` — the element is folded where the refusal used to be.
stylex_test!(
  an_index_read_off_an_array_binding,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ s: { height: FALLBACKS[0] } });
  "#
);

// The second reported row: the read feeds another fallback array, so the
// refusal used to arrive one level in and take both fallbacks with it.
stylex_test!(
  an_index_read_feeding_a_fallback_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = ['1px'];
    const B = [A[0], '2px'];
    export const styles = stylex.create({ s: { height: B } });
  "#
);

// Inside a dynamic style's body the refusal was not an error: the value fell to
// the runtime as `var(--x-height)` where the reference implementation folds a
// static declaration. The same missing fold, wearing the other verdict.
stylex_test!(
  an_index_read_inside_a_dynamic_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ dyn: (h) => ({ height: FALLBACKS[0] }) });
  "#
);

// A fold's own array output is the other receiver, and answers the same index.
stylex_test!(
  an_index_read_off_a_folded_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      s: { content: Object.keys({ a: 1, b: 2 })[1] },
    });
  "#
);

// ── Past the end ────────────────────────────────────────────────────

// An index past the end is `undefined`, so it takes a fallback rather than
// stopping the build — which is the whole reason it answers a value at all.
stylex_test!(
  an_index_past_the_end_takes_a_fallback,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ s: { height: FALLBACKS[7] ?? '2px' } });
  "#
);

// Bare, it is a value the style position refuses for not being one — the
// reference implementation's sentence, reached because the index answered a
// value rather than a refusal about the index.
stylex_test_panic!(
  a_bare_index_past_the_end_is_refused_as_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ s: { height: FALLBACKS[7] } });
  "#
);

// The same inside a dynamic style's body, where the namespace is validated
// after the fold rather than before it.
stylex_test_panic!(
  an_index_past_the_end_in_a_dynamic_style_is_refused,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ dyn: (h) => ({ height: FALLBACKS[7] }) });
  "#
);

// ── The keys that are not indices ───────────────────────────────────

// A digit key that is not the canonical spelling of its number is an ordinary
// property name, so it is `undefined` and takes the fallback rather than
// reading slot zero. Reading it as a slot is the one way this can be
// confidently wrong, and a wrong value ships silently where a refusal does not.
stylex_test!(
  a_non_canonical_digit_key_takes_the_fallback,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px'];
    export const styles = stylex.create({ s: { height: FALLBACKS['00'] ?? '2px' } });
  "#
);

// A key written as digits in a string names the same slot as the number does.
stylex_test!(
  a_string_written_index_names_the_same_slot,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px', '2px'];
    export const styles = stylex.create({ s: { height: FALLBACKS['1'] } });
  "#
);

// ── The shapes that still refuse ────────────────────────────────────

// A spread stands for however many elements its value holds, so the receiver
// refuses to fold at all and the index never runs.
stylex_test_panic!(
  an_index_read_off_an_array_carrying_a_spread_is_refused,
  "s > height > Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = [...['1px']];
    export const styles = stylex.create({ s: { height: FALLBACKS[0] } });
  "#
);

// A hole has no value, so the receiver refuses ahead of the index.
stylex_test_panic!(
  an_index_read_off_an_array_carrying_a_hole_is_refused,
  "s > height > Unexpected error:\nCould not resolve the code being evaluated.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = [, '1px'];
    export const styles = stylex.create({ s: { height: FALLBACKS[1] } });
  "#
);

// A string still refuses an index: its element is a single UTF-16 code unit,
// which can be an unpaired surrogate no Rust string holds. The two array
// receivers agreeing does not make a third one agree with them.
stylex_test_panic!(
  a_string_index_is_still_refused,
  "Unsupported index: 0",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ s: { content: "abc"[0] } });
  "#
);
