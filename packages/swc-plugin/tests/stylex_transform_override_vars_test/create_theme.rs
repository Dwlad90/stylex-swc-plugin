use insta::assert_snapshot;
use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptions, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
  },
};

use crate::utils::transform::stringify_js;

fn tranform(input: &str, override_params: StyleXOptionsParams) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
          cwd: Option::None,
          filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Some(StyleXOptionsParams {
          runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
          dev: Option::Some(true),
          ..override_params
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

  let output_v1 = tranform(input_v1.as_str(), StyleXOptionsParams::default());
  let output_v2 = tranform(input_v2.as_str(), StyleXOptionsParams::default());

  assert_snapshot!(output_v1);
  assert_snapshot!(output_v2);

  assert_eq!(output_v1, output_v2);
}

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution()),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution()),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution()),
      ..StyleXOptionsParams::default()
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
