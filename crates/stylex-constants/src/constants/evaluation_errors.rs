// pub static IMPORT_FILE_PARSING_ERROR: &str = r#"There was error when attempting to parse the imported file.
// Please ensure that the 'babelrc' file is configured to be able to parse this file."#;

// pub static IMPORT_FILE_EVAL_ERROR: &str = r#"There was an error when attempting to evaluate the imported file.
// Please ensure that the imported file is self-contained and does not rely on dynamic behavior."#;

// pub static DEFAULT_IMPORT: &str = r#"Error: Cannot use default imports.

// Please define your styles without depending on values imported from other files.

// You *may* use named imports to use variables defined with `defineVars` in a file with `.stylex.js` or `.stylex.ts` file.
// See: https://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables for more information."#;

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

// pub static UNINITIALIZED_CONST: &str = "Referenced constant is not initialized.\n\n";

pub static UNDEFINED_CONST: &str = "Referenced constant is not defined.";

pub static OBJECT_METHOD: &str = "Unsupported object method.\n\n";

pub fn unsupported_operator(op: impl AsRef<str>) -> String {
  let op = op.as_ref();
  format!("Unsupported operator: {}\n\n", op)
}

pub fn unsupported_expression(type_: impl AsRef<str>) -> String {
  let type_ = type_.as_ref();
  format!("Unsupported expression: {}\n\n", type_)
}
