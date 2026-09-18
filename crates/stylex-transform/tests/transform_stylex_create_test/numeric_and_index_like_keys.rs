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
//! Both readings apply to a namespace name as well as to a style key, because
//! `create` receives an object too. A namespace that enumerates first is
//! compiled first, so its rules reach the stylesheet first.
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

// ── The order an index-like namespace takes ─────────────────────────

// The same reading applies to the namespace names, because `create` receives an
// object too. `0` is an index and comes first, and `+0` keeps its written place
// among the string keys -- so the namespaces reach the emitted object, and their
// rules the stylesheet, in the order `0`, `+0`, `root`.
stylex_test!(
  an_index_namespace_is_declared_before_a_signed_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      '+0': { color: 'red' },
      0: { color: 'blue' },
      root: { color: 'green' },
    });
  "#
);

// Every rule the index reading makes, over the namespace names: an index sorts
// numerically rather than by its spelling, `01` and the empty name are ordinary
// string keys, and `4294967295` is one too because it is the one integer below
// 2^32 that no array can index.
stylex_test!(
  index_like_namespaces_take_the_enumeration_order,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      z: { color: 'red' },
      2: { color: 'blue' },
      '01': { color: 'green' },
      0: { color: 'black' },
      4294967295: { color: 'white' },
      4294967294: { color: 'yellow' },
      '': { color: 'transparent' },
    });
  "#
);

// A dynamic style is ordered by the same reading, and carries its function and
// its `@property` rule with it. The rules come out in the order the namespaces
// do, so the width property is declared before the color one.
stylex_test!(
  an_index_namespace_orders_its_dynamic_style_too,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      named: (c) => ({ color: c }),
      0: (w) => ({ width: w }),
    });
  "#
);

// An index sorts by its value, not by its spelling, so `9` comes before `10`
// and `10` before `100`. A lexicographic sort would give the written order back.
stylex_test!(
  index_namespaces_sort_by_value_and_not_by_spelling,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      10: { color: 'red' },
      9: { color: 'blue' },
      100: { color: 'green' },
    });
  "#
);

// ── A computed key that is not a string or a number ─────────────────

// A computed key names the property `String(key)`, which is what the language
// names it. Every row refused here before, as a key with no name.
//
// None of them is a property anybody writes on purpose. What decided the
// answer is that the refusal was inherited rather than chosen: the key was read
// by the converter that spells a string, which answers for a string and a
// number and nothing else.
//
// Every row is measured against `@stylexjs/babel-plugin@0.19.0` and agrees with
// it, class name included: `x11kd1pe{true:red}`, `x5c1kzx{false:red}`,
// `x1e7wc9j{null:red}`, `xc954yn{1,2:red}` and `x1xfulhv{:red}`. The last two
// are not declarations a stylesheet can carry, and upstream writes them too --
// nothing downstream of the key rejects a name with a comma in it, or an empty
// one, in either compiler.
stylex_test!(
  a_computed_key_names_the_string_the_language_names_it,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      yes: { [true]: 'red' },
      no: { [false]: 'red' },
      nothing: { [null]: 'red' },
      negated: { [!1]: 'red' },
      list: { [[1, 2]]: 'red' },
      emptyList: { [[]]: 'red' },
    });
  "#
);

// A comparison names the word its boolean spells rather than the digit its
// number did. This compiler answered `0` and `1`, so the two compilers wrote
// two different property names for one source with no error either side.
stylex_test!(
  a_comparison_as_a_computed_key_names_its_word,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      lesser: { [1 > 2]: 'red' },
      greater: { [2 > 1]: 'red' },
      same: { [1 === 1]: 'red' },
    });
  "#
);

// An object key names `[object Object]`, which is what the language names it
// and what upstream names it. What the two do with that name then parts:
// upstream writes `.x1ffvn62[object Object]{[object object]:red}`, a selector
// with a space and a bracket in it that no stylesheet can carry, and the CSS
// layer here drops the declaration.
//
// Left as it is. Agreement would mean emitting a broken selector on purpose.
// What the key *names* is the half ticket 49 is about, and it is the text the
// language gives; upstream's declaration spells it in lower case, which is its
// CSS layer rather than its key. The next row shows the drop is of that one
// declaration and not of the namespace around it.
stylex_test!(
  an_object_as_a_computed_key_names_its_default_text,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      empty: { [{}]: 'red' },
    });
  "#
);

// The same key with a value the CSS layer keeps, so what the key names is
// visible rather than inferred from an empty namespace.
stylex_test!(
  an_object_key_beside_one_the_css_layer_keeps,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      mixed: { [{}]: 'red', color: 'blue' },
    });
  "#
);

// A comparison written where a number, a string or a condition is asked for.
// Its boolean reads as `1` or `0` for a number, as its word for a string, and
// as itself for a condition -- which is what the language does and what
// upstream writes, class name for class name.
stylex_test!(
  a_comparison_read_where_a_value_of_another_kind_is_asked_for,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      added: { width: (1 < 2) + 1 },
      negated: { height: -(1 < 2) },
      multiplied: { top: (1 < 2) * 3 },
      interpolated: { content: `x${1 < 2}y` },
      asCondition: { color: (1 < 2) ? 'red' : 'blue' },
    });
  "#
);
