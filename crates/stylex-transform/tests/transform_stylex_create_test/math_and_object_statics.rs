//! Which `Math` and `Object` statics fold, and which are refused.
//!
//! A static folds where the global's own allowlist holds it. An allowlist and
//! not a denylist, because `Object` carries reflective statics that answer with
//! an object from the prototype chain, and a denylist over that surface can
//! never be complete -- the static nobody listed is the next way out. The
//! allowlist is the reference compiler's, name for name, so the two compilers
//! fold and refuse the same calls.
//!
//! Inside the allowlist the surface is the language's: a receiver naming one of
//! these globals is printed into the engine like any other expression, so a
//! listed method needs no entry of its own to fold. That is what ends the one
//! place where *where* a call was written decided whether it folded.
//! `Math.trunc(1.5)` was refused written alone and folded written inside a
//! chain -- and the guard now walks a static exactly as it walks every other
//! call.
//!
//! What is left below the fold is not a surface at all: three statics read own
//! keys, and the receiver they are asked of can be something the engine never
//! sees — this compiler's own function fold, or an array with a hole in it. Those
//! are pinned in `object_own_keys`.
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
// The `Math` surface
// ──────────────────────────────────────────────

/// Every `Math` method the reference compiler folds, folded here to its own
/// class name and rule text.
///
/// One case per method rather than one per behaviour, because the claim under
/// test is exactly that the surface has no holes: twenty-seven of these were
/// refused before this work purely because nobody had listed them.
///
/// The arguments are the same pair throughout, so a method that reads only its
/// first still gets one and a method that reads two gets two — and the answers
/// are whatever the language says, including the four that are `NaN`.
#[test]
fn every_math_method_folds() {
  let cases: &[(&str, &str)] = &[
    ("Math.abs(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.acos(2, 3)", ".x1uhybf7{z-index:NaN}"),
    ("Math.acosh(2, 3)", ".xydejx2{z-index:1.317}"),
    ("Math.asin(2, 3)", ".x1uhybf7{z-index:NaN}"),
    ("Math.asinh(2, 3)", ".xl7lg49{z-index:1.4436}"),
    ("Math.atan(2, 3)", ".x6cd2bu{z-index:1.1071}"),
    ("Math.atan2(2, 3)", ".xe9859i{z-index:.588}"),
    ("Math.atanh(2, 3)", ".x1uhybf7{z-index:NaN}"),
    ("Math.cbrt(2, 3)", ".x2p7n9z{z-index:1.2599}"),
    ("Math.ceil(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.clz32(2, 3)", ".x68pp3s{z-index:30}"),
    ("Math.cos(2, 3)", ".x13bf9xl{z-index:-0.4161}"),
    ("Math.cosh(2, 3)", ".x1lyegjm{z-index:3.7622}"),
    ("Math.exp(2, 3)", ".xwqjgiu{z-index:7.3891}"),
    ("Math.expm1(2, 3)", ".x1jf5g5e{z-index:6.3891}"),
    ("Math.floor(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.fround(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.hypot(2, 3)", ".xzprwg9{z-index:3.6056}"),
    ("Math.imul(2, 3)", ".x1w0boku{z-index:6}"),
    ("Math.log(2, 3)", ".x175ikqp{z-index:.6931}"),
    ("Math.log10(2, 3)", ".x1qq3hxu{z-index:.301}"),
    ("Math.log1p(2, 3)", ".xs3carv{z-index:1.0986}"),
    ("Math.log2(2, 3)", ".x1vjfegm{z-index:1}"),
    ("Math.max(2, 3)", ".xzkaem6{z-index:3}"),
    ("Math.min(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.pow(2, 3)", ".x1q8xho0{z-index:8}"),
    ("Math.round(2, 3)", ".xhtitgo{z-index:2}"),
    ("Math.sign(2, 3)", ".x1vjfegm{z-index:1}"),
    ("Math.sin(2, 3)", ".xdn3m4z{z-index:.9093}"),
    ("Math.sinh(2, 3)", ".x16dd71e{z-index:3.6269}"),
    ("Math.sqrt(2, 3)", ".xheeyc7{z-index:1.4142}"),
    ("Math.tan(2, 3)", ".x4xc9oa{z-index:-2.185}"),
    ("Math.tanh(2, 3)", ".x13vhycq{z-index:.964}"),
    ("Math.trunc(2, 3)", ".xhtitgo{z-index:2}"),
  ];

  for (call, rule) in cases {
    assert_folds("", &format!("zIndex: {},", call), rule);
  }
}

/// The argument list a static is actually written with in a stylesheet: names,
/// arithmetic, and other statics inside it.
///
/// The arithmetic case is the shape a fluid type scale is written in, and it is
/// why a name holding a number has to cross the bridge — every operand of it is
/// a binding.
#[test]
fn a_static_reads_names_and_expressions_as_its_arguments() {
  assert_folds(
    "const a = 2, b = 3;",
    "zIndex: Math.pow(a, b),",
    ".x1q8xho0{z-index:8}",
  );

  assert_folds(
    "",
    "zIndex: Math.max(Math.min(3, 5), Math.abs(-1)),",
    ".xzkaem6{z-index:3}",
  );

  assert_folds(
    "const BASE = 16;",
    "zIndex: Math.round(BASE / Math.pow(1.2, 3) / 0.16) / 100,",
    ".x1fb59af{z-index:.58}",
  );
}

/// Position stops deciding the answer, which is the asymmetry this ticket
/// removes: the same call folds written alone, as the receiver of a chain, and
/// inside a callback the engine runs.
#[test]
fn a_static_folds_wherever_it_is_written() {
  assert_folds("", "zIndex: Math.trunc(1.5),", ".x1vjfegm{z-index:1}");

  assert_folds(
    "",
    "content: Math.trunc(1.9).toFixed(2),",
    ".x1x4qvu5{content:\"1.00\"}",
  );

  assert_folds(
    "",
    "content: [1.5, 2.5].map(n => Math.trunc(n)).join(','),",
    ".x1xix3aw{content:\"1,2\"}",
  );
}

// ──────────────────────────────────────────────
// The `Object` surface
// ──────────────────────────────────────────────

/// Every `Object` static the allowlist holds, folded here to its own class name
/// and rule text.
///
/// `getOwnPropertySymbols` answers the empty list, which is worth pinning
/// because the value crosses back whole rather than being flattened into a list
/// on the way.
#[test]
fn every_object_static_folds() {
  let cases: &[(&str, &str)] = &[
    ("Object.keys({a: 1})", ".x16319ns{content:\"a\"}"),
    ("Object.values({a: 1})", ".xvbgg8e{content:\"1\"}"),
    ("Object.entries({a: 1})", ".x436hi9{content:\"a,1\"}"),
    (
      "Object.getOwnPropertyNames({a: 1})",
      ".x16319ns{content:\"a\"}",
    ),
    (
      "Object.getOwnPropertySymbols({a: 1})",
      ".x14axycx{content:\"\"}",
    ),
    ("Object.isFrozen({a: 1})", ".x9g66vw{content:\"false\"}"),
    ("Object.isSealed({a: 1})", ".x9g66vw{content:\"false\"}"),
    ("Object.isExtensible({a: 1})", ".x1ez55b5{content:\"true\"}"),
    (
      "Object.hasOwn({a: 1}, \"a\")",
      ".x1ez55b5{content:\"true\"}",
    ),
    ("Object.is(1, 1)", ".x1ez55b5{content:\"true\"}"),
  ];

  for (call, rule) in cases {
    assert_folds("", &format!("content: String({}),", call), rule);
  }
}

/// The statics that answer with an object from the prototype chain, each
/// refused under the sentence written for them.
///
/// `Object.getPrototypeOf({})` is `Object.prototype`, whose `constructor` is
/// `Object`, whose `constructor` is `Function`. `create`,
/// `getOwnPropertyDescriptor` and the rest reach the same graph by other
/// routes, so none of them folds.
///
/// They read the one sentence every static outside the allowlist reads. It is a
/// loose fit -- these answer the same thing on every build -- and one sentence
/// is carried anyway, because a second would be a second table to keep.
///
/// Refused wrapped in a coercion as well as alone. Wrapping used to change the
/// answer -- the prototype never crossed the bridge, so `String(...)` folded
/// where the bare read refused -- and the rule now answers the call rather than
/// what its answer would have been.
#[test]
fn a_static_answering_a_prototype_is_refused() {
  let cases: &[&str] = &[
    "Object.getPrototypeOf({a: 1})",
    "Object.setPrototypeOf({}, {})",
    "Object.getOwnPropertyDescriptor({a: 1}, 'a')",
    "Object.getOwnPropertyDescriptors({a: 1})",
    "Object.create({a: 1})",
  ];

  for call in cases {
    let name = call.split('(').next().unwrap_or_default();
    let reason = format!(
      "Cannot fold '{}' at compile time.\nA fold has to answer from the source alone, and this call does not.",
      name
    );

    assert_refuses("", &format!("content: {},", call), &reason);
    assert_refuses("", &format!("content: String({}),", call), &reason);
  }
}

/// A static's answer is a value like any other, so it chains — a key list can be
/// sorted and joined, and a fold of one static can be the argument of the next.
#[test]
fn a_static_result_is_chainable() {
  assert_folds(
    "",
    "transitionProperty: Object.keys({b: 1, a: 2}).sort().join(','),",
    ".x1iq4t92{transition-property:a,b}",
  );

  assert_folds(
    "",
    "content: Object.keys(Object.fromEntries([['a', 1], ['b', 2]])).join(','),",
    ".xprt6xs{content:\"a,b\"}",
  );

  assert_folds(
    "",
    "content: Object.entries(Object.fromEntries(Object.entries({a: 1}))).join(','),",
    ".x436hi9{content:\"a,1\"}",
  );

  assert_folds(
    "const o = {a: 1, b: 2};",
    "zIndex: Object.entries(o).filter(e => e[1] % 2 === 0).length,",
    ".x1vjfegm{z-index:1}",
  );

  assert_folds(
    "",
    "content: Object.keys(Object.groupBy(['a', 'bb'], s => s.length)).join(','),",
    ".x1xix3aw{content:\"1,2\"}",
  );
}

/// A named receiver, which is the other half of the same rule: what a static is
/// given does not have to be written out either.
#[test]
fn a_static_reads_a_named_receiver() {
  assert_folds(
    "const o = { a: 'x', b: 'y' };",
    "content: Object.values(o).join('-'),",
    ".x16jpvqi{content:\"x-y\"}",
  );
}

/// `__proto__` written as a plain key sets the prototype rather than a member,
/// so the object the language sees has one own key.
///
/// This is a divergence closed rather than a behaviour kept. The deleted table
/// read the key as an ordinary property and answered `__proto__,a`, and the
/// fold's inward bridge was built to agree with the table rather than with the
/// reference compiler — one answer between the two paths, wrong in the same way.
/// Both spellings now answer `a`, which is what the language and the reference
/// compiler both say.
#[test]
fn a_proto_key_sets_the_prototype_rather_than_a_property() {
  assert_folds(
    "",
    "content: Object.keys({ __proto__: 'x', a: 'y' }).join(','),",
    ".x16319ns{content:\"a\"}",
  );

  assert_folds(
    "const o = { __proto__: 'x', a: 'y' };",
    "content: Object.keys(o).join(','),",
    ".x16319ns{content:\"a\"}",
  );
}

// ──────────────────────────────────────────────
// The refusals that have to survive
// ──────────────────────────────────────────────

/// The statics the reference compiler refuses by name are refused here too, and
/// each says which name — receiver included, since `assign` is `Object`'s and
/// `random` is `Math`'s and the method alone would not say which.
///
/// `random` answers something new every time it is asked, and a class name is a
/// hash of the declaration it names — so folding it would give one source a
/// different stylesheet on every build. The rest answer by changing the object
/// they were handed rather than by computing one.
///
/// Refused in a chain as well as alone, because a chain is where a nondeterministic
/// answer would otherwise slip through: the link is not the call anything else is
/// ever handed on its own.
#[test]
fn a_static_the_reference_compiler_refuses_is_refused_with_its_own_reason() {
  let cases: &[(&str, &str)] = &[
    ("zIndex: Math.random(),", "Math.random"),
    ("content: Math.random().toString(),", "Math.random"),
    (
      "content: String(Object.assign({}, {a: 1})),",
      "Object.assign",
    ),
    ("content: String(Object.freeze({a: 1})),", "Object.freeze"),
    ("content: String(Object.seal({a: 1})),", "Object.seal"),
    (
      "content: String(Object.defineProperty({}, 'a', {})),",
      "Object.defineProperty",
    ),
    (
      "content: String(Object.preventExtensions({a: 1})),",
      "Object.preventExtensions",
    ),
  ];

  for (body, name) in cases {
    assert_refuses(
      "",
      body,
      &format!("Cannot fold '{}' at compile time.", name),
    );
  }
}

/// A spread argument refuses under the one sentence a spread earns everywhere
/// else, rather than under the fold's own words.
///
/// The deleted table folded `Math.max(...[5, 0.1, 0.3])` by flattening the
/// spread's value into the argument list, which the reference compiler does not
/// do — so this is a divergence closed, not a fold lost.
#[test]
fn a_spread_argument_to_a_static_refuses_as_a_spread() {
  assert_refuses(
    "",
    "zIndex: Math.max(...[1, 2]),",
    "Unsupported expression: SpreadElement",
  );
}

/// A module that declares its own `String` is read as the module's value, not as
/// the global — which is the same question the fold asks of every other name,
/// and the reason the global check asks the binding table first.
#[test]
fn a_locally_declared_global_is_the_modules_own_value() {
  assert_folds(
    "const String = 'abc';",
    "content: String.toUpperCase(),",
    ".xj5ouxf{content:\"ABC\"}",
  );
}

/// `Math` contributes methods and nothing else, so calling it is not a fold in
/// either compiler. The sentence differs — the reference compiler reports what
/// its own machinery tripped over — and this one names the callee, which is the
/// half an author can act on.
#[test]
fn calling_a_global_that_only_carries_methods_refuses() {
  assert_refuses("", "zIndex: Math(1),", "Math is not a function.");
}

// ──────────────────────────────────────────────
// The edges
// ──────────────────────────────────────────────

/// The answers that are not numbers anyone wants, each of which the reference
/// compiler writes into the rule.
///
/// A fold that refused these would fail a build that compiles there, and the
/// class name is a hash of the declaration text — so the text is a contract that
/// a *better* answer still breaks. Where an author is served instead is the CSS
/// layer, which sees the value knowing the property it belongs to.
#[test]
fn an_answer_with_no_useful_value_still_folds() {
  let cases: &[(&str, &str)] = &[
    ("zIndex: Math.max(),", ".xey004h{z-index:-Infinity}"),
    ("zIndex: Math.min(),", ".xbdygrb{z-index:Infinity}"),
    ("zIndex: Math.round(),", ".x1uhybf7{z-index:NaN}"),
    ("width: Math.abs({}),", ".x1c9rq88{width:NaNpx}"),
    ("zIndex: Math.pow(10, 400),", ".xbdygrb{z-index:Infinity}"),
    ("zIndex: Math.round(-0.4),", ".x1ja2u2z{z-index:0}"),
  ];

  for (body, rule) in cases {
    assert_folds("", body, rule);
  }
}

/// A key list is read at the edges a UTF-16 or an ordering mistake would surface
/// on: an integer-like key sorts before a named one whatever order it was
/// written in, a non-ASCII key survives the round trip, and a primitive receiver
/// has the own keys its wrapper has rather than none.
#[test]
fn a_key_list_is_read_at_its_edges() {
  assert_folds(
    "",
    "content: `x${Object.keys({})}y`,",
    ".xg1gb30{content:\"xy\"}",
  );

  assert_folds(
    "",
    "content: Object.keys({2: 'a', 1: 'b', b: 1}).join(','),",
    ".x19vh29w{content:\"1,2,b\"}",
  );

  assert_folds(
    "",
    "content: Object.keys({ 'é中': 1 }).join(','),",
    ".x1j0h2xt{content:\"é中\"}",
  );

  assert_folds(
    "",
    "content: Object.keys('ab').join(','),",
    ".xwj9g4y{content:\"0,1\"}",
  );

  assert_folds(
    "",
    "content: `x${Object.keys(5)}y`,",
    ".xg1gb30{content:\"xy\"}",
  );
}

/// A fold big enough to be worth bounding, at a size the bridge carries: five
/// hundred entries built by one static and counted by another.
///
/// Under the ceiling the fold carries back, so it folds — and it folds to the
/// same number the reference compiler answers, which is what says the whole list
/// crossed rather than a truncated one.
#[test]
fn a_large_key_list_folds_whole() {
  assert_folds(
    "",
    "zIndex: Object.keys(Object.fromEntries(Array.from({length: 500}, (_, i) => ['k' + i, i]))).length,",
    ".x6cuj84{z-index:500}",
  );
}

/// Past the ceiling, the refusal names the limit rather than building an AST of a
/// hundred thousand nodes — and it names it from in front of the engine, because
/// `{ length: 20000 }` says how long the array will be before anything runs.
#[test]
fn a_key_list_past_the_carrying_ceiling_refuses() {
  assert_refuses(
    "",
    "zIndex: Object.keys(Object.fromEntries(Array.from({length: 20000}, (_, i) => ['k' + i, i]))).length,",
    "It declares a length of 20000 elements, and at most 10000 are supported.",
  );
}
