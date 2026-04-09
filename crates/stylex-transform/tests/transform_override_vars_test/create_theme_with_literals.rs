use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

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

stylex_test!(
  transforms_variables_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_runtime_injection_option(RuntimeInjection::Boolean(false))
    .into_pass(),
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
