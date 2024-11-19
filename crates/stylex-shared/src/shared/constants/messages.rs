pub(crate) static ILLEGAL_ARGUMENT_LENGTH: &str = "stylex() should have 1 argument.";
pub(crate) static NON_STATIC_VALUE: &str =
  "Only static values are allowed inside of a stylex.create() call.";
#[allow(dead_code)]
pub(crate) static ESCAPED_STYLEX_VALUE: &str = "Escaping a stylex.create() value is not allowed.";
pub(crate) static UNBOUND_STYLEX_CALL_VALUE: &str =
  "stylex.create calls must be bound to a bare variable.";
#[allow(dead_code)]
pub(crate) static ONLY_TOP_LEVEL: &str =
  "stylex.create() is only allowed at the root of a program.";
pub(crate) static NON_OBJECT_FOR_STYLEX_CALL: &str =
  "stylex.create() can only accept a style object.";
pub static UNKNOWN_PROP_KEY: &str = "Unknown property key";
pub(crate) static MUST_BE_DEFAULT_IMPORT: &str = "Must be default import";
#[allow(dead_code)]
pub(crate) static INVALID_PSEUDO: &str = "Invalid pseudo selector, not on the whitelist.";
pub(crate) static INVALID_PSEUDO_OR_AT_RULE: &str = "Invalid pseudo or at-rule.";
#[allow(dead_code)]
pub(crate) static NO_CONDITIONAL_SHORTHAND: &str =
  "You cannot use conditional style values for a shorthand property.";
#[allow(dead_code)]
pub(crate) static ILLEGAL_NAMESPACE_TYPE: &str = "Only a string literal namespace is allowed here.";
#[allow(dead_code)]
pub(crate) static UNKNOWN_NAMESPACE: &str = "Unknown namespace";
#[allow(dead_code)]
pub(crate) static ILLEGAL_NESTED_PSEUDO: &str =
  "Pseudo objects can't be nested more than one level deep.";
pub(crate) static ILLEGAL_PROP_VALUE: &str =
  "A style value can only contain an array, string or number.";
pub(crate) static ILLEGAL_PROP_ARRAY_VALUE: &str =
  "A style array value can only contain strings or numbers.";
pub(crate) static ILLEGAL_NAMESPACE_VALUE: &str = "A stylex namespace must be an object.";
#[allow(dead_code)]
pub(crate) static INVALID_SPREAD:&str =  "Imported styles spread with a stylex.create call must be type cast as `XStyle` to verify their type.";
#[allow(dead_code)]
pub(crate) static LOCAL_ONLY: &str = "The return value of stylex.create() should not be exported.";
#[allow(dead_code)]
pub(crate) static UNEXPECTED_ARGUMENT: &str =
  "Unexpected argument passed to the stylex() function.";
#[allow(dead_code)]
pub(crate) static EXPECTED_FUNCTION_CALL: &str =
  "Expected a simple function call but found something else.";
#[allow(dead_code)]
pub(crate) static NO_PARENT_PATH: &str = "Unexpected AST node without a parent path.";
pub(crate) static ONLY_TOP_LEVEL_INCLUDES: &str =
  "stylex.include() is only at the top level of a style definition object.";
pub(crate) static DUPLICATE_CONDITIONAL: &str =
  "The same pseudo selector or at-rule cannot be used more than once.";
#[allow(dead_code)]
pub(crate) static NO_PROJECT_ROOT_DIRECTORY: &str =
  "The project root directory `rootDir` is not configured.";
pub(crate) static NON_EXPORT_NAMED_DECLARATION: &str =
  "The return value of stylex.defineVars() must be bound to a named export.";
#[allow(dead_code)]
pub(crate) static ANONYMOUS_THEME: &str = "stylex.createTheme() must be bound to a named constant.";
pub(crate) static ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS:&str =  "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed.";
pub(crate) static BUILT_IN_FUNCTION: &str = "Evaluation built-in functions not supported";

pub(crate) static NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL: &str =
  "stylex.keyframes() can only accept an object.";

pub(crate) static NON_STATIC_KEYFRAME_VALUE: &str =
  "Only static values are allowed inside of a stylex.keyframes() call.";

pub(crate) static NON_OBJECT_KEYFRAME: &str =
  "Every frame within a stylex.keyframes() call must be an object.";

pub(crate) static NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE: &str =
  "stylex.createTheme() can only accept an object as the second argument.";

pub(crate) static LINT_UNCLOSED_FUNCTION: &str = "Rule contains an unclosed function";
pub(crate) static UNPREFIXED_CUSTOM_PROPERTIES: &str = "Unprefixed custom properties";

pub(crate) static NON_CONTIGUOUS_VARS: &str =
  "All variables passed to `stylex.firstThatWorks` must be contiguous.";
