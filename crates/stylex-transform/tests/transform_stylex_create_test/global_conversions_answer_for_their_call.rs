//! What an applied global answers for: the name that is actually in scope, and
//! every argument it was handed.
//!
//! `String`, `Number`, `Object`, `Array` and `Math` are folded by being called
//! rather than by a table of conversions, so two questions decide whether the
//! fold owns a call at all. The first is whose name it is — a module that
//! declares one of these names has written its own function, and folding there
//! would emit a rule the other compiler never writes. The second is how much of
//! the call the conversion behind the fold answers for: three of the four
//! conversions ignore every argument past the first, and `Array` is the one
//! where each argument is an element.
//!
//! The two questions go opposite ways for a *receiver*, and deliberately — see
//! the ruling at the end of this file.
//!
//! Every case below was measured against `@stylexjs/babel-plugin` 0.19.0 and
//! carries the class name or the sentence it produced.

use crate::utils::transform::{assert_folds, assert_refuses};

/// The five names the fold owns: the four conversions plus `Math`, which is a
/// receiver rather than a function and is shadowed by the same rule.
const OWNED_NAMES: [&str; 5] = ["String", "Number", "Object", "Array", "Math"];

/// A call applying `name`, written as a style value.
fn applied(name: &str) -> String {
  format!("color: {}(1)", name)
}

// ──────────────────────────────────────────────
// The name that is in scope
// ──────────────────────────────────────────────

// A hoisted `function` is the module's own binding, so the call is the author's
// function and is refused for the declaration it names — upstream's own
// `Unsupported expression: FunctionDeclaration`, on all five names.
#[test]
fn a_function_declaration_shadows_every_global_the_fold_owns() {
  for name in OWNED_NAMES {
    assert_refuses(
      &format!("function {}(x) {{ return 'no'; }}", name),
      &applied(name),
      "Unsupported expression: FunctionDeclaration",
    );
  }
}

// The same for a `class`, which binds the name the same way and is refused for
// its own declaration kind upstream.
#[test]
fn a_class_declaration_shadows_every_global_the_fold_owns() {
  for name in OWNED_NAMES {
    assert_refuses(
      &format!("class {} {{}}", name),
      &applied(name),
      "Unsupported expression: ClassDeclaration",
    );
  }
}

// An import binds the name too, and what it holds is in another file — so both
// compilers refuse for the file, in the same sentence: `Could not resolve the
// path to the imported file.`
#[test]
fn an_import_binding_shadows_every_global_the_fold_owns() {
  for name in OWNED_NAMES {
    assert_refuses(
      &format!("import {{ {} }} from './helpers';", name),
      &applied(name),
      "Could not resolve the path to the imported file.",
    );
  }
}

// A binding written to after it was declared is refused for the write, exactly
// as upstream refuses it — the declaration is no longer a sound stand-in at the
// use site, whatever name it happens to have.
#[test]
fn a_reassigned_binding_shadows_every_global_the_fold_owns() {
  for name in OWNED_NAMES {
    assert_refuses(
      &format!("let {} = () => 'a'; {} = () => 'b';", name, name),
      &applied(name),
      "Referenced value is not a constant.",
    );
  }
}

// An arrow the module bound is a function this compiler can call, so the shadow
// is honoured by *calling it* rather than by refusing: `.x1be0z9o{color:no}`
// upstream and here.
#[test]
fn an_arrow_binding_is_called_as_the_authors_own_function() {
  assert_folds(
    "const String = (x) => 'no';",
    "color: String(1)",
    "color:no",
  );
}

// Nothing declared the name, so it is the global and the conversion answers:
// `.xrkmrrc{color:red}` is what `String('red')` folds to.
#[test]
fn an_unbound_name_is_still_the_global() {
  assert_folds("", "color: String('red')", "color:red");
}

// ──────────────────────────────────────────────
// Every argument the call was handed
// ──────────────────────────────────────────────

// A token group has no JavaScript form to cross the bridge as, so the whole call
// is handed back and the conversion behind the fold answers it. `Array` is the
// conversion with no surplus: each argument is an element, and a style array is
// a fallback list — so dropping the arguments past the first drops the fallback.
// Measured: `.x18w8fj9{color:var(--xa513j);color:blue}`.
#[test]
fn an_applied_array_keeps_every_argument_across_a_hand_back() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "color: Array(colors.primary, 'blue')",
    "color:var(--xa513j);color:blue",
  );
}

// Three of them, so the fallback chain is built from every argument rather than
// from the first two: `.x3pokgw{color:var(--x1upuatx,var(--xa513j));color:blue}`.
#[test]
fn an_applied_array_keeps_more_than_two_arguments() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "color: Array(colors.primary, colors.surface, 'blue')",
    "color:blue",
  );
}

// One argument is still one element, which is the reading a surplus rule would
// have hidden: `.x1qfnmnr{color:var(--xa513j)}`.
#[test]
fn an_applied_array_of_one_argument_is_one_element() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "color: Array(colors.primary)",
    "color:var(--xa513j)",
  );
}

// The other three conversions do ignore their surplus, and are measured on the
// same hand-back so the difference is the conversion rather than the path:
// `String` takes the first argument alone — `.xnwtydd{color:xfv597z}`, the
// variable-group hash the group's own `toString` answers.
#[test]
fn the_other_conversions_ignore_the_arguments_past_the_first() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "color: String(colors, 'blue')",
    "color:xfv597z",
  );
}

// `Array(n)` is a length rather than an element, and stays one: the guard reads
// the length in front of the engine, so a lone number never reaches the
// conversion behind the fold. `.x1sn2kax{content:"2px"}` is `Array(2).length`.
#[test]
fn an_applied_array_of_one_number_is_still_a_length() {
  assert_folds("", "content: Array(2).length", r#"content:"2px""#);
}

// ──────────────────────────────────────────────
// The receiver, which is ruled on the other way
// ──────────────────────────────────────────────

// A `Math` receiver reads no value across the bridge — the printed source names
// it and the language answers — so a declaration of the name changes nothing
// about the static that folds. Upstream ignores the shadow here as well:
// `.xfo62xy{width:2px}` under `function Math() {}`.
#[test]
fn a_function_declaration_does_not_shadow_a_static_receiver() {
  assert_folds("function Math() {}", "width: Math.max(1, 2)", "width:2px");
}

// **The ruling.** A receiver the module bound to a value of its own is refused
// here where upstream folds it: `const Math = { trunc: () => 9 };
// Math.trunc(1.5)` is `1px` upstream, which reads the shadow's *name* and the
// global's *method*. That is a bug there rather than a rule worth matching — the
// answer belongs to neither the author's object nor the language — and refusing
// is the safe direction, since a refusal leaves the call to the runtime where a
// wrong fold writes a wrong declaration.
//
// It goes the opposite way from the callee rule above for one reason: folding a
// shadowed *callee* produces output where the other compiler produces none, and
// folding a shadowed *receiver* is what the other compiler already does. The
// dangerous direction is the one that invents a class name, and only the callee
// rule can.
#[test]
fn a_declared_receiver_refuses_rather_than_reading_through_the_shadow() {
  assert_refuses(
    "const Math = { trunc: () => 9 };",
    "width: Math.trunc(1.5)",
    "Cannot fold 'trunc' at compile time.",
  );
}

// ──────────────────────────────────────────────
// The edges of both rules
// ──────────────────────────────────────────────

// Parentheses change nothing about which name is written, so the shadow is
// honoured however many of them wrap it — and it used to fold here. Both
// compilers refuse; the sentences differ because the dispatch below the fold
// reads a callee written as a bare name and a parenthesised one reaches its
// catch-all, where upstream names the declaration. A refusal either way is what
// the rule is for, and message text is not a parity obligation (`ADR 0008`).
#[test]
fn parentheses_do_not_hide_a_shadowed_callee() {
  assert_refuses(
    "function String(x) { return 'no'; }",
    "color: (((String)))(1)",
    "Unsupported expression: CallExpression",
  );
}

// A dynamic style's parameter binds the name for the body under it, and holds no
// compile-time value — so the call is left for the runtime rather than folded,
// which is what upstream emits too.
#[test]
fn a_dynamic_style_parameter_shadows_the_conversion() {
  let output = crate::utils::transform::fold_module(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (String) => ({ color: String(1) }),
      });
    "#,
  );

  assert!(
    output.contains("String(1)"),
    "expected the call to survive into the emitted function, got:\n{}",
    output
  );
}

// Ten arguments, so "every argument" is not two: the join reads them all in
// order — `.x1ca8ygt{content:"xfv597z-a-b-c-d-e-f-g-h-i"}`.
#[test]
fn an_applied_array_keeps_a_long_argument_list() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "content: Array(colors, 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i').join('-')",
    r#"content:"xfv597z-a-b-c-d-e-f-g-h-i""#,
  );
}

// The length is the argument count rather than the first argument, on the same
// hand-back: `.xblpyw3{content:"4px"}`.
#[test]
fn an_applied_array_counts_every_argument_as_an_element() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "content: Array(colors, 1, 2, 3).length",
    r#"content:"4px""#,
  );
}

// `null` and `undefined` are elements like any other and join as nothing, so the
// argument count still shows in the separators —
// `.x1ib8hzf{content:"xfv597z--b"}` for either spelling.
#[test]
fn an_applied_array_keeps_a_nullish_argument_as_an_element() {
  for nullish in ["undefined", "null"] {
    assert_folds(
      "import { colors } from 'colors.stylex.js';",
      &format!("content: Array(colors, {}, 'b').join('-')", nullish),
      r#"content:"xfv597z--b""#,
    );
  }
}

// `NaN` is a value the fold carries rather than a refusal, here as everywhere
// else: `.x168mcy0{content:"xfv597z-NaN"}`.
#[test]
fn an_applied_array_keeps_a_nan_argument() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "content: Array(colors, NaN).join('-')",
    r#"content:"xfv597z-NaN""#,
  );
}

// A spread is refused where it is written, before any argument is read — the
// argument list is unknowable without the operand's length. Upstream refuses the
// same source, for the call rather than for the spread.
#[test]
fn an_applied_array_refuses_a_spread_argument() {
  assert_refuses(
    "import { colors } from 'colors.stylex.js';",
    "content: Array(...[colors, 'a']).join('-')",
    "Unsupported expression: SpreadElement",
  );
}

// A declarator holding something other than an object is the same ruling as the
// object above and refuses for the same reason: upstream folds
// `Math.trunc(1.5)` here too, reading the global's method off a name the module
// gave a string.
#[test]
fn a_declared_receiver_refuses_whatever_the_declarator_holds() {
  assert_refuses(
    "const Math = 'x';",
    "content: Math.trunc(1.5)",
    "Cannot fold 'trunc' at compile time.",
  );
}

// A number **does** reach the conversion behind the fold, because the hand-back
// is decided by the whole expression rather than by one argument: an array hole
// elsewhere in the call declines it, and the number arrives with no ceiling ever
// applied to the length it declares. Refused rather than read as an element,
// which is what it used to become — `Array([, 1].length).length` answered `1`
// where JavaScript says `2`. The reference compiler refuses the same source,
// for its own reason: an array hole leaves its evaluator nothing to resolve.
#[test]
fn a_length_reaching_the_conversion_behind_the_fold_is_refused() {
  for read in ["length", "join('-')"] {
    assert_refuses(
      "",
      &format!("content: Array([, 1].length).{}", read),
      "Cannot bound the array 'Array' would build.",
    );
  }
}

// An argument that resolved to something other than a number is still an
// element, so the refusal above is about a length and not about the hand-back:
// `.x1fy28pd{content:"1px"}`, which is the array of one the reference compiler
// builds here too.
#[test]
fn an_argument_that_is_not_a_number_is_still_one_element() {
  assert_folds(
    "import { colors } from 'colors.stylex.js';",
    "content: Array(colors.primary.length).length",
    r#"content:"1px""#,
  );
}

// An import never shadows a *receiver*, which is the receiver rule read on the
// binding kind that has no declarator at all — and the reference compiler folds
// it too: `.x1i1rx1s{width:1px}`, the language's `Math.trunc`, whatever the
// import holds.
#[test]
fn an_import_does_not_shadow_a_static_receiver() {
  assert_folds(
    "import { Math } from './helpers';",
    "width: Math.trunc(1.5)",
    "width:1px",
  );
}
