use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(
        "/stylex/packages/TestTheme.stylex.js".into(),
      ))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

fn stylex_transform_haste(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(
        "/stylex/packages/TestTheme.stylex.js".into(),
      ))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
  })
}

stylex_test!(
  creates_theme_override_for_nested_vars,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: {
        primary: "var(--x1n06l0x)",
        secondary: "var(--xtjtmik)",
      },
      spacing: {
        sm: "var(--xaymh01)",
        lg: "var(--x8i77v7)",
      },
      __varGroupHash__: "xop34xu",
    };
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: {
        primary: 'green',
        secondary: 'darkgreen',
      },
    });
  "#
);

stylex_test!(
  transforms_nested_theme_overrides_for_imported_theme_ref,
  |tr| stylex_transform_haste(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { vars } from 'tokens.stylex';
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: {
        primary: 'blue',
      },
    });
  "#
);

stylex_test!(
  transforms_nested_theme_from_commonjs_namespace_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const stylex = require('@stylexjs/stylex');
    export const vars = stylex.unstable_defineVarsNested({
      color: {
        primary: 'red',
      },
    });
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: {
        primary: 'blue',
      },
    });
  "#
);

stylex_test!(
  partial_override_only_some_branches,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: {
        primary: "var(--x1n06l0x)",
        secondary: "var(--xtjtmik)",
      },
      spacing: {
        sm: "var(--xaymh01)",
        lg: "var(--x8i77v7)",
      },
      __varGroupHash__: "xop34xu",
    };
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: {
        primary: 'red',
      },
    });
  "#
);

stylex_test!(
  conditional_override_with_media,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: {
        primary: "var(--x1n06l0x)",
        secondary: "var(--xtjtmik)",
      },
      spacing: {
        sm: "var(--xaymh01)",
        lg: "var(--x8i77v7)",
      },
      __varGroupHash__: "xop34xu",
    };
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: {
        primary: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
        },
      },
    });
  "#
);

stylex_test!(
  works_with_named_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      bg: {
        primary: "var(--x1gxda9x)",
      },
      __varGroupHash__: "xop34xu",
    };
    import { unstable_createThemeNested } from '@stylexjs/stylex';
    export const theme = unstable_createThemeNested(vars, {
      bg: { primary: 'black' },
    });
  "#
);

stylex_test!(
  override_css_uses_correct_var_hashes_from_define_vars_nested,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested({
      button: { bg: 'red' },
    });
    export const theme = stylex.unstable_createThemeNested(vars, {
      button: { bg: 'green' },
    });
  "#
);

// The development class names a theme carries beside its override class, which
// are written from the declarator's name and the file name.
fn dev_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_dev(true)
      .with_filename(FileName::Real(
        "/stylex/packages/TestTheme.stylex.js".into(),
      ))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

stylex_test!(
  adds_dev_class_names_to_a_nested_theme,
  |tr| dev_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: {
        primary: "var(--x1n06l0x)",
      },
      __varGroupHash__: "xop34xu",
    };
    export const theme = stylex.unstable_createThemeNested(vars, {
      color: { primary: 'green' },
    });
  "#
);

// Test mode writes the development names and then marks the object compiled. A
// theme object is marked already, so the two modes answer the same thing for a
// nested theme -- which is what this asserts, rather than recording the same
// module twice.
#[test]
fn test_mode_gives_a_nested_theme_the_development_names() {
  let compile = |dev: bool, test: bool| {
    crate::utils::transform::stringify_js(
      r#"
        import * as stylex from '@stylexjs/stylex';
        export const vars = {
          color: {
            primary: "var(--x1n06l0x)",
          },
          __varGroupHash__: "xop34xu",
        };
        export const theme = stylex.unstable_createThemeNested(vars, {
          color: { primary: 'green' },
        });
      "#,
      ts_syntax(),
      |tr| {
        build_test_transform(tr.comments.clone(), |b| {
          b.with_runtime_injection()
            .with_dev(dev)
            .with_test(test)
            .with_filename(FileName::Real(
              "/stylex/packages/TestTheme.stylex.js".into(),
            ))
            .with_unstable_module_resolution(ModuleResolution::common_js(Some(
              "/stylex/packages/".to_string(),
            )))
        })
      },
    )
  };

  let under_test = compile(false, true);

  assert!(
    under_test.contains("TestTheme__theme"),
    "test mode left the theme without its development name:\n{under_test}"
  );
  assert_eq!(
    under_test,
    compile(true, false),
    "test mode and development mode disagree on a nested theme"
  );
}
