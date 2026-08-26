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
    "Expression is too deeply nested to evaluate at compile time.\n\
     At most {} levels of nested evaluation are supported.\n\n",
    limit
  )
}

/// A method whose answer depends on locale data the compiler does not carry.
///
/// The engine resolves these against the root locale, so
/// `"i".toLocaleUpperCase("tr")` comes back `I` where the language says `İ`.
/// Folding would write a wrong value into the stylesheet, which is worse than
/// writing none, so the fold declines and says which method it declined.
pub fn locale_sensitive_method(method: &str) -> String {
  format!(
    "Cannot fold '{}' at compile time.\nIts answer depends on locale data the compiler does not carry.\n\n",
    method
  )
}

/// A method called on a number written into the source as a literal.
///
/// Every `Number.prototype` method throws on one of those in the reference
/// implementation, which applies the method without a receiver. Refusing keeps
/// both compilers rejecting the same input. A number a fold *produced* is a
/// different shape and folds in both.
pub fn numeric_literal_receiver(method: &str) -> String {
  format!(
    "Cannot call '{}' on a number literal.\nOnly a number a fold produced can be a method receiver.\n\n",
    method
  )
}

/// A length-amplifying call whose result length could not be bounded.
///
/// The engine bounds loop iterations, recursion and stack, but not allocation:
/// growth inside a native builtin is not a counted loop. So the length has to
/// be readable, and under the ceiling, before the call is evaluated at all.
pub fn unbounded_amplified_length(method: &str, limit: f64) -> String {
  format!(
    "Cannot bound the string '{}' would build.\nIts length must be a number literal of at most {}, on a receiver that is not itself a call.\n\n",
    method, limit
  )
}

/// A folded object with more properties than the fold's budget.
///
/// Shaped after [`array_length_too_large`] and bounded by the same number, for
/// the same reason: every property crossing back becomes an AST node, which
/// costs far more as a tree than it did as a value in the engine.
pub fn object_size_too_large(limit: u64) -> String {
  format!(
    "Object is too large to evaluate at compile time.\nAt most {} properties are supported.\n\n",
    limit
  )
}

/// A folded value with no form this evaluator carries — a function, a symbol,
/// `undefined`, a BigInt, or an object that is not a plain one.
///
/// Names the kind rather than the expression, because the expression is what
/// the author wrote and the kind is what the language answered with; only the
/// second says why an otherwise valid call folds to nothing usable.
///
/// Phrased so the kind needs no article in front of it. That is what lets the
/// caller pass the language's own `typeof` straight through instead of keeping
/// a word per kind of its own, and the second line is what tells an author
/// whose value is `object` that theirs is not a plain one.
pub fn unfoldable_fold_result(kind: &str) -> String {
  format!(
    "Cannot carry a folded {} back from the engine.\nOnly strings, numbers, booleans, null, arrays and plain objects can be folded.\n\n",
    kind
  )
}

/// A call the engine threw on, reported in the engine's own words under this
/// compiler's naming of the call that produced them.
///
/// A throw is an answer rather than a fault of the fold: `[].reduce(f)` and
/// `"a".repeat(-1)` throw in the language too, and the sentence the language
/// wrote says more than any sentence this compiler could substitute for it.
///
/// The method is named alongside it because the language's sentence does not
/// always name it. A call to a method that does not exist reads a property that
/// is `undefined` and then calls it, so the throw is `not a callable function` —
/// which tells an author nothing the code frame has not already shown them.
pub fn engine_threw(method: &str, message: &str) -> String {
  format!("Cannot fold '{}' at compile time.\n{}\n\n", method, message)
}

/// A named property read that leads off the value the author wrote.
///
/// `constructor` on a literal is `String`, and `String.constructor` is
/// `Function`, which compiles a string into a body — so two reads and a call
/// are arbitrary code inside the compiler, answering differently on every build
/// and able to write to a prototype every later fold in the build shares.
/// `call`, `apply` and `bind` are what turn an unapplied function back into a
/// call, so they are refused with it.
///
/// Names the property rather than the method call it sat in, because the
/// property is the whole of the reason: the same call on any other name folds.
pub fn escaping_property(property: &str) -> String {
  format!(
    "Cannot fold a read of '{}' at compile time.\n\
     It leads off the value that was written and onto the language's own function graph.\n\n",
    property
  )
}

/// A length-amplifying call written inside a callback.
///
/// [`unbounded_amplified_length`] bounds one evaluation, which is what a length
/// written into the source can bound. A callback body is evaluated once per
/// element of a receiver nothing measured, so the same written bound is
/// multiplied by a count the source never states — and two calls each inside
/// the bound build a length that is not.
pub fn amplification_inside_a_callback(method: &str) -> String {
  format!(
    "Cannot bound the string '{}' would build inside a callback.\n\
     A callback runs once per element, so a length written into the source bounds one evaluation rather than the call.\n\n",
    method
  )
}

/// A folded string longer than the fold will carry back.
///
/// [`unbounded_amplified_length`] bounds what one written call may be *asked*
/// to build; this bounds what actually came back, whatever produced it. Shaped
/// after [`array_length_too_large`] and bounded by the same number as the
/// argument is, because it is the same string measured on the other side.
pub fn folded_string_too_large(limit: f64) -> String {
  format!(
    "Folded string is too large to evaluate at compile time.\n\
     At most {} characters are supported.\n\n",
    limit
  )
}

/// A binding whose resolved value is too large to carry into a fold.
///
/// The transport passes a resolved value as an *argument* rather than printing
/// it into the source, so the printed text stays the size of the expression
/// however large the value is. What that does not shrink is the value itself:
/// it is copied into the engine, so it is bounded on the way in by the same
/// number that bounds a folded string on the way out.
///
/// Names the binding rather than the method, because the size is a property of
/// what the name holds and the same call on a shorter value folds.
pub fn bound_value_too_large(name: &str, limit: f64) -> String {
  format!(
    "Cannot carry the value of '{}' into a fold.\nAt most {} characters are supported.\n\n",
    name, limit
  )
}

/// A printed fold that did not evaluate to the function it was printed as.
///
/// The fold prints its expression as an arrow and calls it, so the value the
/// engine answers is a function by construction. Answered as a refusal rather
/// than asserted, because a broken invariant inside an evaluation whose whole
/// contract is that it may fail must not abort the build.
pub fn uncallable_printed_fold(method: &str) -> String {
  format!(
    "Cannot fold '{}' at compile time.\nThe printed expression did not compile to a function.\n\n",
    method
  )
}

/// A method call the fold declined, where the receiver's own value was fine.
///
/// The whole prototype surface folds through the engine, so a call arriving at
/// the older dispatch with a usable receiver was declined for one of two reasons:
/// something in it has no compile-time value, or it is written in a shape the
/// fold's guard does not read. The sentence names neither, because the arm cannot
/// tell them apart and a sentence that named the wrong one would send an author
/// looking in the wrong place. It says what is true of both.
///
/// Names the method, because that is what an author has to look at -- where the
/// node kind would only tell them they wrote a call.
pub fn unfoldable_call(method: &str) -> String {
  format!(
    "Cannot fold '{}' at compile time.\nIts receiver or one of its arguments is not in a form the compiler can evaluate.\n\n",
    method
  )
}
