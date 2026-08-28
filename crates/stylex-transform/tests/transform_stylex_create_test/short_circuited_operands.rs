//! The operand of a short-circuiting form that never runs, as the fold's guard
//! sees it.
//!
//! The guard reads every leaf of an expression to decide whether the whole of it
//! could fold, and reading one has effects beyond the answer — see the crate
//! glossary's *Dead operand*. So `enabled && colors.glow` with `enabled` false
//! must leave no import behind and must not refuse the call it sits in.
//!
//! Runtime injection is on throughout, so each snapshot records the rule text
//! beside the class name; the import list above it is the half these cases are
//! really about, and the one a CSS-only assertion would miss.
//!
//! Every class name and rule below is `@stylexjs/babel-plugin@0.19.0`'s own for
//! the same module. The *import lists* are deliberately not, where a dead theme
//! read is involved: the reference implementation evaluates both sides of a
//! logical expression under forked states, so it queues the compensating import
//! for a token no stylesheet reads. This compiler's logical node has been lazy
//! for that reason since it was written — the same module with no call in it
//! emits no such import here already — and these cases are the fold's guard
//! agreeing with the evaluator it stands in front of.

use crate::utils::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// A theme read on the branch that never runs
// ─────────────────────────────────────────────────────────────────────────────

// The reported shape: a token read behind a compile-time-false guard, inside an
// expression the fold claims. The stylesheet says `color:red` and the module
// gets no `import "tokens.stylex";` — the token never reached the stylesheet, so
// nothing has to keep the file that defines it alive.
stylex_test!(
  a_dead_theme_read_leaves_no_import,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| b.with_treeshake_compensation(true)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'tokens.stylex';
    const enabled = false;
    export const styles = stylex.create({
      a: { color: [(enabled && colors.glow) || 'red'].join('') },
    });
  "#
);

// The same module with compensation off, which is the setting that decides
// whether an import is ever added at all. Nothing here could add one, and that
// is the point: pinned beside the case above so a fix that had merely stopped
// compensating would be visible as the different thing it is.
stylex_test!(
  a_dead_theme_read_with_no_compensation,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'tokens.stylex';
    const enabled = false;
    export const styles = stylex.create({
      a: { color: [(enabled && colors.glow) || 'red'].join('') },
    });
  "#
);

// The other half of the pair: the guard true, and the token carried through the
// fold rather than around it. A group crosses to the engine as the string its
// own `toString` answers, so this is the shape where a *successful* fold is what
// read the token — and the compensating import has to be there. A fix that
// unwound the queue for every speculation rather than not speculating at all
// would pass the case above and fail this one.
//
// `.xmwyr5j{color:xr4ttzw}` — the group's own hash rather than a `var(--…)`,
// because a group crossing as a value is not a member of it. Measured: the
// reference implementation emits that class and that rule for this module, and
// the compensating import with them.
stylex_test!(
  a_live_theme_read_keeps_its_import,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| b.with_treeshake_compensation(true)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'tokens.stylex';
    const enabled = true;
    export const styles = stylex.create({
      a: { color: [(enabled && colors) || 'red'].join('') },
    });
  "#
);

// Every short-circuiting form, each with the token on the side that never runs:
// `&&` behind a falsy guard, `||` behind a truthy one, `??` behind a value that
// is neither `null` nor `undefined`, and `?:` on both of its arms. One
// namespace each so the import list is one answer for all five.
stylex_test!(
  every_short_circuiting_form_skips_its_dead_operand,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| b.with_treeshake_compensation(true)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'tokens.stylex';
    const off = false;
    const on = 'red';
    export const styles = stylex.create({
      and: { color: [(off && colors.glow) || 'red'].join('') },
      or: { color: [on || colors.glow].join('') },
      nullish: { color: [on ?? colors.glow].join('') },
      conditionalCons: { color: [on ? 'red' : colors.glow].join('') },
      conditionalAlt: { color: [off ? colors.glow : on].join('') },
    });
  "#
);

// ─────────────────────────────────────────────────────────────────────────────
// A refusal on the branch that never runs
// ─────────────────────────────────────────────────────────────────────────────

// A dead branch naming a module function. The name resolves to nothing a fold
// can carry, so walking it refused the whole call with a rule and the build
// failed over a branch that cannot run. Now the walk never reaches it and the
// live operand folds: `.x1e2nbdu{color:red}`.
stylex_test!(
  a_dead_branch_naming_a_module_function_does_not_refuse_the_call,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const enabled = false;
    function pick() {}
    export const styles = stylex.create({
      a: { color: [(enabled && pick) || 'red'].join('') },
    });
  "#
);

// The same for a read the guard refuses by name rather than by what it resolves
// to. `constructor` leads off the written value and onto the language's function
// graph, which is a rule with a sentence of its own — and a sentence about a
// branch nothing evaluates is a build failure with no cause.
stylex_test!(
  a_dead_branch_reading_an_escaping_property_does_not_refuse_the_call,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const enabled = false;
    export const styles = stylex.create({
      a: { color: [(enabled && 'x'.constructor) || 'red'].join('') },
    });
  "#
);

// ─────────────────────────────────────────────────────────────────────────────
// Where the short circuit may not be read
// ─────────────────────────────────────────────────────────────────────────────

// Inside a callback the module is not what binds the names, so which side runs
// cannot be read from it: `flag` here is the element, not the module's `false`.
// Both sides are walked, `suffix` becomes a parameter of the printed arrow, and
// the fold answers `1px 2px` — a walk that had read the module's `flag` would
// have left `suffix` unbound and the engine reaching for a name nothing gave it.
stylex_test!(
  a_short_circuit_inside_a_callback_carries_both_sides,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const flag = false;
    const suffix = 'px';
    export const styles = stylex.create({
      a: { margin: [1, 2].map((flag) => flag && flag + suffix).join(' ') },
    });
  "#
);

// The conditional form of the same shadowing, where reading the module's `flag`
// would have taken one arm for every element. `.xpwwz5d{padding:1px 2px}` is
// the arms alternating with the element, which only both-sides-carried answers.
stylex_test!(
  a_conditional_inside_a_callback_carries_both_arms,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const flag = false;
    const suffix = 'px';
    export const styles = stylex.create({
      a: { padding: [1, 2].map((flag) => (flag ? flag + suffix : suffix)).join(' ') },
    });
  "#
);

// A guard whose truthiness the walk cannot read at all — `0 ?? x` is the one
// the operator itself declines to decide. Both sides are walked, so the token
// read still queues its import, and the declaration falls to the runtime the way
// it did before. Conservative rather than clever: a guess here would pick the
// wrong operand.
stylex_test!(
  an_undecidable_guard_carries_both_sides,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| b.with_treeshake_compensation(true)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'tokens.stylex';
    const zero = 0;
    export const styles = stylex.create({
      a: (props) => ({ color: [(zero ?? colors.glow), props.x].join('') }),
    });
  "#
);

// ─────────────────────────────────────────────────────────────────────────────
// Shapes that stress the walk rather than the rule
// ─────────────────────────────────────────────────────────────────────────────

// The guard still asks about every operand it does reach, so a live branch
// naming a module function refuses exactly as it always did. Paired with the
// dead case above, this is what says the fix is laziness and not a rule that
// stopped being applied.
stylex_test_panic!(
  a_live_branch_naming_a_module_function_still_refuses,
  "a > color > Cannot carry the function 'pick' into a fold.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const enabled = true;
    function pick() {}
    export const styles = stylex.create({
      a: { color: [(enabled && pick) || 'red'].join('') },
    });
  "#
);

// Short circuits nested through each other, where the dead branch is reached
// only by deciding the two above it. `.x1e2nbdu{color:red}` — the value is the
// innermost live operand, and nothing on the way to it was resolved.
stylex_test!(
  nested_short_circuits_decide_from_the_outside_in,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const off = false;
    const on = true;
    function pick() {}
    export const styles = stylex.create({
      a: { color: [((off && pick) || (on ? 'red' : pick)) || pick].join('') },
    });
  "#
);

// A dead branch that would not merely have refused but would have been
// expensive to price: a `repeat` past the character ceiling. Skipping it is the
// same skip, and the point is that the walk never asks what it costs.
stylex_test!(
  a_dead_branch_past_the_allocation_ceiling_is_never_priced,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const off = false;
    export const styles = stylex.create({
      a: { color: [(off && 'x'.repeat(400000000)) || 'red'].join('') },
    });
  "#
);
