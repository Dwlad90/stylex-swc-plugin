//! Tests for evaluation error message functions and static constants.

use crate::constants::evaluation_errors::*;

#[test]
fn test_unsupported_operator() {
  assert_eq!(unsupported_operator("+"), "Unsupported operator: +\n\n");
  assert_eq!(unsupported_operator("**"), "Unsupported operator: **\n\n");
}

#[test]
fn test_unsupported_expression() {
  assert_eq!(
    unsupported_expression("AwaitExpression"),
    "Unsupported expression: AwaitExpression\n\n"
  );
}

#[test]
fn test_unreadable_index() {
  assert_eq!(
    unreadable_index("0"),
    "Unsupported index: 0\nThis index could not be read at compile time.\n\n"
  );
  assert!(unreadable_index("12").contains("index: 12"));
}

#[test]
fn test_uncoercible_value() {
  assert_eq!(
    uncoercible_value("String"),
    "Cannot coerce this value at compile time.\nOnly static values can be passed to String().\n\n"
  );
  assert!(uncoercible_value("Number").contains("Number()"));
}

#[test]
fn test_array_length_too_large() {
  assert_eq!(
    array_length_too_large(10_000),
    "Array length is too large to evaluate at compile time.\nAt most 10000 elements are supported.\n\n"
  );
}

#[test]
fn test_expression_too_deep() {
  assert_eq!(
    expression_too_deep(32),
    "Expression is too deeply nested to evaluate at compile time.\nAt most 32 levels of nested evaluation are supported.\n\n"
  );
  // The depth is the caller's, not a constant baked into the message: a
  // configured ceiling has to be the number an author reads.
  assert!(expression_too_deep(320).contains("At most 320 levels"));
}

#[test]
fn test_not_a_function() {
  assert_eq!(
    not_a_function("Math"),
    "Math is not a function.\nOnly its methods can be called.\n\n"
  );
}

#[test]
fn test_static_constants() {
  assert!(!PATH_WITHOUT_NODE.is_empty());
  assert!(!UNEXPECTED_MEMBER_LOOKUP.is_empty());
  assert!(!IMPORT_PATH_RESOLUTION_ERROR.is_empty());
  assert!(!NON_CONSTANT.is_empty());
  assert!(!UNDEFINED_CONST.is_empty());
  assert!(!OBJECT_METHOD.is_empty());
}

// The refusals a fold hands an author when it declines a call. Each names the
// method or the limit it refused on, so each is asserted with that half in
// place rather than as a fixed sentence — the name is the part that says what
// to write instead.

#[test]
fn test_locale_sensitive_method() {
  assert_eq!(
    locale_sensitive_method("toLocaleUpperCase"),
    "Cannot fold 'toLocaleUpperCase' at compile time.\nIts answer depends on locale data the compiler does not carry.\n\n"
  );
  assert!(locale_sensitive_method("localeCompare").contains("'localeCompare'"));
}

#[test]
fn test_numeric_literal_receiver() {
  assert_eq!(
    numeric_literal_receiver("toFixed"),
    "Cannot call 'toFixed' on a number literal.\nOnly a number a fold produced can be a method receiver.\n\n"
  );
}

#[test]
fn test_unbounded_amplified_length() {
  assert_eq!(
    unbounded_amplified_length("repeat", 1_000_000),
    "Cannot bound the string 'repeat' would build.\nIts length must resolve to a number of at most 1000000, on a receiver whose own length can be read.\n\n"
  );
  // The limit is the caller's, like the depth ceiling above: a bound an author
  // can raise has to be the number they read.
  assert!(unbounded_amplified_length("padStart", 32).contains("at most 32"));
}

// The other half of the same question, and the one an author reaches by asking
// for a length that *was* read: it names what was asked for beside the limit,
// because the two together say whether this is a typo or a project that has
// outgrown the ceiling.
#[test]
fn test_amplified_length_too_large() {
  // A call that pads builds what it was asked for, so there is one number and
  // it is named once.
  assert_eq!(
    amplified_length_too_large("padStart", 1_200_000, 1_200_000, 1_000_000),
    "Cannot bound the string 'padStart' would build.\nIt asks for 1200000 characters, and at most 1000000 are supported.\n\n"
  );
  // A call that repeats builds the count times its receiver, and the count is
  // the only one of the two an author can find in what they wrote -- so both
  // are named, rather than a total that appears nowhere in the source.
  assert_eq!(
    amplified_length_too_large("repeat", 600_000, 1_200_000, 1_000_000),
    "Cannot bound the string 'repeat' would build.\nIt asks for 600000 copies of the value it is called on, which is 1200000 characters, and at most 1000000 are supported.\n\n"
  );
  // Both refusals of an amplifying call open with the same line, so an author
  // learns to recognise one class of failure rather than two.
  assert!(
    amplified_length_too_large("padEnd", 9, 9, 8)
      .starts_with("Cannot bound the string 'padEnd' would build.")
  );
}

#[test]
fn test_object_size_too_large() {
  assert_eq!(
    object_size_too_large(10_000),
    "Object is too large to evaluate at compile time.\nAt most 10000 properties are supported.\n\n"
  );
}

#[test]
fn test_unfoldable_fold_result() {
  assert_eq!(
    unfoldable_fold_result("function"),
    "Cannot carry a folded function back from the engine.\nOnly strings, numbers, booleans, null, arrays and plain objects can be folded.\n\n"
  );
  // The kind is passed through without an article in front of it, which is what
  // lets the caller hand over the language's own `typeof` unchanged.
  assert!(unfoldable_fold_result("symbol").starts_with("Cannot carry a folded symbol back"));
  assert!(unfoldable_fold_result("bigint").starts_with("Cannot carry a folded bigint back"));
}

#[test]
fn test_engine_threw() {
  assert_eq!(
    engine_threw(
      "reduce",
      "TypeError: Reduce of empty array with no initial value"
    ),
    "Cannot fold 'reduce' at compile time.\nTypeError: Reduce of empty array with no initial value\n\n"
  );
  // The method is named by this compiler because the engine's own sentence does
  // not always name it: a call to a method that does not exist reads
  // `undefined` and calls it, so the language says only this much.
  assert!(
    engine_threw("unsupported", "TypeError: not a callable function").contains("'unsupported'")
  );
}

#[test]
fn test_escaping_property() {
  assert_eq!(
    escaping_property("constructor"),
    "Cannot fold a read of 'constructor' at compile time.\nIt leads off the value that was written and onto the language's own function graph.\n\n"
  );
  // The three names that turn an unapplied function back into a call read the
  // same way, because the property is the whole of the reason.
  assert!(escaping_property("bind").contains("'bind'"));
}

#[test]
fn test_amplification_inside_a_callback() {
  assert_eq!(
    amplification_inside_a_callback("repeat"),
    "Cannot bound the string 'repeat' would build inside a callback.\nA callback runs once per element, so a length written into the source bounds one evaluation rather than the call.\n\n"
  );
  assert!(amplification_inside_a_callback("padStart").contains("'padStart'"));
}

#[test]
fn test_folded_string_too_large() {
  assert_eq!(
    folded_string_too_large(1_000_000),
    "Folded string is too large to evaluate at compile time.\nAt most 1000000 characters are supported.\n\n"
  );
  // The number an author reads is the limit they can raise, so it is the one
  // the caller passed rather than a constant spelled again here.
  assert!(folded_string_too_large(32).contains("At most 32"));
}

#[test]
fn test_bound_value_too_large() {
  assert_eq!(
    bound_value_too_large("big", 1_000_000),
    "Cannot carry the value of 'big' into a fold.\nAt most 1000000 characters are supported.\n\n"
  );
  // The binding is named rather than the method, because the size belongs to
  // what the name holds and the same call on a shorter value folds.
  assert!(bound_value_too_large("fonts", 32).contains("'fonts'"));
  assert!(bound_value_too_large("fonts", 32).contains("At most 32"));
}

#[test]
fn test_uncallable_printed_fold() {
  assert_eq!(
    uncallable_printed_fold("trim"),
    "Cannot fold 'trim' at compile time.\nThe printed expression did not compile to a function.\n\n"
  );
  assert!(uncallable_printed_fold("join").contains("'join'"));
}

#[test]
fn test_unfoldable_call() {
  assert_eq!(
    unfoldable_call("startsWith"),
    "Cannot fold 'startsWith' at compile time.\nIts receiver or one of its arguments is not in a form the compiler can evaluate.\n\n"
  );
  // Neither of the two reasons the caller cannot tell apart is named, so the
  // sentence stays true whichever one applied.
  assert!(!unfoldable_call("trim").contains("argument must"));
  assert!(unfoldable_call("trim").contains("'trim'"));
}
