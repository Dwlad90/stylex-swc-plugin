use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

fn get_default_opts() -> StyleXOptionsParams {
  StyleXOptionsParams {
    unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
    class_name_prefix: Some("x".to_string()),
    ..StyleXOptionsParams::default()
  }
}

static OUTPUT_OF_STYLEX_DEFINE_VARS: &str = r#"
import stylex from 'stylex';
export const buttonTheme = {
    "--bgColor": "var(--bgColor)",
    "--bgColorDisabled": "var(--bgColorDisabled)",
    "--cornerRadius": "var(--cornerRadius)",
    "--fgColor": "var(--fgColor)",
    __varGroupHash__: "x568ih9"
};
"#;

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
      runtime_injection: Some(RuntimeInjection::Boolean(false)),
      ..get_default_opts()
    })
  ),
  transforms_variables_object,
  format!(
    r#"
      {}
      export const buttonThemePositive = stylex.createTheme(buttonTheme, {{
            '--bgColor': {{
              default: 'green',
              '@media (prefers-color-scheme: dark)': 'lightgreen',
              '@media print': 'transparent',
            }},
            '--bgColorDisabled': {{
              default: 'antiquewhite',
              '@media (prefers-color-scheme: dark)': 'floralwhite',
            }},
            '--cornerRadius': {{ default: '6px' }},
            '--fgColor': 'coral',
      }});
    "#,
    OUTPUT_OF_STYLEX_DEFINE_VARS
  )
  .as_str()
);
