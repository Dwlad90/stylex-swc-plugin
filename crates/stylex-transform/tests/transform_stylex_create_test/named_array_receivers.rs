//! Giving an array a name stops changing whether the call on it compiles, and
//! the two dispatch arms that answered for an array become one.
//!
//! The fold guard asks whether every leaf of an expression resolves to a value
//! the bridge can carry, and an array is now one of those values — so the whole
//! of `Array.prototype` folds on a name exactly as it folds on a list written
//! out. Two arms used to answer for an array instead: one for the list the
//! evaluation produced and one for the array literal a fold produced, and they
//! carried different method names, which is why a mapped list could not be
//! joined.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options, so each case asserts
//! agreement with the reference compiler rather than agreement with this
//! compiler's own previous answer.

use crate::utils::{
  prelude::*,
  transform::{assert_folds, assert_refuses, base_style_module as module, fold_module as fold},
};

// ──────────────────────────────────────────────
// The prototype surface, on a name
// ──────────────────────────────────────────────

/// Every non-mutating method of `Array.prototype` the reference compiler folds
/// on a named receiver, folded to the reference compiler's own class name and
/// rule text.
///
/// One case per method rather than one per behaviour, because the claim under
/// test is exactly that the surface has no holes: eleven of these were refused
/// before this work purely because the receiver was a binding, and the method
/// nobody listed is the bug the deleted table kept producing.
#[test]
fn every_array_method_folds_on_a_named_receiver() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['b', 'a'];",
      "content: a.at(1),",
      ".x16319ns{content:\"a\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.concat(['c']).join(','),",
      ".xubg6hi{transition-property:b,a,c}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.every(x => x) ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.filter(x => x === 'a').join(','),",
      ".xrzjt9{transition-property:a}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.find(x => x === 'a'),",
      ".x16319ns{content:\"a\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "zIndex: a.findIndex(x => x === 'a'),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.findLast(x => x),",
      ".x16319ns{content:\"a\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "zIndex: a.findLastIndex(x => x),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: [a, ['c']].flat().join(','),",
      ".xubg6hi{transition-property:b,a,c}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.flatMap(x => [x, x]).join(','),",
      ".x1er8xet{transition-property:b,b,a,a}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.includes('a') ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "zIndex: a.indexOf('a'),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.join(','),",
      ".x1ep8ulr{transition-property:b,a}",
    ),
    (
      "const a = ['b', 'a'];",
      "zIndex: a.lastIndexOf('a'),",
      ".x1vjfegm{z-index:1}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.map(x => x + x).join(','),",
      ".x9dy171{transition-property:bb,aa}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.reduce((t, x) => t + x, ''),",
      ".xay2nfz{content:\"ba\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.reduceRight((t, x) => t + x, ''),",
      ".xarbti{content:\"ab\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.slice(1).join(','),",
      ".xrzjt9{transition-property:a}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.some(x => x === 'a') ? 'y' : 'n',",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.toReversed().join(','),",
      ".x1iq4t92{transition-property:a,b}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.toSorted().join(','),",
      ".x1iq4t92{transition-property:a,b}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.toSpliced(1, 1).join(','),",
      ".x13785oo{transition-property:b}",
    ),
    (
      "const a = ['b', 'a'];",
      "content: a.toString(),",
      ".x1yru101{content:\"b,a\"}",
    ),
    (
      "const a = ['b', 'a'];",
      "transitionProperty: a.with(0, 'c').join(','),",
      ".x6jvtrd{transition-property:c,a}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// Chains, and the arms that disagreed
// ──────────────────────────────────────────────

/// A chain folds at every link when a middle link is a binding — the shape the
/// ticket opens with, and the one a per-link name table cannot answer, because
/// the second link's receiver is a value the first link produced.
#[test]
fn a_chain_folds_at_every_link_when_a_middle_link_is_a_binding() {
  assert_folds(
    "const a = ['1px', 'solid']; const sep = ' ';",
    "transitionProperty: a.concat(['red']).join(sep),",
    ".xq06qqy{transition-property:1px solid red}",
  );

  // Array to string and back again, so the chain crosses both prototypes twice.
  assert_folds(
    "const a = ['b', 'a'];",
    "content: a.join(',').split(',').join('-'),",
    ".x1y9cpk8{content:\"b-a\"}",
  );
}

/// A mapped list can be joined. This is precisely what the two arms disagreed
/// about: one accepted `join` for the list the evaluation produced, the other
/// refused it for the array literal a fold produced, so the chain died at its
/// second link.
#[test]
fn a_mapped_list_can_be_joined() {
  assert_folds(
    "const a = ['ab'];",
    "content: a.map(x => x.toUpperCase()).join(''),",
    ".xpf4ll6{content:\"AB\"}",
  );

  // The mapped list as a value in its own right, rather than as a receiver.
  assert_folds(
    "const a = ['a', 'b'];",
    "content: a.map(x => x).length,",
    ".x1sn2kax{content:\"2px\"}",
  );
}

/// A chain hanging off one of the statics the dispatch below still folds. The
/// static is a link nothing else is ever handed on its own, so the guard hands
/// back only the outermost call — and the engine answers the whole chain.
///
/// Pinned because this is the shape the deleted array table was answering for:
/// `Object.entries(o).filter(f)` folded through it, and nothing else would have
/// noticed it stopping.
#[test]
fn a_chain_off_a_static_folds_end_to_end() {
  assert_folds(
    "const o = { a: 1, b: 2 };",
    "transitionProperty: Object.keys(o).join(','),",
    ".x1iq4t92{transition-property:a,b}",
  );

  assert_folds(
    "const o = { a: 1, b: 2 };",
    "zIndex: Object.entries(o).filter(e => e[1] % 2 === 0).length,",
    ".x1vjfegm{z-index:1}",
  );

  assert_folds(
    "const o = { a: 'x', b: 'y' };",
    "content: Object.values(o).join('-'),",
    ".x16jpvqi{content:\"x-y\"}",
  );
}

/// A filter whose callback answers a truthy *string* rather than a number.
///
/// The deleted implementation decided a callback's truthiness by converting its
/// result to a number, so this reported `Value in not a number` — a panic
/// carrying an internal sentence, on input both compilers otherwise fold. The
/// language decides truthiness now, and the shape is pinned in both spellings so
/// a later change cannot bring the old rule back on one of them.
#[test]
fn a_filter_with_a_truthy_non_numeric_callback_result_folds() {
  assert_folds(
    "const a = ['b', 'a'];",
    "content: a.filter(v => v).join('-'),",
    ".x1y9cpk8{content:\"b-a\"}",
  );

  assert_folds(
    "",
    "content: ['b', 'a'].filter(v => v).join('-'),",
    ".x1y9cpk8{content:\"b-a\"}",
  );
}

// ──────────────────────────────────────────────
// What a name may hold
// ──────────────────────────────────────────────

/// The element kinds an array carries across the bridge. A number, a boolean and
/// `null` cross as elements even though a name holding one alone does not, and
/// the reference compiler's coercions are what the joined text has to match.
#[test]
fn every_element_kind_an_array_may_hold_crosses() {
  assert_folds(
    "const a = [1, 'a', true, null];",
    "content: a.join('|'),",
    ".xpvqmbd{content:\"1|a|true|\"}",
  );

  assert_folds(
    "const a = [4, 8];",
    "padding: a.map(size => size + 'px').join(' '),",
    ".xdqdrvq{padding:4px 8px}",
  );

  // An empty array is a value, not an absent one.
  assert_folds(
    "const a = [];",
    "content: a.join(','),",
    ".x14axycx{content:\"\"}",
  );
}

/// A nested array crosses whole, and an element of one is readable inside a
/// callback. Nesting is where the inward bound applies rather than being
/// refused for existing, which is what the string-only bridge did.
#[test]
fn a_nested_array_crosses_and_its_elements_are_readable() {
  assert_folds(
    "const a = [['x'], ['y']];",
    "content: a.join(''),",
    ".xg1gb30{content:\"xy\"}",
  );

  assert_folds(
    "const a = [['1px'], ['2px']];",
    "transitionProperty: a.map(p => p[0]).join(','),",
    ".x1y3v900{transition-property:1px,2px}",
  );
}

/// A plain object crosses too, which is what makes a member read off a named
/// object a usable receiver — the divergence recorded when only strings crossed.
#[test]
fn a_named_object_is_a_usable_receiver_through_a_member_read() {
  assert_folds(
    "const o = { a: '1px' };",
    "content: o.a.toUpperCase(),",
    ".x10ivmzu{content:\"1PX\"}",
  );
}

/// Two names, each holding an array, in the receiver and the argument position
/// of one call — so the transport carries a parameter list of arrays rather than
/// the one-name case.
#[test]
fn two_named_arrays_in_one_call_are_both_carried() {
  assert_folds(
    "const a = ['b', 'a']; const b = ['d', 'c'];",
    "transitionProperty: a.concat(b).join(','),",
    ".x5uou8z{transition-property:b,a,d,c}",
  );
}

/// The index a callback is handed, which is the second parameter the language
/// passes and a shape the deleted implementation never gave a callback at all.
#[test]
fn a_callback_reads_the_index_the_language_passes_it() {
  assert_folds(
    "const a = ['a'];",
    "content: a.map((x, i) => x + i).join(''),",
    ".xypuhvd{content:\"a0\"}",
  );
}

// ──────────────────────────────────────────────
// What a name does not make foldable
// ──────────────────────────────────────────────

/// A mutating method on a named receiver refuses, and refuses on the rule the
/// *binding* broke rather than on the method. The reference compiler refuses
/// each of these too — measured — because its mutation test disqualifies the
/// binding wherever the mutation is written, including at the read itself.
///
/// The behaviour proved when the method rule was deleted, re-proved now that a
/// named receiver reaches the guard: nothing about carrying an array inward
/// makes a mutated binding readable.
#[test]
fn a_mutating_method_on_a_named_receiver_still_refuses() {
  for body in [
    "transitionProperty: a.sort().join(','),",
    "transitionProperty: a.reverse().join(','),",
    "zIndex: a.push('c'),",
  ] {
    assert_refuses(
      "const a = ['b', 'a'];",
      body,
      "Referenced value is not a constant.",
    );
  }

  // And where the mutation sits away from the read, which is the position a
  // disagreement would be least visible in.
  assert_refuses(
    "const a = ['b', 'a']; a.push('c');",
    "transitionProperty: a.join(','),",
    "Referenced value is not a constant.",
  );
}

/// A hole and a spread are refused in the declaration the name is bound by, so
/// neither ever reaches the bridge. Both compilers reject both.
#[test]
fn a_hole_or_a_spread_in_the_named_array_refuses() {
  assert_refuses(
    "const a = [, 'a'];",
    "content: a.join('-'),",
    "Could not resolve the code being evaluated.",
  );

  assert_refuses(
    "const b = ['a']; const a = [...b, 'c'];",
    "transitionProperty: a.join(','),",
    "Unsupported expression: SpreadElement",
  );
}

/// A method whose answer is not a value the bridge carries back refuses on the
/// way out, naming the kind the language answered with. The reference compiler
/// folds it and then rejects the value as a style value, so both compilers
/// reject the input.
///
/// An iterator is the shape: it is an object, and not a plain one, so `typeof`
/// alone would not tell an author why theirs does not fold.
#[test]
fn a_method_answering_something_unfoldable_refuses_on_the_way_out() {
  assert_refuses(
    "const a = ['a'];",
    "content: a.keys().length,",
    "Cannot carry a folded object back from the engine.",
  );
}

/// A read that walks off the elements and onto the language's function graph is
/// refused inside a callback exactly as it is anywhere else — the guard walks
/// the callback body, so the rule applies at every leaf of it.
#[test]
fn an_escaping_property_inside_a_callback_is_refused() {
  assert_refuses(
    "const a = ['a'];",
    "content: a.map(x => x.constructor)[0],",
    "Cannot fold a read of 'constructor' at compile time.",
  );

  // And spelled as a computed key, which is the same read.
  assert_refuses(
    "const a = ['a'];",
    "content: a.map(x => x['constructor'])[0],",
    "Cannot fold a read of 'constructor' at compile time.",
  );
}

/// A resolved value with more entries than the fold will copy into the engine is
/// refused on the way in, naming the binding rather than the method: the size
/// belongs to what the name holds, and the same call on a smaller value folds.
///
/// A count rather than a length, because entries and text are two costs. Ten
/// thousand empty strings hold no text at all and are still ten thousand values
/// to build.
#[test]
fn a_named_array_past_the_entry_bound_names_the_binding() {
  let elements = std::iter::repeat_n("''", 10_001)
    .collect::<Vec<_>>()
    .join(",");

  assert_refuses(
    &format!("const big = [{}];", elements),
    "content: big.join(''),",
    "Cannot carry the value of 'big' into a fold.",
  );
}

/// A name holding a number crosses, so `Number.prototype` is reachable on one —
/// the statics need a named number as an argument, and a bridge that carried one
/// there but not as a receiver would be deciding by position again.
///
/// The refusal that has to survive it is about how the receiver was *written*: a
/// number literal in the source is still refused, because the reference compiler
/// applies the method without a receiver and throws on it.
#[test]
fn a_name_holding_a_number_is_a_receiver_and_a_written_one_is_not() {
  assert_folds(
    "const n = 5;",
    "content: n.toFixed(1),",
    ".xqj1kdb{content:\"5.0\"}",
  );
  assert_folds(
    "const b = true;",
    "content: b.toString(),",
    ".x1ez55b5{content:\"true\"}",
  );

  assert_refuses(
    "",
    "content: (5).toFixed(1),",
    "Cannot call 'toFixed' on a number literal.",
  );
}

/// An element whose text would be a syntax hazard if it were printed into the
/// source instead of passed to it. This is the property the transport exists
/// for, asserted on an array rather than on a string: the elements are never
/// text, so a quote, a backslash, a newline, a backtick, a `${` and an
/// unbalanced parenthesis all fold exactly.
///
/// Asserted as the class name, which is a hash of the declaration, so matching
/// the reference compiler's is the claim that both built the same declaration
/// from the same value. The rule text reaches the output inside a JavaScript
/// string literal, where the emitter escapes these a second time.
#[test]
fn elements_that_could_not_be_printed_safely_still_fold_exactly() {
  let cases: &[(&str, &str, &str)] = &[
    (
      r#"const a = ['a"b', 'c'];"#,
      "content: a.join('|'),",
      "x1aihos",
    ),
    (
      r"const a = ['a\\b', 'c'];",
      "content: a.join('|'),",
      "x1np0pou",
    ),
    (r"const a = ['a\nb'];", "content: a.join('|'),", "x38goau"),
    (
      "const a = ['a`b', 'a${x}b'];",
      "content: a.join('|'),",
      "xiki0v9",
    ),
    ("const a = [')', '('];", "content: a.join(''),", "x1dfdin2"),
    ("const a = ['a\u{0}b'];", "content: a.join(''),", "x1lad8bx"),
    (
      "const a = ['café'];",
      "content: a.map(x => x.normalize('NFC')).join(''),",
      "x1kzf2xh",
    ),
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

/// An element holding half a surrogate pair crosses as code units, because no
/// Rust string can hold one and the engine's strings are UTF-16.
///
/// The declaration is what the reference compiler writes — the outward bridge
/// has to land in a string literal, so it substitutes the replacement character,
/// and so does upstream's own output. The class name diverges, which is the
/// divergence already recorded for a value written out, so this pins the round
/// trip on the rule text alone.
#[test]
fn an_element_holding_half_a_surrogate_pair_crosses_and_comes_back() {
  let output = fold(&module(
    "const a = ['\\uD83D', 'b'];",
    "content: a.join('|'),",
  ));

  assert!(
    output.contains("content:\"\u{FFFD}|b\""),
    "expected the replacement character to reach the declaration, got:\n{}",
    output
  );
}

/// A key an identifier cannot spell, read through a computed member. The key
/// crosses as text like any other, so the read the author wrote is the read the
/// engine makes.
#[test]
fn a_key_that_is_not_an_identifier_is_readable_through_a_computed_member() {
  assert_folds(
    "const o = { 'a-b': '1px' };",
    "content: o['a-b'].toUpperCase(),",
    ".x10ivmzu{content:\"1PX\"}",
  );
}

/// An array of objects, which is the shape a list of conditions is written as,
/// and the one that needs both halves of the bridge at once.
#[test]
fn an_array_of_objects_crosses_and_its_properties_are_readable() {
  assert_folds(
    "const a = [{ a: '1px' }, { a: '2px' }];",
    "transitionProperty: a.map(o => o.a).join(','),",
    ".x1y3v900{transition-property:1px,2px}",
  );
}

/// An absent argument is the language's default, not an absent value: `join()`
/// separates with a comma.
#[test]
fn a_method_called_with_no_argument_takes_the_language_s_default() {
  assert_folds(
    "const a = ['b', 'a'];",
    "content: a.join(),",
    ".x1yru101{content:\"b,a\"}",
  );
}

/// Two callbacks in one chain, each running over what the last produced — the
/// shape a per-link table has to answer twice and the engine answers once.
#[test]
fn two_callbacks_in_one_chain_both_run() {
  assert_folds(
    "const a = ['x'];",
    "content: a.map(v => v + 'y').map(v => v + 'z').join(''),",
    ".x1jrf645{content:\"xyz\"}",
  );
}

/// A string a theme reference resolved to, held as an element. The reference
/// itself never crosses: what the fold reads is the `var(--…)` string the
/// resolution produced, and resolving it is what mutates compiler state, so it
/// happens before the bridge rather than across it.
///
/// The assertion is the upper-cased reference, because that is the only thing
/// here a fold can produce — an unfolded `vars.primary` reaches the declaration
/// as `var(--…)` too.
#[test]
fn a_theme_resolved_string_is_a_usable_element() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import { vars } from 'vars.stylex.js';
      const parts = [vars.primary];
      export const styles = stylex.create({
        base: { content: parts.join('').toUpperCase() },
      });
    "#,
  );

  assert!(
    output.contains("content:\"VAR(--"),
    "expected the resolved theme string to cross as an element and be upper-cased, got:\n{}",
    output
  );
}

/// Inside a dynamic style function the parameter has no compile-time value, so
/// the call is left for the runtime rather than failing the build. Naming a
/// value is what makes a fold possible; a parameter is not a value yet.
#[test]
fn a_call_on_a_dynamic_parameter_is_still_left_to_the_runtime() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (parts) => ({ transitionProperty: parts.join(',') }),
      });
    "#,
  );

  assert!(
    output.contains("transition-property:var(--"),
    "expected the declaration to defer to a custom property, got:\n{}",
    output
  );

  assert!(
    output.contains("parts.join(',')"),
    "expected the call itself to survive into the runtime value, got:\n{}",
    output
  );
}

/// Nesting past the fold's budget is a diagnostic rather than a stack overflow,
/// on a value a name holds exactly as on one written out. The budget the message
/// names is the one the resolution spends first, and both are the same number
/// with the same sentence.
#[test]
fn a_named_array_nested_past_the_budget_names_the_depth_rule() {
  assert_refuses(
    &format!("const deep = {}'x'{};", "[".repeat(40), "]".repeat(40)),
    "content: deep.flat(40).join(''),",
    "Expression is too deeply nested to evaluate at compile time.",
  );
}

/// The text bound applies through an array, and names the binding: one very
/// long element is the same megabyte as one very long string, and the same call
/// on a shorter value folds.
#[test]
fn an_element_past_the_text_bound_names_the_binding() {
  assert_refuses(
    &format!("const big = ['{}'];", "x".repeat(1_000_001)),
    "content: big.join(''),",
    "Cannot carry the value of 'big' into a fold.",
  );
}

/// A folded value is a value like any other and goes on through the CSS guards:
/// a join that spells a second declaration is refused where it would close the
/// rule being generated. The reference compiler emits it, which is a divergence
/// this compiler pins on purpose and not one this fold introduces.
#[test]
fn a_folded_value_still_reaches_the_css_guards() {
  assert_refuses(
    "const a = ['red;', 'margin:0'];",
    "color: a.join(''),",
    "Rule contains a `{`, `}` or `;` outside of a string or comment",
  );
}

/// A global the dispatch below folds is a receiver and not a value. Its *name*
/// carries nothing across the bridge, so the guard refuses it and names it —
/// where admitting it would fold a function's own source text into a
/// declaration. Both compilers refuse.
#[test]
fn a_global_read_as_a_value_rather_than_a_receiver_refuses() {
  assert_refuses(
    "const a = ['a'];",
    "content: a.concat(String).join(''),",
    "Cannot carry the global 'String' into a fold.",
  );
}

/// A static inside a callback, which is a nested call twice over: the callback
/// runs in the engine and the static is a link of the chain inside it.
#[test]
fn a_static_inside_a_callback_folds() {
  assert_folds(
    "",
    "content: ['a'].map(x => Object.keys({ b: x }).join('')).join(''),",
    ".xa8xio6{content:\"b\"}",
  );
}
