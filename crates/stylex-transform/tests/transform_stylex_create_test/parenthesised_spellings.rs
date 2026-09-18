//! A parenthesis is not a different expression.
//!
//! A parenthesis is a node in this compiler's tree and none in the reference
//! implementation's, so a reader that matches a bare node sees a different
//! expression from the one the author wrote. Every case here compiles the
//! parenthesised spelling to exactly what the bare one compiles to, which is
//! also what `@stylexjs/babel-plugin@0.19.0` compiles both to.
//!
//! The defect has two shapes, and the second is the worse one. A reader that
//! validates refuses, and the build stops on an expression the same author
//! could have written without the brackets. A reader that dispatches hands the
//! call back untransformed, and the author gets no error and no styles.
//!
//! Ticket 47 of `.scratch/split-transform-crate` holds the whole list, and the
//! case that decided each site -- the ones that were fixed and the ones whose
//! two spellings already agreed and were struck.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// The argument of `create`, read by the validator and by the evaluator beside
// it. Both stopped the build with `create() can only accept an object.`
stylex_test!(
  a_parenthesised_create_argument,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create(({ a: { color: 'red' } }));
  "#
);

// The argument of `keyframes`, which stopped the build the same way. The three
// producers that take one object argument share the reader.
stylex_test!(
  a_parenthesised_keyframes_argument,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fade = stylex.keyframes(({ from: { color: 'red' } }));
  "#
);

// A producer call parenthesised as the whole initializer. The declarator was
// recorded under no key, so the call was never found and reached the runtime
// untransformed -- the silent shape.
stylex_test!(
  a_parenthesised_producer_initializer,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fade = (stylex.keyframes({ from: { color: 'red' } }));
    export const fallback = (stylex.positionTry({ positionAnchor: '--a' }));
    export const vtc = (stylex.viewTransitionClass({ group: { color: 'red' } }));
  "#
);

// `create` parenthesised as the whole initializer, which is the shape that
// reads through every index at once: the declarator, the top-level expression,
// and the style variable whose namespace object the pruner later rewrites.
//
// It stopped the build with `create() calls must be bound to a bare variable.`
// A namespace read beside it, so an entry that is used and one that is not are
// both here -- a lookup that answered the wrong declarator would drop the used
// one and leave the element unstyled.
stylex_test!(
  a_parenthesised_create_initializer,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = (stylex.create({
      used: { color: 'red' },
      unused: { color: 'blue' },
    }));
    export const p = stylex.props(styles.used);
  "#
);

// The two lookups that answer "is this call bound to a bare variable" are asked
// together, and a parenthesis that blinded only one of them decided which of
// two refusals an author read. Both read through it now, so the answer is one.
//
// `defineMarker` is where the pair is asked. The call is recognised as bound
// now and reaches the refusal upstream gives for the same source, word for
// word, rather than being told it is not bound to a bare variable.
stylex_test_panic!(
  a_parenthesised_define_marker_refuses_as_the_bare_one_does,
  "Unable to generate hash for defineMarker()",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const m = (stylex.defineMarker());
  "#
);

// The callee, at both levels. `(stylex.create)({…})` and `(stylex).create({…})`
// name the function the bare spelling names, and neither was recognised as a
// StyleX call at all -- the whole module reached the runtime with no styles.
//
// A `props` read sits beside each, because the callee is read twice: once to
// dispatch the producer, and once by the predicate that tells a producer call
// from a consumer one. A fix in only one of the two leaves the consumer call
// classed and never transformed, which drops the namespace the pruner keeps.
stylex_test!(
  a_parenthesised_create_callee,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = (stylex.create)({ a: { color: 'red' } });
    export const other = (stylex).create({ b: { display: 'flex' } });
    export const p = (stylex.props)(styles.a, other.b);
  "#
);

// Nested to three levels, and on both sides of the dot at once. Every reader
// unwraps in a loop rather than one layer deep, so depth changes nothing.
stylex_test!(
  a_deeply_parenthesised_create_callee,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = (((stylex.create)))({ a: { color: 'red' } });
    export const other = ((((stylex))).create)({ b: { display: 'flex' } });
  "#
);

// The merge arguments. `stylex.props((styles.a))` reached no arm of the reader
// that resolves an argument, and `stylex.props(([a, b]))` was never flattened
// into its elements -- both handed the whole merge back to the runtime, with no
// error for the author to read.
stylex_test!(
  a_parenthesised_props_argument,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color: 'red' },
      b: { display: 'flex' },
    });
    export const one = stylex.props((styles.a));
    export const many = stylex.props(([styles.a, styles.b]));
    export const nested = stylex.props((((styles.a))), ([(styles.b)]));
    export const receiver = stylex.props((styles).a);
    export const computed = stylex.props(styles[('a')]);
  "#
);

// A conditional and a logical merge, whose two sides are read by the same
// resolver. Both were already answered through the parenthesis; they are here
// so a reader that stops unwrapping cannot pass this file.
stylex_test!(
  a_parenthesised_conditional_props_argument,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color: 'red' },
      b: { display: 'flex' },
    });
    export const picked = (cond) => stylex.props(cond ? (styles.a) : (styles.b));
    export const guarded = (cond) => stylex.props(cond && (styles.b));
  "#
);

// A declarator bound to a pattern declares no name, so the call's own position
// is the only thing that marks it program level. Recorded bare, the
// parenthesised call was not marked, and the transform hoisted its result into
// a variable the author did not write.
stylex_test!(
  a_parenthesised_pattern_bound_initializer,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const { a } = (stylex.create({ a: { color: 'red' } }));
    export const p = stylex.props(a);
  "#
);

// A top-level array holding the call, parenthesised. The span index that
// answers "is this call inside a recorded array" and the validator that asks
// the same question of the same shape now read it the same way.
//
// The bare spelling is beside it in the same module, so the snapshot shows the
// two side by side rather than in two files under two harnesses.
stylex_test!(
  a_parenthesised_top_level_array_initializer,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const bare = [stylex.create({ a: { color: 'red' } })];
    export const wrapped = ([stylex.create({ b: { display: 'flex' } })]);
  "#
);

// A callee inside a style value, which the evaluator reads rather than the
// visitor. A function the module bound, a method on an object it bound, and one
// of this compiler's own functions -- each at both levels of the callee.
stylex_test!(
  a_parenthesised_callee_in_a_style_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const pick = () => 'red';
    const palette = { pick: () => 'blue' };
    export const styles = stylex.create({
      a: { color: (pick)() },
      b: { color: (palette.pick)() },
      c: { color: (palette).pick() },
      d: { transform: (stylex.firstThatWorks)('translate(1px)', 'none') },
      e: { transform: (stylex).firstThatWorks('translate(2px)', 'none') },
      f: { color: (((pick)))() },
    });
  "#
);

// ──────────────────────────────────────────────
// The guard
// ──────────────────────────────────────────────

/// Every shape in the class, compiled both ways and compared.
///
/// The snapshots above say what one spelling prints. This says the two
/// spellings print the *same thing*, which is the claim a new reader can break.
/// A reader added later that matches a bare node fails here for the shape it
/// reads, without anyone having to remember that parentheses are a node in this
/// tree and none in the reference implementation's.
///
/// Each row is `(shape, bare, wrapped)`, and the two sources differ only by the
/// brackets -- a row whose sources differ in anything else would compare two
/// modules rather than two spellings of one.
#[test]
fn every_shape_in_the_class_compiles_alike_in_both_spellings() {
  const IMPORT: &str = "import * as stylex from '@stylexjs/stylex';";

  let rows: [(&str, String, String); 24] = [
    (
      "the create argument",
      format!("{IMPORT} export const s = stylex.create({{ a: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const s = stylex.create(({{ a: {{ color: 'red' }} }}));"),
    ),
    (
      "the create callee's property",
      format!("{IMPORT} export const s = stylex.create({{ a: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const s = (stylex.create)({{ a: {{ color: 'red' }} }});"),
    ),
    (
      "the create callee's receiver",
      format!("{IMPORT} export const s = stylex.create({{ a: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const s = (stylex).create({{ a: {{ color: 'red' }} }});"),
    ),
    (
      "the create initializer",
      format!("{IMPORT} export const s = stylex.create({{ a: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const s = (stylex.create({{ a: {{ color: 'red' }} }}));"),
    ),
    (
      "a producer initializer",
      format!("{IMPORT} export const f = stylex.keyframes({{ from: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const f = (stylex.keyframes({{ from: {{ color: 'red' }} }}));"),
    ),
    (
      "a producer argument",
      format!("{IMPORT} export const f = stylex.keyframes({{ from: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const f = stylex.keyframes(({{ from: {{ color: 'red' }} }}));"),
    ),
    (
      "a pattern-bound initializer",
      format!(
        "{IMPORT} const {{ a }} = stylex.create({{ a: {{ color: 'red' }} }}); export const p = stylex.props(a);"
      ),
      format!(
        "{IMPORT} const {{ a }} = (stylex.create({{ a: {{ color: 'red' }} }})); export const p = stylex.props(a);"
      ),
    ),
    (
      "a top-level array initializer",
      format!("{IMPORT} export const all = [stylex.create({{ a: {{ color: 'red' }} }})];"),
      format!("{IMPORT} export const all = ([stylex.create({{ a: {{ color: 'red' }} }})]);"),
    ),
    (
      "a props argument",
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }} }}); export const p = stylex.props(s.a);"
      ),
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }} }}); export const p = stylex.props((s.a));"
      ),
    ),
    (
      "a props argument's receiver",
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }} }}); export const p = stylex.props(s.a);"
      ),
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }} }}); export const p = stylex.props((s).a);"
      ),
    ),
    (
      "a props array argument",
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }}, b: {{ display: 'flex' }} }}); export const p = stylex.props([s.a, s.b]);"
      ),
      format!(
        "{IMPORT} const s = stylex.create({{ a: {{ color: 'red' }}, b: {{ display: 'flex' }} }}); export const p = stylex.props(([s.a, s.b]));"
      ),
    ),
    (
      "a bound function called in a style value",
      format!(
        "{IMPORT} const pick = () => 'red'; export const s = stylex.create({{ a: {{ color: pick() }} }});"
      ),
      format!(
        "{IMPORT} const pick = () => 'red'; export const s = stylex.create({{ a: {{ color: (pick)() }} }});"
      ),
    ),
    (
      "a method called in a style value",
      format!(
        "{IMPORT} const o = {{ pick: () => 'red' }}; export const s = stylex.create({{ a: {{ color: o.pick() }} }});"
      ),
      format!(
        "{IMPORT} const o = {{ pick: () => 'red' }}; export const s = stylex.create({{ a: {{ color: (o.pick)() }} }});"
      ),
    ),
    (
      "one of this compiler's own functions",
      format!(
        "{IMPORT} export const s = stylex.create({{ a: {{ transform: stylex.firstThatWorks('translate(1px)', 'none') }} }});"
      ),
      format!(
        "{IMPORT} export const s = stylex.create({{ a: {{ transform: (stylex.firstThatWorks)('translate(1px)', 'none') }} }});"
      ),
    ),
    (
      "one of this compiler's own functions, at the receiver",
      format!(
        "{IMPORT} export const s = stylex.create({{ a: {{ transform: stylex.firstThatWorks('translate(1px)', 'none') }} }});"
      ),
      format!(
        "{IMPORT} export const s = stylex.create({{ a: {{ transform: (stylex).firstThatWorks('translate(1px)', 'none') }} }});"
      ),
    ),
    (
      "a dynamic style function",
      format!(
        "{IMPORT} export const s = stylex.create({{ root: (color) => ({{ backgroundColor: 'red', color }}) }});"
      ),
      format!(
        "{IMPORT} export const s = stylex.create({{ root: ((color) => ({{ backgroundColor: 'red', color }})) }});"
      ),
    ),
    (
      "a dynamic style call",
      format!(
        "{IMPORT} const s = stylex.create({{ root: (color) => ({{ color }}) }}); export const p = stylex.props(s.root('red'));"
      ),
      format!(
        "{IMPORT} const s = stylex.create({{ root: (color) => ({{ color }}) }}); export const p = stylex.props((s.root)('red'));"
      ),
    ),
    (
      "a CommonJS require of the StyleX module",
      "const stylex = require('@stylexjs/stylex'); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
      "const stylex = (require('@stylexjs/stylex')); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
    ),
    (
      "a CommonJS require of the atoms module",
      format!(
        "{IMPORT} const css = require('@stylexjs/atoms'); export const p = stylex.props(css.display.flex);"
      ),
      format!(
        "{IMPORT} const css = (require('@stylexjs/atoms')); export const p = stylex.props(css.display.flex);"
      ),
    ),
    (
      "the require callee",
      "const stylex = require('@stylexjs/stylex'); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
      "const stylex = (require)('@stylexjs/stylex'); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
    ),
    (
      "the required module name",
      "const stylex = require('@stylexjs/stylex'); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
      "const stylex = require(('@stylexjs/stylex')); export const s = stylex.create({ a: { color: 'red' } });".to_string(),
    ),
    (
      "a destructured require",
      "const { create } = require('@stylexjs/stylex'); export const s = create({ a: { color: 'red' } });".to_string(),
      "const { create } = (require)(('@stylexjs/stylex')); export const s = create({ a: { color: 'red' } });".to_string(),
    ),
    (
      "the create initializer, with a namespace the pruner drops",
      format!(
        "{IMPORT} const s = stylex.create({{ used: {{ color: 'red' }}, unused: {{ color: 'blue' }} }}); export function A(c) {{ return c ? s.used : s; }}"
      ),
      format!(
        "{IMPORT} const s = (stylex.create({{ used: {{ color: 'red' }}, unused: {{ color: 'blue' }} }})); export function A(c) {{ return c ? s.used : s; }}"
      ),
    ),
    (
      "a computed namespace key",
      format!("{IMPORT} export const s = stylex.create({{ ['a']: {{ color: 'red' }} }});"),
      format!("{IMPORT} export const s = stylex.create({{ [('a')]: {{ color: 'red' }} }});"),
    ),
  ];

  for (shape, bare, wrapped) in &rows {
    assert_spellings_agree(shape, bare, wrapped);
  }
}
