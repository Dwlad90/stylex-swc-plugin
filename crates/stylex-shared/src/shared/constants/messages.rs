// Functions that generate messages with parameters
pub fn illegal_argument_length(fn_name: &str, arg_length: usize) -> String {
  let plural = if arg_length == 1 { "" } else { "s" };
  format!(
    "{}() should have {} argument{}.",
    fn_name, arg_length, plural
  )
}

pub fn non_static_value(fn_name: &str) -> String {
  format!(
    "Only static values are allowed inside of a {}() call.",
    fn_name
  )
}

pub fn non_style_object(fn_name: &str) -> String {
  format!("{}() can only accept an object.", fn_name)
}

pub fn non_export_named_declaration(fn_name: &str) -> String {
  format!(
    "The return value of {}() must be bound to a named export.",
    fn_name
  )
}

pub fn unbound_call_value(fn_name: &str) -> String {
  format!("{}() calls must be bound to a bare variable.", fn_name)
}

pub fn cannot_generate_hash(fn_name: &str) -> String {
  format!(
    "Unable to generate hash for {}(). Check that the file has a valid extension and that unstable_moduleResolution is configured.",
    fn_name
  )
}

// Static constants
pub static DUPLICATE_CONDITIONAL: &str =
  "The same pseudo selector or at-rule cannot be used more than once.";

pub static ESCAPED_STYLEX_VALUE: &str = "Escaping a create() value is not allowed.";

pub static ILLEGAL_NESTED_PSEUDO: &str = "Pseudo objects can't be nested more than one level deep.";

pub static ILLEGAL_PROP_VALUE: &str = "A style value can only contain an array, string or number.";

pub static ILLEGAL_PROP_ARRAY_VALUE: &str =
  "A style array value can only contain strings or numbers.";

pub static ILLEGAL_NAMESPACE_VALUE: &str = "A StyleX namespace must be an object.";

pub static INVALID_CONST_KEY: &str = r#"Keys in defineConsts() cannot start with "--"."#;

pub static INVALID_PSEUDO: &str = "Invalid pseudo selector, not on the whitelist.";

pub static INVALID_PSEUDO_OR_AT_RULE: &str = "Invalid pseudo or at-rule.";

pub static LINT_UNCLOSED_FUNCTION: &str = "Rule contains an unclosed function";

pub static LOCAL_ONLY: &str = "The return value of create() should not be exported.";

pub static NON_OBJECT_KEYFRAME: &str = "Every frame within a keyframes() call must be an object.";

pub static NON_CONTIGUOUS_VARS: &str =
  "All variables passed to firstThatWorks() must be contiguous.";

pub static NO_OBJECT_SPREADS: &str = "Object spreads are not allowed in create() calls.";

pub static ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS: &str = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed.";

pub static ONLY_TOP_LEVEL: &str = "create() is only allowed at the root of a program.";

pub static UNKNOWN_PROP_KEY: &str = "Unknown property key";

pub(crate) static UNPREFIXED_CUSTOM_PROPERTIES: &str = "Unprefixed custom properties";

pub(crate) static NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE: &str =
  "createTheme() can only accept an object as the second argument.";

pub(crate) static BUILT_IN_FUNCTION: &str = "Evaluation built-in functions not supported";

pub(crate) static THEME_IMPORT_KEY_AS_OBJECT_KEY: &str =
  "Theme import keys cannot be used as object keys. Please use a valid object key.";

pub(crate) static POSITION_TRY_INVALID_PROPERTY: &str = "Invalid property in `positionTry()` call. It may only contain, positionAnchor, positionArea, inset properties (top, left, insetInline etc.), margin properties, size properties (height, inlineSize, etc.), and self-alignment properties (alignSelf, justifySelf, placeSelf)";

pub(crate) static VIEW_TRANSITION_CLASS_INVALID_PROPERTY: &str = "Invalid property in `viewTransitionClass()` call. It may only contain group, imagePair, old, and new properties";

pub(crate) static INVALID_MEDIA_QUERY_SYNTAX: &str = "Invalid media query syntax.";
