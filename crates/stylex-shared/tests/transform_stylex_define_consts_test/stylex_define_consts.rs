use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{test, test_transform},
  },
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  transforms_constants_object,
  r#"
        import stylex from 'stylex';
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
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  transforms_constants_object_with_named_import,
  r#"
        import { defineConsts } from 'stylex';
        export const breakpoints = defineConsts({
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
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  transforms_constants_object_with_import_asterisk,
  r#"
        import * as foo from 'stylex';
        export const colors = foo.defineConsts({
          primary: '#ff0000',
          secondary: '#00ff00',
          tertiary: '#0000ff',
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  handles_special_characters_in_constant_names,
  r#"
        import stylex from 'stylex';
        export const sizes = stylex.defineConsts({
          'max-width': '1200px',
          'font-size*large': '18px',
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  handles_numeric_keys,
  r#"
        import stylex from 'stylex';
        export const levels = stylex.defineConsts({
          1: 'one',
          2: 'two',
        });
    "#
);

#[test]
#[should_panic(expected = r#"Keys in defineConsts() cannot start with "--"."#)]
fn throws_error_for_constant_keys_that_asterisk_with_dash_dash() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const spacing = stylex.defineConsts({
              '--small': '8px',
              '--medium': '16px',
              '--large': '24px',
            });
        "#,
    r#""#,
  )
}
