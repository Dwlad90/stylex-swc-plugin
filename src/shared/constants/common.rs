#[allow(dead_code)]
pub(crate) static DEFAULT_INJECT_PATH: &str = "@stylexjs/stylex/lib/stylex-inject";
pub(crate) static ILLEGAL_ARGUMENT_LENGTH:&str =  "stylex() should have 1 argument.";
#[allow(dead_code)]
pub(crate) static NON_STATIC_VALUE:&str =  "Only static values are allowed inside of a stylex.create() call.";
#[allow(dead_code)]
pub(crate) static ESCAPED_STYLEX_VALUE:&str =  "Escaping a stylex.create() value is not allowed.";
#[allow(dead_code)]
pub(crate) static UNBOUND_STYLEX_CALL_VALUE:&str =  "stylex.create calls must be bound to a bare variable.";
#[allow(dead_code)]
pub(crate) static ONLY_TOP_LEVEL:&str =  "stylex.create() is only allowed at the root of a program.";
pub(crate) static NON_OBJECT_FOR_STYLEX_CALL:&str =  "stylex.create() can only accept a style object.";
#[allow(dead_code)]
pub(crate) static UNKNOWN_PROP_KEY:&str =  "Unknown property key";
#[allow(dead_code)]
pub(crate) static INVALID_PSEUDO:&str =  "Invalid pseudo selector, not on the whitelist.";
#[allow(dead_code)]
pub(crate) static INVALID_PSEUDO_OR_AT_RULE:&str =  "Invalid pseudo or at-rule.";
#[allow(dead_code)]
pub(crate) static NO_CONDITIONAL_SHORTHAND:&str =  "You cannot use conditional style values for a shorthand property.";
#[allow(dead_code)]
pub(crate) static ILLEGAL_NAMESPACE_TYPE:&str =  "Only a string literal namespace is allowed here.";
#[allow(dead_code)]
pub(crate) static UNKNOWN_NAMESPACE:&str =  "Unknown namespace";
#[allow(dead_code)]
pub(crate) static ILLEGAL_NESTED_PSEUDO:&str =  "Pseudo objects can't be nested more than one level deep.";
#[allow(dead_code)]
pub(crate) static ILLEGAL_PROP_VALUE:&str =  "A style value can only contain an array, string or number.";
#[allow(dead_code)]
pub(crate) static ILLEGAL_PROP_ARRAY_VALUE:&str =  "A style array value can only contain strings or numbers.";
#[allow(dead_code)]
pub(crate) static ILLEGAL_NAMESPACE_VALUE:&str =  "A stylex namespace must be an object.";
#[allow(dead_code)]
pub(crate) static INVALID_SPREAD:&str =  "Imported styles spread with a stylex.create call must be type cast as `XStyle` to verify their type.";
#[allow(dead_code)]
pub(crate) static LOCAL_ONLY:&str =  "The return value of stylex.create() should not be exported.";
#[allow(dead_code)]
pub(crate) static UNEXPECTED_ARGUMENT:&str =  "Unexpected argument passed to the stylex() function.";
#[allow(dead_code)]
pub(crate) static EXPECTED_FUNCTION_CALL:&str =  "Expected a simple function call but found something else.";
#[allow(dead_code)]
pub(crate) static NO_PARENT_PATH:&str =  "Unexpected AST node without a parent path.";
#[allow(dead_code)]
pub(crate) static ONLY_TOP_LEVEL_INCLUDES:&str =  "stylex.include() is only at the top level of a style definition object.";
#[allow(dead_code)]
pub(crate) static DUPLICATE_CONDITIONAL:&str =  "The same pseudo selector or at-rule cannot be used more than once.";
#[allow(dead_code)]
pub(crate) static NO_PROJECT_ROOT_DIRECTORY:&str =  "The project root directory `rootDir` is not configured.";
#[allow(dead_code)]
pub(crate) static NON_EXPORT_NAMED_DECLARATION:&str =  "The return value of stylex.defineVars() must be bound to a named export.";
#[allow(dead_code)]
pub(crate) static ANONYMOUS_THEME:&str =  "stylex.createTheme() must be bound to a named constant.";
