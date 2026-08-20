// pub static IMPORT_FILE_PARSING_ERROR: &str = r#"There was error when
// attempting to parse the imported file. Please ensure that the 'babelrc' file
// is configured to be able to parse this file."#;

/// A reference that resolves to a *default* import specifier. A theme file is
/// read through its named exports, so a default binding names a value from a
/// file this compiler never evaluates -- there is nothing to fold, and folding
/// the theme reference a named specifier would have answered with would invent
/// a variable the theme file does not define.
///
/// Says "error evaluating the imported file" rather than "cannot use default
/// imports" because that is the reference implementation's text for this input.
/// It has a message of its own for a default import -- `DEFAULT_IMPORT`, still
/// commented out below -- and does not reach it from here; the reference chain
/// refuses a default specifier with this one instead, and the two compilers
/// agreeing on the text is what makes a diagnostic portable between them.
pub static IMPORT_FILE_EVAL_ERROR: &str = r#"There was an error when attempting to evaluate the imported file.
Please ensure that the imported file is self-contained and does not rely on dynamic behavior.
"#;

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

/// A reference whose binding is declared later in the module. Declarations are
/// collected module-wide, so the initializer is reachable from a use site that
/// precedes it — and inlining it there emits CSS for a value the program does
/// not hold yet.
pub static USED_BEFORE_DECLARATION: &str = "Referenced value is used before declaration.\n\n";

/// A reference to one of `undefined` / `Infinity` / `NaN` that names a binding
/// rather than the global. The three are ordinary binding names to the
/// language, so anything in scope can take one over -- a `const`, a function
/// parameter, a catch binding -- and the global is then not what the reference
/// reads. There is no folded value to answer with either, so the fold refuses.
///
/// Says "not initialized" rather than "shadowed" because that is the reference
/// implementation's text, and it is the text for the same reason: the step is
/// reached only where the binding carries no value the evaluator holds.
pub static UNINITIALIZED_CONST: &str = "Referenced constant is not initialized.\n\n";

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

/// `Array(n)` was given a length that is legal in JavaScript but past the
/// fold's own budget, which the coercions own and pass in.
pub fn array_length_too_large(limit: usize) -> String {
  format!(
    "Array length is too large to evaluate at compile time.\nAt most {} elements are supported.\n\n",
    limit
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

/// An index was read off a string or an array and could not be folded.
///
/// Names the index rather than the node kind the rest of these reasons name. The
/// kind is the half an author can already see — `"abc"[0]` and `"abc".length`
/// are both a member access on a StringLiteral — and which index was asked for
/// is the half that says where to look.
///
/// Deliberately says only that this index could not be read, because which
/// indices can is a property of the receiver rather than of the language: an
/// array a fold produced is indexed, and a string is not.
pub fn unreadable_index(index: &str) -> String {
  format!(
    "Unsupported index: {}\nThis index could not be read at compile time.\n\n",
    index
  )
}

/// The one answer every spread in a value position earns.
///
/// A spread is refused wherever it appears — an array element, a call argument,
/// at any nesting — because the reference implementation evaluates elements and
/// arguments as *paths*, so a spread reaches its terminal
/// `UNSUPPORTED_EXPRESSION(path.node.type)` arm whatever it spreads and before
/// the operand is looked at.
///
/// A constant rather than an `unsupported_expression("SpreadElement")` at each
/// of the eleven sites that need it: the node kind is the same string every
/// time, and a site that spelled it differently would be a divergence no test
/// names.
pub const SPREAD_ELEMENT: &str = "Unsupported expression: SpreadElement\n\n";

pub fn unsupported_operator(op: &str) -> String {
  format!("Unsupported operator: {}\n\n", op)
}

pub fn unsupported_expression(type_: &str) -> String {
  format!("Unsupported expression: {}\n\n", type_)
}

/// An expression nested past the fold's recursion budget.
///
/// Without a bound of its own the evaluator's real limit is the thread's stack,
/// and its failure a process abort rather than a diagnostic. The budget makes
/// the limit a number the compiler owns, and the refusal an ordinary one.
///
/// Says *nested evaluation* rather than nested expressions, because the budget
/// counts the fold's own steps and a source level is not always one of them: a
/// member read descends to its object and to the value it lands on, a spread
/// descends to the object it copies, and a parenthesis is unwrapped before the
/// fold is asked at all. Naming source levels would put a number in the message
/// that the input beside it contradicts.
///
/// Shaped after [`array_length_too_large`]: a fold that gives up on size says
/// so, and says how much it was willing to do.
pub fn expression_too_deep(limit: usize) -> String {
  format!(
    "Expression is too deeply nested to evaluate at compile time.\nAt most {} levels of nested evaluation are supported.\n\n",
    limit
  )
}
