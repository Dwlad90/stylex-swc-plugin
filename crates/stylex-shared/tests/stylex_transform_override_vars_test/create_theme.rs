use insta::assert_snapshot;
use stylex_shared::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  StyleXTransform,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

use crate::utils::transform::stringify_js;

fn get_default_opts() -> StyleXOptionsParams {
  StyleXOptionsParams {
    unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
    class_name_prefix: Some("x".to_string()),
    ..StyleXOptionsParams::default()
  }
}

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
          cwd: None,
          filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Some(&mut StyleXOptionsParams {
          runtime_injection: Some(false),
          dev: Some(true),
          ..get_default_opts()
        }),
      )
    },
  )
}

static OUTPUT_OF_STYLEX_DEFINE_VARS: &str = r#"
import stylex from 'stylex';
export const buttonTheme = {
    bgColor: "var(--xgck17p)",
    bgColorDisabled: "var(--xpegid5)",
    cornerRadius: "var(--xrqfjmn)",
    fgColor: "var(--x4y59db)",
    __themeName__: "x568ih9"
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      ..get_default_opts()
    })
  ),
  transforms_variables_object,
  format!(
    r#"
    {}
    export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  transforms_variables_object_and_add_stylex_inject_in_dev_mode,
  format!(
    r#"
    {}
    export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
      ..get_default_opts()
    })
  ),
  transforms_variables_object_in_non_haste_env,
  format!(
    r#"
    {}
    export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
      ..get_default_opts()
    })
  ),
  transforms_variables_object_in_non_haste_dev_env,
  format!(
    r#"
    {}
    export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      ..get_default_opts()
    })
  ),
  transforms_multiple_variables_objects_in_a_single_file,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  transforms_multiple_variables_objects_in_a_single_file_in_dev_mode,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  transforms_variables_objects_with_references_to_local_variables,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  allows_references_to_local_variables_with_static_values,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  allows_template_literal_references,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  allows_pure_complex_expressions,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..get_default_opts()
    })
  ),
  transforms_variables_object_in_common_js_with_nested_file_path,
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS, CREATE_THEME
  )
  .as_str()
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..get_default_opts()
    })
  ),
  transforms_typed_object_overrides,
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
