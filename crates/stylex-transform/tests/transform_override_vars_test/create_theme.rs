use crate::utils::prelude::*;
use insta::assert_snapshot;
use swc_core::common::FileName;

use crate::utils::transform::stringify_js;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b.with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into(),
    )))
  })
}

fn transform(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(FileName::Real(
        "/stylex/packages/TestTheme.stylex.js".into(),
      ))
      .with_runtime_injection_option(RuntimeInjection::Boolean(false))
      .with_dev(true)
      .into_pass()
  })
}

static OUTPUT_OF_STYLEX_DEFINE_VARS: &str = r#"
    import stylex from 'stylex';
    export const buttonTheme = {
      bgColor: "var(--xgck17p)",
      bgColorDisabled: "var(--xpegid5)",
      cornerRadius: "var(--xrqfjmn)",
      fgColor: "var(--x4y59db)",
      __varGroupHash__: "x568ih9"
    };
  "#;

static CREATE_THEME: &str = r#"{
  bgColor: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  bgColorDisabled: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  cornerRadius: { default: '6px' },
  fgColor: 'coral',
}"#;

static CREATE_THEME_WITH_DIFFERENT_ORDER: &str = r#"{
  bgColorDisabled: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  fgColor: { default: 'coral' },
  bgColor: {
    default: 'green',
    '@media print': 'transparent',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
  },
  cornerRadius: '6px',
}"#;

#[test]
fn variables_order_does_not_change_the_class_name_hash() {
  let input_v1 = format!(
    r#"
      {}
      const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  );

  let input_v2 = format!(
    r#"
      {}
      const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME_WITH_DIFFERENT_ORDER
  );

  let output_v1 = transform(input_v1.as_str());
  let output_v2 = transform(input_v2.as_str());

  assert_snapshot!(output_v1);
  assert_snapshot!(output_v2);

  assert_eq!(output_v1, output_v2);
}

stylex_test!(
  transforms_variables_object,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(false))
  }),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_variables_object_and_add_stylex_inject_in_dev_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_variables_object_in_non_haste_env,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(false))
      .with_unstable_module_resolution(ModuleResolution::common_js(None))
  }),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_variables_object_in_non_haste_dev_env,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_dev(true)
      .with_unstable_module_resolution(ModuleResolution::common_js(None))
  }),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_multiple_variables_objects_in_a_single_file,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(false))
  }),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
        export const buttonThemeNew = stylex.createTheme(buttonTheme, {{
              bgColor: 'skyblue',
              cornerRadius: '8px',
          }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_multiple_variables_objects_in_a_single_file_in_dev_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
        export const buttonThemeMonochromatic = stylex.createTheme(
          buttonTheme, {{
              bgColor: 'white',
              bgColorDisabled: 'black',
              cornerRadius: '0px',
          }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_variables_objects_with_references_to_local_variables,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      const RADIUS = 10;
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            bgColor: {{
                default: 'green',
                '@media (prefers-color-scheme: dark)': 'lightgreen',
                '@media print': 'transparent',
            }},
            bgColorDisabled: {{
                default: 'antiquewhite',
                '@media (prefers-color-scheme: dark)': 'floralwhite',
            }},
            cornerRadius: {{ default: RADIUS }},
            fgColor: 'coral',
        }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);

stylex_test!(
  allows_references_to_local_variables_with_static_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      const COLOR = 'coral';
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            bgColor: {{
                default: 'green',
                '@media (prefers-color-scheme: dark)': 'lightgreen',
                '@media print': 'transparent',
            }},
            bgColorDisabled: {{
                default: 'antiquewhite',
                '@media (prefers-color-scheme: dark)': 'floralwhite',
            }},
            cornerRadius: {{ default: '6px' }},
            fgColor: COLOR,
        }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);

stylex_test!(
  allows_template_literal_references,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      const name = 'light';
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            bgColor: {{
                default: `${{name}}green`,
                '@media (prefers-color-scheme: dark)': 'lightgreen',
                '@media print': 'transparent',
            }},
            bgColorDisabled: {{
                default: 'antiquewhite',
                '@media (prefers-color-scheme: dark)': 'floralwhite',
            }},
            cornerRadius: {{ default: '6px' }},
            fgColor: 'coral',
        }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);

stylex_test!(
  allows_pure_complex_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      const RADIUS = 2;
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            bgColor: {{
                default: 'green',
                '@media (prefers-color-scheme: dark)': 'lightgreen',
                '@media print': 'transparent',
            }},
            bgColorDisabled: {{
                default: 'antiquewhite',
                '@media (prefers-color-scheme: dark)': 'floralwhite',
            }},
            cornerRadius: {{ default: RADIUS * 2 }},
            fgColor: 'coral',
        }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);

stylex_test!(
  transforms_variables_object_in_common_js_with_nested_file_path,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/stylex/packages/utils/vars.stylex.js".into(),
    ))
    .with_dev(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
  }),
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

stylex_test!(
  transforms_typed_object_overrides,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_dev(true)),
  format!(
    r#"
      {}
      export const RADIUS = 2;
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            bgColor: stylex.types.color({{
                default: 'green',
                '@media (prefers-color-scheme: dark)': 'lightgreen',
                '@media print': 'transparent',
            }}),
            bgColorDisabled: stylex.types.color({{
                default: 'antiquewhite',
                '@media (prefers-color-scheme: dark)': 'floralwhite',
            }}),
            cornerRadius: stylex.types.length({{ default: RADIUS * 2 }}),
            fgColor: stylex.types.color('coral'),
        }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);
