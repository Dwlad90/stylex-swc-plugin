use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
  set_mixed_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  set_custom_property,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real("MyComponent.js".into()))
      .with_unstable_module_resolution(ModuleResolution {
        root_dir: None,
        theme_file_extension: None,
        ..ModuleResolution::haste(None)
      })
      .with_runtime_injection()
  }),
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
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

stylex_test!(
  dynamic_style_in_after_generates_valid_at_property_with_inherits,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      repro: (color) => ({
        '::after': {
          color,
        },
      }),
    });
  "#
);

stylex_test!(
  dynamic_style_in_hover_generates_at_property_with_inherits_false,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      repro: (color) => ({
        ':hover': {
          color,
        },
      }),
    });
  "#
);

// The module reported in issue #1251, verbatim. Neither variable sits on a
// pseudo element, so both `@property` rules must declare `inherits: false`.
stylex_test!(
  default_and_hover_dynamic_values_generate_at_property_with_inherits_false,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: (backgroundColor, backgroundColorHover) => ({
        backgroundColor: {
          default: backgroundColor,
          ':hover': backgroundColorHover,
        },
      }),
    });
  "#
);

stylex_test!(
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

stylex_test!(
  media_query_values_with_nullish_coalescing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (a, b, c) => ({
        fontSize: {
          default: a ? '16px' : undefined,
          '@media (min-width: 800px)': b ? '18px' : undefined,
          '@media (min-width: 1280px)': c ? '20px' : undefined,
        }
      }),
    });
    stylex.props(styles.root(true, false, true));
  "#
);

stylex_test!(
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

stylex_test!(
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

stylex_test!(
  template_literal_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  binary_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  unary_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  logical_expressions_safe_left_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  logical_expressions_safe_right_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  nullish_coalescing_safe_left_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  conditional_expressions_safe_branches,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  conditional_expressions_safe_branches_parenthesized,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (color, isDark) => ({
        backgroundColor: isDark ? ('black') : 'white',
      })
    });
  "#
);

stylex_test!(
  complex_nested_safe_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  complex_safe_ternary_expressions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  nullish_coalescing_with_object_type,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      fn: (opt: { height?: number }) => ({
          height: opt.height ?? null,
        }),
      });
  "#
);

stylex_test!(
  nullish_coalescing_with_object_type_and_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  nullish_coalescing_with_boolean_type,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      fn: (opt?: { isPressed: boolean }) => ({
          outline: { true: 'red', false: 'blue' }[String(!!opt?.isPressed)],
        }),
      });
  "#
);

// ──────────────────────────────────────────────
// A dynamic parameter that shadows an imported binding (#1266)
//
// Every case below aborted the build before the import lookup compared the
// binding rather than the name: the parameter resolved to the theme it shadows,
// evaluation answered a confident theme reference, and a theme reference has no
// expression form for the style-value consumer to emit.
//
// These run under `haste` resolution and a real filename because a theme import
// has to resolve for the case to be about the shadowing rather than about the
// path.
// ──────────────────────────────────────────────

fn shadowing_transform(comments: TestComments) -> impl Pass {
  stylex_transform(comments, |b| {
    b.with_filename(swc_core::common::FileName::Real("MyComponent.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_runtime_injection()
  })
}

stylex_test!(
  dynamic_param_shadows_a_named_theme_import,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      zIndex: (zIndex) => ({ zIndex }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_an_aliased_theme_import,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex as zi } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zi._10 },
      dyn: (zi) => ({ zIndex: zi }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_theme_import_referenced_nowhere_else,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (zIndex) => ({ zIndex }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_theme_import_used_only_outside_create,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const raised = zIndex._10;

    export const styles = stylex.create({
      dyn: (zIndex) => ({ zIndex }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_theme_import_read_by_a_sibling_key,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (zIndex) => ({
        zIndex,
        ':hover': { zIndex: 1 },
      }),
      raised: { zIndex: zIndex._10 },
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_theme_import_inside_nested_conditions,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({
        zIndex: {
          default: zIndex,
          ':hover': {
            default: zIndex,
            '@media (min-width: 600px)': {
              default: zIndex,
              ':focus': zIndex,
            },
          },
        },
      }),
    });
  "#
);

stylex_test!(
  a_theme_import_read_as_a_computed_key_beside_a_dynamic_param,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { vars } from 'vars.stylex.js';

    export const styles = stylex.create({
      wrapper: { [vars.color]: 'red' },
      dyn: (color) => ({ [vars.color]: color }),
    });
  "#
);

// The next two shadow a namespace and a default import rather than a named one.
// Their `get_import_from` arms already compared the binding, so the shadowing
// half of each was never broken -- they are here so a later edit cannot regress
// all three arms to a name match at once.
//
// Neither snapshot is a parity claim. Measured against `@stylexjs/babel-plugin`
// 0.19.0, both import kinds diverge *without any shadowing*: it refuses a
// namespace theme import with `Referenced constant is not defined.` and a default
// theme import with the imported-file evaluation error, where we accept both and
// answer a theme reference. The divergence is about the import kind, not about
// the parameter, and it is tracked separately.
stylex_test!(
  dynamic_param_shadows_a_namespace_theme_import,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as tokens from 'tokens.stylex.js';

    export const styles = stylex.create({
      wrapper: { color: tokens.color },
      dyn: (tokens) => ({ color: tokens }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_default_theme_import,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({
      wrapper: { color: tokens.color },
      dyn: (tokens) => ({ color: tokens }),
    });
  "#
);

stylex_test!(
  dynamic_param_shadows_a_module_level_const,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const gap = '10px';

    export const styles = stylex.create({
      wrapper: { rowGap: gap },
      dyn: (gap) => ({ rowGap: gap }),
    });
  "#
);

stylex_test!(
  a_theme_import_read_beside_an_unshadowed_dynamic_param,
  |tr| shadowing_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (level) => ({ zIndex: level }),
    });
  "#
);
