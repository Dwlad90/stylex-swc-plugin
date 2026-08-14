// pub static IMPORT_FILE_PARSING_ERROR: &str = r#"There was error when
// attempting to parse the imported file. Please ensure that the 'babelrc' file
// is configured to be able to parse this file."#;

// pub static IMPORT_FILE_EVAL_ERROR: &str = r#"There was an error when
// attempting to evaluate the imported file. Please ensure that the imported
// file is self-contained and does not rely on dynamic behavior."#;

// pub static DEFAULT_IMPORT: &str = r#"Error: Cannot use default imports.

// Please define your styles without depending on values imported from other
// files.

// You *may* use named imports to use variables defined with `defineVars` in a
// file with `.stylex.js` or `.stylex.ts` file. See: https://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables for more information."#;

pub static PATH_WITHOUT_NODE: &str = r#"Unexpected error:
Could not resolve the code being evaluated."#;

pub static UNEXPECTED_MEMBER_LOOKUP: &str = r#"Unexpected error:
Could not determine the property being accessed."#;

pub static IMPORT_PATH_RESOLUTION_ERROR: &str = r#"Could not resolve the path to the imported file.
Please ensure that the theme file has a .stylex.js or .stylex.ts extension and follows the
rules for defining variables:

https://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables
"#;

pub static NON_CONSTANT: &str = "Referenced value is not a constant.\n\n";

// pub static USED_BEFORE_DECLARATION: &str =
//   "Referenced value is used before declaration.\n\n";

// pub static UNINITIALIZED_CONST: &str = "Referenced constant is not
// initialized.\n\n";

pub static UNDEFINED_CONST: &str = "Referenced constant is not defined.";

pub static OBJECT_METHOD: &str = "Unsupported object method.\n\n";

/// An argument evaluated to no value at all while the evaluation stayed
/// confident, so the arguments that follow it no longer line up with what was
/// written. Refusing beats folding a shifted argument list.
pub static ARGUMENT_WITHOUT_VALUE: &str =
  "An argument has no compile-time value.\nEvery argument must evaluate to a static value.\n\n";

/// A callable global was given a value the compiler cannot coerce — a
/// function, whose string form is its own source text. Deopting says so
/// instead of folding to a value that would be confidently wrong. The callee
/// is named because which coercion refused is the first thing an author needs.
pub fn uncoercible_value(callee: &str) -> String {
  format!(
    "Cannot coerce this value at compile time.\nOnly static values can be passed to {}().\n\n",
    callee
  )
}

/// `Array(n)` was given a count that is not an array length — a fraction, a
/// negative, `NaN`, or a value at or past `2 ** 32`. JavaScript raises a
/// `RangeError` for each of these, so there is no array to fold.
pub static INVALID_ARRAY_LENGTH: &str = "Invalid array length.\n\n";

/// The most holes `Array(n)` will materialise.
///
/// A length is only a count until the array exists, and every hole costs the
/// width of an evaluated value, so `Array(2 ** 32 - 1)` — a length JavaScript
/// accepts — is an allocation the compiler does not survive. Bounded at a
/// count no stylesheet reaches: a counted array used as a style value is
/// refused whatever its length, and the one shape that folds to something
/// usable, the join `String(Array(n))`, is `n - 1` commas.
pub const MAX_FOLDED_ARRAY_LENGTH: usize = 65_536;

/// `Array(n)` was given a length that is legal in JavaScript but past
/// [`MAX_FOLDED_ARRAY_LENGTH`].
pub fn array_length_too_large() -> String {
  format!(
    "Array length is too large to evaluate at compile time.\nAt most {} elements are supported.\n\n",
    MAX_FOLDED_ARRAY_LENGTH
  )
}

/// A global that only contributes methods was called as a function — `Math(x)`.
///
/// It reaches the fold at all because its methods are foldable, so the callee
/// is recognised and the call is not. Naming the callee says what the author
/// wrote wrong.
pub fn not_a_function(callee: &str) -> String {
  format!(
    "{} is not a function.\nOnly its methods can be called.\n\n",
    callee
  )
}

pub fn unsupported_operator(op: &str) -> String {
  format!("Unsupported operator: {}\n\n", op)
}

pub fn unsupported_expression(type_: &str) -> String {
  format!("Unsupported expression: {}\n\n", type_)
}
