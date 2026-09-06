//! One printed expression is parsed once and re-run many times, and the answers
//! stay the answers.
//!
//! The engine is one leaked instance per thread, and so is the memo beside it:
//! a compiled script built while one file was folding is re-run while the next
//! one folds. That is the whole point — parsing is most of what a warm fold
//! costs — and it is also the only way this could go wrong, so the cases here
//! are pairs of files that print the *same* text and must not answer the same
//! value.
//!
//! Same text with different values is unreachable inside one file: a module
//! cannot declare one name twice, and `stylex.create` must be bound to a bare
//! variable at the top level. Two compiles on one thread is therefore not a
//! contrivance but the shape the risk actually has.
//!
//! Every rule below is measured output of `@stylexjs/babel-plugin` 0.19.0 under
//! the same options.

use crate::utils::transform::{assert_folds, base_style_module as module, fold_module as fold};

/// Panics unless each file in turn emits the rule recorded beside it.
///
/// One helper rather than a list of separate cases, because what these assert is
/// the *sequence* — the second compile re-runs what the first one left in the
/// memo — and the order they are written in is the order they have to run in.
#[track_caller]
fn assert_each_file_folds(cases: &[(&str, &str, &str)]) {
  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// The same printed text, different values
// ──────────────────────────────────────────────

/// Two files whose folds print `(s)=>s.toUpperCase()` answer their own strings.
///
/// A memo holding the *answer* rather than the compiled script would give the
/// second file the first one's value, and would do it silently: both rules are
/// well-formed CSS.
#[test]
fn a_shared_printed_expression_answers_each_files_own_value() {
  assert_each_file_folds(&[
    (
      "const s = 'abc';",
      "content: s.toUpperCase(),",
      ".xj5ouxf{content:\"ABC\"}",
    ),
    (
      "const s = 'xyz';",
      "content: s.toUpperCase(),",
      ".x1vxwqg1{content:\"XYZ\"}",
    ),
  ]);
}

/// The same claim where the carried value is an array rather than a string, so
/// the argument is an object the engine builds fresh for each call.
#[test]
fn a_shared_printed_expression_over_an_array_answers_each_files_own_value() {
  assert_each_file_folds(&[
    (
      "const p = ['1px', 'solid', 'red'];",
      "outline: p.join(' '),",
      ".x1rragdb{outline:1px solid red}",
    ),
    (
      "const p = ['2px', 'dashed', 'blue'];",
      "outline: p.join(' '),",
      ".x1nevxea{outline:2px dashed blue}",
    ),
  ]);
}

/// And where it is a number, whose method is on `Number.prototype`.
#[test]
fn a_shared_printed_expression_over_a_number_answers_each_files_own_value() {
  assert_each_file_folds(&[
    (
      "const n = 4;",
      "zIndex: n.toFixed(0),",
      ".xoegz02{z-index:4}",
    ),
    (
      "const n = 9;",
      "zIndex: n.toFixed(0),",
      ".xk3oba8{z-index:9}",
    ),
  ]);
}

// ──────────────────────────────────────────────
// The same file, twice
// ──────────────────────────────────────────────

/// Compiling one file twice answers the same thing twice.
///
/// The second compile is the first memo hit anything can observe from here, and
/// the receiver is a mutating method on purpose: a memo that had kept the array
/// its first run sorted, rather than the script that builds a fresh one, would
/// answer the second run from a value the first one already reordered.
#[test]
fn recompiling_one_file_answers_what_it_answered() {
  let source = module("", "content: ['b', 'a'].sort().join(','),");

  let first = fold(&source);
  let second = fold(&source);

  assert_eq!(first, second);
  assert!(first.contains(".xprt6xs{content:\"a,b\"}"), "got:\n{first}");
}

/// A fold written out rather than named goes through the same memo, because
/// what the memo holds is a compiled script and the bare form is one too.
#[test]
fn a_written_out_receiver_folds_the_same_on_every_compile() {
  for _ in 0..3 {
    assert_folds(
      "",
      "content: 'ab'.repeat(3),",
      ".x5ryvnc{content:\"ababab\"}",
    );
  }
}
