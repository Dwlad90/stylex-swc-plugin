//! `undefined` inside a value a name carries crosses the bridge into the
//! engine.
//!
//! The bridge carried strings, numbers, booleans and `null`, and nothing for
//! the one value the grammar has no literal for — so a named array or object
//! holding `undefined` declined the whole fold, while the same array written
//! out folded. That was an omission rather than a decided rule: the four
//! refusals the fold names are argued for, and this was not one of them.
//!
//! `void 0` is the same value under a different spelling and reaches the bridge
//! as the same identifier, so one arm answers both.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options, so each case asserts
//! agreement with the reference compiler rather than agreement with this
//! compiler's own previous answer.

use crate::utils::{
  prelude::*,
  transform::{assert_folds, assert_refuses},
};

// ──────────────────────────────────────────────
// Both spellings, in a named array
// ──────────────────────────────────────────────

/// The shape the gap was reported as, and the spelling beside it. A name is the
/// only difference between these and the literals that folded all along.
#[test]
fn both_spellings_of_undefined_cross_inside_a_named_array() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['a', undefined, 'b'];",
      "content: a.join('-'),",
      ".x62v6bd{content:\"a--b\"}",
    ),
    (
      "const a = ['a', void 0, 'b'];",
      "content: a.join('-'),",
      ".x62v6bd{content:\"a--b\"}",
    ),
    // The default separator renders it the same way, so the value rather than
    // the argument is what the empty piece comes from.
    (
      "const a = ['a', undefined, 'b'];",
      "content: a.join(),",
      ".x16cvmne{content:\"a,,b\"}",
    ),
    // Nothing but the value, so the join has no other piece to hide it behind.
    (
      "const a = [undefined];",
      "content: a.join('-') || 'empty',",
      ".x15y8a3i{content:\"empty\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// Every element of a carried array reaches the engine, so the value is found
/// as deep as it is written — including under a second spelling one level in.
#[test]
fn an_undefined_nested_inside_a_named_array_crosses_too() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['a', [undefined, 'b']];",
      "content: a.flat().join('-'),",
      ".x62v6bd{content:\"a--b\"}",
    ),
    (
      "const a = ['a', [void 0, 'b']];",
      "content: a.flat().join('-'),",
      ".x62v6bd{content:\"a--b\"}",
    ),
    (
      "const a = ['a', [undefined, ['b', undefined]]];",
      "content: a.flat(2).join('-'),",
      ".xovswd4{content:\"a--b-\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The methods that answer *about* an element rather than rendering it see the
/// same value, so the array a fold reads is the array the author wrote and not
/// one shortened by a hole.
#[test]
fn a_carried_undefined_is_an_element_the_array_methods_count() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['a', undefined, 'b'];",
      "zIndex: a.length,",
      ".xzkaem6{z-index:3}",
    ),
    (
      "const a = ['a', undefined, 'b'];",
      "zIndex: a.indexOf(undefined),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const a = [undefined, 'a', undefined];",
      "zIndex: a.lastIndexOf(undefined),",
      ".xhtitgo{z-index:2}",
    ),
    (
      "const a = ['a', undefined, 'b'];",
      "content: a.includes(undefined) ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const a = [undefined];",
      "content: a.some(x => x === undefined) ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const a = ['a', undefined, 'b'];",
      "transitionProperty: a.slice(1).join(','),",
      ".x1v5bcxw{transition-property:,b}",
    ),
    (
      "const a = [undefined];",
      "transitionProperty: a.concat(['b']).join(','),",
      ".x1v5bcxw{transition-property:,b}",
    ),
    (
      "const a = ['a', undefined];",
      "content: String(a.at(1)),",
      ".x1pjt2f5{content:\"undefined\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// A named object
// ──────────────────────────────────────────────

/// A property whose value is `undefined` is a property the object has, so the
/// key is one `Object.keys` answers — the case the ticket names.
#[test]
fn an_object_property_holding_undefined_folds() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const o = {a: undefined};",
      "content: Object.keys(o).join(','),",
      ".x16319ns{content:\"a\"}",
    ),
    (
      "const o = {a: undefined, b: 1};",
      "content: Object.keys(o).join(','),",
      ".xprt6xs{content:\"a,b\"}",
    ),
    (
      "const o = {a: undefined, b: 1};",
      "content: Object.values(o).join(','),",
      ".x18vzqz4{content:\",1\"}",
    ),
    (
      "const o = {a: void 0, b: 1};",
      "content: Object.values(o).join(','),",
      ".x18vzqz4{content:\",1\"}",
    ),
    (
      "const o = {a: undefined, b: 1};",
      "content: Object.entries(o).flat().join(','),",
      ".x1hvs042{content:\"a,,b,1\"}",
    ),
    // The value nested a level down, under both a key and an index.
    (
      "const o = {a: {b: undefined, c: 1}};",
      "content: Object.keys(o.a).join(','),",
      ".x168xesk{content:\"b,c\"}",
    ),
    (
      "const o = {a: [undefined, {b: undefined}]};",
      "content: Object.keys(o.a[1]).join(','),",
      ".xa8xio6{content:\"b\"}",
    ),
    // `undefined` as a *key* is the ordinary string of that name, and is not
    // this value at all.
    (
      "const o = {undefined: 1};",
      "content: Object.keys(o).join(','),",
      ".x1pjt2f5{content:\"undefined\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// Read back out of the value rather than rendered inside it, so the identifier
/// the bridge answers with reaches the coercions that name it.
#[test]
fn a_carried_undefined_reads_back_as_the_value_it_is() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const o = {a: undefined};",
      "content: String(o.a),",
      ".x1pjt2f5{content:\"undefined\"}",
    ),
    (
      "const a = [undefined];",
      "content: typeof a[0],",
      ".x1pjt2f5{content:\"undefined\"}",
    ),
    (
      "const o = {a: undefined};",
      "content: o.a ?? 'fallback',",
      ".xlejn2x{content:\"fallback\"}",
    ),
    // The whole array coerced to a string, which renders the value as nothing
    // in both the explicit and the template spelling.
    (
      "const a = ['a', undefined];",
      "content: String(a),",
      ".x3j72fc{content:\"a,\"}",
    ),
    (
      "const a = ['a', undefined];",
      "content: `${a}`,",
      ".x3j72fc{content:\"a,\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// A method that can build a string far larger than the one it was handed is
/// admitted only against a width the guard could read, so the guard above the
/// bridge has to read this value too. It could not, and went on refusing the
/// call the bridge had just made carryable.
///
/// The width it reads is the width of the name, because a callback holding the
/// element renders it that way — a join renders it as nothing, so the name is
/// the wider of the two readings and the safe one to bound against. `null` next
/// door is read the same way, for the same reason.
#[test]
fn an_amplifying_method_can_read_how_wide_a_carried_undefined_renders() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['a', undefined, 'b'];",
      "content: a.join('-').padEnd(8, 'x'),",
      ".x1nfswek{content:\"a--bxxxx\"}",
    ),
    (
      "const a = ['a', undefined, 'b'];",
      "content: a.join('').padStart(9, '.'),",
      ".xieuv9q{content:\".......ab\"}",
    ),
    (
      "const o = {a: undefined, b: 'q'};",
      "content: Object.values(o).join('-').padEnd(6, 'z'),",
      ".x1pdfw4o{content:\"-qzzzz\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// At size
// ──────────────────────────────────────────────

/// The value costs an entry like any other element, so a binding full of them
/// crosses while it fits the entry ceiling and names that ceiling once it does
/// not. Five thousand of them fold to the reference compiler's own rule.
#[test]
fn a_binding_full_of_undefined_is_bounded_by_the_entry_ceiling() {
  let fill = |count: usize| {
    let mut elements = vec!["undefined"; count];
    elements.push("'z'");

    format!("const a = [{}];", elements.join(", "))
  };

  assert_folds(
    &fill(5_000),
    "content: a.join('') + '!',",
    ".xar2xyp{content:\"z!\"}",
  );

  assert_refuses(
    &fill(10_001),
    "content: a.join('') + '!',",
    "At most 10000 elements and properties are supported.",
  );
}

/// Nesting the value as deep as the evaluator's own ceiling allows, so what the
/// bridge answers for is a leaf a long walk reaches rather than a top-level one.
#[test]
fn an_undefined_at_the_bottom_of_a_deeply_nested_binding_crosses() {
  let levels = 20;
  let nested = format!("{}undefined{}", "[".repeat(levels), "]".repeat(levels));

  assert_folds(
    &format!("const a = [{}, 'z'];", nested),
    "content: a.flat(20).join('-'),",
    ".xagp7ty{content:\"-z\"}",
  );
}

// ──────────────────────────────────────────────
// What still does not fold
// ──────────────────────────────────────────────

/// A module binding of the name is not the value: the evaluator refuses a
/// shadowed `undefined` ahead of the bridge, so the name never reaches the
/// bridge as the global and no fold reads it as one. Pinned under both
/// declaration keywords, because matching the name alone is what the inward arm
/// rests on. The reference compiler refuses the same input, in its own words.
#[test]
fn a_shadowed_undefined_is_not_the_value() {
  for keyword in ["const", "let"] {
    assert_refuses(
      &format!("{} undefined = 'x'; const a = ['a', undefined];", keyword),
      "content: a.join('-'),",
      "Referenced constant is not initialized.",
    );
  }
}

/// A *callback* parameter of the name is a different question, and the engine
/// answers it: the value crossed before the arrow was printed, and the language
/// shadows the name inside the body exactly as it shadows any other. So the
/// element is still the value and the parameter is still the element.
#[test]
fn a_callback_parameter_named_undefined_shadows_only_inside_the_callback() {
  assert_folds(
    "const a = ['a', undefined];",
    "content: a.map((undefined) => String(undefined)).join('-'),",
    ".x1nf5qzh{content:\"a-undefined\"}",
  );
}

/// The value crosses back out, and the check that owns style values is then the
/// one that refuses it — the same division of labour, and the same sentence,
/// the reference compiler has. It used to be the fold that refused, which named
/// a rule for a value the language had answered perfectly well.
#[test]
fn an_undefined_a_fold_answers_is_refused_where_it_lands() {
  let cases: &[(&str, &str)] = &[
    // A search that found nothing.
    ("const a = ['a', 'b'];", "content: a.find(x => x === 'z'),"),
    // A method whose whole answer is nothing.
    ("const a = ['a'];", "content: a.forEach(x => x),"),
    // A read past the end.
    ("const a = ['a'];", "content: a.at(99),"),
  ];

  for (decls, body) in cases {
    assert_refuses(
      decls,
      body,
      "A style value can only contain an array, string or number.",
    );
  }
}
