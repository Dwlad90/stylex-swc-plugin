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
//! Ticket 47 of `.scratch/split-transform-crate` holds the list, the sites it
//! struck, and the ones still open.

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
