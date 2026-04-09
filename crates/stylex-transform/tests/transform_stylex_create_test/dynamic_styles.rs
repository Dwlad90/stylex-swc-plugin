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
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
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
