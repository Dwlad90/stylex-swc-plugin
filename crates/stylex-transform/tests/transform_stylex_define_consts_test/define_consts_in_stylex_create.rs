use crate::utils::prelude::*;
use crate::utils::transform::stringify_js;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  let cwd_path = std::env::current_dir().unwrap();
  let fixture_path = cwd_path.join("tests/fixture/consts");

  build_test_transform(comments, move |b| {
    customize(
      b.with_cwd(fixture_path.clone())
        .with_filename(fixture_path.clone().join("constants.stylex.js").into())
        .with_unstable_module_resolution(ModuleResolution {
          r#type: "commonJS".to_string(),
          root_dir: Some(fixture_path.to_string_lossy().to_string()),
          theme_file_extension: None,
        }),
    )
  })
}

fn transform_with_inline_consts(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    stylex_transform(tr.comments.clone(), |b| b)
  })
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
