//! `Object.keys`, `Object.values` and `Object.entries` of every receiver the
//! evaluator can hand them.
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

// The namespace fold, through all three methods. Upstream answers `when,env` for
// the keys where this answers `when`: `env` is registered as a member read
// rather than as an entry of the fold, so it is absent from the map every one of
// the three readers reads -- including the spread, which answered `when` alone
// before this file existed. The divergence is the fold's contents, not this
// classification, and is filed separately.
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

// The keys of a fold read as a list rather than joined: an index off it, its
// length, and the list spread into another array. A classification that answered
// `[]` gave every one of these a different answer too, and none of them refused.
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
//
// `arrayWithAHole` is the one row here that diverges, and in this compiler's
// favour: `Object.keys([, 'p'])` is `['1']`, because a hole has no key of its
// own, and that is what this answers. Upstream aborts the whole module with
// `Unexpected error:` -- it reads the hole as a node and fails on it, so the
// divergence is a crash on its side rather than a different list.
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
      arrayWithAHole: { fontVariant: `x${Object.keys([, 'p'])}y` },
      nestedObject: { fontStretch: `x${Object.keys({ p: { q: 1 } })}y` },
      numericKeys: { textEmphasis: `x${Object.keys({ 2: 'a', 1: 'b' })}y` },
      nonAsciiKeys: { textDecorationLine: `x${Object.keys({ 'é': 1 })}y` },
    });
  "#
);
