use crate::utils::transform::stringify_js;
use std::path::PathBuf;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

fn transform(input: &str) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(PathBuf::from("/stylex/packages/")),
          filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(ModuleResolution {
            r#type: "commonJS".to_string(),
            root_dir: Some("/stylex/packages/".to_string()),
            theme_file_extension: None,
          }),
          ..Default::default()
        }),
      )
    },
  )
}

fn transform_with_inline_consts(input: &str) -> String {
  use stylex_shared::shared::structures::stylex_options::ModuleResolution;

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
          filename: fixture_path.clone().join("input.stylex.js").into(),
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
fn constants_are_unique() {
  let input1 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input2 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input3 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ margin: '10px' });
      "#;

  let output1 = transform(input1);
  let output2 = transform(input2);
  let output3 = transform(input3);

  // Assert the generated constants are consistent for the same inputs
  assert_eq!(output1, output2);

  // Assert the generated constants are different for different inputs
  assert_ne!(output1, output3);
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constants_object,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constants_object_haste,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constant_names_special_characters,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          'font-size*large': '18px',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constant_names_number,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          1: 'one'
        });
      "#
);

#[test]
fn adds_placeholder_for_constant_value_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { colors } from './input.stylex';

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
        import { breakpoints } from './input.stylex';

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
#[ignore]
fn works_with_dynamic_styles() {
  let input = r#"
      import * as stylex from '@stylexjs/stylex';
      import { breakpoints } from './constants.stylex';

      export const styles = stylex.create({
        nodeEnd: (animationDuration) => ({
          transition: {
            [breakpoints.small]: 'none',
            default: `transform ${animationDuration}ms ease-in-out`,
          },
        }),
      });
    "#;
  let output = transform_with_inline_consts(input);
  insta::assert_snapshot!("works_with_dynamic_styles", output);
}

#[test]
fn adds_multiple_media_query_placeholders_from_constants_stylex() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
        import { breakpoints } from './input.stylex';

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
        import { breakpoints, colors } from './input.stylex';

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
