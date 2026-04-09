use crate::utils::prelude::*;

use crate::evaluation::args_module_transform::ArgsStyleXTransform;

stylex_test_transform!(
  evaluates_empty_object,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = {};
  "#,
  r#"
    ({});
  "#
);

stylex_test_transform!(
  evaluates_static_style_object,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = {
      default: {
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: 1,
      },
    };
  "#,
  r#"
    ({
      default: {
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: 1,
      },
    })
  "#
);

stylex_test_transform!(
  evaluates_object_with_function_styles_identifier,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = {
      default: (width) => ({
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: width,
      }),
    };
  "#,
  r#"
    ({
      default: {
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: "var(--x-borderWidth)",
      }
    })
  "#
);

stylex_test_transform!(
  evaluates_object_with_function_styles_binary_expression,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = {
      default: (width) => ({
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: width * 2 + 'px',
      }),
    };
  "#,
  r#"
    ({
      default: {
        overflow: 'hidden',
        borderStyle: 'dashed',
        borderWidth: "var(--x-borderWidth)",
      }
    })
  "#
);
