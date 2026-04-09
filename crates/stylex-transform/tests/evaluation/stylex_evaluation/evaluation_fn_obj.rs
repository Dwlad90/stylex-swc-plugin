use crate::utils::prelude::*;

use crate::evaluation::args_module_transform::ArgsStyleXTransform;

#[test]
fn evaluates_empty_object() {
  test_transform(
    ts_syntax(),
    Option::None,
    |_| ArgsStyleXTransform::default_with_pass(),
    r#"
          const x = {};
        "#,
    r#"
            ({});
        "#,
  )
}

#[test]
fn evaluates_static_style_object() {
  test_transform(
    ts_syntax(),
    Option::None,
    |_| ArgsStyleXTransform::default_with_pass(),
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
        "#,
  )
}

#[test]
fn evaluates_object_with_function_styles_identifier() {
  test_transform(
    ts_syntax(),
    Option::None,
    |_| ArgsStyleXTransform::default_with_pass(),
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
        "#,
  )
}

#[test]
fn evaluates_object_with_function_styles_binary_expression() {
  test_transform(
    ts_syntax(),
    Option::None,
    |_| ArgsStyleXTransform::default_with_pass(),
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
        "#,
  )
}
