use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test_transform,
};

use crate::evaluation::args_module_transform::ArgsStyleXTransform;

#[test]
fn evaluates_empty_object() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
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
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
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
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
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
              borderWidth: "var(--borderWidth)",
            }
          })
        "#,
  )
}

#[test]
fn evaluates_object_with_function_styles_binary_expression() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
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
              borderWidth: "var(--borderWidth)",
            }
          })
        "#,
  )
}
