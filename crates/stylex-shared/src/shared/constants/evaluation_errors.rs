// pub(crate) static IMPORT_FILE_PARSING_ERROR: &str = r#"There was error when attempting to parse the imported file.
// Please ensure that the 'babelrc' file is configured to be able to parse this file."#;

// pub(crate) static IMPORT_FILE_EVAL_ERROR: &str = r#"There was an error when attempting to evaluate the imported file.
// Please ensure that the imported file is self-contained and does not rely on dynamic behavior."#;

// pub(crate) static DEFAULT_IMPORT: &str = r#"Error: Cannot use default imports.

// Please define your styles without depending on values imported from other files.

// You *may* use named imports to use variables defined with `stylex.defineVars` in a file with `.stylex.js` or `.stylex.ts` file.
// See: https://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables for more information."#;

pub(crate) static PATH_WITHOUT_NODE: &str = r#"Unexpected error:
Could not resolve the code being evaluated."#;

pub(crate) static UNEXPECTED_MEMBER_LOOKUP: &str = r#"Unexpected error:
Could not determine the property being accessed."#;

pub(crate) static IMPORT_PATH_RESOLUTION_ERROR: &str = r#"Could not resolve the path to the imported file.
Please ensure that the theme file has a .stylex.js or .stylex.ts file extension and follows the
rules for defining variariables:

https://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables
"#;

pub(crate) static NON_CONSTANT: &str = "Referenced value is not a constant.\n\n";

// pub(crate) static USED_BEFORE_DECLARATION: &str =
//   "Referenced value is used before declaration.\n\n";

// pub(crate) static UNINITIALIZED_CONST: &str = "Referenced constant is not initialized.\n\n";

pub(crate) static UNDEFINED_CONST: &str = "Referenced constant is not defined.";

pub(crate) static OBJECT_METHOD: &str = "Unsupported object method.\n\n";

pub(crate) fn unsupported_operator(op: &str) -> String {
  format!("Unsupported operator: {}\n\n", op)
}

pub(crate) fn unsupported_expression(type_: &str) -> String {
  format!("Unsupported expression: {}\n\n", type_)
}
