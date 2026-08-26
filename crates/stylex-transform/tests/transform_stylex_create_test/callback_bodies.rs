//! A callback is a JavaScript function, not a shape the fold guard has to
//! recognise.
//!
//! The guard used to admit a callback only where it was an arrow whose body was
//! one expression reading nothing but its own parameters. Adding a statement,
//! destructuring a parameter or closing over a named value broke the build —
//! none of which is hard for an engine, and all of which the engine now answers
//! because the arrow is printed into the transport and parsed there.
//!
//! What the guard still does is name what the arrow binds, so a read of one of
//! those names is not asked of the module, and apply to the body the same rules
//! every other position gets: a callback body is source that really runs.
//!
//! The reference compiler refuses most of these shapes outright, so its build
//! fails where this one folds — there is no output of its to disagree with, and
//! nothing a class name could name differently. Each expected rule below is
//! therefore measured from `@stylexjs/babel-plugin` 0.19.0 on the *value* the
//! fold produces written out as a literal, which is what fixes the class name.

use crate::utils::transform::{assert_folds, assert_refuses, fold_module as fold};

// ──────────────────────────────────────────────
// Bodies the guard used to have to recognise
// ──────────────────────────────────────────────

/// A block body, which is the shape the old guard refused first: statements
/// bind and return, and neither was modelled.
#[test]
fn a_callback_with_a_block_body_folds() {
  assert_folds(
    "const a = [1, 2, 3];",
    "padding: a.map(n => { const v = n * 2; return `${v}px`; }).join(' '),",
    ".xc7ltx4{padding:2px 4px 6px}",
  );
}

/// A block body whose branches return different values, so the fold depends on
/// a statement the guard never reads the meaning of.
#[test]
fn a_callback_branching_inside_its_body_folds() {
  assert_folds(
    "const a = [1, 2];",
    "content: a.map(n => { if (n > 1) { return 'big'; } return 'small'; }).join('-'),",
    ".xbhhg99{content:\"small-big\"}",
  );
}

/// A declaration in a nested block belongs to that block, so the name it binds
/// is the one the body reads rather than a module name of the same spelling.
#[test]
fn a_nested_block_binds_its_own_names() {
  assert_folds(
    "const v = 'px'; const a = [1];",
    "content: a.map(n => { { const v = 'em'; return n + v; } }).join(''),",
    ".xn9gf3i{content:\"1em\"}",
  );
}

/// An empty statement carries nothing and stops nothing.
#[test]
fn a_stray_semicolon_in_a_body_folds() {
  assert_folds(
    "const a = ['a', 'b'];",
    "content: a.map(x => { ; return x; }).join(''),",
    ".xarbti{content:\"ab\"}",
  );
}

// ──────────────────────────────────────────────
// Parameters the guard used to have to recognise
// ──────────────────────────────────────────────

/// An object pattern, which binds by key rather than by position.
#[test]
fn a_callback_destructuring_an_object_parameter_folds() {
  assert_folds(
    "const a = [{ v: 1 }, { v: 2 }];",
    "padding: a.map(({ v }) => v + 'px').join(' '),",
    ".xpwwz5d{padding:1px 2px}",
  );
}

/// An array pattern, which binds by position.
#[test]
fn a_callback_destructuring_an_array_parameter_folds() {
  assert_folds(
    "const a = [[1, 'px'], [2, 'px']];",
    "padding: a.map(([n, u]) => n + u).join(' '),",
    ".xpwwz5d{padding:1px 2px}",
  );
}

/// A default inside a pattern, which is an expression the pattern evaluates
/// where it is written rather than a name it binds.
#[test]
fn a_callback_parameter_default_folds() {
  assert_folds(
    "const a = [[1], [2]];",
    "padding: a.map(([n, u = 'px']) => n + u).join(' '),",
    ".xpwwz5d{padding:1px 2px}",
  );
}

/// A rest element, which binds one name to everything the pattern did not take.
#[test]
fn a_callback_rest_parameter_folds() {
  assert_folds(
    "const a = [[1, 2, 3]];",
    "zIndex: a.map(([f, ...r]) => f + r.length)[0],",
    ".xzkaem6{z-index:3}",
  );
}

/// A computed key in a pattern reads a module name, so the pattern itself
/// carries a value across the bridge.
#[test]
fn a_computed_key_in_a_pattern_resolves_from_the_module() {
  assert_folds(
    "const k = 'v'; const a = [{ v: 1 }];",
    "zIndex: a.map(({ [k]: q }) => q)[0],",
    ".x1vjfegm{z-index:1}",
  );
}

/// An object rest, which binds the properties the pattern did not name.
#[test]
fn a_callback_object_rest_parameter_folds() {
  assert_folds(
    "const a = [{ v: 1, w: 2 }];",
    "zIndex: a.map(({ v, ...rest }) => v + Object.keys(rest).length)[0],",
    ".xhtitgo{z-index:2}",
  );
}

// ──────────────────────────────────────────────
// What a callback may read
// ──────────────────────────────────────────────

/// A name from the surrounding module, which the guard resolves and carries as
/// a parameter of the printed arrow exactly as it does outside a callback.
#[test]
fn a_callback_closing_over_a_named_value_folds() {
  assert_folds(
    "const unit = 'px'; const a = [1, 2, 3];",
    "padding: a.map(n => `${n}${unit}`).join(' '),",
    ".x18ds20s{padding:1px 2px 3px}",
  );
}

/// A name the callback declares shadows a module name of the same spelling, as
/// it does in the module the author wrote.
#[test]
fn a_callback_declaration_shadows_a_module_name() {
  assert_folds(
    "const unit = 'px'; const a = [1];",
    "content: a.map(n => { const unit = 'em'; return n + unit; }).join(''),",
    ".xn9gf3i{content:\"1em\"}",
  );
}

/// The index and the whole array, which are the second and third arguments the
/// language passes a callback.
#[test]
fn a_callback_reads_the_index_and_the_array() {
  assert_folds(
    "const a = ['b', 'a'];",
    "content: a.map((x, i, all) => x + i + all.length).join(''),",
    ".x74pj4i{content:\"b02a12\"}",
  );
}

/// An inner callback reads the outer callback's parameter, so scope is a chain
/// rather than one set of names that replaces the last.
#[test]
fn an_inner_callback_reads_an_outer_parameter() {
  assert_folds(
    "const a = ['a']; const b = ['b'];",
    "content: a.map(x => b.map(y => x + y).join('')).join(''),",
    ".xarbti{content:\"ab\"}",
  );
}

// ──────────────────────────────────────────────
// What a callback still does not make foldable
// ──────────────────────────────────────────────

/// A read that leads onto the language's function graph is refused inside a
/// callback body for the reason it is refused anywhere: the body is source that
/// really runs.
#[test]
fn an_escaping_read_inside_a_body_refuses() {
  assert_refuses(
    "const a = ['x'];",
    "content: a.map(n => { const c = n.constructor; return 'q'; }).join(''),",
    "Cannot fold a read of 'constructor' at compile time.",
  );
}

/// A length-amplifying call is refused inside a block body as it is inside an
/// expression body: a bound written once is multiplied by an element count the
/// source never states.
#[test]
fn an_amplifying_call_inside_a_body_refuses() {
  assert_refuses(
    "const a = [1];",
    "content: a.map(n => { const s = 'x'.repeat(3); return s; }).join(''),",
    "Cannot bound the string 'repeat' would build inside a callback.",
  );
}

/// A statement outside the admitted set is a boundary this fold owns, so it
/// names the statement the author wrote rather than the callback it sat in.
///
/// The engine's loop-iteration count lives on the call frame, so a callback
/// invoked once per element starts a fresh count every time and the bound is
/// multiplied by an element count the source never states.
/// The rest declare a function, a class or a control flow the fold does not
/// read. The reference compiler refuses all four too.
#[test]
fn a_statement_outside_the_admitted_set_names_itself() {
  let cases: &[(&str, &str)] = &[
    (
      "content: a.map(n => { let s = ''; for (let i = 0; i < n; i++) { s += 'x'; } return s; }).join(''),",
      "ForStatement",
    ),
    (
      "content: a.map(n => { function f(x) { return x + 1; } return f(n); }).join(''),",
      "FunctionDeclaration",
    ),
    (
      "content: a.map(n => { switch (n) { default: return 'x'; } }).join(''),",
      "SwitchStatement",
    ),
    (
      "content: a.map(n => { try { return 'x'; } catch (e) { return 'y'; } }).join(''),",
      "TryStatement",
    ),
  ];

  for (body, kind) in cases {
    assert_refuses(
      "const a = [1];",
      body,
      &format!("Cannot fold a callback whose body uses a {}.", kind),
    );
  }
}

/// An assignment is an expression, not a statement, so it is answered where
/// every unmodelled expression is: the value walk does not read it, the call is
/// not this fold's, and the dispatch below names it. The reference compiler
/// refuses it too.
#[test]
fn an_assignment_in_a_body_is_not_this_fold_s_call() {
  assert_refuses(
    "const a = [1];",
    "content: a.map(n => { let v; v = n + 1; return v; }).join(''),",
    "Unsupported expression: ArrowFunctionExpression",
  );
}

/// A name no scope binds and the module cannot resolve is not a value the
/// bridge can carry, so the fold is not attempted and the dispatch below names
/// the call it could not answer — rather than the fold guessing a value for a
/// name it never read.
#[test]
fn a_callback_reading_an_unresolvable_name_refuses() {
  assert_refuses(
    "const a = [1, 2];",
    "content: a.map(n => n + missing).join(''),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

// ──────────────────────────────────────────────
// The edges of the body walk
// ──────────────────────────────────────────────

/// A body that returns nothing answers `undefined`, which is what the language
/// answers and what a `join` then spells as an empty string.
#[test]
fn a_body_that_returns_nothing_folds_to_the_language_s_answer() {
  assert_folds(
    "const a = ['x', 'y'];",
    "content: a.map(x => { return; }).join('-'),",
    ".x1p0vxes{content:\"-\"}",
  );
}

/// A hole in an array pattern binds nothing and skips the element the language
/// skips.
#[test]
fn a_hole_in_a_pattern_skips_its_element() {
  assert_folds(
    "const a = [['skip', 'take']];",
    "content: a.map(([, second]) => second).join(''),",
    ".xccl9c7{content:\"take\"}",
  );
}

/// A default inside a pattern may itself read a module name, which is a value
/// the bridge carries in beside the receiver.
#[test]
fn a_pattern_default_may_read_a_module_name() {
  assert_folds(
    "const unit = 'px'; const a = [[1]];",
    "content: a.map(([n, u = unit]) => n + u).join(''),",
    ".x1fy28pd{content:\"1px\"}",
  );
}

/// A pattern that assigns through a member binds no name, so there is nothing
/// to put in scope and the call is not this module's.
#[test]
fn a_pattern_assigning_through_a_member_refuses() {
  assert_refuses(
    "const o = {}; const a = [[1]];",
    "content: a.map(([o.v]) => 'x').join(''),",
    "Unsupported expression: ArrowFunctionExpression",
  );
}

/// Nesting inside a body spends the same budget the rest of the walk spends, so
/// a body nested past it is refused with the sentence every other depth refusal
/// carries rather than overflowing the engine parser's stack.
#[test]
fn a_body_nested_past_the_budget_refuses() {
  let body = format!(
    "content: a.map(n => {{ {} return 'x'; {} }}).join(''),",
    "{ ".repeat(40),
    "} ".repeat(40)
  );

  assert_refuses(
    "const a = [1];",
    &body,
    "Expression is too deeply nested to evaluate at compile time.",
  );
}

/// A body of many statements is walked, not counted: nothing here bounds how
/// long a callback may be, only how deeply it nests.
#[test]
fn a_long_body_folds() {
  let steps = (0..200)
    .map(|step| format!("const v{} = {};", step, step))
    .collect::<Vec<_>>()
    .join(" ");

  assert_folds(
    "const a = [1];",
    &format!("zIndex: a.map(n => {{ {} return v199 + n; }})[0],", steps),
    ".x8k05lb{z-index:200}",
  );
}

/// A dynamic style function keeps the closure path it has always taken: its
/// parameter has no compile-time value, so nothing about it reaches the fold.
#[test]
fn a_dynamic_style_function_is_untouched() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const sizes = [1, 2];
      export const styles = stylex.create({
        base: (multiplier) => ({
          padding: sizes.map(n => n + 'px').join(' '),
          margin: multiplier * 2,
        }),
      });
    "#,
  );

  assert!(
    output.contains("padding:1px 2px"),
    "expected the folded declaration beside a dynamic one, got:\n{}",
    output
  );
  assert!(
    output.contains("var(--x-margin)"),
    "expected the dynamic declaration to stay a runtime value, got:\n{}",
    output
  );
}

/// A callback reaching a StyleX function is not the engine's to run — the
/// function lives in this compiler's injected map, not in the language — so the
/// fold hands the call back. Nothing below answers it either: the array method
/// implementations were deleted when the prototype moved into the engine, and
/// carrying the function map inward was measured and rejected, because its
/// values are placeholders the engine would throw on.
///
/// So this refuses where the reference compiler folds it to `serif,a,serif,b`.
/// The refusal is a failed build rather than a different class name, so the two
/// compilers do not disagree about any output — one of them has none. Tracked
/// as issue 17 of this effort.
#[test]
fn a_callback_reaching_a_stylex_function_refuses() {
  assert_refuses(
    "import { firstThatWorks } from '@stylexjs/stylex'; const a = ['a', 'b'];",
    "fontFamily: a.map(x => firstThatWorks(x, 'serif')).join(','),",
    "Its receiver or one of its arguments is not in a form the compiler can evaluate.",
  );
}

/// A prototype read through a pattern is the one place `__proto__` is a
/// property and not a key. It folds to what the language answers, and the reads
/// that lead off the prototype and onto the function graph are refused there as
/// they are anywhere.
#[test]
fn a_prototype_read_through_a_pattern_stops_at_the_escaping_rule() {
  assert_folds(
    "const a = [[1]];",
    "content: a.map(({ __proto__: p }) => typeof p).join(''),",
    ".xm7l0um{content:\"object\"}",
  );

  assert_refuses(
    "const a = ['x'];",
    "content: a.map(({ __proto__: p }) => p.constructor)[0],",
    "Cannot fold a read of 'constructor' at compile time.",
  );
}

/// A function expression is not an arrow, so it is not a shape this fold reads
/// at all — as it is not for the reference compiler, which refuses it too.
#[test]
fn a_function_expression_callback_refuses() {
  assert_refuses(
    "const a = ['x'];",
    "content: a.map(function (x) { return x; }).join(''),",
    "Unsupported expression: FunctionExpression",
  );
}

/// A mutating method inside a callback mutates a copy the bridge built, which
/// nothing outside the fold can name — so it folds where the same method on a
/// binding does not.
#[test]
fn a_mutating_method_on_a_callback_parameter_folds() {
  assert_folds(
    "const a = [['b', 'a']];",
    "content: a.map(inner => inner.sort().join('')).join(''),",
    ".xarbti{content:\"ab\"}",
  );
}

/// A destructuring parameter is printed into the transport with the arrow it
/// belongs to, so it spends the same nesting budget every other walk on this
/// bridge spends — a pattern nested past it is refused rather than handed to the
/// engine's parser to descend.
#[test]
fn a_pattern_nested_past_the_budget_refuses() {
  let body = format!(
    "content: a.map(({}v{}) => 'x').join(''),",
    "[".repeat(40),
    "]".repeat(40)
  );

  assert_refuses(
    "const a = [1];",
    &body,
    "Expression is too deeply nested to evaluate at compile time.",
  );
}
