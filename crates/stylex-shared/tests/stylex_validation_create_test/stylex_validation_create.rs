use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn must_be_bound_to_a_variable() {
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
            import stylex from "@stylexjs/stylex";

            stylex.create({});
        "#,
    r#""#,
  )
}

#[test]
fn can_be_item_of_array() {
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
            import stylex from "@stylexjs/stylex";

            export const lotsOfStyles = [stylex.create({})];
        "#,
    r#"
      import stylex from "@stylexjs/stylex";
      export const lotsOfStyles = [
          {}
      ];
    "#,
  )
}

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn must_be_called_at_top_level() {
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
            import stylex from "@stylexjs/stylex";

            if (bar) {
                const styles = stylex.create({});
            }
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object.")]
fn its_only_argument_must_be_a_single_object_non_object() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create(genStyles());
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_argument() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_illegal_argument_length() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({}, {});
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  its_only_argument_must_be_a_single_object_correct_argument_length,
  r#"
        import s from "@stylexjs/stylex";

        const c = s.create({
            base: {
                backgroundColor: 'red',
            },
        });
    "#
);

#[test]
#[should_panic(expected = "A stylex namespace must be an object.")]
fn namespace_values_must_be_an_object() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                namespace: false,
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn namespace_keys_must_be_a_static_value() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                [root]: {
                    backgroundColor: 'red',
                }
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn properties_must_be_a_static_value() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                root: {
                    [backgroundColor]: 'red',
                }
            });
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  namespace_values_can_be_an_empty_object,
  r#"
        import s from "@stylexjs/stylex";

        const c = s.create({
            namespace: {},
        });
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  values_can_be_static_arrays_of_number_or_string_in_stylex_create,
  r#"
    import stylex from "@stylexjs/stylex";

    const styles = stylex.create({
        root: {
            padding: 5,
        },
        rootRoot: {
            backgroundColor: 'red',
        },
        default: {
            transitionDuration: [500],
        },
        defaultDefault: {
            transitionDuration: ['0.5s', '1s'],
        }
    });
    "#
);

#[test]
#[should_panic(expected = "A style array value can only contain strings or numbers.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    transitionDuration: [[], {}],
                },
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_illegal_prop() {
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
            import stylex from "@stylexjs/stylex";

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
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_var() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    backgroundColor: backgroundColor,
                },
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_fn() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    backgroundColor: generateBg(),
                },
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Could not resolve the path to the imported file.\nPlease ensure that the theme file has a .stylex.js or .stylex.ts file extension and follows the\nrules for defining variariables:\n\nhttps://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables"
)]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_named_import_fn()
 {
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
          import stylex from 'stylex';
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
  expected = "Could not resolve the path to the imported file.\nPlease ensure that the theme file has a .stylex.js or .stylex.ts file extension and follows the\nrules for defining variariables:\n\nhttps://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables"
)]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_named_default_fn()
 {
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
          import stylex from 'stylex';
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

#[test]
#[should_panic(expected = "Unsupported expression: FunctionDeclaration")]
fn can_evaluate_single_expr_function_calls() {
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
          import stylex from 'stylex';
          function generateBg () {
            return 'red';
          };
          export const styles = stylex.create({
            root: {
              backgroundColor: generateBg(),
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
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  can_evaluate_single_expr_arrow_function_calls,
  r#"
      import stylex from 'stylex';
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
  can_evaluate_single_expr_arrow_function_calls_with_args,
  r#"
      import stylex from 'stylex';
      const generateBg = color => color + "d";
      export const styles = stylex.create({
        root: {
          backgroundColor: generateBg('re'),
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
  can_evaluate_single_expr_arrow_function_calls_in_object,
  r#"
      import stylex from 'stylex';
      const fns = {
        generateBg: () => "red",
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
  can_evaluate_single_expr_arrow_function_calls_in_object_with_args,
  r#"
      import stylex from 'stylex';
      const fns = {
        generateBg: (color) => "re" + color,
      };
      export const styles = stylex.create({
        root: {
          backgroundColor: fns.generateBg('d'),
        }
      });
    "#
);

#[test]
#[should_panic(expected = "Unsupported expression: Unknown\n\n")]
fn can_evaluate_single_expr_function_calls_in_object() {
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
          import stylex from 'stylex';
          const fns = {
            generateBg: function generateBg () {
              return 'red';
            },
          };
          export const styles = stylex.create({
            root: {
              backgroundColor: fns.generateBg(),
            }
          });
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  values_can_reference_local_bindings_in_stylex_create,
  r#"
    import stylex from "@stylexjs/stylex";

    const bgRed = 'red';
    const bgGray = '#eee';

    const styles = stylex.create({
        var:{
            backgroundColor: bgRed,
        },
        varBg:{
            backgroundColor: bgGray,
        }
    });
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  values_can_be_pure_complex_expressions_in_stylex_create,
  r#"
    import stylex from "@stylexjs/stylex";

    const borderRadius = 2;

    const styles = stylex.create({
        root:{
            borderRadius: borderRadius * 2,
        },
    });
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  values_can_be_template_literal_expressions_in_stylex_create,
  r#"
    import stylex from "@stylexjs/stylex";

    const borderSize = 2;

    const styles = stylex.create({
        root:{
            borderRadius: `${borderSize * 2}px`,
        },
    });
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  pseudo_classes_must_start_with_colon_character,
  r#"
    import stylex from "@stylexjs/stylex";

    const borderSize = 2;

    const styles = stylex.create({
        default: {
            ':hover': {},
        },
    });
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  pseudo_classes_invalid_pseudo,
  r#"
        import stylex from "@stylexjs/stylex";

        const styles = stylex.create({
            default: {
                'color': {
                    default: 'black',
                    ':hover': 'blue'
                },
            },
        });
    "#
);

#[test]
#[should_panic(expected = "Invalid pseudo or at-rule.")]
fn pseudo_classes_throw_invalid_pseudo() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    'color': {
                        default: 'black',
                        '&:hover': 'blue'
                    },
                },
            });
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  pseudo_classes_cannot_be_nested,
  r#"
        import stylex from "@stylexjs/stylex";

        const styles = stylex.create({
            default: {
                ':hover': {
                    ':active': {},
                },
            },
        });
    "#
);

#[test]
#[should_panic(expected = "Object spreads are not allowed in stylex.create call.")]
fn throws_on_object_spread_in_stylex_create() {
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
        import stylex from 'stylex';

        const shared = { foo: { color: 'red' } };

        const styles = stylex.create({
          ...shared,
          bar: { color: 'blue' }
        });
        "#,
    r#""#,
  )
}
