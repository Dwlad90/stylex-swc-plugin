//! A template literal interpolates what `ToString` says, or refuses.
//!
//! An interpolation is `ToString` over whatever the expression evaluated to.
//! The evaluator used to ask a narrower question instead -- is this an `Expr`,
//! is it a `Lit`, and does it have a spelling -- and contributed the *empty
//! string* whenever any link of that chain failed. Nothing said the value had
//! gone missing, so `${null}` compiled to a declaration reading as though the
//! interpolation had not been written, and hashed a class name to match. Six
//! shapes reached that silence, `${undefined}` and `${true}` among them.
//!
//! The bridge every other consumer of a folded value already reads answers all
//! of them, so the chain is gone and the bridge is what runs. What it refuses
//! -- a value with no compile-time string at all -- now deopts, which is the
//! other half of the fix: a build that cannot spell an interpolation says so
//! rather than writing a shorter value than the source describes.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options, and agrees with it except where a case says otherwise.
//! Runtime injection is enabled so each snapshot records the rule text beside
//! the class name: the class name is what a coercion divergence moves, and the
//! rule text is what shows the coerced value behind it.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── The values that used to vanish ──────────────────────────────────

// The whole falsy list plus the two globals that are not literals. Each spells
// itself out; none contributes nothing. `null` and `undefined` are the two the
// old chain lost most quietly, because a property reading `font-family:ab` is
// a plausible thing to have written.
stylex_test!(
  the_primitives_spell_themselves_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      nullValue: { fontFamily: `a${null}b` },
      undefinedValue: { fontFamily: `a${undefined}b` },
      booleanTrue: { fontFamily: `a${true}b` },
      booleanFalse: { fontFamily: `a${false}b` },
      notANumber: { fontFamily: `a${NaN}b` },
      infinite: { fontFamily: `a${Infinity}b` },
      zero: { fontFamily: `a${0}b` },
      emptyString: { fontFamily: `a${''}b` },
    });
  "#
);

// An object takes the `Object.prototype` default, and an array joins its
// elements with commas -- both the language's answers, neither of them a value
// an author means, and agreeing on them is the point: a class name is a hash of
// the declaration text, so two compilers spelling nonsense differently is worse
// than either spelling.
stylex_test!(
  an_object_and_an_array_take_their_language_defaults,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      emptyObject: { fontFamily: `a${{}}b` },
      object: { fontFamily: `a${{ x: 1 }}b` },
      array: { fontFamily: `a${[1, 2]}b` },
      emptyArray: { fontFamily: `a${[]}b` },
      arrayOfNull: { fontFamily: `a${[null]}b` },
      nestedArray: { fontFamily: `a${[[1], [2]]}b` },
      arrayOfObjects: { fontFamily: `a${[{}, {}]}b` },
    });
  "#
);

// The namespace object, folded to the map of function configs this compiler
// registers for it. It is an object upstream -- `import * as stylex` binds one
// whose properties happen to be functions -- so it takes the object default
// too. This is the row that made the fold's string coercion agree with the
// spread's, which had read it as an object all along.
stylex_test!(
  the_folded_namespace_map_takes_the_object_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      folded: { fontFamily: `a${stylex}b` },
      shadowed: (stylex) => ({ height: `${stylex}px` }),
    });
  "#
);

// A theme reference carries its own `toString`, which answers the var-group
// hash rather than the object default -- the one value in this file whose
// string is not a language default.
stylex_test!(
  a_theme_reference_answers_its_own_to_string,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';
    export const styles = stylex.create({
      themed: { fontFamily: `a${zIndex}b` },
    });
  "#
);

// ── Structure the coercion has to survive ───────────────────────────

// More than one interpolation, and one at each end with no surrounding text, so
// the quasi/expression interleaving is read rather than assumed.
stylex_test!(
  several_interpolations_and_the_boundaries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      several: { fontFamily: `${null}a${undefined}b${true}` },
      onlyOne: { fontFamily: `${null}` },
      backToBack: { fontFamily: `${null}${undefined}` },
      trailing: { fontFamily: `a${null}` },
      leading: { fontFamily: `${null}b` },
    });
  "#
);

// A template nested inside an interpolation of another, and one holding an
// escape sequence and a non-ASCII character, so the coercion is shown not to
// disturb the text around it.
stylex_test!(
  a_nested_template_and_escaped_and_non_ascii_text,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      nested: { fontFamily: `a${`x${null}y`}b` },
      escaped: { content: `"\\2014 ${null}"` },
      nonAscii: { fontFamily: `"日本語${null}"` },
      newline: { fontFamily: `a${null}\nb` },
    });
  "#
);

// The interpolation reached through arithmetic, a member read and a call rather
// than written as a value, so the coercion is shown to run on whatever the
// interpolation evaluated to and not on what it was written as.
stylex_test!(
  an_interpolation_reached_through_an_expression,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const O = { a: null };
    const A = [null];

    export const styles = stylex.create({
      arithmetic: { fontFamily: `a${1 / 0}b` },
      member: { fontFamily: `a${O.a}b` },
      missingMember: { fontFamily: `a${O.b}b` },
      indexed: { fontFamily: `a${A[0]}b` },
      pastTheEnd: { fontFamily: `a${A[9]}b` },
      called: { fontFamily: `a${String(null)}b` },
      logical: { fontFamily: `a${null ?? undefined}b` },
    });
  "#
);

// ── What has no string at all ───────────────────────────────────────

// A function, which is the one value with no compile-time string: `String(fn)`
// is its source text and this evaluator keeps none.
//
// A deliberate divergence, and the only one in this file. The reference
// implementation answers here -- with the source text of its *own* evaluator
// closure, an internal artifact of the compiler that would be hashed into a
// class name and written into a stylesheet. Refusing is the answer that serves
// an author, and a refusal is loud where a wrong class name is silent.
stylex_test_panic!(
  a_function_has_no_string_and_refuses,
  "Expected a string value but received a non-string expression.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      fn: { fontFamily: `a${() => 1}b` },
    });
  "#
);

// A function expression bound to a name refuses earlier and for a different
// reason -- the evaluator has no reading of the node at all, so it never
// reaches a coercion. Pinned beside the arrow above because the two look like
// one case and are two, and a change that routed the arrow through this path
// instead would otherwise pass unnoticed.
stylex_test_panic!(
  a_function_reached_through_a_binding_refuses_too,
  "Unsupported expression: FunctionExpression",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const f = function () { return 1; };

    export const styles = stylex.create({
      fn: { fontFamily: `a${f}b` },
    });
  "#
);

// An interpolation the evaluator cannot resolve at all refuses with its own
// reason rather than with the coercion's -- the refusal that was already
// recorded is the one an author needs, and it must not be overwritten by the
// one this position would give.
stylex_test_panic!(
  an_unresolvable_interpolation_keeps_its_own_reason,
  "Referenced constant is not defined",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      unknown: { fontFamily: `a${missing}b` },
    });
  "#
);

// A dynamic style's parameter has no compile-time value, so the whole template
// falls to the runtime rather than refusing -- the interpolation is not missing,
// it is not knowable yet, and those are different answers.
stylex_test!(
  a_dynamic_parameter_in_a_template_falls_to_the_runtime,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ fontFamily: `a${h}b` }),
    });
  "#
);

// `Object()` applied to the fold, which is the other consumer of the same
// classification: the object bridge reads `ToObject`, where a function needs a
// wrapper and an object is its own identity. Reading the map as an object makes
// this agree too, and it is pinned because the string bridge's test above would
// pass whichever way this arm went.
stylex_test!(
  the_folded_namespace_map_wrapped_in_object_takes_the_same_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      wrapped: { fontFamily: `x${Object(stylex)}y` },
    });
  "#
);
