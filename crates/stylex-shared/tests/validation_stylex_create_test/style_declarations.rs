use rustc_hash::FxHashMap;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn invalid_property_non_static_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            root: {
              [backgroundColor]: 'red',
            }
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_boolean() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            default: {
              color: true,
            },
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn invalid_value_non_static_variable() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({
              root: {
                backgroundColor: backgroundColor,
              }
            });
          "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn invalid_value_non_static_function_call() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({
              root: {
                backgroundColor: generateBg(),
              }
            });
          "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Could not resolve the path to the imported file.")]
fn invalid_value_non_static_import_named() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            import {generateBg} from './other-file';
            const styles = stylex.create({
              root: {
                backgroundColor: generateBg(),
              }
            });
          "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Could not resolve the path to the imported file.\nPlease ensure that the theme file has a .stylex.js or .stylex.ts extension and follows the\nrules for defining variables"
)]
fn invalid_value_non_static_import_default() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            import generateBg from './other-file';
            const styles = stylex.create({
              root: {
                backgroundColor: generateBg(),
              }
            });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_empty_object() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            root: {
              color: {},
            },
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "A style array value can only contain strings or numbers.")]
fn invalid_value_array_of_objects() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            root: {
              transitionDuration: [[], {}],
            },
          });
        "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

#[ignore]
#[test]
#[should_panic(expected = "The pseudo selector or at-rule '::before' is not supported")]
fn invalid_object_value_contains_disallowed_key() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({
              root: {
                'color': {
                  '::before': 'blue'
                },
              },
            });
          "#,
    r#""#,
  )
}

#[test]
#[ignore = "This should throw an error but doesn't"]
#[should_panic(expected = "Invalid media query syntax")]
fn invalid_object_value_contains_invalid_media_query_syntax() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut StyleXOptionsParams {
          enable_media_query_order: Some(true),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                'color': {
                  '@media not ((not (min-width: 400px))': 'blue'
                },
              },
            });
          "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

#[test]
#[should_panic(expected = "Rule contains an unclosed function, css rule: * { color: var(--foo }")]
fn invalid_css_variable_unclosed_function() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut options = StyleXOptionsParams::default();
      let mut defined_vars = FxHashMap::default();
      defined_vars.insert("foo".to_string(), "1".to_string());
      options.defined_stylex_css_variables = Some(defined_vars);
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut options),
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({
              root: {
                color: 'var(--foo'
              }
            });
          "#,
    r#""#,
  )
}

#[test]
#[should_panic]
fn invalid_css_variable_unprefixed_custom_property() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut options = StyleXOptionsParams::default();
      let mut defined_vars = FxHashMap::default();
      defined_vars.insert("foo".to_string(), "1".to_string());
      options.defined_stylex_css_variables = Some(defined_vars);
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut options),
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({
              root: {
                color: 'var(foo'
              }
            });
          "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut options = StyleXOptionsParams::default();
    let mut defined_vars = FxHashMap::default();
    defined_vars.insert("foo".to_string(), "1".to_string());
    defined_vars.insert("bar".to_string(), "1".to_string());
    options.defined_stylex_css_variables = Some(defined_vars);
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut options),
    )
  },
  valid_css_variable_defined_custom_properties,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
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
