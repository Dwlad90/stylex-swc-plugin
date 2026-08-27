//! Calling a function through a name, as passing one already folds.
//!
//! A function reached by name crosses as the declaration it came from, and that
//! carriage is what made a *passed* callback fold. Calling one stayed refused
//! for a reason of dispatch rather than of carriage: a call was admitted only
//! where its callee was a member expression or an unshadowed global, so nothing
//! about the function was unreachable — the call on it was simply never asked
//! about.
//!
//! What decides it now is where the call sits, and the line is the one a StyleX
//! function inside a fold already drew. **The outermost call stays the
//! dispatch's**: below the fold a call through a name is resolved this
//! compiler's own way, which is where a dynamic style's own parameters and the
//! injected function map are answered, and the engine holds no value for either.
//! Measured, that path already folds `inner('a')` to the same rule upstream
//! emits, so taking the call would replace a working answer with a narrower one.
//! **A call inside an expression the fold claimed has no second answer**: handing
//! one back hands back the whole expression around it, and the method that would
//! have re-run a callback body moved into the engine.
//!
//! Which declarations qualify is unchanged, because the carriage is unchanged —
//! an arrow with plain parameters and a single expression body, never written to
//! after it was declared. The refusals below name the binding for the same
//! reason a passed callback's do: the call is fine and the declaration is what
//! an author has to change.
//!
//! Every case below was measured against `@stylexjs/babel-plugin` 0.19.0 and
//! carries the class name it produced. The cases that diverge say which way and
//! why.

use crate::utils::transform::{assert_folds, assert_refuses, fold_module as fold};

/// The one-argument arrow most cases below call.
const INNER: &str = "const inner = (y) => y + '!';";

// ──────────────────────────────────────────────
// The shapes the ticket is about
// ──────────────────────────────────────────────

/// The reported row: a callback body calling a function the module named.
#[test]
fn a_callback_calling_a_module_named_function_folds() {
  assert_folds(
    INNER,
    "content: ['b', 'a'].map((x) => inner(x)).join(','),",
    ".xydq94n{content:\"b!,a!\"}",
  );
}

/// The shape that needs nothing to cross at all: the callee is a name the
/// callback itself binds, so the engine resolves it where it binds it.
#[test]
fn a_callback_calling_a_name_it_binds_folds() {
  assert_folds(
    INNER,
    "content: [inner].map((f) => f('q')).join(''),",
    ".x1si44xw{content:\"q!\"}",
  );
}

/// A callee that is itself a fold: `make('!')` answers the arrow `map` then
/// runs, so the call and the callback are one expression the engine evaluates.
#[test]
fn a_callee_that_is_itself_a_fold_folds() {
  assert_folds(
    "const make = (s) => (x) => x + s;",
    "content: ['a'].map(make('!')).join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

// ──────────────────────────────────────────────
// Where the call sits
// ──────────────────────────────────────────────

/// The outermost call is handed back, and nothing is lost by it: the dispatch
/// below answers with the rule upstream emits. This is the measurement the
/// decision rests on, so it is asserted rather than described.
#[test]
fn the_outermost_named_call_is_answered_below_the_fold() {
  assert_folds(INNER, "content: inner('a'),", ".x1bt3ucs{content:\"a!\"}");
}

/// The same call one link inside a chain, which the fold does own.
#[test]
fn a_named_call_as_the_receiver_of_a_method_folds() {
  assert_folds(
    INNER,
    "content: inner('a').toUpperCase(),",
    ".xmy19su{content:\"A!\"}",
  );
}

/// A longer chain on the same receiver, so the call is a middle link rather
/// than the one the method reads directly.
#[test]
fn a_named_call_under_a_chain_folds() {
  assert_folds(
    INNER,
    "content: inner('a').split('').join('-'),",
    ".x10mpz3i{content:\"a-!\"}",
  );
}

/// Inside an argument rather than a receiver, which is the other position a
/// claimed expression reaches.
#[test]
fn a_named_call_beside_a_callback_parameter_folds() {
  assert_folds(
    INNER,
    "content: ['a'].map((x) => x + inner('z')).join(''),",
    ".xlbkr1s{content:\"az!\"}",
  );
}

/// A hole of a template, which is a value in its own right.
#[test]
fn a_named_call_in_a_template_hole_folds() {
  assert_folds(
    INNER,
    "content: [`${inner('a')}`].join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

/// A branch of a conditional, where only one side is ever evaluated.
#[test]
fn a_named_call_in_a_conditional_folds() {
  assert_folds(
    INNER,
    "content: [true ? inner('a') : 'b'].join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

/// A callback nested in a callback, so the call is reached through two scopes
/// neither of which binds the name.
#[test]
fn a_named_call_inside_a_nested_callback_folds() {
  assert_folds(
    INNER,
    "content: [['a']].map((row) => row.map((x) => inner(x)).join('')).join('|'),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

// ──────────────────────────────────────────────
// What the call is written as
// ──────────────────────────────────────────────

/// A parenthesised callee is the same name, and the guard reads through the
/// parens exactly as every other position on this bridge does.
#[test]
fn a_parenthesised_named_callee_folds() {
  assert_folds(
    INNER,
    "content: [(inner)('a')].join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

/// A global reads through parens too, which is what makes the sentence above
/// true of every callee rather than of a module binding only.
#[test]
fn a_parenthesised_global_callee_folds() {
  assert_folds(
    "",
    "content: [(String)('a')].join(''),",
    ".x16319ns{content:\"a\"}",
  );
}

/// And a global read as the receiver of a static, which is the other position
/// the same question is asked in.
#[test]
fn a_parenthesised_global_receiver_folds() {
  assert_folds(
    "",
    "width: [(Math).max(1, 2)].join('') + 'px',",
    ".xfo62xy{width:2px}",
  );
}

/// More than one argument, proving the call is applied rather than handed the
/// first argument alone.
#[test]
fn a_named_call_of_several_arguments_folds() {
  assert_folds(
    "const j = (a, b, c) => a + b + c;",
    "content: [j('1', '2', '3')].join(''),",
    ".xnbkil8{content:\"123\"}",
  );
}

/// No arguments at all, which is the call with nothing to walk.
#[test]
fn a_named_call_of_no_arguments_folds() {
  assert_folds(
    "const z = () => 'zz';",
    "content: [z()].join(''),",
    ".x12eua3f{content:\"zz\"}",
  );
}

/// An answer that is an array rather than a string, so the value crosses back
/// out as one and the method on it folds.
#[test]
fn a_named_call_answering_an_array_folds() {
  assert_folds(
    "const mk = (x) => [x, x];",
    "content: mk('a').join('-'),",
    ".x1vc6ejb{content:\"a-a\"}",
  );
}

/// An answer that is an object, read through a property.
#[test]
fn a_named_call_answering_an_object_folds() {
  assert_folds(
    "const mk = (x) => ({ v: x });",
    "content: [mk('a').v].join(''),",
    ".x16319ns{content:\"a\"}",
  );
}

/// A number rather than a string, so the arithmetic is the language's.
#[test]
fn a_named_call_answering_a_number_folds() {
  assert_folds(
    "const two = () => 2;",
    "width: [two() * 3].join('') + 'px',",
    ".x1v4s8kt{width:6px}",
  );
}

/// Three calls of the same name in one expression, each applied to the answer
/// of the one inside it.
#[test]
fn named_calls_nested_in_each_other_fold() {
  assert_folds(
    INNER,
    "content: [inner(inner(inner('a')))].join(''),",
    ".xg35vm4{content:\"a!!!\"}",
  );
}

// ──────────────────────────────────────────────
// Which name the call reaches
// ──────────────────────────────────────────────

/// A declaration naming a second function: the chain crosses as a chain, each
/// link the default of the parameter the link before it became.
#[test]
fn a_named_call_reaching_a_second_declaration_folds() {
  assert_folds(
    "const b = (x) => x + '2'; const a = (x) => b(x) + '1';",
    "content: a('q').toUpperCase(),",
    ".x90hz9p{content:\"Q21\"}",
  );
}

/// The same, called rather than chained, so the whole answer comes from the two
/// declarations alone.
#[test]
fn a_named_call_through_two_declarations_folds() {
  assert_folds(
    "const b = (x) => x + '2'; const a = (x) => b(x) + '1';",
    "content: [a('q')].join(''),",
    ".x178ci5m{content:\"q21\"}",
  );
}

/// A callback parameter shadows the module's function, and the engine reads the
/// parameter — the language's answer rather than this walk's.
#[test]
fn a_callback_parameter_shadows_the_module_function() {
  assert_folds(
    INNER,
    "content: ['a'].map((inner) => inner + '?').join(''),",
    ".x15phss4{content:\"a?\"}",
  );
}

/// A module binding spelled as a global is the module's own value, so the call
/// is the author's function and not `String`.
#[test]
fn a_function_named_after_a_global_folds_as_the_function() {
  assert_folds(
    "const String = (y) => y + '!';",
    "content: [String('a')].join(''),",
    ".x1bt3ucs{content:\"a!\"}",
  );
}

// ──────────────────────────────────────────────
// The declarations that do not qualify
// ──────────────────────────────────────────────

/// A block body is outside the set the transport carries, and the refusal names
/// the binding rather than the call around it.
#[test]
fn a_called_block_bodied_arrow_names_the_binding() {
  assert_refuses(
    "const inner = (y) => { return y + '!'; };",
    "content: [inner('a')].join(''),",
    "Cannot carry the function 'inner' into a fold.",
  );
}

/// A `function` declaration is hoisted and has no initializer to print.
#[test]
fn a_called_function_declaration_names_the_binding() {
  assert_refuses(
    "function inner(y) { return y + '!'; }",
    "content: [inner('a')].join(''),",
    "Cannot carry the function 'inner' into a fold.",
  );
}

/// A binding written to after it was declared is refused for the write, with no
/// position check — the read above the write is dead too.
#[test]
fn a_called_reassigned_binding_names_the_binding() {
  assert_refuses(
    "let inner = (y) => y + '!'; inner = (y) => y;",
    "content: [inner('a')].join(''),",
    "Cannot carry the function 'inner' into a fold.",
  );
}

/// A declaration naming a function declared after it: the second name has no
/// value yet, so the first cannot cross. Upstream refuses it too, in its own
/// words about a value used before declaration.
#[test]
fn a_called_declaration_reaching_forward_names_the_binding() {
  assert_refuses(
    "const a = (x) => b(x) + '1'; const b = (x) => x + '2';",
    "content: [a('q')].join(''),",
    "Cannot carry the function 'b' into a fold.",
  );
}

/// A declaration that names itself has no bottom, and the nesting budget is
/// what stops the walk going round. Upstream refuses it as well.
#[test]
fn a_called_recursive_declaration_runs_out_of_depth() {
  assert_refuses(
    "const r = (x) => x.length > 2 ? x : r(x + 'a');",
    "content: [r('q')].join(''),",
    "Expression is too deeply nested",
  );
}

// ──────────────────────────────────────────────
// Callees that are not a name
// ──────────────────────────────────────────────

/// An arrow applied where it is written is not a name, and both compilers leave
/// it alone.
#[test]
fn an_arrow_applied_in_place_is_not_a_candidate() {
  assert_refuses(
    "",
    "content: [((x) => x + 'z')('a')].join(''),",
    "Unsupported expression: CallExpression",
  );
}

/// A callee that is itself a call — the curried arrow applied twice — is not a
/// name either, and again both compilers refuse.
#[test]
fn a_callee_that_is_a_call_is_not_a_candidate() {
  assert_refuses(
    "const make = (s) => (x) => x + s;",
    "content: [make('!')('a')].join(''),",
    "Unsupported expression: CallExpression",
  );
}

// ──────────────────────────────────────────────
// What the call is handed, and what it builds
// ──────────────────────────────────────────────

/// A spread needs a scope the printed source does not carry, so it is the same
/// refusal every other position gives it. Upstream refuses it too.
#[test]
fn a_spread_argument_to_a_named_call_refuses() {
  assert_refuses(
    "const j = (a, b) => a + b; const xs = ['1', '2'];",
    "content: [j(...xs)].join(''),",
    "Unsupported expression: SpreadElement",
  );
}

/// A name holding something that is not a function reaches the engine and the
/// language answers, which is the sentence an author gets. Upstream refuses it
/// too, in words of its own.
#[test]
fn calling_a_name_that_holds_a_string_reads_the_engine() {
  assert_refuses(
    "const s = 'a';",
    "content: [s('x')].join(''),",
    "not a callable function",
  );
}

/// A name the module declares nothing for is handed back rather than refused,
/// so the dispatch below answers — which is what a dynamic style's own
/// parameter depends on.
#[test]
fn calling_a_name_the_module_never_bound_is_handed_back() {
  assert_refuses(
    "",
    "content: [nope('x')].join(''),",
    "Referenced constant is not defined.",
  );
}

/// The declaration runs once per evaluation of the call, so a length written
/// into it is bounded like any other rather than refused as a body over a
/// receiver nothing measured.
#[test]
fn a_length_inside_a_called_declaration_is_bounded() {
  assert_folds(
    "const big = () => 'ab'.repeat(20);",
    "content: [big()].join(''),",
    ".x12qqa4i{content:\"abababababababababababababababababababab\"}",
  );
}

/// The same declaration called from inside a callback, where the count is the
/// product of the receiver's elements and the call's own one evaluation.
#[test]
fn a_length_inside_a_called_declaration_multiplies_in_a_callback() {
  assert_folds(
    "const big = () => 'ab'.repeat(20);",
    "content: ['q', 'r'].map((c) => c + big()).join(''),",
    ".x1mlx6cq{content:\"qababababababababababababababababababababrabababababababababababababababababababab\"}",
  );
}

/// A length past the ceiling refuses on the arithmetic, before the string is
/// built.
#[test]
fn a_length_past_the_ceiling_inside_a_called_declaration_refuses() {
  assert_refuses(
    "const big = () => 'ab'.repeat(2000000);",
    "content: [big()].join(''),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// A length on a *parameter* is the one the guard still cannot read: the
/// parameter holds an argument, and an argument's width is not something this
/// reading measures. Upstream folds it, so this is acceptance divergent — the
/// same unreadable-length category `Array.from` on a call already sits in.
#[test]
fn a_length_on_a_called_parameter_refuses() {
  assert_refuses(
    "const big = (x) => x.repeat(20);",
    "content: [big('ab')].join(''),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// An amplifying call in the *argument* is bounded where it is written, so the
/// bound is the ordinary one and the call folds.
#[test]
fn an_amplifying_argument_to_a_named_call_folds() {
  assert_folds(
    INNER,
    "content: [inner('a'.repeat(5))].join(''),",
    ".xumrpq7{content:\"aaaaa!\"}",
  );
}

// ──────────────────────────────────────────────
// Rules the call does not escape
// ──────────────────────────────────────────────

/// A locale-sensitive method inside the declaration is refused there, since
/// every rule applies at every link of what the fold claimed.
#[test]
fn a_locale_sensitive_method_inside_a_called_declaration_refuses() {
  assert_refuses(
    "const inner = (y) => y.toLocaleUpperCase();",
    "content: [inner('a')].join(''),",
    "Cannot fold 'toLocaleUpperCase' at compile time.",
  );
}

/// So is a read that walks off the value and onto the language's function
/// graph.
#[test]
fn an_escaping_read_inside_a_called_declaration_refuses() {
  assert_refuses(
    "const inner = (y) => y.constructor;",
    "content: [inner('a')].join(''),",
    "Cannot fold a read of 'constructor' at compile time.",
  );
}

// ──────────────────────────────────────────────
// What the dispatch below keeps
// ──────────────────────────────────────────────

/// A StyleX function is still answered by its own carriage rather than by this
/// one: it is asked about before the callee's name is, so the injected function
/// map is untouched.
#[test]
fn a_stylex_function_keeps_its_own_carriage() {
  assert_folds(
    "import { firstThatWorks } from '@stylexjs/stylex';",
    "fontFamily: firstThatWorks('a', 'b').join('+'),",
    ".x1qkmhv{font-family:b+a}",
  );
}

/// A dynamic style's own parameter has no compile-time value, so a call reading
/// one is handed back whole and left for the runtime — the build does not fail
/// and the call survives into the output.
#[test]
fn a_named_call_on_a_dynamic_parameter_is_left_for_the_runtime() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const inner = (y) => y + '!';
      export const styles = stylex.create({
        base: (c) => ({ content: [inner(c)].join('') }),
      });
    "#,
  );

  assert!(
    output.contains("inner(c)"),
    "expected the call to survive into the output, got:\n{}",
    output
  );
}

/// The constant half of the same style folds while the dynamic half does not,
/// which is the fold and the dispatch answering one declaration each.
#[test]
fn a_named_call_beside_a_dynamic_parameter_still_folds() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const inner = (y) => y + '!';
      export const styles = stylex.create({
        base: (c) => ({ content: [inner('a')].join(''), color: c }),
      });
    "#,
  );

  assert!(
    output.contains(".x1bt3ucs{content:\"a!\"}"),
    "expected the constant half to fold, got:\n{}",
    output
  );
}
