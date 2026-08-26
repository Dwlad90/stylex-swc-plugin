//! Giving a string a name stops changing whether the call on it compiles.
//!
//! The guard in front of the fold used to ask whether an expression was
//! *written out*. It now asks whether every leaf *resolves to a value the fold
//! can carry*, so a name resolving to a string is a receiver like any other:
//! the expression is printed as an arrow taking the name as a parameter and the
//! resolved value is passed to it as an argument.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options, so each case asserts
//! agreement with the reference compiler rather than agreement with this
//! compiler's own previous answer. The pairing is what makes that assertion
//! meaningful: where a case has a written-out twin, both spellings must reach
//! the same class, because a class name is a hash of the declaration and two
//! spellings of one value have only one declaration between them.

use crate::utils::{
  prelude::*,
  transform::{assert_folds, assert_refuses, base_style_module as module, fold_module as fold},
};

// ──────────────────────────────────────────────
// The prototype surface, on a name
// ──────────────────────────────────────────────

/// Every non-locale method and property of `String.prototype` the reference
/// compiler folds on a named receiver, folded to the reference compiler's own
/// class name and rule text.
///
/// Written as one case per method rather than one assertion per behaviour,
/// because the claim under test is exactly that the surface has no holes: a
/// method missing from this list is the bug the table this work deletes kept
/// producing.
#[test]
fn every_string_method_folds_on_a_named_receiver() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const s = 'abc';",
      "content: s.at(1),",
      ".xa8xio6{content:\"b\"}",
    ),
    (
      "const s = 'abc';",
      "content: s.charAt(1),",
      ".xa8xio6{content:\"b\"}",
    ),
    (
      "const s = 'abc';",
      "zIndex: s.charCodeAt(0),",
      ".x1orgfts{z-index:97}",
    ),
    (
      "const s = 'abc';",
      "zIndex: s.codePointAt(0),",
      ".x1orgfts{z-index:97}",
    ),
    (
      "const s = '4';",
      "content: s.concat('px'),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = 'abc';",
      "content: s.endsWith('c') ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const s = 'abc';",
      "content: s.includes('b') ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const s = 'abc';",
      "zIndex: s.indexOf('b'),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const s = 'abcb';",
      "zIndex: s.lastIndexOf('b'),",
      ".xzkaem6{z-index:3}",
    ),
    (
      "const s = 'ﬁ';",
      "content: s.normalize('NFKC'),",
      ".xxs960y{content:\"fi\"}",
    ),
    (
      "const s = 'x';",
      "content: s.padEnd(4, '-'),",
      ".x2g552z{content:\"x---\"}",
    ),
    (
      "const s = '7';",
      "content: s.padStart(3, '0'),",
      ".x1ibju3u{content:\"007\"}",
    ),
    (
      "const s = 'ab';",
      "content: s.repeat(2),",
      ".xvxxpsj{content:\"abab\"}",
    ),
    (
      "const s = 'a-b';",
      "content: s.replace('-', '_'),",
      ".xezx3ef{content:\"a_b\"}",
    ),
    (
      "const s = 'a-b-c';",
      "content: s.replaceAll('-', '_'),",
      ".x1i9diqq{content:\"a_b_c\"}",
    ),
    (
      "const s = 'abc';",
      "zIndex: s.search('b'),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const s = '4pxx';",
      "content: s.slice(0, 3),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = 'a b';",
      "fontFamily: s.split(' '),",
      ".x1fw431j{font-family:a;font-family:b}",
    ),
    (
      "const s = 'abc';",
      "content: s.startsWith('a') ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const s = 'x4px';",
      "content: s.substring(1),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = 'x4px';",
      "content: s.substr(1),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = 'AbC';",
      "content: s.toLowerCase(),",
      ".x6ojgef{content:\"abc\"}",
    ),
    (
      "const s = 'AbC';",
      "content: s.toUpperCase(),",
      ".xj5ouxf{content:\"ABC\"}",
    ),
    (
      "const s = 'abc';",
      "content: s.toString(),",
      ".x6ojgef{content:\"abc\"}",
    ),
    (
      "const s = '  4px  ';",
      "content: s.trim(),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = '  4px';",
      "content: s.trimStart(),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = '4px  ';",
      "content: s.trimEnd(),",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "const s = 'abc';",
      "content: s.valueOf(),",
      ".x6ojgef{content:\"abc\"}",
    ),
    (
      "const s = 'abc';",
      "zIndex: s.length,",
      ".xzkaem6{z-index:3}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// Where the name sits
// ──────────────────────────────────────────────

/// A name is admitted in every position the walk reaches, not only as the
/// receiver — which is the property that keeps one guard walk honest: a shape
/// accepted as a receiver cannot be refused as an argument.
#[test]
fn a_name_folds_in_every_position_the_walk_reaches() {
  // As an argument, on a receiver written out.
  assert_folds(
    "const sep = '_';",
    "content: 'a-b'.replace('-', sep),",
    ".xezx3ef{content:\"a_b\"}",
  );

  // As both, and as a middle link of a chain.
  assert_folds(
    "const s = ' a-b ';",
    "content: s.trim().replace('-', '_'),",
    ".xezx3ef{content:\"a_b\"}",
  );

  // Inside a template literal, whose holes are values in their own right.
  assert_folds(
    "const s = '4';",
    "content: `${s}px`.concat('!'),",
    ".xtziort{content:\"4px!\"}",
  );

  // Free inside a callback the engine runs, alongside the callback's own
  // parameter.
  assert_folds(
    "const sep = '-';",
    "content: ['a','b'].map(x => x + sep).join(''),",
    ".x1a3njs5{content:\"a-b-\"}",
  );
}

/// Two names, and one name read twice.
///
/// The second is the case a printed arrow can get wrong: a parameter list that
/// repeated the name would be a syntax error, so the transport carries one
/// parameter per name however often the expression reads it.
#[test]
fn a_name_read_more_than_once_is_carried_once() {
  assert_folds(
    "const a = 'a'; const b = 'b';",
    "content: a.concat(b),",
    ".xarbti{content:\"ab\"}",
  );

  assert_folds(
    "const s = 'a';",
    "content: s.concat(s),",
    ".x1rifm2z{content:\"aa\"}",
  );
}

/// A callback parameter shadowing a resolved name shadows it in the printed
/// arrow too, because the parameter is printed with the author's own spelling
/// and the language's scoping does the rest.
///
/// Both readings appear in one declaration, so a transport that leaked the outer
/// value into the callback — or the callback's value out of it — would fold
/// `"aZ"` or `"abb"` rather than the measured `"abZ"`.
#[test]
fn a_callback_parameter_shadowing_a_resolved_name_still_shadows_it() {
  assert_folds(
    "const x = 'Z';",
    "content: ['a'].map(x => x + 'b').join('') + x,",
    ".x14eshuq{content:\"abZ\"}",
  );
}

/// A string a theme reference already resolved to is a usable receiver. The
/// reference itself never crosses: what the fold reads is the `var(--…)` string
/// the resolution produced, and resolving it is what mutates compiler state, so
/// it happens before the bridge rather than across it.
///
/// The assertion is the *upper-cased* reference, because that is the only thing
/// here a fold can produce. An unfolded `vars.primary` reaches the declaration as
/// `var(--…)` too, so asserting that would pass whether or not the string crossed
/// — which is the whole claim.
#[test]
fn a_string_a_theme_reference_resolved_to_is_a_usable_receiver() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import { vars } from 'vars.stylex.js';
      const name = vars.primary;
      export const styles = stylex.create({
        base: { content: name.toUpperCase() },
      });
    "#,
  );

  assert!(
    output.contains("content:\"VAR(--"),
    "expected the resolved theme string to reach the fold and be upper-cased, got:\n{}",
    output
  );
}

/// A bound value that would be a syntax hazard if it were written into the
/// printed source instead of passed to it.
///
/// This is the property the transport exists for. Substituting the value as a
/// literal would put a quote, a backslash, a backtick, a `${` or an unbalanced
/// parenthesis into a program the engine then has to parse — and the ones that
/// parse anyway would parse as something else. Passed as an argument, none of
/// them is ever text: the printed arrow says only `s`, whatever `s` holds.
///
/// Asserted as the class name rather than the rule text, which is the one place
/// in this file where those differ. A class name is a hash of the declaration,
/// so matching the reference compiler's is the exact claim — that both compilers
/// built the same declaration from the same value. The rule *text* reaches the
/// output inside a JavaScript string literal, so the emitter escapes these
/// characters a second time and the comparison would be about escaping rather
/// than about the value.
#[test]
fn a_bound_value_that_could_not_be_printed_safely_still_folds_exactly() {
  let cases: &[(&str, &str, &str)] = &[
    // A quote and a backslash — the two characters that end or escape a literal.
    (r#"const s = 'a"b';"#, "content: s.trim(),", "x1qt7ki0"),
    (r"const s = 'a\\b';", "content: s.trim(),", "xyspoqz"),
    // A newline, which no single-quoted literal may contain at all.
    (r"const s = 'a\nb';", "content: s.trim(),", "x38goau"),
    // A backtick and a template hole, which would only be syntax inside the
    // template literal the walk also admits.
    ("const s = 'a`b';", "content: s.trim(),", "x2xhglk"),
    ("const s = 'a${b}c';", "content: s.trim(),", "xtcvy6c"),
    // An unbalanced parenthesis, which would close the call it was printed into.
    ("const s = ')';", "content: s.concat('x'),", "x1ue5yi"),
    // A NUL and a non-ASCII scalar, neither of which survives a lossy step.
    ("const s = 'a\u{0}b';", "content: s.trim(),", "x1lad8bx"),
    (
      "const s = 'café';",
      "content: s.normalize('NFC'),",
      "x1kzf2xh",
    ),
    // The empty string, which is a value and not an absent one.
    ("const s = '';", "content: s.padStart(2, '-'),", "xj1ogob"),
  ];

  for (decls, body, class) in cases {
    let output = fold(&module(decls, body));

    assert!(
      output.contains(&format!(".{}{{", class)),
      "expected `{}` with `{}` to reach class `{}`, got:\n{}",
      body,
      decls,
      class,
      output
    );
  }
}

/// The shapes an author actually writes a name for: half a `var()` reference, a
/// custom property name, a vendor-prefixed keyword. Each is a string whose text
/// is CSS rather than prose, and none of them is parsed as CSS by the fold — it
/// is a JavaScript string until the declaration is built from it.
#[test]
fn a_name_holding_css_text_folds_to_the_declaration_the_text_spells() {
  assert_folds(
    "const s = 'var(--x';",
    "content: s.concat(')'),",
    ".x1mk9o7m{content:var(--x)}",
  );

  assert_folds(
    "const p = '--my-color';",
    "color: 'var('.concat(p).concat(')'),",
    ".xnp28ea{color:var(--my-color)}",
  );

  assert_folds(
    "const s = '-webkit-box';",
    "display: s.trim(),",
    ".x104kibb{display:-webkit-box}",
  );
}

/// Four names in one chain, so the transport carries a parameter list rather
/// than the one-name case every other test here exercises.
#[test]
fn a_chain_reading_several_names_carries_all_of_them() {
  assert_folds(
    "const a = '1'; const b = '2'; const c = '3'; const d = '4';",
    "content: a.concat(b).concat(c).concat(d),",
    ".xjbko8d{content:\"1234\"}",
  );
}

// ──────────────────────────────────────────────
// What a name does not make foldable
// ──────────────────────────────────────────────

/// A name that shadows one of the globals the older dispatch folds is the
/// module's own value, and is resolved like any other name.
///
/// Measured, the reference compiler folds both of these through the binding, so
/// treating the name as the global would refuse input it compiles. The receiver
/// question is asked of the binding table before anything is resolved, which is
/// the same question that dispatch's callee branch already asks.
#[test]
fn a_name_shadowing_a_global_is_resolved_as_the_binding() {
  assert_folds(
    "const String = 'AbC';",
    "content: String.toUpperCase(),",
    ".xj5ouxf{content:\"ABC\"}",
  );

  assert_folds(
    "const Number = '7';",
    "content: Number.padStart(3, '0'),",
    ".x1ibju3u{content:\"007\"}",
  );
}

/// Where the global is *not* shadowed it stays the dispatch below's call, so the
/// surfaces that have not moved to the engine yet keep folding. `Math.round(1)`
/// is `1` in the reference compiler even with a local `Math` that would answer
/// `2` — it reads the global there too — so the unshadowed case is the one this
/// pins, and the shadowed one is left to tickets 07 and 09 with that measurement
/// recorded here.
#[test]
fn an_unshadowed_global_still_folds_through_the_dispatch_below() {
  assert_folds("", "zIndex: Math.round(1.5),", ".xhtitgo{z-index:2}");
}

/// A locale-sensitive method is refused on a named receiver exactly as on a
/// written-out one. The reference compiler folds `s.toLocaleUpperCase('tr')` to
/// `İ`; the engine carries no locale data and would answer `I`, and a wrong
/// value in a stylesheet is worse than none.
#[test]
#[should_panic(expected = "base > content > Cannot fold 'toLocaleUpperCase' at compile time.")]
fn a_locale_sensitive_method_is_refused_on_a_name_too() {
  fold(&module(
    "const s = 'i';",
    "content: s.toLocaleUpperCase('tr'),",
  ));
}

/// A receiver that resolves to nothing is not this module's call at all, so the
/// dispatch below keeps answering for it, in the sentence it wrote before the
/// guard read anything.
///
/// The guard's failed read is a speculation and leaves no trace: a refusal
/// recorded against the name would have replaced this with the memo's `Could not
/// resolve the code being evaluated.` for every later reader.
#[test]
#[should_panic(expected = "base > content > Referenced constant is not defined.")]
fn a_receiver_that_resolves_to_nothing_keeps_the_dispatch_s_own_sentence() {
  fold(&module("", "content: missing.toUpperCase(),"));
}

/// Inside a dynamic style function the parameter has no compile-time value, so
/// the same call is left for the runtime rather than failing the build. Naming
/// a value is what makes a fold possible; a parameter is not a value yet.
///
/// Two assertions, because the custom-property declaration alone would be there
/// for any dynamic style at all: the call has to reach the *runtime expression*
/// as well, still spelled as the call the author wrote. A fold that wrongly
/// answered here would leave a literal in the rule and no `toUpperCase` in the
/// output.
#[test]
fn a_call_on_a_dynamic_parameter_is_still_left_to_the_runtime() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (label) => ({ content: label.toUpperCase() }),
      });
    "#,
  );

  assert!(
    output.contains("content:var(--"),
    "expected the declaration to defer to a custom property, got:\n{}",
    output
  );

  assert!(
    output.contains("label.toUpperCase()"),
    "expected the call itself to survive into the runtime value, got:\n{}",
    output
  );
}

/// A name bound to something the bridge does not carry is handed back rather
/// than refused, so the dispatch that owns those values keeps answering for
/// them. The primitives, arrays and plain objects all cross; a regular
/// expression is a value this evaluator holds no reading of, and neither
/// compiler folds a call on one.
#[test]
#[should_panic(expected = "base > content > Unsupported expression: RegExpLiteral")]
fn a_name_bound_to_a_value_the_bridge_cannot_carry_is_handed_back() {
  fold(&module("const re = /a/;", "content: re.test('a'),"));
}

/// A value longer than the fold will carry is refused on the way *in*, naming
/// the binding rather than the method: the size belongs to what the name holds,
/// and the same call on a shorter value folds.
///
/// The printed source stays the size of the expression whatever the value is —
/// that is the point of passing it as an argument — so this bound is the only
/// thing standing between a fold and copying a megabyte into the engine.
#[test]
#[should_panic(expected = "Cannot carry the value of 'big' into a fold.")]
fn a_resolved_value_past_the_size_bound_names_the_binding() {
  fold(&module(
    &format!("const big = '{}';", "x".repeat(1_000_001)),
    "content: big.trim(),",
  ));
}

/// An unpaired surrogate and an astral scalar, reached through a name.
///
/// This is the only path that exercises the inward bridge's ill-formed reading:
/// a JavaScript string literal may hold half a surrogate pair and no Rust `str`
/// can, so the value crosses as code units rather than as text. Without that
/// reading the value could not cross at all.
///
/// The *declaration* is what the reference compiler writes — the outward bridge
/// has to land in a `Lit::Str`, so it substitutes the replacement character, and
/// so does upstream's own output. The class name diverges, because upstream
/// hashes the surrogate it still holds where this compiler hashes what it
/// substituted. That divergence is the one `engine_fold_tests::a_fold_whose_
/// result_is_an_unpaired_surrogate_becomes_the_replacement_character` records
/// for a value written out; asserted here on the rule text alone so this case
/// pins the round trip rather than re-pinning the divergence.
#[test]
fn a_named_value_holding_half_a_surrogate_pair_crosses_and_comes_back() {
  for (decls, body) in [
    ("const s = '\\uD83D';", "content: s.concat(''),"),
    ("const s = '\\uD83D\\uDE00a';", "content: s.slice(1),"),
  ] {
    let output = fold(&module(decls, body));

    assert!(
      output.contains("content:\"\u{FFFD}"),
      "expected `{}` with `{}` to write the replacement character, got:\n{}",
      body,
      decls,
      output
    );
  }
}

/// Names do not raise the nesting ceiling. The bound is the engine parser's own
/// stack, which a name says nothing about — the printed arrow is as deep as the
/// expression whether its leaves were written out or resolved.
#[test]
#[should_panic(expected = "Expression is too deeply nested to evaluate at compile time.")]
fn a_chain_of_names_past_the_nesting_bound_still_refuses() {
  fold(&module(
    "const s = 'a';",
    &format!("content: s{},", ".concat('b')".repeat(400)),
  ));
}

/// The inputs both compilers reject, so a name cannot be used to smuggle a value
/// past a rule.
///
/// Each is rejected upstream too — reassignment as `Unsupported expression:
/// CallExpression`, and the read off `undefined` as a thrown `TypeError`. Only
/// the sentences differ, which is not a parity obligation; what matters is that
/// resolving names did not turn a rejected input into a folded one.
///
/// Each case names the sentence it must refuse *with*, not merely that it
/// refused. A bare "it panicked" would be satisfied by a refusal for any reason
/// at all, including a later wrong one — and the rule that fired is the whole of
/// what these cases are about.
#[test]
fn a_name_cannot_carry_a_value_past_a_rule() {
  let refusals = [
    // Reassigned, so the name has no single value. The binding rule answers,
    // which is the same rule a written-out receiver would have met.
    (
      "let s = 'a'; s = 'b';",
      "content: s.toUpperCase(),",
      "Referenced value is not a constant.",
    ),
    // Read off `undefined`, which has no properties in either compiler. The name
    // is one the engine holds, so it is printed rather than resolved and the
    // language's own throw is the sentence — upstream reports the same fault in
    // its own words.
    (
      "",
      "content: undefined.toUpperCase(),",
      "cannot convert 'null' or 'undefined' to object",
    ),
    // A name holding a length past the ceiling is refused exactly as a written
    // one is: the bound is read from the value, so naming it is not a way round
    // it. The small counterpart — a name holding a length under the ceiling,
    // which now folds and agrees with upstream — is in `amplification_ceilings`.
    (
      "const n = 200000000;",
      "content: 'x'.repeat(n),",
      "Cannot bound the string 'repeat' would build.",
    ),
  ];

  for (decls, body, sentence) in refusals {
    assert_refuses(decls, body, sentence);
  }
}

/// A template literal reaching the engine hands its holes to the engine's own
/// number-to-string, where every other position in this compiler uses the
/// hand-written coercions. The two have to agree, and this is where they are made
/// to say so.
///
/// The cases are the seam they could part company on: a number whose shortest
/// spelling is exponential, and one whose written form has a trailing zero. A
/// coercion that echoed the source text would write `1e21px` and `1.50px`; one
/// that re-serialises writes `1e+21px` and `1.5px`. Both compilers write the
/// second, measured.
///
/// A named hole reads the same coercion, and has to: a name holding a number is
/// a value the bridge carries now, so the hole resolves to it and the engine
/// prints it. If the two coercions had parted company, a hole written as `1e21`
/// and one written as a name holding it would spell the same declaration
/// differently — which is a class name, not a formatting preference.
#[test]
fn a_template_hole_is_coerced_the_way_the_reference_compiler_coerces_it() {
  assert_folds(
    "",
    "content: `${1e21}px`.trim(),",
    ".x1h4pnls{content:\"1e+21px\"}",
  );
  assert_folds(
    "",
    "content: `${0.0000001}px`.trim(),",
    ".x82korf{content:\"1e-7px\"}",
  );
  assert_folds(
    "",
    "content: `${1.50}px`.trim(),",
    ".xe54kcq{content:\"1.5px\"}",
  );

  assert_folds(
    "const n = 1e21;",
    "content: `${n}px`.trim(),",
    ".x1h4pnls{content:\"1e+21px\"}",
  );
}
