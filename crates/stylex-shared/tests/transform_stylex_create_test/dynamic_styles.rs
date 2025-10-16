use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  style_function,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: (color) => ({
              backgroundColor: 'red',
              color,
            })
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  style_function_and_object,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            one: (color) => ({
              color: color,
            }),
            two: {
              color: 'black',
            },
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  style_function_with_custom_properties,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: (bgColor, otherColor) => ({
              '--background-color': bgColor,
              '--otherColor': otherColor,
            }),
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  set_number_unit,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (width) => ({
                width,
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_mixed_values,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (width) => ({
        width,
        backgroundColor: 'red',
        height: width + 100,
      })
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        filename: swc_core::common::FileName::Real("MyComponent.js".into()),
        ..PluginPass::default()
      },
      Some(&mut config),
    )
  },
  set_custom_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            import {vars} from 'vars.stylex.js';

            export const styles = stylex.create({
              root: (width) => ({
                [vars.width]: width
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  valid_pseudo_class,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color) => ({
                backgroundColor: {
                  ':hover': color,
                },
                color: {
                  ':hover': color,
                }
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  pseudo_class_generated_order,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (hover, active, focus) => ({
                color: {
                  ':hover': hover,
                  ':active': active,
                  ':focus': focus,
                  ':nth-child(2n)': 'purple',
                },
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  before_and_after,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: (a, b) => ({
                '::before': {
                  color: a
                },
                '::after': {
                  color: b
                },
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  placeholder,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: (color) => ({
                '::placeholder': {
                  color,
                },
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  thumb,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: (width) => ({
                '::thumb': {
                  width,
                },
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  before_containing_pseudo_classes,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: (color) => ({
                '::before': {
                  color: {
                    default: 'red',
                    ':hover': color,
                  }
                },
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  media_queries,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (a, b, c) => ({
                width: {
                  default: 'color-mix(' + color + ', blue)',
                  '@media (min-width: 1000px)': b,
                  '@media (min-width: 2000px)': c,
                }
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  supports_queries,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (a, b, c) => ({
                color: {
                  default: a,
                  '@supports (hover: hover)': b,
                  '@supports not (hover: hover)': c,
                }
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  media_query_with_pseudo_classes,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (a, b, c) => ({
                fontSize: {
                  default: a,
                  '@media (min-width: 800px)': {
                    default: b,
                    ':hover': c
                  }
                }
              }),
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  template_literal_expressions,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color) => ({
                backgroundColor: `${color}`,
                color: `${color}px`,
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  binary_expressions,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (width, height) => ({
                width: width + 100,
                height: height * 2,
                margin: width - 50,
                padding: height / 2,
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  unary_expressions,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (value) => ({
                opacity: -value,
                transform: +value,
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  logical_expressions_safe_left_side,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color) => ({
                backgroundColor: color || 'red',
                color: color || 'black',
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  logical_expressions_safe_right_side,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color) => ({
                backgroundColor: 'red' || color,
                color: 'black' || color,
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  nullish_coalescing_safe_left_side,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color) => ({
                backgroundColor: color ?? 'red',
                color: color ?? 'black',
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  conditional_expressions_safe_branches,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color, isDark) => ({
                backgroundColor: isDark ? 'black' : 'white',
                color: isDark ? color : 'black',
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  conditional_expressions_safe_branches_parenthesized,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (color, isDark) => ({
                backgroundColor: isDark ? ('black') : 'white',
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  complex_nested_safe_expressions,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (width, height, color) => ({
                width: (width + 100) || 200,
                height: (height * 2) ?? 300,
                backgroundColor: `${color}` || 'red',
                color: (-color) || 'black',
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  complex_safe_ternary_expressions,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: (isDark, isLarge, isActive, width, height, color) => ({
                backgroundColor: isDark ? (isLarge ? 'black' : 'gray') : (isActive ? 'blue' : 'white'),
                color: isDark ? (color || 'white') : (color ?? 'black'),
                width: isLarge ? (width + 100) : (width - 50),
                height: isActive ? (height * 2) : (height / 2),
                margin: isDark ? ((width + height) || 20) : ((width - height) ?? 10),
                padding: isLarge ? ((width * height) + 50) : ((width / height) - 25),
                fontSize: isDark ? (isLarge ? (width + 20) : (width - 10)) : (isActive ? (height + 15) : (height - 5)),
                opacity: isLarge ? (isActive ? 1 : 0.8) : (isDark ? 0.9 : 0.7),
                transform: isActive ? (isLarge ? 'scale(1.2)' : 'scale(1.1)') : (isDark ? 'rotate(5deg)' : 'rotate(-5deg)'),
              })
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  nullish_coalescing_with_object_type,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      fn: (opt: { height?: number }) => ({
        height: opt.height ?? null,
      }),
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  nullish_coalescing_with_object_type_and_array,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      fn: (opt: { size?: 'xlarge' | 'large' | 'medium' | 'small' }) => ({
        borderRadius: {
          xlarge: 16,
          large: 12,
          medium: 8,
          small: 8,
        }[opt?.size ?? 'large'],
      }),
    });
  "#
);
