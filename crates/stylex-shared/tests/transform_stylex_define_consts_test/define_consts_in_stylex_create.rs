use crate::utils::transform::stringify_js;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

fn transform_with_inline_consts(input: &str) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      let cwd_path = std::env::current_dir().unwrap();
      let fixture_path = cwd_path.join("tests/fixture/consts");

      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(fixture_path.clone()),
          filename: fixture_path.clone().join("constants.stylex.js").into(),
        },
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(ModuleResolution {
            r#type: "commonJS".to_string(),
            root_dir: Some(fixture_path.to_string_lossy().to_string()),
            theme_file_extension: None,
          }),
          ..Default::default()
        }),
      )
    },
  )
}

#[test]
fn adds_placeholder_for_constant_value_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { colors } from './constants.stylex';

        export const styles = stylex.create({
          root: {
            backgroundColor: colors.background,
          },
        });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!(
    "adds_placeholder_for_constant_value_from_constants_stylex",
    output
  );
}

#[test]
fn adds_media_query_placeholder_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { breakpoints } from './constants.stylex';

        export const styles = stylex.create({
          root: {
            color: {
              default: 'red',
              [breakpoints.small]: 'blue',
            },
          },
        });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!("adds_media_query_placeholder_from_constants_stylex", output);
}

#[test]
#[ignore]
fn works_with_first_that_works() {
  let input = r#"
      import * as stylex from '@stylexjs/stylex';
      import { colors } from './constants.stylex';

      export const styles = stylex.create({
        nodeEnd: (animationDuration) => ({
          foo: {
            color: stylex.firstThatWorks(colors.background, 'transparent'),
          },
        }),
      });
    "#;

  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!("works_with_first_that_works", output);
}

#[test]
fn works_with_dynamic_styles_constants() {
  let input = r#"
      import * as stylex from '@stylexjs/stylex';
      import { colors } from './constants.stylex';

      export const styles = stylex.create({
        node: (padding) => ({
          padding: padding,
          color: colors.background,
        }),
      });
    "#;

  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!("works_with_dynamic_styles_constants", output);
}

#[test]
fn works_with_dynamic_styles_at_rules() {
  let input = r#"
      import * as stylex from '@stylexjs/stylex';
      import { breakpoints } from './constants.stylex';

      export const styles = stylex.create({
        node: (color) => ({
          color: {
            [breakpoints.small]: 'blue',
            default: color,
          },
        }),
      });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!("works_with_dynamic_styles_at_rules", output);
}

#[test]
fn adds_multiple_media_query_placeholders_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { breakpoints } from './constants.stylex';

        export const styles = stylex.create({
          root: {
            color: {
              default: 'red',
              [breakpoints.small]: 'blue',
              [breakpoints.big]: 'yellow',
            },
          },
        });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!(
    "adds_multiple_media_query_placeholders_from_constants_stylex",
    output
  );
}

#[test]
fn adds_nested_media_query_placeholders_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { breakpoints, colors } from './constants.stylex';

        export const styles = stylex.create({
          root: {
            color: {
              default: 'black',
              [breakpoints.big]: {
                default: colors.red,
                [breakpoints.small]: colors.blue,
              },
            },
          },
        });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!(
    "adds_nested_media_query_placeholders_from_constants_stylex",
    output
  );
}
