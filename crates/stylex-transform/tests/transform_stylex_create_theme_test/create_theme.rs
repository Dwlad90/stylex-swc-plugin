use crate::utils::prelude::*;
use crate::utils::transform::stringify_js;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_pass(PluginPass::test_default())
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        ))),
    )
  })
}

stylex_test!(
  theme_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        },
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        radius: '6px'
      });
  "#
);

stylex_test!(
  theme_object_with_unstable_conditional_identity,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
      color: stylex.unstable_conditional({
        default: 'green',
        '@media (prefers-color-scheme: dark)': 'lightgreen',
      }),
    });
  "#
);

stylex_test!(
  theme_object_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution::haste(None))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        },
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        radius: '6px'
      });
  "#
);

stylex_test!(
  theme_object_deep_in_file_tree,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/stylex/packages/src/css/vars.stylex.js".into(),
    ))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        },
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        radius: '6px'
      });
  "#
);

stylex_test!(
  literal_tokens_theme_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      "--color": "var(--color)",
      "--otherColor": "var(--otherColor)",
      "--radius": "var(--radius)",
      __varGroupHash__: "xop34xu"
    };

    export const theme = stylex.createTheme(vars, {
        '--color': 'green',
        '--otherColor': 'purple',
        '--radius': 6
      });
  "#
);

stylex_test!(
  local_variable_theme_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    const themeObj = {
      color: {
        default: 'green',
        '@media (prefers-color-scheme: dark)': 'lightgreen',
        '@media print': 'transparent',
      },
      otherColor: {
        default: 'antiquewhite',
        '@media (prefers-color-scheme: dark)': 'floralwhite',
      },
      radius: '6px'
    };
    export const theme = stylex.createTheme(vars, themeObj);
  "#
);

stylex_test!(
  local_variables_used_in_theme_objects,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    const RADIUS = 10;
    export const theme = stylex.createTheme(vars, {
        radius: RADIUS
      });
  "#
);

stylex_test!(
  template_literals_used_in_theme_objects,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    const name = 'light';
    export const theme = stylex.createTheme(vars, {
        color: `${name}green`
      });
  "#
);

stylex_test!(
  expressions_used_in_theme_objects,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    const RADIUS = 10;
    export const theme = stylex.createTheme(vars, {
        radius: RADIUS * 2
      });
  "#
);

stylex_test!(
  stylex_types_used_in_theme_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    const RADIUS = 10;
    export const theme = stylex.createTheme(vars, {
        color: stylex.types.color({
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        }),
        otherColor: stylex.types.color({
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        }),
        radius: stylex.types.length({ default: RADIUS * 2 })
      });
  "#
);

stylex_test!(
  multiple_theme_objects_same_vars,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        },
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        radius: '6px'
      });
      export const otherTheme = stylex.createTheme(vars, {
          color: 'skyblue',
          radius: '8px',
        });
  "#
);

stylex_test!(
  multiple_theme_objects_different_vars,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/stylex/packages/otherVars.stylex.js".into(),
    ))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        },
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        radius: '6px'
      });
  "#
);

stylex_test!(
  themes_are_indifferent_to_order_of_keys,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        radius: '6px',
        otherColor: {
          default: 'antiquewhite',
          '@media (prefers-color-scheme: dark)': 'floralwhite',
        },
        color: {
          default: 'green',
          '@media (prefers-color-scheme: dark)': 'lightgreen',
          '@media print': 'transparent',
        }
      });
  "#
);

stylex_test!(
  debug_adds_debug_data,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
      .with_debug(true)
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

stylex_test!(
  debug_adds_debug_data_for_npm_packages,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_debug(true)
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

stylex_test!(
  debug_adds_debug_data_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
      .with_debug(true)
      .with_unstable_module_resolution(ModuleResolution::haste(None))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

stylex_test!(
  debug_adds_debug_data_for_npm_packages_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_debug(true)
    .with_unstable_module_resolution(ModuleResolution::haste(None))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      otherColor: "var(--x1e3it8h)",
      radius: "var(--x1onrunl)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

stylex_test!(
  adds_dev_data,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
      .with_dev(true)
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xwx8imx)",
      otherColor: "var(--xaaua2w)",
      radius: "var(--xbbre8)",
      __varGroupHash__: "xop34xu"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

stylex_test!(
  options_runtime_injection_true,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xwx8imx)",
      otherColor: "var(--xaaua2w)",
      radius: "var(--xbbre8)",
      __varGroupHash__: "xop34xu"
    };

    export const theme = stylex.createTheme(vars, {
        color: 'orange'
      });
  "#
);

// A parenthesis is not a different argument. Read bare, the theme object in
// parentheses stopped the build on a shape upstream compiles, and the callee
// in parentheses was not read as a `createTheme` call at all.
//
// Ticket 47 of `.scratch/split-transform-crate` holds the class; the other
// shapes it covers are in
// `transform_stylex_create_test/parenthesised_spellings.rs`.
stylex_test!(
  a_parenthesised_theme_argument_and_callee,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xt4ziaz)",
      __varGroupHash__: "x1xohuxq"
    };

    export const theme = stylex.createTheme(vars, ({ color: 'green' }));
    export const other = (stylex.createTheme)(vars, { color: 'blue' });
    export const third = (stylex).createTheme((vars), { color: 'red' });
  "#
);

// ──────────────────────────────────────────────
// The guard
// ──────────────────────────────────────────────

/// Every `createTheme` shape in the parenthesis class, compiled both ways and
/// compared.
///
/// The snapshot above says what one spelling prints, which a reader added later
/// that matches a bare node cannot fail: it changes only the parenthesised
/// spelling, and a snapshot of that spelling records the change as correct.
/// This compares the two spellings of one module instead.
#[test]
fn every_create_theme_shape_compiles_alike_in_both_spellings() {
  fn compiled(source: &str) -> String {
    stringify_js(source, ts_syntax(), |tr| {
      stylex_transform(tr.comments.clone(), |b| b)
    })
  }

  const HEAD: &str = "import * as stylex from '@stylexjs/stylex';\n\
    export const vars = { color: \"var(--xt4ziaz)\", __varGroupHash__: \"x1xohuxq\" };\n";

  let rows: [(&str, String, String); 4] = [
    (
      "the theme object",
      format!("{HEAD}export const t = stylex.createTheme(vars, {{ color: 'green' }});"),
      format!("{HEAD}export const t = stylex.createTheme(vars, ({{ color: 'green' }}));"),
    ),
    (
      "the createTheme callee",
      format!("{HEAD}export const t = stylex.createTheme(vars, {{ color: 'green' }});"),
      format!("{HEAD}export const t = (stylex.createTheme)(vars, {{ color: 'green' }});"),
    ),
    (
      "the createTheme callee's receiver",
      format!("{HEAD}export const t = stylex.createTheme(vars, {{ color: 'green' }});"),
      format!("{HEAD}export const t = (stylex).createTheme(vars, {{ color: 'green' }});"),
    ),
    (
      "the variable group argument",
      format!("{HEAD}export const t = stylex.createTheme(vars, {{ color: 'green' }});"),
      format!("{HEAD}export const t = stylex.createTheme((vars), {{ color: 'green' }});"),
    ),
  ];

  for (shape, bare, wrapped) in &rows {
    assert_spellings_agree_with(shape, bare, wrapped, compiled);
  }
}

// Test mode writes the development names and then marks the object compiled. A
// theme object is marked already, so the two modes answer the same thing --
// which is what this asserts, rather than recording `adds_dev_data` twice.
#[test]
fn test_mode_gives_a_theme_the_development_names() {
  let compile = |dev: bool, test: bool| {
    stringify_js(
      r#"
        import * as stylex from '@stylexjs/stylex';
        export const vars = {
          color: "var(--xwx8imx)",
          otherColor: "var(--xaaua2w)",
          radius: "var(--xbbre8)",
          __varGroupHash__: "xop34xu"
        };

        export const theme = stylex.createTheme(vars, {
            color: 'orange'
          });
      "#,
      ts_syntax(),
      |tr| {
        stylex_transform(tr.comments.clone(), move |b| {
          b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
            .with_dev(dev)
            .with_test(test)
        })
      },
    )
  };

  let under_test = compile(false, true);

  assert!(
    under_test.contains("Foo__theme"),
    "test mode left the theme without its development name:\n{under_test}"
  );
  assert_eq!(
    under_test,
    compile(true, false),
    "test mode and development mode disagree on a theme"
  );
}
