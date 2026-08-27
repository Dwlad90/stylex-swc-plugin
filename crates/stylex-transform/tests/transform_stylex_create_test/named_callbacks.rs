//! Giving a callback a name stops changing whether the call on it folds.
//!
//! The transport carries a resolved *value* beside the printed source, and a
//! function has no value form to carry — so a callback written out in place
//! folded while the same arrow reached through a name did not. What crosses for
//! a function instead is the declaration it came from, printed back as the
//! default of the parameter the name became. The engine then binds the name
//! exactly where the module bound it, which is what makes shadowing the
//! language's answer rather than this walk's.
//!
//! Which declarations qualify is decided by what the name resolves to, and the
//! set turns out to be the reference compiler's own: an arrow with plain
//! parameters and a single expression body, never written to after it was
//! declared.
//!
//! A name outside that set gets one of two answers, and which one is the point.
//! A binding the *module* declares as a function is refused, naming the binding
//! — a block body, a destructured, defaulted or rest parameter, a `function` of
//! either spelling, and a binding written to after it was declared. A name the
//! module declares nothing for is *handed back* instead: a dynamic style's own
//! parameter holds a function nobody can see at compile time, and refusing it
//! would fail a build over a call that was only ever going to run at runtime.
//! So the three answers a name can get are refuse-and-name-it, fold, and
//! not-mine — and the last is why the guard asks the resolution rather than the
//! spelling.
//!
//! Every case below was measured against `@stylexjs/babel-plugin` 0.19.0 and
//! carries the class name it produced.

use crate::utils::transform::{assert_folds, assert_refuses, fold_module as fold};

// ──────────────────────────────────────────────
// The shape the ticket is about
// ──────────────────────────────────────────────

/// The row the generated prototype sweep found: an array method whose callback
/// is nothing but a name.
#[test]
fn a_named_arrow_callback_folds() {
  assert_folds(
    "const upper = (part) => part.toUpperCase();",
    "content: ['b', 'a'].map(upper).join(', '),",
    ".x154217v{content:\"B, A\"}",
  );
}

/// A comparator, which is the other callback an author writes twice and so the
/// other one they give a name.
#[test]
fn a_named_comparator_folds() {
  assert_folds(
    "const byLength = (a, b) => a.length - b.length;",
    "content: ['bbb', 'a', 'cc'].sort(byLength).join(','),",
    ".x15fdvvp{content:\"a,cc,bbb\"}",
  );
}

/// A reducer takes two parameters and a seed, so it proves the arrow is called
/// with everything the method passes rather than with the first argument only.
#[test]
fn a_named_reducer_folds() {
  assert_folds(
    "const add = (total, n) => total + n;",
    "content: [1, 2, 3].reduce(add, 0) + 'px',",
    ".xf88rig{content:\"6px\"}",
  );
}

/// The index is the second parameter every array method passes, and a named
/// arrow has to receive it as an inline one does.
#[test]
fn a_named_callback_reading_the_index_folds() {
  assert_folds(
    "const withIndex = (part, index) => part + index;",
    "content: ['b', 'a'].map(withIndex).join(','),",
    ".x1cf7pwt{content:\"b0,a1\"}",
  );
}

// ──────────────────────────────────────────────
// The statics the sweep reaches a named callback through
// ──────────────────────────────────────────────

/// `Object.groupBy` is where the generated sweep names a *function* at all: a
/// prototype subject names only its receiver, so this is the one surface that
/// found the gap.
#[test]
fn a_named_callback_on_a_namespace_static_folds() {
  assert_folds(
    "const size = (x) => (x.length > 1 ? 'long' : 'short');",
    "content: Object.keys(Object.groupBy(['a', 'bb'], size)).join(','),",
    ".x13qc2oz{content:\"short,long\"}",
  );
}

/// `Array.from` takes its callback second, and carries a declared length as
/// well — so it is the static where the callback and the entry ceiling meet.
#[test]
fn a_named_callback_on_array_from_folds() {
  assert_folds(
    "const twice = (n) => n * 2;",
    "content: Array.from([1, 2], twice).join(','),",
    ".xgcyv7z{content:\"2,4\"}",
  );
}

// ──────────────────────────────────────────────
// What the declaration is allowed to read
// ──────────────────────────────────────────────

/// A name the declaration reads is a module name, and it has to cross with the
/// declaration rather than be looked for where the declaration was printed.
#[test]
fn a_named_callback_reading_a_module_name_folds() {
  assert_folds(
    "const suffix = '!'; const shout = (part) => part.toUpperCase() + suffix;",
    "content: ['b', 'a'].map(shout).join(', '),",
    ".x15snhxx{content:\"B!, A!\"}",
  );
}

/// The value the declaration reads is a composite, so what crosses beside the
/// printed default is a whole array rather than one string.
#[test]
fn a_named_callback_closing_over_a_module_array_folds() {
  assert_folds(
    "const parts = ['a', 'b']; const join = (sep) => parts.join(sep);",
    "content: ['-'].map(join).join(''),",
    ".x1t42mo{content:\"a-b\"}",
  );
}

/// A declaration reading nothing at all still has to cross, because the arrow
/// is what the call needs and not the names in it.
#[test]
fn a_named_callback_with_no_parameters_folds() {
  assert_folds(
    "const zero = () => 'z';",
    "content: ['a'].map(zero).join(''),",
    ".x1609fvb{content:\"z\"}",
  );
}

/// A template literal in the body is written-out syntax with holes, so it
/// reaches the engine as the author wrote it.
#[test]
fn a_named_callback_writing_a_template_literal_folds() {
  assert_folds(
    "const bang = (x) => `${x}!`;",
    "content: ['a', 'b'].map(bang).join(','),",
    ".xbxkmrw{content:\"a!,b!\"}",
  );
}

/// An arrow inside the declaration is a callback of its own, one level further
/// from the name than the walk started.
#[test]
fn a_named_callback_nesting_an_arrow_folds() {
  assert_folds(
    "const rows = (x) => ['1', '2'].map(y => x + y).join('');",
    "content: ['a', 'b'].map(rows).join(','),",
    ".x1v73bzx{content:\"a1a2,b1b2\"}",
  );
}

// ──────────────────────────────────────────────
// Where the name is read from
// ──────────────────────────────────────────────

/// A name resolving to another name is a chain, and each link crosses as the
/// link before it — which is what walking the declaration before recording it
/// buys, since a default may only read a parameter already standing.
#[test]
fn a_chain_of_aliases_folds() {
  assert_folds(
    "const up = (x) => x.toUpperCase(); const a1 = up; const a2 = a1; const a3 = a2;",
    "content: ['b', 'a'].map(a3).join(','),",
    ".x54jj2b{content:\"B,A\"}",
  );
}

/// One name read twice is one parameter, because a repeated parameter is a
/// syntax error in the arrow the fold is printed into.
#[test]
fn a_named_callback_read_twice_folds() {
  assert_folds(
    "const up = (x) => x.toUpperCase();",
    "content: ['a'].map(up).join('') + ['b'].map(up).join(''),",
    ".xpf4ll6{content:\"AB\"}",
  );
}

/// A name read from inside another callback is still a module name, so it
/// crosses as one rather than being asked of the scope the reading was in.
#[test]
fn a_named_callback_read_inside_another_callback_folds() {
  assert_folds(
    "const up = (x) => x.toUpperCase();",
    "content: [['a'], ['b']].map(g => g.map(up).join('')).join('-'),",
    ".x1xcwzc{content:\"A-B\"}",
  );
}

/// Which of two functions the call gets is decided by the language, so both
/// names cross and the branch is answered where it was written.
#[test]
fn a_named_callback_chosen_by_a_condition_folds() {
  assert_folds(
    "const up = (x) => x.toUpperCase(); const down = (x) => x.toLowerCase();",
    "content: ['A'].map(true ? down : up).join(''),",
    ".x16319ns{content:\"a\"}",
  );
}

/// A `let` nothing writes to holds its initializer as surely as a `const`
/// does, and the resolution says so rather than the keyword.
#[test]
fn a_let_the_module_never_writes_to_folds() {
  assert_folds(
    "let up = (x) => x.toUpperCase();",
    "content: ['a'].map(up).join(''),",
    ".x171tc9c{content:\"A\"}",
  );
}

/// An exported declaration is a declaration, and being reachable from another
/// file changes nothing about what this one holds.
#[test]
fn an_exported_declaration_folds() {
  assert_folds(
    "export const up = (x) => x.toUpperCase();",
    "content: ['a'].map(up).join(''),",
    ".x171tc9c{content:\"A\"}",
  );
}

// ──────────────────────────────────────────────
// Shadowing, which the printed arrow has to get right
// ──────────────────────────────────────────────

/// A callback parameter spelled like a module name shadows it, and nothing
/// carries the module name in — the guard never asks the module for a name the
/// callback binds.
#[test]
fn a_callback_parameter_shadows_the_name_it_repeats() {
  assert_folds(
    "const up = (x) => x.toUpperCase();",
    "content: ['b'].map(up => up + '!').join(','),",
    ".x1vnqnxr{content:\"b!\"}",
  );
}

/// The same spelling on both sides: the declaration's own parameter shadows the
/// module name inside its body, and the module name still reaches the call
/// beside it.
#[test]
fn a_declarations_parameter_shadows_a_carried_name() {
  assert_folds(
    "const p = 'Z'; const f = (p) => p + 'x';",
    "content: ['b', 'a'].map(f).join(p),",
    ".x1qfpz3r{content:\"bxZax\"}",
  );
}

/// A value and a function cross under one call by two different routes, so the
/// arguments and the printed parameters have to stay lined up.
#[test]
fn a_value_and_a_function_cross_together() {
  assert_folds(
    "const sep = '-'; const up = (x) => x.toUpperCase();",
    "content: ['a', 'b'].map(up).join(sep),",
    ".x1xcwzc{content:\"A-B\"}",
  );
}

// ──────────────────────────────────────────────
// Declarations the transport cannot take
// ──────────────────────────────────────────────

/// A block body is the shape the evaluator answers no callback for, and the
/// reference compiler refuses it too. Written out in place the same body folds,
/// which is the one asymmetry this ticket did not close.
#[test]
fn a_block_bodied_declaration_names_the_binding() {
  assert_refuses(
    "const upper = (part) => { return part.toUpperCase(); };",
    "content: ['b', 'a'].map(upper).join(', '),",
    "Cannot carry the function 'upper' into a fold.",
  );
}

/// A destructured parameter, refused on both sides for the same reason a block
/// body is.
#[test]
fn a_destructured_parameter_names_the_binding() {
  assert_refuses(
    "const first = ([a]) => a;",
    "content: [['x', 'y'], ['p', 'q']].map(first).join(','),",
    "Cannot carry the function 'first' into a fold.",
  );
}

/// A defaulted parameter, likewise.
#[test]
fn a_defaulted_parameter_names_the_binding() {
  assert_refuses(
    "const tag = (p, s = '!') => p + s;",
    "content: ['b', 'a'].map(tag).join(','),",
    "Cannot carry the function 'tag' into a fold.",
  );
}

/// A rest parameter, likewise.
#[test]
fn a_rest_parameter_names_the_binding() {
  assert_refuses(
    "const firstOf = (...xs) => xs[0];",
    "content: ['b', 'a'].map(firstOf).join(','),",
    "Cannot carry the function 'firstOf' into a fold.",
  );
}

/// A `function` declaration is hoisted and has no declarator to read an
/// initializer from, so it is named through the declaration list instead.
#[test]
fn a_function_declaration_names_the_binding() {
  assert_refuses(
    "function upper(part) { return part.toUpperCase(); }",
    "content: ['b', 'a'].map(upper).join(', '),",
    "Cannot carry the function 'upper' into a fold.",
  );
}

/// A function expression has a declarator and an initializer, and is refused
/// for its spelling rather than for where it sits.
#[test]
fn a_function_expression_names_the_binding() {
  assert_refuses(
    "const upper = function (part) { return part.toUpperCase(); };",
    "content: ['b', 'a'].map(upper).join(', '),",
    "Cannot carry the function 'upper' into a fold.",
  );
}

/// A binding written to after it was declared holds something the initializer
/// no longer describes, which is the resolution's answer rather than the
/// transport's.
#[test]
fn a_reassigned_binding_names_the_binding() {
  assert_refuses(
    "let upper = (p) => p.toUpperCase(); upper = (p) => p;",
    "content: ['b', 'a'].map(upper).join(','),",
    "Cannot carry the function 'upper' into a fold.",
  );
}

/// A binding mutated in place, for the same reason.
#[test]
fn a_mutated_binding_names_the_binding() {
  assert_refuses(
    "const upper = (p) => p.toUpperCase(); upper.tag = 1;",
    "content: ['b', 'a'].map(upper).join(','),",
    "Cannot carry the function 'upper' into a fold.",
  );
}

/// The *sentence* names the declaration rather than the call, because the call
/// is fine and the method is not what the author has to change. The code frame
/// still spans the call, which is where the expression is: the frame follows the
/// node being evaluated, and nothing here moves it.
#[test]
fn the_refusal_names_the_declaration_rather_than_the_method() {
  assert_refuses(
    "const upper = (part) => { return part.toUpperCase(); };",
    "content: ['b', 'a'].map(upper).join(', '),",
    "Its declaration is not one the compiler can evaluate.",
  );
}

// ──────────────────────────────────────────────
// Rules that survive inside a declaration the walk reached
// ──────────────────────────────────────────────

/// A declaration is walked by the same guard everything else is, so a read that
/// escapes onto the function graph is refused inside one.
#[test]
fn an_escaping_read_inside_a_declaration_still_refuses() {
  assert_refuses(
    "const f = (x) => x.constructor;",
    "content: ['a'].map(f).join(''),",
    "Cannot fold a read of 'constructor' at compile time.",
  );
}

/// A locale-sensitive method, likewise — the answer would depend on data the
/// engine does not carry wherever it was written.
#[test]
fn a_locale_sensitive_method_inside_a_declaration_still_refuses() {
  assert_refuses(
    "const f = (x) => x.toLocaleUpperCase();",
    "content: ['a'].map(f).join(''),",
    "Cannot fold 'toLocaleUpperCase' at compile time.",
  );
}

/// The body of a declaration reached as a callback is a callback body, so the
/// rule that a length written into one bounds a single evaluation applies to it.
#[test]
fn an_amplifying_call_inside_a_declaration_still_refuses() {
  assert_refuses(
    "const big = (x) => x.repeat(3);",
    "content: ['a', 'b'].map(big).join('-'),",
    "Cannot bound the string 'repeat' would build inside a callback.",
  );
}

/// A chain long enough to reach the configured depth refuses there rather than
/// running out of stack, which is what walking a declaration one level in buys.
#[test]
fn an_alias_chain_past_the_configured_depth_refuses() {
  let mut declarations = String::from("const a0 = (x) => x.toUpperCase();");

  for step in 1..=200 {
    declarations.push_str(&format!(" const a{step} = a{};", step - 1));
  }

  assert_refuses(
    &declarations,
    "content: ['b'].map(a200).join(''),",
    "Expression is too deeply nested to evaluate at compile time.",
  );
}

/// Two declarations naming each other: the first reads the second above its own
/// declaration, which the resolution refuses before the walk can go round.
///
/// The general sentence rather than the one naming a binding, and correctly so:
/// what the walk reached was a *call* on a name, which is a shape the guard does
/// not admit at all — so the call is handed back rather than refused. Issue 22
/// carries that.
#[test]
fn two_declarations_naming_each_other_refuse() {
  assert_refuses(
    "const a = (x) => b(x); const b = (x) => a(x);",
    "content: ['q'].map(a).join(''),",
    "Cannot fold 'map' at compile time.",
  );
}

/// A declaration reading a name declared below it, which both compilers refuse
/// for the read being early.
///
/// The general sentence again, and for the neighbouring reason: the name that
/// stopped the walk is `tail`, which holds a string rather than a function, so
/// there is no binding for a function's sentence to name.
#[test]
fn a_declaration_reading_a_later_name_refuses() {
  assert_refuses(
    "const up = (x) => x.toUpperCase() + tail; const tail = '!';",
    "content: ['a'].map(up).join(''),",
    "Cannot fold 'map' at compile time.",
  );
}

// ──────────────────────────────────────────────
// The name the module declares nothing for
// ──────────────────────────────────────────────

/// A dynamic style's own parameter, which is the third answer: not folded, and
/// not refused either.
///
/// Nothing the module declares says what the parameter holds, so the fold hands
/// the call back and the dispatch below leaves it for the runtime — where a
/// refusal would have failed a build over a call that was always going to run
/// there. Measured on `@stylexjs/babel-plugin` 0.19.0, which emits the same
/// class and the same custom property.
#[test]
fn a_parameter_holding_the_callback_is_left_for_the_runtime() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (f) => ({ content: ['b', 'a'].map(f).join(',') }),
      });
    "#,
  );

  assert!(
    output.contains(".x1p70blb{content:var(--x-content)}"),
    "expected the declaration to compile to a custom property, got:\n{}",
    output
  );
  assert!(
    output.contains("].map(f).join(',')"),
    "expected the call to survive into the runtime function, got:\n{}",
    output
  );
}
