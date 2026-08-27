//! A StyleX function inside a fold.
//!
//! The injected function map is not JavaScript — its values are this compiler's
//! own Rust functions — so the engine cannot be handed the map. One of those
//! functions answers from its arguments and touches nothing else, and that one
//! travels as a function of the engine's own: `firstThatWorks` reorders the
//! fallbacks it was given and folds the CSS variables among them into one
//! `var()` chain.
//!
//! What it buys is the callback. `a.map(x => firstThatWorks(x, 'serif'))` is one
//! JavaScript call per element, which is the engine's job — and before this the
//! whole declaration refused, because the array methods that would have run it
//! had moved into the engine and the function had not.
//!
//! Every rule below is measured on `@stylexjs/babel-plugin` 0.19.0, and the
//! declaration text *and* the class name are asserted, since the class name is a
//! hash of the declaration and is what a migrating build compares. The cases
//! that diverge say which way and why.

use crate::utils::transform::{assert_folds, assert_refuses};

/// The bindings every case that only needs the import writes.
const IMPORT: &str = "import { firstThatWorks } from '@stylexjs/stylex';";

/// The same, with the one-element receiver most of the argument cases fold over.
const IMPORT_AND_ONE: &str = "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a'];";

// ──────────────────────────────────────────────
// The shape the ticket is about
// ──────────────────────────────────────────────

/// The reported shape: a callback naming the function, over a named array.
#[test]
fn a_callback_naming_the_function_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "fontFamily: a.map(x => firstThatWorks(x, 'serif')).join(','),",
    ".x10gm80u{font-family:serif,a,serif,b}",
  );
}

/// The namespace spelling, which is how the documentation writes every StyleX
/// call. The name that crosses is the namespace's, holding this one function,
/// so the printed call reads exactly as it was written.
#[test]
fn the_namespace_spelling_folds_to_the_same_rule() {
  assert_folds(
    "const a = ['a', 'b'];",
    "fontFamily: a.map(x => stylex.firstThatWorks(x, 'serif')).join(','),",
    ".x10gm80u{font-family:serif,a,serif,b}",
  );
}

/// A renamed import and a renamed namespace, since what the guard asks is what
/// the module imported and not how it spelled it.
#[test]
fn a_renamed_import_folds() {
  assert_folds(
    "import { firstThatWorks as ftw } from '@stylexjs/stylex'; const a = ['a'];",
    "content: a.map(x => ftw(x, 'z')).join(''),",
    ".x1skdnzx{content:\"z,a\"}",
  );

  assert_folds(
    "import * as sx from '@stylexjs/stylex'; const a = ['a'];",
    "content: a.map(x => sx.firstThatWorks(x, 'z')).join(''),",
    ".x1skdnzx{content:\"z,a\"}",
  );
}

// ──────────────────────────────────────────────
// The receivers the checklist asked to be measured
// ──────────────────────────────────────────────

/// A string receiver, reached through `split` so the callback gets characters.
#[test]
fn a_string_receiver_folds() {
  assert_folds(
    IMPORT,
    "content: 'ab'.split('').map(c => firstThatWorks(c, 'z')).join('|'),",
    ".x19ywfap{content:\"z,a|z,b\"}",
  );
}

/// An object receiver, reached through `Object.values`. It folds to the same
/// rule as the string one, which is the point: the receiver decides nothing.
#[test]
fn an_object_receiver_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const o = { p: 'a', q: 'b' };",
    "content: Object.values(o).map(c => firstThatWorks(c, 'z')).join('|'),",
    ".x19ywfap{content:\"z,a|z,b\"}",
  );
}

/// Every array method that takes a callback reaches it the same way, so a fold
/// through `filter`, `reduce` and `flatMap` is a fold through one mechanism
/// rather than three.
#[test]
fn the_other_callback_methods_fold() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: a.filter(x => firstThatWorks(x, 'z').length > 1).join(''),",
    ".xarbti{content:\"ab\"}",
  );

  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: a.reduce((acc, x) => acc + firstThatWorks(x, 'z').join(''), ''),",
    ".x11dygsq{content:\"zazb\"}",
  );

  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: a.flatMap(x => firstThatWorks(x, 'z')).join('-'),",
    ".xlrbkoc{content:\"z-a-z-b\"}",
  );
}

/// Outside a callback entirely: the function's answer is the receiver of a fold.
/// This refused before too — the receiver was a value nothing below the fold
/// could reach a method on.
#[test]
fn the_answer_as_a_folds_receiver_folds() {
  assert_folds(
    IMPORT,
    "content: firstThatWorks('a', 'b').join('+'),",
    ".xl7k27s{content:\"b+a\"}",
  );
}

/// A chain on both sides of the call, so nothing about the fold depends on the
/// StyleX call being the outermost or the innermost link.
#[test]
fn a_chain_around_the_call_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['b', 'a'];",
    "content: a.map(x => firstThatWorks(x, 'z')).map(v => v.join('/')).sort().slice(0, 2).join('|').toUpperCase(),",
    ".x1dhd5j6{content:\"Z/A|Z/B\"}",
  );
}

/// A call nested inside another one, which is a fold whose argument is a fold.
#[test]
fn the_function_calling_itself_folds() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(firstThatWorks('var(--a)', 'b'), 'c')).join(''),",
    ".x5yy3i9{content:c,var(--a, b)}",
  );
}

/// An inner arrow, so the scope the call is read in is the callback's and not
/// the module's.
#[test]
fn a_call_inside_a_nested_callback_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = [['a']];",
    "content: a.map(o => o.map(x => firstThatWorks(x, 'z')).join('')).join(''),",
    ".x1skdnzx{content:\"z,a\"}",
  );
}

/// Two calls in one body, which is the case the transport's one-parameter-per-
/// name rule answers: a repeated parameter would be a syntax error in the
/// printed arrow.
#[test]
fn two_calls_in_one_body_carry_one_parameter() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(x, 'y').join('') + firstThatWorks(x, 'z').join('')).join(''),",
    ".xi0aakj{content:\"yaza\"}",
  );
}

/// A branch, a nested object and a property read around the call, so the
/// answer is a value the rest of the body reads like any other.
#[test]
fn the_answer_is_a_value_the_body_reads() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: a.map(x => x === 'a' ? firstThatWorks(x, 'z').join('') : x).join('|'),",
    ".x19mr7yj{content:\"za|b\"}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => ({ v: firstThatWorks(x, 'z') }).v.join('-')).join(''),",
    ".x1qgjv93{content:\"z-a\"}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(x, 'z').length).join(''),",
    ".xhs4kdw{content:\"2\"}",
  );
}

// ──────────────────────────────────────────────
// What the function answers, over the values the engine holds
// ──────────────────────────────────────────────

/// No variable among the arguments: they come back reversed, and nothing is
/// read as text at all.
#[test]
fn arguments_with_no_variable_reverse() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('p', 'q', 'r').join('/')).join(''),",
    ".x1q0lzzk{content:\"r/q/p\"}",
  );
}

/// One variable and the value behind it, which is the whole point of the
/// function: `var(--x, fallback)`.
#[test]
fn a_variable_and_its_fallback_fold_to_one_chain() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)', 'q')).join(''),",
    ".xmki5ic{content:var(--p, q)}",
  );
}

/// Variables nest into one chain however many of them there are, and the chain
/// stops at the first value after them — so an argument past that value is
/// dropped, which is what the reference implementation does.
#[test]
fn the_chain_stops_at_the_first_value_after_the_variables() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--1)', 'var(--2)', 'var(--3)', 'var(--4)', 'f')).join(''),",
    ".xz6oxkw{content:var(--1, var(--2, var(--3, var(--4, f))))}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('p', 'var(--q)', 'var(--r)', 's', 't')).join(''),",
    ".x84mho9{content:var(--q, var(--r, s)),p}",
  );
}

/// A variable with nothing behind it, and one whose fallback is the empty
/// string: both answer the bare `var()`, because an empty fallback leaves the
/// chain with nothing to fold into.
#[test]
fn a_variable_with_no_usable_fallback_answers_the_bare_reference() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)')).join(''),",
    ".x6rj7mk{content:var(--p)}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)', '')).join(''),",
    ".x6rj7mk{content:var(--p)}",
  );
}

/// A bare `--name` is a variable's name rather than a reference to it, so the
/// chain wraps it — the one place the fold adds a `var()` an author did not
/// write.
#[test]
fn a_bare_variable_name_is_wrapped() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--a)', '--b')).join(''),",
    ".x16ao0rn{content:var(--a, var(--b))}",
  );
}

/// Values that are not strings keep their own form: a number, a boolean, `null`
/// and an array are answered as they were handed in, and only reach text where
/// the surrounding fold writes them out. Which is why an argument is read as a
/// variable only when it *is* a string spelling one.
#[test]
fn arguments_that_are_not_strings_keep_their_form() {
  assert_folds(
    IMPORT,
    "content: [1, 2].map(n => firstThatWorks(n, 'z')).join('|'),",
    ".xls0piv{content:\"z,1|z,2\"}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(['a', 'b'], 'c')).join(''),",
    ".x15grb4v{content:\"c,a,b\"}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(null, 'c')).join(''),",
    ".x1b4eune{content:\"c,\"}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(false, 0, '').join('|')).join(''),",
    ".xe1h6ct{content:\"|0|false\"}",
  );
}

/// No arguments at all, which answers an empty list.
#[test]
fn no_arguments_answer_an_empty_list() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks()).join(''),",
    ".x14axycx{content:\"\"}",
  );
}

/// An argument that is itself a fold — a template literal with a hole, and a
/// method call on the callback's own parameter.
#[test]
fn an_argument_that_is_itself_a_fold_folds_first() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(`var(--${x})`, 'z')).join(''),",
    ".xje2597{content:var(--a, z)}",
  );

  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(x.toUpperCase(), 'z')).join(''),",
    ".x1udsdws{content:\"z,A\"}",
  );
}

/// Text outside ASCII crosses both ways unchanged, which is what fixes the
/// class name for a file that is not written in English.
#[test]
fn text_outside_ascii_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['ü'];",
    "content: a.map(x => firstThatWorks(x, 'é')).join(''),",
    ".x65ffq2{content:\"é,ü\"}",
  );
}

// ──────────────────────────────────────────────
// The names that are not this function
// ──────────────────────────────────────────────

/// A callback parameter of the same spelling is the callback's, not the
/// module's import: the engine binds it when it runs the callback, and the
/// guard carries nothing for it.
#[test]
fn a_callback_parameter_shadows_the_import() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(firstThatWorks => firstThatWorks + '!').join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

/// The module's own function of the same name is not a StyleX function, and
/// nothing about it changed: the fold hands the call back, as it does for any
/// function this compiler cannot see the body of. Refused where the reference
/// compiler folds it, which is the shape issue 19 carries.
#[test]
fn a_local_function_of_the_same_name_is_not_this_one() {
  assert_refuses(
    "const firstThatWorks = (x) => x + '!'; const a = ['a'];",
    "content: a.map(x => firstThatWorks(x)).join(''),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

/// The function handed over as the callback itself rather than called inside
/// one. Both compilers refuse: what the reference implementation's map holds
/// under the name is a configuration object, and calling it complains that an
/// object is not a function.
#[test]
fn the_function_as_the_callback_itself_refuses() {
  assert_refuses(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: a.map(firstThatWorks).join('|'),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

/// A StyleX function read as a value instead of called. Refused here, where the
/// reference compiler answers `object` — its evaluator holds the function as a
/// configuration object, and this one hands the engine something it may call and
/// not something it may read. Nothing an author writes for a stylesheet asks
/// this question, and answering it would mean giving the engine a value whose
/// own properties lead back into the compiler.
#[test]
fn the_function_read_as_a_value_refuses() {
  assert_refuses(
    IMPORT_AND_ONE,
    "content: a.map(x => typeof firstThatWorks).join(''),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

/// A StyleX function that writes into the build is not one the engine may call.
/// `keyframes` hashes a rule, injects it and answers the name it chose, so
/// running it once per element would inject once per element from inside a
/// speculative evaluation. Refused here, where the reference compiler folds it —
/// recorded as a gap rather than as a boundary anyone wants.
#[test]
fn a_function_that_writes_into_the_build_refuses() {
  assert_refuses(
    "import { keyframes } from '@stylexjs/stylex'; const a = ['a'];",
    "content: a.map(x => keyframes({ from: { opacity: 0 } })).join(''),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

/// An argument the bridge cannot carry, inside a callback: the refusal names the
/// function and what its arguments have to be, rather than the array method
/// around it. Nothing below the fold can answer for a callback body, so the
/// vaguer sentence would be the last word.
///
/// The reference compiler folds the first of these to `[object Object]`, which is
/// its own evaluator's spelling of its own function map and not a declaration
/// anyone wants; it refuses the second, in words of its own about the name.
#[test]
fn an_argument_with_no_value_names_the_function_inside_a_callback() {
  assert_refuses(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(stylex, 'z')).join(''),",
    "Only static values can be passed to firstThatWorks().",
  );

  assert_refuses(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks(missing, 'z')).join(''),",
    "Only static values can be passed to firstThatWorks().",
  );
}

/// The same argument outside a callback is handed back rather than refused, so
/// the sentence stays the dispatch's — which is the point: outside a callback the
/// dispatch may still own the call around this one, and a rule raised here would
/// take a fold away from it.
#[test]
fn an_argument_with_no_value_outside_a_callback_stays_the_dispatchs() {
  assert_refuses(
    IMPORT,
    "content: firstThatWorks(stylex, 'z').join(''),",
    "Function argument must be a static expression.",
  );
}

/// A spread argument, refused by both compilers in the same words: the spread
/// needs a scope, and the sentence is the one every other position gives it.
#[test]
fn a_spread_argument_refuses() {
  assert_refuses(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "content: ['q'].map(x => firstThatWorks(...a)).join(''),",
    "Unsupported expression: SpreadElement",
  );
}

/// A call written on its own is still the older dispatch's, deliberately: it
/// resolves its arguments this compiler's own way, a theme reference included,
/// and the engine has no value for one of those. So the two fallbacks stay two
/// declarations rather than becoming one folded array.
#[test]
fn a_call_written_on_its_own_stays_below_the_fold() {
  assert_folds(
    IMPORT,
    "fontFamily: firstThatWorks('a', 'serif'),",
    ".xb4bdrg{font-family:serif;font-family:a}",
  );
}

// ──────────────────────────────────────────────
// Sizes, and the bounds that still hold
// ──────────────────────────────────────────────

/// Five hundred elements, each one call, and a chain over the answers. The
/// function allocates in proportion to its arguments, so a callback running
/// once per element grows the answer and not the work per call.
#[test]
fn a_receiver_of_many_elements_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = Array.from({ length: 500 }, (_, i) => String(i));",
    "content: a.map(x => firstThatWorks(x, 'z').join('')).join('|').length,",
    ".x1n66ihh{content:\"2389px\"}",
  );
}

/// A two-thousand-character fallback, which is a chain the engine builds and
/// carries back whole.
#[test]
fn a_long_fallback_folds() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a']; const long = 'y'.repeat(2000);",
    "content: a.map(x => firstThatWorks('var(--p)', long)).join('').length,",
    ".x1ts7t98{content:\"2010px\"}",
  );
}

/// Five thousand arguments written out, which is a chain five thousand levels
/// deep built inside one call and carried back as one string. The chain is a
/// loop rather than a recursion, so what bounds it is the folded-string ceiling
/// and not the stack.
#[test]
fn thousands_of_written_arguments_fold() {
  let many = (0..5000)
    .map(|index| format!("'var(--v{})'", index))
    .collect::<Vec<_>>()
    .join(", ");

  assert_folds(
    IMPORT_AND_ONE,
    &format!(
      "content: a.map(x => firstThatWorks({})).join('').length,",
      many
    ),
    ".x1tssbl{content:\"68888px\"}",
  );
}

/// Two hundred calls nested inside one another, which the fold's own depth
/// ceiling refuses — a configured ceiling an author can raise, not a rule about
/// this function. The reference compiler has no such ceiling and folds it.
#[test]
fn calls_nested_past_the_depth_ceiling_refuse() {
  let mut nested = String::from("'x'");

  for _ in 0..200 {
    nested = format!("firstThatWorks({}, 'var(--v)')", nested);
  }

  assert_refuses(
    IMPORT_AND_ONE,
    &format!("content: a.map(x => {}).join(''),", nested),
    "Expression is too deeply nested to evaluate at compile time.",
  );
}

/// An argument whose text the fold has to read, reached through an object the
/// call built: the engine runs that `toString`, and it reaches this same
/// function again. It folds, where the reference compiler throws — its own
/// reduce calls `startsWith` on whatever the chain bottoms out on, so any
/// argument that is not a string ends its build.
#[test]
fn an_argument_whose_text_the_engine_has_to_ask_for_folds() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)', { toString: () => firstThatWorks('var(--q)', 'r') })).join(''),",
    ".xvfr7q5{content:var(--p, var(--q, r))}",
  );
}

/// A variable name built out of the callback's own parameter, which is what an
/// author writing the same transform over a list of tokens actually writes.
#[test]
fn a_variable_name_built_from_the_parameter_folds() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--' + x + ')', 'z')).join(''),",
    ".xje2597{content:var(--a, z)}",
  );
}

/// The callback amplification bound still applies inside the body, and is still
/// read off the amplifying call rather than this one — so a length the receiver's
/// element count keeps inside the ceiling folds beside a StyleX function, and one
/// past it is refused by the amplifying call's own name.
#[test]
fn an_amplifying_call_beside_it_is_still_bounded() {
  assert_folds(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)', 'x'.repeat(3))).join(''),",
    ".xwd2ya6{content:var(--p, xxx)}",
  );

  assert_refuses(
    IMPORT_AND_ONE,
    "content: a.map(x => firstThatWorks('var(--p)', 'x'.repeat(1000001))).join(''),",
    "Cannot bound the string 'repeat' would build.",
  );
}
