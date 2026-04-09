use crate::utils::prelude::*;
use rustc_hash::FxHashMap;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test_panic!(
  invalid_property_non_static_value,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        [backgroundColor]: 'red',
      }
    });
  "#
);

stylex_test_panic!(
  invalid_value_boolean,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      default: {
        color: true,
      },
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_variable,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        backgroundColor: backgroundColor,
      }
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_function_call,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        backgroundColor: generateBg(),
      }
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_import_named,
  "Could not resolve the path to the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import {generateBg} from './other-file';
    const styles = stylex.create({
      root: {
        backgroundColor: generateBg(),
      }
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_import_default,
  "Could not resolve the path to the imported file.\nPlease ensure that the theme file has a .stylex.js or .stylex.ts extension and follows the\nrules for defining variables",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import generateBg from './other-file';
    const styles = stylex.create({
      root: {
        backgroundColor: generateBg(),
      }
    });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_empty_object,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: {},
      },
    });
  "#
);

stylex_test_panic!(
  invalid_value_array_of_objects,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        transitionDuration: [[], {}],
      },
    });
  "#
);

stylex_test!(
  valid_value_number,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        padding: 5,
      }
    });
  "#
);

stylex_test!(
  valid_value_string,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        backgroundColor: 'red',
      }
    });
  "#
);

stylex_test!(
  valid_value_array_of_numbers,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        transitionDuration: [500],
      },
    });
  "#
);

stylex_test!(
  valid_value_array_of_strings,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        transitionDuration: ['0.5s'],
      },
    });
  "#
);

stylex_test!(
  valid_value_single_expr_function_call,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const generateBg = () => 'red';
    export const styles = stylex.create({
      root: {
        backgroundColor: generateBg(),
      }
    });
  "#
);

stylex_test!(
  valid_value_single_expr_function_call_in_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const fns = {
      generateBg: () => 'red',
    };
    export const styles = stylex.create({
      root: {
        backgroundColor: fns.generateBg(),
      }
    });
  "#
);

stylex_test!(
  valid_value_local_variable,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const bg = '#eee';
    const styles = stylex.create({
      root: {
        backgroundColor: bg,
      }
    });
  "#
);

stylex_test!(
  valid_value_pure_complex_expression,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const borderRadius = 2;
    const styles = stylex.create({
      root: {
        borderRadius: borderRadius * 2,
      }
    });
  "#
);

stylex_test!(
  valid_value_template_literal_expressions,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const borderSize = 2;
    const styles = stylex.create({
      root: {
        borderRadius: `${borderSize * 2}px`,
      }
    });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_object_value_contains_disallowed_key,
  "The pseudo selector or at-rule '::before' is not supported",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        'color': {
          '::before': 'blue'
        },
      },
    });
  "#
);

stylex_test_panic!(
  #[ignore = "This should throw an error but doesn't"],
  invalid_object_value_contains_invalid_media_query_syntax,
  "Invalid media query syntax",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_enable_media_query_order(true).with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        'color': {
          '@media not ((not (min-width: 400px))': 'blue'
        },
      },
    });
  "#
);

stylex_test!(
  valid_object_value_key_is_default,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: {
          default: 'red'
        }
      },
    });
  "#
);

stylex_test!(
  valid_object_value_key_starts_with_colon,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: {
          ':hover': 'green'
        }
      },
    });
  "#
);

stylex_test!(
  valid_object_value_multiple_valid_keys,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: {
          default: 'red',
          ':hover': 'green'
        }
      },
    });
  "#
);

stylex_test!(
  valid_object_value_nested_pseudo_classes,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        ':hover': {
          ':active': 'red'
        },
      },
    });
  "#
);

stylex_test_panic!(
  invalid_css_variable_unclosed_function,
  "Rule contains an unclosed function, css rule: * { color: var(--foo }",
  |tr| {
    let mut defined_vars = FxHashMap::default();
    defined_vars.insert("foo".to_string(), "1".to_string());
    stylex_transform(tr.comments.clone(), |b| {
      b.with_defined_stylex_css_variables(defined_vars)
        .with_runtime_injection()
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: 'var(--foo'
      }
    });
  "#
);

stylex_test_panic!(
  invalid_css_variable_unprefixed_custom_property,
  "Rule contains an unclosed function",
  |tr| {
    let mut defined_vars = FxHashMap::default();
    defined_vars.insert("foo".to_string(), "1".to_string());
    stylex_transform(tr.comments.clone(), |b| {
      b.with_defined_stylex_css_variables(defined_vars)
        .with_runtime_injection()
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: 'var(foo'
      }
    });
  "#
);

stylex_test!(
  valid_css_variable_defined_custom_properties,
  |tr| {
    let mut defined_vars = FxHashMap::default();
    defined_vars.insert("foo".to_string(), "1".to_string());
    defined_vars.insert("bar".to_string(), "1".to_string());
    stylex_transform(tr.comments.clone(), |b| {
      b.with_defined_stylex_css_variables(defined_vars)
        .with_runtime_injection()
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        backgroundColor: 'var(--foo)',
        color: 'var(--bar)'
      }
    });
  "#
);

stylex_test!(
  valid_css_variable_undefined_custom_properties,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        color: 'var(--bar)'
      }
    });
  "#
);

stylex_test!(
  legacy_pseudo_classes_must_start_with_colon_character,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        ':hover': {},
      },
    });
  "#
);
