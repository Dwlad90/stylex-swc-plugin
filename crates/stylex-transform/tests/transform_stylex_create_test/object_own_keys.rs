//! `Object.keys`, `Object.values` and `Object.entries` of every receiver the
//! evaluator can hand them.
//!
//! The three statics fold in the engine now, like every other one, so what this
//! file is about is the receiver they cannot be asked of there: a folded
//! function map is not a JavaScript value at all. That is the only receiver
//! still answered below the fold, which is why the own-keys reader survived the
//! deletion of the two static name tables.
//!
//! Three places read "what own enumerable properties does this value have", and
//! the folded namespace map used to be classified differently in each: the
//! spread arm answered its keys, the object bridge answered "an object", and the
//! receiver normalizer had no arm for it at all -- so it read as "not an
//! object", and `Object.keys(stylex)` answered `[]`. That is the one answer that
//! is neither a refusal nor the truth, and it was reachable while the same
//! compiler spread those keys correctly one function away. All three now read a
//! fold through `function_fold_to_object`.
//!
//! `null` and `undefined` are the other half of the same classification. They
//! have no `ToObject`, so the language throws rather than answering `[]`, and
//! they are refused here rather than folded -- the reference implementation
//! stops the build on both.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options. Where the two still differ the case is a
//! `validation_stylex_create_test` row or a filed ticket, and says so.
//!
//! Two receivers are measured elsewhere because this harness cannot carry them.
//! A theme reference needs a resolvable `.stylex.js` module, so it is a
//! `rs-compiler` parity row: both compilers answer the empty list, each because
//! it holds a stand-in for a group whose keys live in another file. And spreading
//! a fold's key list back into an array refuses in both, which is
//! `validation_stylex_create_test::invalid_values::a_spread_key_list_of_a_fold_is_refused`.
//!
//! Runtime injection is on so each snapshot records the rule text beside the
//! class name: the keys are interpolated into a value, so the rule text is the
//! list itself.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── A fold as the receiver ──────────────────────────────────────────

// The namespace fold, through all three methods, and identical to upstream in
// all three. Getting the last of it took two changes: the classification this
// file is named for, and then `env` into the fold itself -- it was registered as
// a member read only, so the keys list was one short of the member reads that
// already worked. That is why the values row reads two objects and the entries
// row two pairs.
stylex_test!(
  the_three_methods_over_the_namespace_fold,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      keys: { fontFamily: `x${Object.keys(stylex)}y` },
      values: { fontWeight: `x${Object.values(stylex)}y` },
      entries: { content: `x${Object.entries(stylex)}y` },
    });
  "#
);

// The same fold reached by an alias and by a spread into a plain object, so the
// answer does not depend on how the map got to the receiver. The spread row is
// the reader that was already right, recorded beside the one that was not.
stylex_test!(
  the_fold_reached_by_an_alias_and_by_a_spread,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    const SPREAD = { ...sx };
    export const styles = sx.create({
      aliased: { fontFamily: `x${Object.keys(sx)}y` },
      spread: { fontWeight: `x${Object.keys(SPREAD)}y` },
    });
  "#
);

// One entry of the fold rather than the map. `fn` is the key the fold's object
// form carries for a single config, and upstream answers the same one -- it is
// each compiler's internal shape, and they happen to spell it alike.
stylex_test!(
  a_single_config_as_the_receiver,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { keyframes, firstThatWorks } from '@stylexjs/stylex';
    export const styles = stylex.create({
      keyframesKeys: { fontFamily: `x${Object.keys(keyframes)}y` },
      firstThatWorksKeys: { fontWeight: `x${Object.keys(firstThatWorks)}y` },
      counted: { width: Object.keys(keyframes).length },
    });
  "#
);

// The keys of a fold read as a list rather than joined: an index off it and its
// length. A classification that answered `[]` gave both of these a different
// answer too, and neither refused.
stylex_test!(
  the_key_list_of_a_fold_read_as_a_list,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      indexed: { fontFamily: Object.keys(stylex)[0] },
      counted: { width: Object.keys(stylex).length },
      pastTheEnd: { fontWeight: `x${Object.keys(stylex)[99]}y` },
    });
  "#
);

// The key list of a fold, asked the same question again. Such a list is an
// array in the second of its two spellings -- the literal a fold wrote -- and
// its own keys are its indices. A reader that knew only the evaluator's own
// list answered the empty one here.
//
// All three rows are byte-identical to upstream, class name included.
stylex_test!(
  the_key_list_of_a_fold_as_the_receiver,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      keys: { fontFamily: `x${Object.keys(Object.keys(stylex))}y` },
      values: { fontWeight: `x${Object.values(Object.keys(stylex))}y` },
      counted: { width: Object.keys(Object.entries(stylex)).length },
    });
  "#
);

// ── The receivers that are not objects ──────────────────────────────

// A primitive has an object wrapper carrying no own keys, so all of these fold
// to the empty list -- except a string, whose wrapper carries an index per code
// unit.
stylex_test!(
  a_primitive_receiver_has_no_own_keys,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      number: { fontFamily: `x${Object.keys(5)}y` },
      zero: { fontWeight: `x${Object.keys(0)}y` },
      notANumber: { content: `x${Object.keys(NaN)}y` },
      infinite: { fontStyle: `x${Object.keys(Infinity)}y` },
      yes: { fontVariant: `x${Object.keys(true)}y` },
      emptyString: { fontStretch: `x${Object.keys('')}y` },
      string: { textEmphasis: `x${Object.keys('ab')}y` },
      nonAsciiString: { textDecorationLine: `x${Object.keys('é中')}y` },
    });
  "#
);

// An object and an array as the receiver, which is the case the classification
// was already right about -- recorded so a change to the fold arm that broke
// them fails here.
stylex_test!(
  a_plain_object_or_array_receiver,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      empty: { fontFamily: `x${Object.keys({})}y` },
      object: { fontWeight: `x${Object.keys({ p: 1, q: 2 })}y` },
      array: { content: `x${Object.keys(['p', 'q'])}y` },
      emptyArray: { fontStyle: `x${Object.keys([])}y` },
      nestedObject: { fontStretch: `x${Object.keys({ p: { q: 1 } })}y` },
      numericKeys: { textEmphasis: `x${Object.keys({ 2: 'a', 1: 'b' })}y` },
      nonAsciiKeys: { textDecorationLine: `x${Object.keys({ 'é': 1 })}y` },
    });
  "#
);

// An array holding a hole, in any position, stops the module in both compilers
// and under the same sentence. The receiver is the value the array folded to,
// and an array with a hole folds to no value at all -- so the own-keys reader
// is never reached and the array's own refusal is what the author reads.
//
// This used to answer `['1']` here: the reader walked the array literal a
// second time out of the syntax, past the hole. Recorded as ticket 48 of
// `.scratch/split-transform-crate`.
stylex_test_panic!(
  an_array_receiver_holding_a_hole_is_refused,
  "Could not resolve the code being evaluated.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      arrayWithAHole: { fontVariant: `x${Object.keys([, 'p'])}y` },
    });
  "#
);
