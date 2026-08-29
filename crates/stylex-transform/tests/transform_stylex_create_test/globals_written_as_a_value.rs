//! A global whose *name* reaches a fold where a value belongs.
//!
//! `[…].filter(Boolean)` is how the shape is written, and it is not the only
//! one: an element of an array, an operand, the base of a static property read
//! — every position that walks a value can hold one. None of them folds. The
//! bridge carries values, and a global is not a value it can carry: the engine
//! holds the real one, and printing the name would fold calls the reference
//! compiler refuses.
//!
//! What changes here is only the sentence. Before, such a name resolved to
//! nothing and the fold handed the whole call back, so an author read that a
//! constant they never wrote was not defined. Now the refusal names the global
//! it is about.
//!
//! The set is the globals this fold *knows* — the five callees plus `Boolean`.
//! A global it folds nothing of, `parseInt` or `Symbol`, is still handed back
//! and still reads as a missing constant: naming those would mean writing down a
//! list of globals nothing else in the compiler uses.
//!
//! Measured against `@stylexjs/babel-plugin` 0.19.0: it refuses every case
//! below, so both compilers reject the same input and only the wording differs.
//! A binding of the same spelling is the module's in both, and folds.

use crate::utils::transform::{assert_folds, assert_refuses};

// ──────────────────────────────────────────────
// The reported shape
// ──────────────────────────────────────────────

/// The row the ticket was opened on.
#[test]
fn a_global_as_a_filter_callback_names_itself() {
  assert_refuses(
    "",
    "fontFamily: ['Arial', false].filter(Boolean).join(', '),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// The other globals, on the same terms
// ──────────────────────────────────────────────

#[test]
fn number_as_a_map_callback_names_itself() {
  assert_refuses(
    "",
    "fontFamily: ['1', '2'].map(Number).join(', '),",
    "Cannot carry the global 'Number' into a fold.",
  );
}

#[test]
fn string_as_a_map_callback_names_itself() {
  assert_refuses(
    "",
    "fontFamily: [1, 2].map(String).join(', '),",
    "Cannot carry the global 'String' into a fold.",
  );
}

#[test]
fn array_as_a_map_callback_names_itself() {
  assert_refuses(
    "",
    "content: [1].map(Array).length + '',",
    "Cannot carry the global 'Array' into a fold.",
  );
}

#[test]
fn object_as_a_map_callback_names_itself() {
  assert_refuses(
    "",
    "content: [1].map(Object).length + '',",
    "Cannot carry the global 'Object' into a fold.",
  );
}

/// `Math` is not callable, and reaches the same position for the same reason:
/// the name stands where a value belongs.
#[test]
fn math_as_a_map_callback_names_itself() {
  assert_refuses(
    "",
    "content: [1].map(Math).length + '',",
    "Cannot carry the global 'Math' into a fold.",
  );
}

// ──────────────────────────────────────────────
// Every position that walks a value, not just an argument
// ──────────────────────────────────────────────

/// An element of a receiver rather than an argument of the call on it.
#[test]
fn a_global_written_into_an_array_names_itself() {
  assert_refuses(
    "",
    "content: [Boolean].join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// The base of a static property read. The receiver of a static *call* is a
/// different rule and still folds — see below.
#[test]
fn a_static_property_read_inside_a_fold_names_its_global() {
  assert_refuses(
    "",
    "content: [Math.PI].join(''),",
    "Cannot carry the global 'Math' into a fold.",
  );
}

/// A value of an object written into a fold.
#[test]
fn a_global_as_an_object_value_names_itself() {
  assert_refuses(
    "",
    "content: Object.keys({ a: String }).join(''),",
    "Cannot carry the global 'String' into a fold.",
  );
}

/// An operand of an expression the fold walks.
#[test]
fn a_global_as_an_operand_names_itself() {
  assert_refuses(
    "",
    "content: ['a'].concat(Boolean + '').join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// Parentheses change nothing about which name was written.
#[test]
fn a_parenthesised_global_names_itself() {
  assert_refuses(
    "",
    "content: [(Boolean)].join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// A callback deep inside another callback's body is still a value position.
#[test]
fn a_global_inside_a_nested_callback_names_itself() {
  assert_refuses(
    "",
    "content: [[1, 0], [2]].map((xs) => xs.filter(Boolean).length).join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// What the rule must not take
// ──────────────────────────────────────────────

/// A global *called* is a different shape and unchanged: the engine holds the
/// function and the language answers.
#[test]
fn a_called_global_still_folds() {
  assert_folds("", "width: Number('4') + 'px',", ".x51ohtg{width:4px}");
}

/// A static's receiver carries no value across the bridge, so it never reaches
/// this rule.
#[test]
fn a_static_call_on_a_global_still_folds() {
  assert_folds(
    "",
    "content: Object.keys({ a: 1, b: 2 }).join('-'),",
    ".x1t42mo{content:\"a-b\"}",
  );
}

/// `Math.max` is the static the fold is asked for most, and its receiver is not
/// a value either.
#[test]
fn a_math_static_still_folds() {
  assert_folds("", "width: Math.max(1, 2) + 'px',", ".xfo62xy{width:2px}");
}

// ──────────────────────────────────────────────
// A binding of the same spelling is the module's
// ──────────────────────────────────────────────

/// A `const` holding an arrow is the author's own function, and both compilers
/// call it. Measured: upstream folds this to `font-family:false`.
#[test]
fn a_const_shadowing_a_global_is_still_the_modules() {
  assert_folds(
    "const Boolean = (x) => !x;",
    "fontFamily: ['Arial', false].filter(Boolean).join(', '),",
    ".x6oxdzz{font-family:false}",
  );
}

/// A `function` of the same spelling is a binding the fold cannot carry, and it
/// is named as the binding it is rather than as a global.
#[test]
fn a_function_shadowing_a_global_is_named_as_a_binding() {
  assert_refuses(
    "function Boolean(x) { return !x; }",
    "fontFamily: ['Arial', false].filter(Boolean).join(', '),",
    "Cannot carry the function 'Boolean' into a fold.",
  );
}

/// A callback parameter of the same spelling is the engine's to bind, and the
/// fold runs it.
#[test]
fn a_callback_parameter_shadowing_a_global_folds() {
  assert_folds(
    "",
    "content: [1, 2].map((Boolean) => Boolean + 1).join('-'),",
    ".x1hcpz05{content:\"2-3\"}",
  );
}

/// A `let` reassigned after it was declared is the module's binding too, and is
/// refused for the write rather than as a global.
#[test]
fn a_reassigned_binding_shadowing_a_global_is_not_the_global() {
  assert_refuses(
    "let Boolean = (x) => !x; Boolean = (x) => x;",
    "fontFamily: ['Arial', false].filter(Boolean).join(', '),",
    "Cannot carry the function 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// An operand the language never evaluates
// ──────────────────────────────────────────────

/// The guard walks exactly the arms the language runs, so a global in an arm
/// nothing takes is not written into anything. Upstream folds this too.
#[test]
fn a_global_in_an_arm_that_never_runs_folds() {
  assert_folds(
    "",
    "content: [true ? 'a' : Boolean].join(''),",
    ".x16319ns{content:\"a\"}",
  );
}

/// The same for a short-circuited right operand.
#[test]
fn a_global_in_a_short_circuited_operand_folds() {
  assert_folds(
    "",
    "content: ['a'].concat(false && Boolean).join(''),",
    ".xgeuyrc{content:\"afalse\"}",
  );
}

/// The arm that *is* taken is walked, and the global in it is named.
#[test]
fn a_global_in_the_arm_that_runs_names_itself() {
  assert_refuses(
    "",
    "content: [false ? 'a' : Boolean].join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// The positions left, and inputs at the sizes the fold is priced for
// ──────────────────────────────────────────────

/// A hole of a template literal is a value in its own right.
#[test]
fn a_global_in_a_template_hole_names_itself() {
  assert_refuses(
    "",
    "content: [`${Boolean}`].join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// A unary operand is walked, so `typeof` reaches the rule rather than folding
/// to `\"function\"` off a name the bridge never carried.
#[test]
fn a_global_under_a_unary_operator_names_itself() {
  assert_refuses(
    "",
    "content: [typeof Boolean].join(''),",
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// A spread inside an array needs a scope the printed source does not carry, so
/// it is refused before anything inside it is walked — this rule never sees the
/// global. Upstream refuses the input too.
#[test]
fn a_spread_refuses_before_the_global_inside_it_is_reached() {
  assert_refuses(
    "",
    "content: [...[Boolean]].join(''),",
    "Unsupported expression: SpreadElement",
  );
}

/// Two hundred elements ahead of the global, so the rule is reached after a
/// walk long enough to be the expensive half rather than in front of one.
#[test]
fn a_global_last_in_a_large_array_still_names_itself() {
  let elements = ["'a'"; 200].join(", ");

  assert_refuses(
    "",
    &format!("content: [{}, Boolean].join(''),", elements),
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

/// Twenty levels of array around the global. The name is reached by descending,
/// so a nesting the walk still has room for must still name it — and one it has
/// no room for is the depth rule's to answer, not this one's.
#[test]
fn a_deeply_nested_global_still_names_itself() {
  let nested = format!("{}Boolean{}", "[".repeat(20), "]".repeat(20));

  assert_refuses(
    "",
    &format!("content: {}.flat(20).join(''),", nested),
    "Cannot carry the global 'Boolean' into a fold.",
  );
}

// ──────────────────────────────────────────────
// The call and the callback agree
// ──────────────────────────────────────────────

/// `Boolean` is the one global here that is never folded as a callee either:
/// the reference compiler does not hold it among the ones it applies, so
/// `Boolean(x)` refuses in both compilers exactly as `filter(Boolean)` does.
/// The two positions agree, which is the point.
#[test]
fn a_called_boolean_refuses_as_the_callback_does() {
  assert_refuses(
    "",
    "fontFamily: Boolean(1) ? 'Arial' : 'serif',",
    "Referenced constant is not defined.",
  );
}
