//! `??`, `||` and `&&` around a style value fold at compile time.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1254, where a guarded
//! design token failed the build with `For string expressions, only addition is
//! supported, got "??"`. The expected class names and rule text are measured
//! output of `@stylexjs/babel-plugin@0.19.0` for the same input.
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

// The reproduction from issue #1254, verbatim:
// `.x1v5h5rg{border-radius:0 0 .25rem .25rem}`.
stylex_test!(
  nullish_in_a_template_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const radius = { s: '0.25rem' };
    export const styles = stylex.create({
      a: { borderRadius: `0 0 ${radius.s ?? ''} ${radius.s ?? ''}` },
    });
  "#
);

// `.x9hkwd3{margin:4px 2px}` — `||` and `&&` fold inside a template literal on
// the same terms `??` does.
stylex_test!(
  or_and_and_in_a_template_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const space = { s: '4px' };
    const fallback = '8px';
    export const styles = stylex.create({
      a: { margin: `${space.s || fallback} ${space.s && '2px'}` },
    });
  "#
);

// `.x1e2nbdu{color:red}` — the guard folds in a direct style value too, not
// only inside a template literal.
stylex_test!(
  nullish_in_a_direct_style_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({
      a: { color: color.primary ?? 'blue' },
    });
  "#
);

// `.x1u857p9{background-color:green}` — a property simply missing from an
// object is `undefined`, which the operator also takes its right side for.
stylex_test!(
  nullish_takes_the_fallback_for_a_missing_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({
      a: { backgroundColor: color.missing ?? 'green' },
    });
  "#
);

// `.xju2f9n{color:blue}` — a `null` left side is one of the two the operator
// takes its right side for.
stylex_test!(
  nullish_takes_the_fallback_for_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const token = null;
    export const styles = stylex.create({ a: { color: token ?? 'blue' } });
  "#
);

// `.x1e2nbdu{color:red}` for both — `void x` is the third spelling of
// `undefined`, and the operators take their right side for it the way they do
// for the other two. The operand is never evaluated, so the string it is
// applied to here neither reaches the fold nor could deopt it.
stylex_test!(
  nullish_and_or_take_the_fallback_for_void,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color: void 0 ?? 'red' },
      b: { color: void 'blue' || 'red' },
    });
  "#
);

// `.xju2f9n{color:blue}` and `.x1u857p9{background-color:green}` — `||` takes
// the fallback for an empty string, `&&` takes the right side for a set one.
stylex_test!(
  or_and_and_over_strings,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const empty = '';
    const set = 'red';
    export const styles = stylex.create({
      a: { color: empty || 'blue', backgroundColor: set && 'green' },
    });
  "#
);

// `.x1e2nbdu{color:red}` and `.x17z2mba:hover{color:blue}` — the winning
// operand is returned as the object it is, and the nested conditions inside it
// are read as usual.
stylex_test!(
  a_winning_object_stays_an_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const config = null;
    export const styles = stylex.create({
      a: { color: config ?? { default: 'red', ':hover': 'blue' } },
    });
  "#
);

// `.x1e565ft{font-family:Arial;font-family:sans-serif}` — a winning array is
// still the fallback list it was written as.
stylex_test!(
  a_winning_array_stays_an_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const list = null;
    export const styles = stylex.create({
      a: { fontFamily: list ?? ['Arial', 'sans-serif'] },
    });
  "#
);

// A falsy confident left side is returned as it is, and the empty string it
// wins with is a blank value, so the property is left undeclared and compiles
// to `null`. The reference implementation returns the same operand and then
// crashes on it downstream with a bare `TypeError`, which is not a behaviour
// worth reproducing.
stylex_test!(
  and_returns_a_falsy_left_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const empty = '';
    export const styles = stylex.create({ a: { color: empty && 'green' } });
  "#
);

// The reference implementation's nullish guard tests the left side's
// truthiness rather than its nullishness, so a left side that is falsy but
// present refuses to fold and deopts with `unknown error`. The restriction is
// inherited rather than corrected: folding here where the reference
// implementation does not would be a silent CSS difference between two builds
// of the same source.
//
// The message is asserted rather than the mere fact of failure — before the
// operator was implemented at all these inputs failed too, for the unrelated
// reason that every `??` was refused.
//
// The property path is asserted with it. A value that genuinely cannot fold has
// to be findable inside a large style object, and the deopt reason alone would
// name every such value identically.
stylex_test_panic!(
  nullish_refuses_a_zero_left_side,
  "a > flexGrow > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const zero = 0;
    export const styles = stylex.create({ a: { flexGrow: zero ?? 5 } });
  "#
);

stylex_test_panic!(
  nullish_refuses_a_false_left_side,
  "a > color > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const off = false;
    export const styles = stylex.create({ a: { color: off ?? 'red' } });
  "#
);

stylex_test_panic!(
  nullish_refuses_an_empty_string_left_side,
  "a > color > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const blank = '';
    export const styles = stylex.create({ a: { color: blank ?? 'red' } });
  "#
);

// A missing property reads as `undefined` whether or not a logical operator is
// waiting for it, so a bare one now reaches the style-value check and fails the
// build there. Before, it deopted and the whole declaration fell to the runtime
// instead, which is the shape that kept `token.missing ?? fallback` from
// folding.
//
// The sentence is the reference implementation's, byte for byte: `undefined` is
// a value the evaluator is confident about, and a value position refuses it for
// not being a style value rather than reporting that nothing was static.
stylex_test_panic!(
  a_bare_missing_property_is_rejected_as_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({ a: { color: color.missing } });
  "#
);

// ─────────────────────────────────────────────────────────────────────────────
// Issue #1265 — a right operand the evaluator cannot fold must not abort
// ─────────────────────────────────────────────────────────────────────────────

// The reduced reproduction from
// https://github.com/Dwlad90/stylex-swc-plugin/issues/1265, verbatim.
//
// `"documentation".startsWith(lowerQuery)` has a runtime argument, so nothing
// here is foldable and nothing should be folded — the condition belongs in the
// output as written. It aborted the build instead, because the evaluator
// reaches the right operand of `&&` under a forked confidence and the arm that
// refused the method aborted rather than deopting.
//
// This is the output `0.18.3` and `0.18.4-rc.1` produce, and the one
// `@stylexjs/babel-plugin@0.19.0` produces: two rules emitted, `showAlternate`
// preserved as a runtime condition.
stylex_test!(
  an_unfoldable_method_call_inside_a_runtime_condition_survives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      base: { color: 'black' },
      alternate: { color: 'red' },
    });

    export function Section({ query, lowerQuery }) {
      const showAlternate = query.length > 0 && "documentation".startsWith(lowerQuery);

      return <section sx={[styles.base, showAlternate && styles.alternate]} />;
    }
  "#
);

// The property behind the symptom, at the seam that broke: for each of the
// three operators, a right operand the evaluator cannot fold sends the
// declaration to the runtime and the build survives. The symptom test above
// would have passed at rc.1 and said nothing about why; this is the one that
// would have caught `1322be8c1`.
stylex_test!(
  an_unfoldable_right_operand_falls_to_the_runtime_for_every_logical_operator,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      a: (props) => ({ color: props.flag && "documentation".startsWith(props.q) }),
      b: (props) => ({ color: props.flag || "documentation".startsWith(props.q) }),
      c: (props) => ({ color: props.flag ?? "documentation".startsWith(props.q) }),
    });
  "#
);

// The second input reported on the same issue. It refused with `The array
// method 'includes' is not yet supported in static evaluation.` — a different
// arm of the evaluator, reached the same way and failing the same way, which is
// why the fix had to be a split rather than one more supported method.
//
// `VIEWS` is a compile-time array, so the receiver folds and only the argument
// is unknown; the call still cannot be folded and belongs in the output as
// written. Same expected output as above, and the one
// `@stylexjs/babel-plugin@0.19.0` produces: two rules emitted, `isHView`
// preserved as a runtime condition.
stylex_test!(
  an_unfoldable_array_method_inside_a_runtime_condition_survives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const VIEWS = ['grid', 'list'];

    const styles = stylex.create({
      base: { color: 'black' },
      hview: { color: 'red' },
    });

    export function View({ hView }) {
      const isHView = VIEWS.includes(hView);

      return <div sx={[styles.base, isHView && styles.hview]} />;
    }
  "#
);

// The reporter's fuller module, with the condition split across two statements:
// `lowerQuery` is itself a method call on a runtime receiver, so the operand
// that cannot be folded is reached through a binding rather than written in
// place. Nothing about the seam changes, which is the point of pinning it —
// the fold is refused wherever the unfoldable node is found.
//
// `borderTop: none` declares nothing: under the default
// `property-specificity` resolution the shorthand is rejected, and the default
// `propertyValidationMode: silent` drops it without a word. Only the `display`
// rule reaches the stylesheet, which is what `@stylexjs/babel-plugin@0.19.0`
// emits for this module too.
stylex_test!(
  a_runtime_condition_reached_through_a_binding_survives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from "@stylexjs/stylex";

    const styles = stylex.create({
      base: { display: "block" },
      alternate: { borderTop: "none" },
    });

    export function Component({ query }) {
      const lowerQuery = query.toLowerCase();
      const showAlternate =
        query.length > 0 && "documentation".startsWith(lowerQuery);

      return (
        <section sx={[styles.base, showAlternate && styles.alternate]} />
      );
    }
  "#
);

// The same seam across the shapes an unfoldable call can take, because the
// evaluator refuses each of them from a different arm and a split that missed
// one would abort exactly as `startsWith` did. One component per shape keeps
// the emitted lookup table linear — a single `sx` array with N conditions
// builds 2^N entries.
//
// `join`, `Object.keys` and `concat` are the three methods here the evaluator
// does fold, so those cases reach the refusal *after* a fold rather than
// instead of one: the argument is unknown, the receiver came from a call, and
// the chain refuses only at its outer link.
//
// Every one of these is `identical` against `@stylexjs/babel-plugin@0.19.0`,
// except `Some` — see the test below it.
stylex_test!(
  every_shape_of_unfoldable_call_survives_a_runtime_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from "@stylexjs/stylex";

    const VIEWS = ['grid', 'list'];
    const SIZES = { small: 1 };

    const styles = stylex.create({
      base: { color: 'black' },
      on: { color: 'red' },
    });

    // A method on a runtime receiver.
    export const Lower = ({ q }) => <i sx={[styles.base, q.toLowerCase() && styles.on]} />;

    // A method the evaluator does fold, refused for its argument alone.
    export const Join = ({ q }) => <i sx={[styles.base, VIEWS.join(q) && styles.on]} />;

    // A call on the result of a call that does fold.
    export const Keys = ({ q }) => <i sx={[styles.base, Object.keys(SIZES).includes(q) && styles.on]} />;

    // The inner call folds, the outer one cannot.
    export const Chain = ({ q }) => <i sx={[styles.base, "documentation".concat("s").startsWith(q) && styles.on]} />;

    // The call is optional, so the node the evaluator meets is not a plain one.
    export const Optional = ({ q }) => <i sx={[styles.base, q?.startsWith("a") && styles.on]} />;

    // A method no fold exists for under any receiver.
    export const Unknown = ({ q }) => <i sx={[styles.base, q.somethingUnknown() && styles.on]} />;

    // A receiver the evaluator has no fold for at all, rather than a method it
    // does not know on a receiver it does.
    export const Constructed = ({ q }) => <i sx={[styles.base, new Set(VIEWS).has(q) && styles.on]} />;
  "#
);

// `Array.prototype.some` with an arrow callback, kept apart from the table
// above because it is the one shape where the two compilers disagree — and it
// disagrees in this compiler's favour.
//
// `@stylexjs/babel-plugin@0.19.0` aborts the build with `Unsupported
// expression: ObjectPattern`: it evaluates the callback body, resolves `v ===
// q` down to `q`'s binding, reaches the destructured parameter and throws from
// inside an evaluation that is allowed to fail — the same defect #1265 reports
// here, still present upstream. It aborts only through the binding: the same
// call written inline in the `sx` array, as the table above writes its cases,
// compiles there.
//
// Reported upstream rather than reproduced: a build that survives is the
// correct answer, and matching an abort would mean re-introducing the bug this
// issue is about.
stylex_test!(
  an_unfoldable_callback_argument_survives_where_upstream_aborts,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from "@stylexjs/stylex";

    const VIEWS = ['grid', 'list'];

    const styles = stylex.create({
      base: { color: 'black' },
      on: { color: 'red' },
    });

    export function Some({ q }) {
      const matches = VIEWS.some(v => v === q);

      return <i sx={[styles.base, matches && styles.on]} />;
    }
  "#
);

// ─────────────────────────────────────────────────────────────────────────────
// The same seam, from the shapes a downstream build reported
// ─────────────────────────────────────────────────────────────────────────────

// Three more spellings of the refused `includes` above, reduced from a
// downstream report that hit them on a compiler predating the split. Each one
// reaches the refusal by a route the cases above do not, and each is
// `identical` against `@stylexjs/babel-plugin@0.19.0`.

// The guard is `!!hView`, not `hView`: the left operand is a unary expression
// the evaluator cannot fold either, so the fold is refused before the operator
// ever asks about the right side. The right side is still evaluated — that is
// what decides which side the deopt names — and the declaration falls to the
// runtime whole.
stylex_test!(
  an_unfoldable_array_method_behind_a_double_negated_guard_survives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const VIEWS = ['grid', 'list'];

    const styles = stylex.create({
      base: { color: 'black' },
      hview: { color: 'red' },
    });

    export function View({ hView }) {
      const isHView = !!hView && VIEWS.includes(hView);

      return <div sx={[styles.base, isHView && styles.hview]} />;
    }
  "#
);

// The receiver is an array literal written in place rather than a binding, so
// the evaluator meets it as `EvaluateResultValue::Expr(Expr::Array)` — a
// different arm from the folded `Vec` the named `VIEWS` produces, refusing with
// `The method 'includes' is not yet supported in static evaluation.` The
// refusal has to deopt from there too.
stylex_test!(
  an_unfoldable_array_method_on_an_array_literal_survives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      base: { color: 'black' },
      hview: { color: 'red' },
    });

    export function View({ hView }) {
      const isHView = hView && ['grid', 'list'].includes(hView);

      return <div sx={[styles.base, isHView && styles.hview]} />;
    }
  "#
);

// The unfoldable call decides a *namespace key* rather than a condition: it
// picks `gridType` through an `if`, a template literal builds the key from it,
// and the namespace is read by that key. Nothing about the style object is
// unknown — both namespaces compile — and the lookup itself is what stays in
// the output.
stylex_test!(
  a_namespace_read_by_a_key_an_unfoldable_array_method_decides,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const WIDE_VIEWS = ['grid', 'list'];

    const styles = stylex.create({
      base: { color: 'black' },
      regularGrid: { display: 'grid' },
      wideGrid: { display: 'flex' },
    });

    export function View({ hView }) {
      let gridType = 'regular';

      if (hView && WIDE_VIEWS.includes(hView)) {
        gridType = 'wide';
      }

      const grid = `${gridType}Grid`;

      return <div sx={[styles.base, styles[grid]]} />;
    }
  "#
);
