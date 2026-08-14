//! `??`, `||`, `&&` and `+` fold the same way when they meet each other.
//!
//! The operators are pinned one at a time next door — the logical three in
//! `logical_operators.rs`, the dispatch of `+` in `string_concatenation.rs`,
//! and the coercion its string side reaches for in `string_coercion.rs`. What
//! is left is the combinations, where one operator's result is the other's
//! operand, and where a folded operand has to land in the middle of a value
//! that is otherwise static.
//!
//! The remaining regression row from the same matrix — a logical operator on a
//! parameter of a dynamic style function, which cannot fold and must go on
//! emitting the runtime code it emits today — is pinned by
//! `dynamic_styles.rs::nullish_coalescing_safe_left_side` and is deliberately
//! not restated here: a second snapshot of the same output would only record
//! the same claim twice.
//!
//! The expected class names and rule text are measured output of
//! `@stylexjs/babel-plugin@0.19.0` for the same input, with `dev: false`,
//! `treeshakeCompensation: true` and `commonJS` module resolution.
//!
//! Runtime injection is enabled so each snapshot records the emitted rule text
//! next to the class name: the class name is what a divergence in the folded
//! value would move, and the rule text is what proves the value itself is
//! right.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// `.x1e2nbdu{color:red}` and `.x1i1rx1s{width:1px}` — the winning operand is
// returned verbatim, and an addition is a value like any other, so the operand
// arrives already folded rather than as an expression the caller has to finish.
stylex_test!(
  a_logical_whose_winner_is_an_addition,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const token = null;
    export const styles = stylex.create({
      a: { color: token ?? 're' + 'd' },
      b: { width: token || 1 + 'px' },
    });
  "#
);

// `.x1ii1mnw{border-radius:1px 2px}` and `.xg1gb30{content:"xy"}` — and the
// other way round, where the fold that has to happen first is the logical one.
// Both spellings reach the string result of `+`, one through a fallback that
// wins on a missing property and one through a string that is already there.
stylex_test!(
  an_addition_whose_operand_is_a_logical,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const radius = { s: '0.25rem' };
    export const styles = stylex.create({
      a: { borderRadius: (radius.missing ?? '1px') + ' 2px' },
      b: { content: 'x' + (radius.missing || 'y') },
    });
  "#
);

// `.x1e2nbdu{color:red}` — a chained `??` falls through twice, so the operand
// the first one loses with is the second one's left side rather than its
// result.
stylex_test!(
  a_nullish_chain_falls_through_to_its_last_operand,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const first = null;
    const second = null;
    export const styles = stylex.create({
      a: { color: first ?? second ?? 'red' },
    });
  "#
);

// `.x1t391ir{background-color:blue}` — `&&` reads the operand `||` chose
// rather than the expression it was written as, which is the whole of what
// "returned verbatim" has to mean for a nested operator.
stylex_test!(
  an_and_over_the_operand_an_or_chose,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const first = null;
    export const styles = stylex.create({
      a: { backgroundColor: (first || 'green') && 'blue' },
    });
  "#
);

// `.x1n33nnf{box-shadow:0 0 4px blue}` and `.xxr9l41{margin:4px 8px}` — the
// folded operand lands in the middle of a value the rest of which is static,
// which is the shape the reporter's own source has: the surrounding text is
// what makes a stray space or a lost fallback visible in the rule. Both
// operators fold in the same value here — an addition supplying the static
// part's number, and `&&` reached through a `+` rather than a template, which
// is the one pairing the cases above leave out.
stylex_test!(
  a_logical_beside_a_static_part,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({
      a: { boxShadow: `0 0 ${2 + 2}px ${color.accent ?? 'blue'}` },
      b: { margin: '4px ' + (color.primary && '8px') },
    });
  "#
);

// The inherited restriction is decided before the winning operand is looked
// at, so a `??` whose left side is falsy but present refuses even where the
// operand it would have won with folds perfectly well on its own. The
// reference implementation refuses this too, with the same `unknown error`
// its truthiness guard produces for a bare `0`.
stylex_test_panic!(
  a_nullish_over_an_addition_still_refuses_a_falsy_left_side,
  "unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const zero = 0;
    export const styles = stylex.create({ a: { flexGrow: zero ?? 1 + 1 } });
  "#
);
