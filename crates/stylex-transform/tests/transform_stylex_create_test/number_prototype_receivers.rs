//! Giving a number a name makes `Number.prototype` reachable, and the one
//! refusal that shape could have cost is kept.
//!
//! The fold guard admits a name holding a number because the `Math` statics
//! need one — `Math.round(BASE / 4)` has a binding at every operand — and it
//! follows from that, rather than from a list of number methods, that
//! `const n = 255; n.toString(16)` folds. So the surface below is the
//! language's, and it is measured method by method because a surface is only
//! claimed to have no holes by naming every one of them.
//!
//! What could have been lost by accident is the refusal in the other direction.
//! The reference compiler applies a number method without a receiver, so
//! `(1.5).toFixed(1)` throws there; folding it here would emit a declaration for
//! a module that build rejects. That rule reads how the receiver was *written*,
//! which is why a number a fold produced and a negated literal — neither of
//! which is a literal — are receivers, and why a written one stays refused in
//! every position a call can be written in.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options, so each case asserts
//! agreement with the reference compiler rather than agreement with this
//! compiler's own previous answer.

use crate::utils::{
  prelude::*,
  transform::{assert_folds, assert_refuses},
};

/// The bindings most cases here share: one number, written once.
const N: &str = "const n = 255;";

// ──────────────────────────────────────────────
// The prototype surface, on a name
// ──────────────────────────────────────────────

/// Every `Number.prototype` method the reference compiler folds on a named
/// receiver, folded to the reference compiler's own class name and rule text.
///
/// The surface is five methods wide, and every one of them was refused before
/// this work — there was no number method table at all, so a number receiver
/// failed in every position. `toLocaleString` is the sixth and is refused; its
/// case is with the other boundaries below.
///
/// Each method is asserted with an argument and, where the language allows one,
/// without, because the default is a different answer rather than the same one:
/// `toPrecision()` is `toString()` and `toPrecision(4)` is not.
#[test]
fn every_number_method_folds_on_a_named_receiver() {
  let cases: &[(&str, &str)] = &[
    ("content: n.toString(),", r#".x14joq6f{content:"255"}"#),
    ("content: n.toString(2),", r#".xc3su1a{content:"11111111"}"#),
    ("content: n.toString(16),", r#".x1lovsyd{content:"ff"}"#),
    ("content: n.toString(36),", r#".xxp7y3q{content:"73"}"#),
    ("content: n.toFixed(0),", r#".x14joq6f{content:"255"}"#),
    ("content: n.toFixed(2),", r#".xvxrx5i{content:"255.00"}"#),
    (
      "content: n.toExponential(),",
      r#".xv9mj3t{content:"2.55e+2"}"#,
    ),
    (
      "content: n.toExponential(2),",
      r#".xv9mj3t{content:"2.55e+2"}"#,
    ),
    ("content: n.toPrecision(),", r#".x14joq6f{content:"255"}"#),
    (
      "content: n.toPrecision(4),",
      r#".x1irgx1l{content:"255.0"}"#,
    ),
    ("zIndex: n.valueOf(),", ".xfwdq03{z-index:255}"),
  ];

  for (body, rule) in cases {
    assert_folds(N, body, rule);
  }
}

/// The `Object.prototype` methods a number inherits, which are part of the
/// surface a receiver reaches even though no number method table would have
/// listed them.
///
/// All three answer `false` on a primitive, which is the language's answer and
/// the reference compiler's.
#[test]
fn a_named_number_reaches_the_object_prototype_it_inherits() {
  let bodies = [
    "content: n.hasOwnProperty('x').toString(),",
    "content: n.isPrototypeOf({}).toString(),",
    "content: n.propertyIsEnumerable('x').toString(),",
  ];

  for body in bodies {
    assert_folds(N, body, r#".x9g66vw{content:"false"}"#);
  }
}

/// The argument each method reads, in every form a stylesheet writes one: a
/// name, arithmetic, a string method's answer, another fold's element, and the
/// coercions the language performs on an argument that is not a number at all.
///
/// A number is not special as an argument — the guard walks a call's arguments
/// with the same walk it walks its receiver — but the radix and digit arguments
/// are where a number method's answer changes most, so this is where a walk that
/// stopped reading one would show.
#[test]
fn a_number_method_reads_its_argument_in_every_form() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const n = 255; const r = 16;",
      "content: n.toString(r),",
      r#".x1lovsyd{content:"ff"}"#,
    ),
    (
      N,
      "content: n.toFixed(1 + 1),",
      r#".xvxrx5i{content:"255.00"}"#,
    ),
    (
      N,
      "content: n.toFixed('abc'.length),",
      r#".x14c6rvr{content:"255.000"}"#,
    ),
    (
      N,
      "content: n.toString([16].at(0)),",
      r#".x1lovsyd{content:"ff"}"#,
    ),
    // A fractional radix truncates, a string digit count is coerced, a boolean
    // is one digit and `null` is none. The language answers all four, and
    // folding them is agreement rather than leniency.
    (
      N,
      "content: n.toString(16.9),",
      r#".x1lovsyd{content:"ff"}"#,
    ),
    (
      N,
      "content: n.toFixed('2'),",
      r#".xvxrx5i{content:"255.00"}"#,
    ),
    (
      N,
      "content: n.toFixed(true),",
      r#".x1irgx1l{content:"255.0"}"#,
    ),
    (
      N,
      "content: n.toFixed(null),",
      r#".x14joq6f{content:"255"}"#,
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// Receivers that are not literals
// ──────────────────────────────────────────────

/// The receivers that hold a number without being a number written into the
/// source, which is the distinction the refusal below rests on.
///
/// A negated literal is a unary expression rather than a literal, and so is a
/// literal with a unary plus in front of it — the reference compiler folds both,
/// because what it cannot call a method on is the literal itself. Everything
/// else here is a number some earlier fold produced.
#[test]
fn a_receiver_holding_a_number_without_being_one_folds() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "",
      "content: (-5).toFixed(1),",
      r#".x1wrq4vz{content:"-5.0"}"#,
    ),
    (
      "",
      "content: (+5).toFixed(1),",
      r#".xqj1kdb{content:"5.0"}"#,
    ),
    (
      "",
      "content: (- -5).toFixed(1),",
      r#".xqj1kdb{content:"5.0"}"#,
    ),
    (
      "",
      "content: (-1e-7).toFixed(2),",
      r#".xoflgb4{content:"-0.00"}"#,
    ),
    (
      "",
      "content: [1, 2].indexOf(2).toFixed(1),",
      r#".x126jtd5{content:"1.0"}"#,
    ),
    (
      "",
      "content: 'abc'.length.toFixed(1),",
      r#".xvsn4lw{content:"3.0"}"#,
    ),
    (
      "",
      "content: Math.abs(-3).toString(2),",
      r#".x599ugr{content:"11"}"#,
    ),
    (
      N,
      "content: (n / 2).toFixed(2),",
      r#".x151x72u{content:"127.50"}"#,
    ),
    (
      N,
      "content: (((((n + 1) * 2) - 3) / 4) % 5).toFixed(3),",
      r#".xk7q5mp{content:"2.250"}"#,
    ),
    (
      "const a = [1, 2, 3];",
      "content: a.reduce((t, x) => t + x, 0).toFixed(2),",
      r#".xvn4y84{content:"6.00"}"#,
    ),
    // A number reached through a name, an element, a property, a nested
    // property, and a fold over an object's values. None of them is written as
    // a literal in the position the call is made on.
    (
      "const a = 255; const b = a;",
      "content: b.toString(16),",
      r#".x1lovsyd{content:"ff"}"#,
    ),
    (
      "const a = [1.5];",
      "content: a[0].toFixed(1),",
      r#".x21w37q{content:"1.5"}"#,
    ),
    (
      "const o = { n: 1.5 };",
      "content: o.n.toFixed(1),",
      r#".x21w37q{content:"1.5"}"#,
    ),
    (
      "const o = { a: { b: 1.5 } };",
      "content: o.a.b.toFixed(2),",
      r#".xrv7f5n{content:"1.50"}"#,
    ),
    (
      "const o = { a: 1.5 };",
      "content: Object.values(o).at(0).toFixed(2),",
      r#".xrv7f5n{content:"1.50"}"#,
    ),
    // A `let` nothing writes to is as constant as a `const`, and a name a
    // callback binds is a number the engine handed it.
    (
      "let n = 255;",
      "content: n.toString(16),",
      r#".x1lovsyd{content:"ff"}"#,
    ),
    (
      "",
      "content: [1, 2].map(x => x.toFixed(1)).join(','),",
      r#".x1m0d8z9{content:"1.0,2.0"}"#,
    ),
    // The two globals that name a number. Neither is a literal, and the
    // reference compiler folds a method call on both.
    (
      "",
      "content: NaN.toFixed(1),",
      r#".x115s0ju{content:"NaN"}"#,
    ),
    (
      "",
      "content: Infinity.toFixed(1),",
      r#".x1map9xf{content:"Infinity"}"#,
    ),
    // A name may also take the global's name over, and then it is the module's
    // own number.
    (
      "const Number = 1.5;",
      "content: Number.toFixed(1),",
      r#".x21w37q{content:"1.5"}"#,
    ),
    // A name spelled like one of the methods is still just a name.
    (
      "const toFixed = 1.5;",
      "content: toFixed.toFixed(1),",
      r#".x21w37q{content:"1.5"}"#,
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// The refusal that has to survive
// ──────────────────────────────────────────────

/// A number written into the source as a literal is refused as a receiver,
/// wherever the call is written.
///
/// The reference compiler applies the method without a receiver and throws, so
/// every one of these fails the build there. Folding any of them would emit a
/// declaration for a module the reference implementation rejects, and the corpus
/// entry `modules-06-numeric-literal-receiver` is where that agreement is
/// recorded across the two compilers.
///
/// The positions are the ones a rule read from syntax could be lost in: alone, a
/// float, an exponent, a hexadecimal, the receiver of a longer chain, an
/// argument to another fold, inside a callback the engine runs, inside a
/// fallback array, and under two conditions.
#[test]
fn a_written_number_receiver_is_refused_wherever_it_is_written() {
  let cases: &[(&str, &str, &str)] = &[
    ("", "content: (5).toFixed(1),", "toFixed"),
    ("", "content: (1.5).toFixed(1),", "toFixed"),
    ("", "content: (1e3).toFixed(1),", "toFixed"),
    ("", "content: (0xff).toString(16),", "toString"),
    ("", "content: (1_000).toString(16),", "toString"),
    ("", "content: (5).toFixed(1).trim(),", "toFixed"),
    (N, "content: n.toString((2).valueOf()),", "valueOf"),
    (
      "",
      "content: [1].map(x => (5).toFixed(1)).join(''),",
      "toFixed",
    ),
    ("", "content: [(5).toFixed(1), 'a'],", "toFixed"),
    (
      "",
      "':hover': { ':focus': { content: (5).toFixed(1) } },",
      "toFixed",
    ),
  ];

  for (decls, body, method) in cases {
    assert_refuses(
      decls,
      body,
      &format!("Cannot call '{}' on a number literal.", method),
    );
  }
}

/// A call the fold declined on a number receiver says which rule declined it,
/// rather than naming the receiver's node kind.
///
/// Both compilers refuse `n.toFixed(window.x)`, and the reference compiler
/// refuses it for the argument. This compiler reported `Unsupported expression:
/// NumericLiteral`, which tells an author only that they wrote a number — the
/// dispatch behind the fold recognised a string and an array receiver as ones
/// whose prototypes fold whole, and a number and a boolean had never been added
/// to that list.
///
/// A boolean is here for the same reason it is a carryable receiver at all:
/// there is no sentence that would admit a number and refuse it, and leaving it
/// out would be a table of one. It also carried an early bail of its own, one
/// that predated the fold and answered `Unsupported expression: BooleanLiteral`
/// for a receiver written out — so a written boolean read a worse sentence than
/// a named one, which is position deciding the answer again.
///
/// What changed there is the sentence and not the verdict, and the two cases
/// that fold below are how that stays checkable: a boolean receiver folded
/// before this and folds now, because the fold accepts one and the bail was only
/// ever reached once the fold had declined.
#[test]
fn a_declined_number_call_names_the_rule_that_declined_it() {
  assert_folds(
    "",
    "content: true.toString(),",
    r#".x1ez55b5{content:"true"}"#,
  );
  assert_folds(
    "",
    "content: true.toString().toUpperCase(),",
    r#".x1nx6bbg{content:"TRUE"}"#,
  );

  let cases: &[(&str, &str, &str)] = &[
    (N, "content: n.toFixed(undefined),", "toFixed"),
    (N, "content: n.toExponential(undefined),", "toExponential"),
    (
      "const b = true;",
      "content: b.toString(undefined),",
      "toString",
    ),
    ("", "content: true.toString(undefined),", "toString"),
  ];

  for (decls, body, method) in cases {
    assert_refuses(
      decls,
      body,
      &format!("Cannot fold '{}' at compile time.", method),
    );
    assert_refuses(
      decls,
      body,
      "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
    );
  }

  // An argument naming something the module never declares is the reference
  // compiler's own sentence, reached now that the receiver no longer answers
  // first.
  assert_refuses(
    N,
    "content: n.toFixed(window.x),",
    "Referenced constant is not defined.",
  );
}

// ──────────────────────────────────────────────
// The arguments the language itself refuses
// ──────────────────────────────────────────────

/// A radix or digit count outside the range the language allows, which throws
/// in both compilers.
///
/// These are the engine's own throws surfaced as refusals, so what is asserted
/// is the method name and that the fold declined — the two engines word a
/// `RangeError` differently and neither wording is a contract.
#[test]
fn an_argument_the_language_refuses_refuses_the_fold() {
  let cases: &[(&str, &str)] = &[
    ("content: n.toString(1),", "toString"),
    ("content: n.toString(37),", "toString"),
    ("content: n.toString(-16),", "toString"),
    ("content: n.toFixed(-1),", "toFixed"),
    ("content: n.toFixed(101),", "toFixed"),
    ("content: n.toPrecision(0),", "toPrecision"),
    ("content: n.toPrecision(101),", "toPrecision"),
  ];

  for (body, method) in cases {
    assert_refuses(
      N,
      body,
      &format!("Cannot fold '{}' at compile time.", method),
    );
  }
}

// ──────────────────────────────────────────────
// The numeric edges
// ──────────────────────────────────────────────

/// The values a number method is asked of at the edges of what an `f64` holds,
/// spelled the way the reference compiler spells them.
///
/// This is where the two number-to-string paths could part company: a value
/// whose shortest spelling is exponential, a negative zero, the largest and
/// smallest magnitudes, and a sum whose binary error only appears past the
/// fifteenth digit.
#[test]
fn the_numeric_edges_fold_to_the_same_text() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const n = NaN;",
      "content: n.toFixed(1),",
      r#".x115s0ju{content:"NaN"}"#,
    ),
    (
      "const n = NaN;",
      "content: n.toString(16),",
      r#".x115s0ju{content:"NaN"}"#,
    ),
    (
      "const n = Infinity;",
      "content: n.toFixed(2),",
      r#".x1map9xf{content:"Infinity"}"#,
    ),
    (
      "const n = -Infinity;",
      "content: n.toPrecision(3),",
      r#".x15azjam{content:"-Infinity"}"#,
    ),
    (
      "const n = -0;",
      "content: n.toFixed(1),",
      r#".x5ll9hi{content:"0.0"}"#,
    ),
    (
      "const n = 0;",
      "content: n.toExponential(3),",
      r#".x1e7vrk0{content:"0.000e+0"}"#,
    ),
    (
      "const n = 0.5;",
      "content: n.toString(2),",
      r#".x9sl052{content:"0.1"}"#,
    ),
    (
      "const n = 1e21;",
      "content: n.toString(),",
      r#".x1fddv7c{content:"1e+21"}"#,
    ),
    (
      "const n = 1e21;",
      "content: n.toFixed(2),",
      r#".x1fddv7c{content:"1e+21"}"#,
    ),
    (
      "const n = 1e-7;",
      "content: n.toString(),",
      r#".x14z8vq8{content:"1e-7"}"#,
    ),
    (
      "const n = 0.000001234;",
      "content: n.toPrecision(2),",
      r#".x1ggo1ay{content:"0.0000012"}"#,
    ),
    (
      "const n = -1.005;",
      "content: n.toFixed(2),",
      r#".xqx91n3{content:"-1.00"}"#,
    ),
    (
      "const n = 9007199254740991;",
      "content: n.toString(36),",
      r#".xd490xy{content:"2gosa7pa2gv"}"#,
    ),
    (
      "const n = 1.7976931348623157e308;",
      "content: n.toString(),",
      r#".x149nvq6{content:"1.7976931348623157e+308"}"#,
    ),
    (
      "const n = 5e-324;",
      "content: n.toExponential(2),",
      r#".x1a2obyy{content:"4.94e-324"}"#,
    ),
    (
      "const n = 1e308;",
      "content: (n * 10).toString(),",
      r#".x1map9xf{content:"Infinity"}"#,
    ),
    (
      "const n = 0.1 + 0.2;",
      "content: n.toFixed(20),",
      r#".x6kvs7{content:"0.30000000000000004441"}"#,
    ),
    (
      "const n = 1234567890123456789012345678901234567890;",
      "content: n.toString(),",
      r#".xenc2fw{content:"1.2345678901234568e+39"}"#,
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The longest text each method will build, which is the most a number receiver
/// can cost.
///
/// A number is a fixed sixty-four bits, so the only thing that grows here is the
/// digit count an argument asks for — and the language caps every one of those
/// at a hundred. So a number receiver has no amplifying method at all, and these
/// are its ceilings rather than a bound this compiler chose.
#[test]
fn the_longest_text_a_number_method_builds_folds() {
  assert_folds(
    N,
    "content: n.toFixed(100),",
    &format!(r#".x1sxa9nx{{content:"255.{}"}}"#, "0".repeat(100)),
  );

  assert_folds(
    "const n = 1.5;",
    "content: n.toPrecision(100),",
    &format!(r#".xo6ne2g{{content:"1.5{}"}}"#, "0".repeat(98)),
  );
}

// ──────────────────────────────────────────────
// Chains, callbacks and scale
// ──────────────────────────────────────────────

/// A number crossing into and out of the other two prototypes, repeatedly.
///
/// One chain is what two separate method tables could never agree on: a number
/// becomes a string, the string becomes an array, the array becomes a string
/// again, and a number method is called on the result of all of it.
#[test]
fn a_number_folds_at_every_link_of_a_long_chain() {
  assert_folds(
    N,
    "content: n.toString(2).split('').reverse().join('').slice(0, 6)\
     .replace('1', '0').toUpperCase().concat(n.toFixed(1)),",
    r#".x1vszy3e{content:"011111255.0"}"#,
  );

  assert_folds(
    "const n = 1023;",
    "zIndex: n.toString(2).split('').filter(d => d === '1').length.toFixed(0)\
     .concat('0').length,",
    ".xzkaem6{z-index:3}",
  );

  assert_folds(
    N,
    "content: n.toString(16).toUpperCase(),",
    r#".x1ghniby{content:"FF"}"#,
  );

  assert_folds(
    N,
    "content: n.toFixed(2).replace('.', '-').split('-').join('_')\
     .toUpperCase().toLowerCase(),",
    r#".xtidg7b{content:"255_00"}"#,
  );
}

/// A number method called once per element of a list, and once per argument of a
/// sweep, which is how a stylesheet reaches the whole surface at once.
#[test]
fn a_number_method_folds_inside_a_callback() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const n = 1234567;",
      "content: [2, 8, 10, 16, 36].map(r => n.toString(r)).join('|'),",
      r#".x18ze96b{content:"100101101011010000111|4553207|1234567|12d687|qglj"}"#,
    ),
    (
      "const n = 1.23456789;",
      "content: [0, 1, 2, 3, 4, 5].map(d => n.toFixed(d)).join('|'),",
      r#".x73nmk4{content:"1|1.2|1.23|1.235|1.2346|1.23457"}"#,
    ),
    (
      "const n = 1234.5678;",
      "content: [1, 2, 4, 8, 16].map(p => n.toPrecision(p)).join('|'),",
      r#".x4ozbo5{content:"1e+3|1.2e+3|1235|1234.5678|1234.567800000000"}"#,
    ),
    (
      "const n = 0.00012345;",
      "content: [0, 1, 3, 6].map(d => n.toExponential(d)).join('|'),",
      r#".x1vswg2u{content:"1e-4|1.2e-4|1.234e-4|1.234500e-4"}"#,
    ),
    (
      "const a = [1, 2];",
      "content: a.map(x => x.toFixed(1)).join(','),",
      r#".x1m0d8z9{content:"1.0,2.0"}"#,
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// Two hundred numbers, each converted at base thirty-six and joined.
///
/// Well inside every ceiling the fold carries — the result is a few hundred
/// characters, not a million — and there to say that a number receiver costs
/// nothing per element beyond the conversion itself. The declaration is the
/// reference compiler's own, written out rather than recomputed here: a test
/// that built the expected digits from its own base-36 conversion would be
/// asserting agreement with itself.
#[test]
fn two_hundred_named_numbers_fold_one_by_one() {
  let elements = (1..=200)
    .map(|value| value.to_string())
    .collect::<Vec<_>>()
    .join(", ");

  let digits = "123456789abcdefghijklmnopqrstuvwxyz101112131415161718191a1b1c1d1e1f1g1h1\
     i1j1k1l1m1n1o1p1q1r1s1t1u1v1w1x1y1z202122232425262728292a2b2c2d2e2f2g2h2\
     i2j2k2l2m2n2o2p2q2r2s2t2u2v2w2x2y2z303132333435363738393a3b3c3d3e3f3g3h3\
     i3j3k3l3m3n3o3p3q3r3s3t3u3v3w3x3y3z404142434445464748494a4b4c4d4e4f4g4h4\
     i4j4k4l4m4n4o4p4q4r4s4t4u4v4w4x4y4z505152535455565758595a5b5c5d5e5f5g5h5\
     i5j5k";

  assert_folds(
    &format!("const a = [{}];", elements),
    "content: a.map(x => x.toString(36)).join(''),",
    &format!(r#".x1rissi5{{content:"{}"}}"#, digits),
  );
}

/// A named number at the depth a stylesheet actually nests to, and beside the
/// other values a declaration holds.
#[test]
fn a_named_number_folds_wherever_the_declaration_is() {
  assert_folds(
    N,
    "'@media (min-width: 1px)': { ':hover': { '::before': { content: n.toString(16) } } },",
    r#"@media (min-width: 1px){.x642wml.x642wml:hover::before{content:"ff"}}"#,
  );

  assert_folds(
    N,
    "content: [n.toString(16), 'a'],",
    r#".x1t9j397{content:"ff";content:"a"}"#,
  );

  assert_folds(
    N,
    "content: ({ 255: 'x' })[n.toString()],",
    r#".x1qj0nkt{content:"x"}"#,
  );

  assert_folds(
    N,
    "content: ({ [n.toString(16)]: 'x' }).ff,",
    r#".x1qj0nkt{content:"x"}"#,
  );
}

// ──────────────────────────────────────────────
// The boundaries a number receiver does not cross
// ──────────────────────────────────────────────

/// `toLocaleString` on a number is the one method of the surface that is
/// refused, and it is refused for the receiver it cannot be separated from.
///
/// On a number it formats against locale data the engine does not carry, so
/// folding it would write a wrong declaration rather than none — and which
/// receiver a chain will produce is not knowable before evaluating it, so one
/// name cannot be both admitted and refused. The reference compiler folds it
/// against the host's own locale, which is why the corpus records this as a
/// divergence held deliberately.
#[test]
fn to_locale_string_is_refused_on_a_number() {
  assert_refuses(
    N,
    "content: n.toLocaleString(),",
    "Cannot fold 'toLocaleString' at compile time.",
  );

  assert_refuses(
    N,
    "content: n.toLocaleString('de-DE'),",
    "Its answer depends on locale data the compiler does not carry.",
  );
}

/// The reads that lead off a number and onto the language's function graph.
///
/// `n.constructor` is `Number`, whose own `constructor` compiles a string into a
/// function, and `call`, `apply` and `bind` are what turn an unapplied method
/// back into a call. The reference compiler folds all of these; refusing them is
/// the boundary that keeps arbitrary code out of the compiler, and it holds for
/// a number receiver exactly as it does for a string one.
#[test]
fn a_read_that_escapes_a_number_is_refused() {
  let cases: &[(&str, &str)] = &[
    ("content: n.constructor('5'),", "constructor"),
    ("content: n.toString.call(n),", "call"),
    ("content: n.toString.apply(n),", "apply"),
  ];

  for (body, property) in cases {
    assert_refuses(
      N,
      body,
      &format!("Cannot fold a read of '{}' at compile time.", property),
    );
  }
}

/// The receivers and shapes a number-looking call still refuses, each with the
/// rule that owns it rather than this file.
///
/// A `BigInt` is not a number this evaluator carries, a reassigned binding is
/// not a constant, and a call whose receiver is itself an amplifying call has no
/// bound the source states. All three are refused by the reference compiler too,
/// except the last, whose divergence is recorded in the corpus.
#[test]
fn the_shapes_around_a_number_receiver_keep_their_own_rules() {
  assert_refuses("", "content: (5n).toString(),", "BigIntLiteral");

  assert_refuses(
    "let n = 255; n = 16;",
    "content: n.toString(16),",
    "Referenced value is not a constant.",
  );

  assert_refuses(
    N,
    "content: n.toString().repeat(3),",
    "Cannot bound the string 'repeat' would build.",
  );
}
