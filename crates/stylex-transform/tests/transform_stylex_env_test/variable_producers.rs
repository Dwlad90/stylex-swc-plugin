//! `stylex.env.<name>` read inside the calls that declare variables and
//! constants: `defineVars`, `defineConsts` and `createTheme`, each in its plain
//! and its nested form.
//!
//! Every one of these evaluates its argument with a map of its own, and each
//! map says what the StyleX namespace has. `env` is one of the things it has,
//! however the map is built, so the same value reads the same way in all of
//! them -- and a module that writes one environment value in a `create` call
//! and the same value in a theme no longer stops on the second.

use crate::transform_stylex_env_test::brand_env;
use crate::utils::prelude::*;
use swc_core::common::FileName;

/// A module that declares variables, which `defineVars` and `defineConsts` are
/// only allowed in.
fn variable_module_transform(comments: TestComments) -> impl Pass + use<> {
  build_test_transform(comments, |b| {
    b.with_env(brand_env())
      .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
      .with_runtime_injection()
  })
}

/// A module that writes a theme over a variable group it names itself.
fn theme_module(comments: TestComments) -> impl Pass + use<> {
  build_test_transform(comments, |b| {
    b.with_env(brand_env())
      .with_pass(PluginPass::test_default())
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
      .with_runtime_injection()
  })
}

stylex_test!(
  the_env_folds_inside_a_define_vars_call,
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      brand: stylex.env.brandPrimary,
    });
  "#
);

// The same value under a condition, which is the nested form of the call.
stylex_test!(
  the_env_folds_inside_a_nested_define_vars_value,
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      brand: {
        default: stylex.env.brandPrimary,
        '@media (min-width: 600px)': 'blue',
      },
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_define_consts_call,
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      brand: stylex.env.brandPrimary,
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_nested_define_consts_value,
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      brand: { primary: stylex.env.brandPrimary },
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_create_theme_call,
  |tr| theme_module(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      brand: "var(--xt4ziaz)",
      __varGroupHash__: "x1xohuxq"
    };
    export const theme = stylex.createTheme(vars, {
      brand: stylex.env.brandPrimary,
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_nested_create_theme_value,
  |tr| theme_module(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      brand: "var(--xt4ziaz)",
      __varGroupHash__: "x1xohuxq"
    };
    export const theme = stylex.createTheme(vars, {
      brand: {
        default: stylex.env.brandPrimary,
        '@media (min-width: 600px)': 'blue',
      },
    });
  "#
);

// A bare namespace declares no variable, in either call. The sentence is the
// one the reference implementation gives for the same source.
stylex_test_panic!(
  a_bare_namespace_declares_no_variable,
  "Default value is not defined for brand variable.",
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      brand: stylex,
    });
  "#
);

stylex_test_panic!(
  a_bare_namespace_declares_no_constant,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| variable_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      brand: stylex,
    });
  "#
);
