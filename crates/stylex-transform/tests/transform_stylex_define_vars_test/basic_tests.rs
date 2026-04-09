use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  tokens_as_null,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: null,
      nextColor: {
        default: null
      },
      otherColor: {
        default: null,
        '@media (prefers-color-scheme: dark)': null,
        '@media print': null,
      },
    });
  "#
);

stylex_test!(
  tokens_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      nextColor: {
        default: 'green'
      },
      otherColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    });
  "#
);

stylex_test!(
  tokens_object_haste,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      nextColor: {
        default: 'green'
      },
      otherColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    });
  "#
);

stylex_test!(
  tokens_object_deep_in_file_tree,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/stylex/packages/src/css/vars.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
  "#
);

stylex_test!(
  tokens_object_with_nested_at_rules,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': {
          default: 'lightblue',
          '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.3 -0.4)',
        }
      },
    });
  "#
);

stylex_test!(
  literal_tokens_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      '--color': 'red',
      '--otherColor': {
        default: 'blue',
        ':hover': 'lightblue',
      },
    });
  "#
);

stylex_test!(
  local_variable_tokens_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const tokens = {
      '--color': 'red',
      '--nextColor': {
        default: 'green'
      },
      '--otherColor': {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    };
    export const vars = stylex.defineVars(tokens)
  "#
);

stylex_test!(
  local_variables_used_in_tokens_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    export const vars = stylex.defineVars({
      color: COLOR
    });
  "#
);

stylex_test!(
  template_literals_used_in_tokens_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const NUMBER = 10;
    export const vars = stylex.defineVars({
      size: `${NUMBER}rem`
    });
  "#
);

stylex_test!(
  expressions_used_in_tokens_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const NUMBER = 10;
    export const vars = stylex.defineVars({
      radius: NUMBER * 2
    });
  "#
);

stylex_test!(
  stylex_types_used_in_tokens_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: stylex.types.color({
        default: 'red',
        '@media (prefers-color-scheme: dark)': 'white',
        '@media print': 'black',
      })
    });
  "#
);

stylex_test!(
  multiple_variables_objects_same_file,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
    export const otherVars = stylex.defineVars({
      otherColor: 'orange'
    });
  "#
);

stylex_test!(
  multiple_variables_objects_dependency,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
    export const otherVars = stylex.defineVars({
      otherColor: vars.color
    });
  "#
);

stylex_test!(
  multiple_variables_objects_different_files_first,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
  "#
);

stylex_test!(
  multiple_variables_objects_different_files_second,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const otherVars = stylex.defineVars({
      otherColor: 'orange'
    });
  "#
);
