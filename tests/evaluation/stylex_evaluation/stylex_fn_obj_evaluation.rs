use swc_core::ecma::parser::{Syntax, TsConfig};

use crate::evaluation::args_module_transform::ArgsModuleTransformVisitor;

#[test]
fn evaluates_empty_object() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| ArgsModuleTransformVisitor::default(),
        r#"
          const x = {};
        "#,
        r#"
            ({});
        "#,
        false,
    )
}

#[test]
fn evaluates_static_style_object() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| ArgsModuleTransformVisitor::default(),
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
        false,
    )
}

#[test]
fn evaluates_object_with_function_styles_identifier() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| ArgsModuleTransformVisitor::default(),
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
              borderWidth: "var(--borderWidth, revert)",
            }
          })
        "#,
        false,
    )
}


#[test]
fn evaluates_object_with_function_styles_binary_expression() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| ArgsModuleTransformVisitor::default(),
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
              borderWidth: "var(--borderWidth, revert)",
            }
          })
        "#,
        false,
    )
}
