use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real("/stylex/packages/tokens.stylex.js".into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

fn stylex_transform_haste(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real("/stylex/packages/tokens.stylex.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
  })
}

fn stylex_transform_haste_with_filename(
  comments: TestComments,
  filename: &'static str,
) -> impl Pass {
  build_test_transform(comments, move |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(filename.into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
  })
}

// Cross-package commonJS tests need a filename that resolves to a real
// package.json on disk (the path resolver walks up to find one). We anchor
// these tests under the crate directory so package.json lookup succeeds.
fn commonjs_consumer_filename(rel: &str) -> String {
  format!(
    "{}/tests/__virtual__/app/{}",
    env!("CARGO_MANIFEST_DIR"),
    rel
  )
}

fn commonjs_root_dir() -> String {
  format!("{}/tests/__virtual__/app/", env!("CARGO_MANIFEST_DIR"))
}

fn stylex_transform_commonjs_consumer(comments: TestComments) -> impl Pass {
  let filename = commonjs_consumer_filename("src/components/Card.js");
  let root_dir = commonjs_root_dir();
  build_test_transform(comments, move |b| {
    b.with_filename(FileName::Real(filename.clone().into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
  })
}

fn stylex_transform_commonjs_consumer_with_filename(
  comments: TestComments,
  rel_filename: &'static str,
) -> impl Pass {
  let filename = commonjs_consumer_filename(rel_filename);
  let root_dir = commonjs_root_dir();
  build_test_transform(comments, move |b| {
    b.with_filename(FileName::Real(filename.clone().into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
  })
}

stylex_test!(
  transforms_nested_vars_with_conditionals,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested({
      button: {
        background: stylex.unstable_conditional({
          default: 'red',
          '@media (prefers-color-scheme: dark)': 'blue',
        }),
        color: 'white',
      },
    });
  "#
);

stylex_test!(
  transforms_nested_vars_from_commonjs_namespace_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const stylex = require('@stylexjs/stylex');
    export const vars = stylex.unstable_defineVarsNested({
      color: {
        primary: 'red',
      },
    });
  "#
);

stylex_test!(
  transforms_deeply_nested_vars_with_named_alias_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineVarsNested as defineVarsNested } from '@stylexjs/stylex';
    export const vars = defineVarsNested({
      button: {
        primary: {
          background: '#00ff00',
          border: {
            radius: '8px',
          },
        },
        secondary: {
          background: '#cccccc',
        },
      },
    });
  "#
);

stylex_test!(
  works_with_renamed_named_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineVarsNested as defineNested } from '@stylexjs/stylex';
    export const tokens = defineNested({
      spacing: {
        sm: '4px',
      },
    });
  "#
);

stylex_test!(
  transforms_nested_vars_from_commonjs_destructured_alias_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const { unstable_defineVarsNested: defineVarsNested } = require('@stylexjs/stylex');
    export const vars = defineVarsNested({
      color: {
        primary: 'red',
      },
    });
  "#
);

stylex_test!(
  cross_file_nested_token_access_in_stylex_create,
  |tr| stylex_transform_haste(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from 'tokens.stylex';
    export const styles = stylex.create({
      card: {
        backgroundColor: tokens.button.primary.background,
        color: tokens.button.primary.color,
      },
    });
  "#
);

stylex_test!(
  basic_nested_tokens,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      button: {
        background: 'red',
        color: 'blue',
      },
    });
  "#
);

stylex_test!(
  deeply_nested_tokens_3_levels,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      button: {
        primary: {
          background: '#00FF00',
        },
        secondary: {
          background: '#CCCCCC',
        },
      },
    });
  "#
);

stylex_test!(
  conditional_media_values_inside_nesting,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      button: {
        color: {
          default: 'blue',
          '@media (prefers-color-scheme: dark)': 'lightblue',
        },
      },
    });
  "#
);

stylex_test!(
  mixed_flat_and_nested_values,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      flatValue: 'red',
      nested: {
        deep: 'blue',
      },
    });
  "#
);

stylex_test!(
  works_with_named_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineVarsNested } from '@stylexjs/stylex';
    export const tokens = unstable_defineVarsNested({
      spacing: {
        sm: '4px',
        lg: '16px',
      },
    });
  "#
);

stylex_test!(
  produces_different_hashes_for_different_nested_keys,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      a: { x: 'red' },
      b: { x: 'blue' },
    });
  "#
);

stylex_test!(
  deeply_nested_conditional_with_supports,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      color: {
        primary: {
          default: 'blue',
          '@media (prefers-color-scheme: dark)': {
            default: 'lightblue',
            '@supports (color: oklch(0 0 0))': 'oklch(0.7 -0.3 -0.4)',
          },
        },
      },
    });
  "#
);

stylex_test!(
  works_with_stylex_unstable_conditional_for_media_values,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const DARK = '@media (prefers-color-scheme: dark)';
    export const tokens = stylex.unstable_defineVarsNested({
      button: {
        primary: {
          background: {
            default: stylex.unstable_conditional({ default: '#2563eb', [DARK]: '#3b82f6' }),
            hovered: stylex.unstable_conditional({ default: '#1d4ed8', [DARK]: '#2563eb' }),
          },
        },
      },
    });
  "#
);

stylex_test!(
  works_with_named_import_of_unstable_conditional,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineVarsNested, unstable_conditional as cond } from '@stylexjs/stylex';
    const DARK = '@media (prefers-color-scheme: dark)';
    export const tokens = unstable_defineVarsNested({
      color: {
        accent: cond({ default: '#2563eb', [DARK]: '#60a5fa' }),
      },
    });
  "#
);

stylex_test!(
  unstable_conditional_is_backward_compatible_heuristic_still_works_without_it,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineVarsNested({
      color: {
        accent: { default: 'blue', '@media (prefers-color-scheme: dark)': 'dark' },
      },
    });
  "#
);

stylex_test!(
  unstable_conditional_works_in_flat_define_vars_too,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const DARK = '@media (prefers-color-scheme: dark)';
    export const tokens = stylex.defineVars({
      accent: stylex.unstable_conditional({ default: 'blue', [DARK]: 'lightblue' }),
    });
  "#
);

stylex_test!(
  cross_file_deeply_nested_token_access_3_levels,
  |tr| stylex_transform_haste_with_filename(tr.comments.clone(), "/stylex/packages/component.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from 'tokens.stylex';
    export const styles = stylex.create({
      card: {
        backgroundColor: tokens.design.color.surface.primary,
      },
    });
  "#
);

stylex_test!(
  cross_file_mixed_flat_and_nested_access,
  |tr| stylex_transform_haste_with_filename(tr.comments.clone(), "/stylex/packages/component.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from 'tokens.stylex';
    export const styles = stylex.create({
      card: {
        color: tokens.accent,
        backgroundColor: tokens.button.bg,
      },
    });
  "#
);

stylex_test!(
  cross_file_nested_tokens_used_in_create_theme_flat_key_format,
  |tr| stylex_transform_haste_with_filename(
    tr.comments.clone(),
    "/stylex/packages/theme.stylex.js"
  ),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from 'tokens.stylex';
    export const darkTheme = stylex.createTheme(tokens, {
      'color.primary': 'lightblue',
      'color.secondary': 'darkgrey',
    });
  "#
);

stylex_test!(
  cross_package_nested_token_access_from_node_modules_in_stylex_create,
  |tr| stylex_transform_commonjs_consumer(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from '@design-system/tokens/src/tokens.stylex';
    export const styles = stylex.create({
      card: {
        color: tokens.color.accent,
        backgroundColor: tokens.color.surface,
        padding: tokens.spacing.md,
      },
    });
  "#
);

stylex_test!(
  cross_package_deeply_nested_token_access_3_levels_from_node_modules,
  |tr| stylex_transform_commonjs_consumer(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from '@design-system/tokens/src/tokens.stylex';
    export const styles = stylex.create({
      card: {
        backgroundColor: tokens.design.color.surface.primary,
      },
    });
  "#
);

stylex_test!(
  cross_package_create_theme_nested_overriding_tokens_from_node_modules,
  |tr| stylex_transform_commonjs_consumer_with_filename(
    tr.comments.clone(),
    "src/themes/dark.stylex.js"
  ),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { tokens } from '@design-system/tokens/src/tokens.stylex';
    export const darkTheme = stylex.createTheme(tokens, {
      'color.accent': 'lightblue',
      'color.bg': '#111',
    });
  "#
);
