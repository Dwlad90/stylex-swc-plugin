use std::env;

use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

use crate::utils::transform::stringify_js;

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  supports_debug_data_haste_v1,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        foo: {
          color: 'red'
        },
        'bar-baz': {
          display: 'block'
        },
        1: {
          fontSize: '1em'
        }
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
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  supports_debug_data_haste_v2,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        foo: {
          color: 'red'
        },
        'bar-baz': {
          display: 'block'
        },
        1: {
          fontSize: '1em'
        }
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
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  supports_debug_data_commonjs_v1,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        foo: {
          color: 'red'
        },
        'bar-baz': {
          display: 'block'
        },
        1: {
          fontSize: '1em'
        }
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
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  supports_debug_data_commonjs_v2,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        foo: {
          color: 'red'
        },
        'bar-baz': {
          display: 'block'
        },
        1: {
          fontSize: '1em'
        }
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
  transforms_style_object,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
            }
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
  transforms_style_object_with_import_wildcard,
  r#"
        import * as foo from 'stylex';
        const styles = foo.create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
            }
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
  transforms_style_object_with_named_imports,
  r#"
        import {create} from 'stylex';
        const styles = create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
            }
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
  transforms_style_when_have_unassigned_variable,
  r#"
        import {create} from 'stylex';

        let color;

        const styles = create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
            }
        });

        color = 'red';
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
  transforms_style_object_with_custom_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                '--background-color': 'red',
            }
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
  transforms_style_object_with_custom_property_as_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                '--final-color': 'var(--background-color)',
            }
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
  transforms_style_object_with_webkit_property_as_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                WebkitBoxOrient: "vertical",
                WebkitLineClamp: 2,
                display: "-webkit-box",
                overflow: "hidden"
            }
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
  transforms_multiple_namespaces,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
            },
            default2: {
                color: 'blue',
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
  does_not_transform_attr_fn_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                content: 'attr(some-attribute)',
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
    PluginPass {
      filename: FileName::Real(
        format!("{}/MyComponent.js", env::current_dir().unwrap().display()).into(),
      ),
      ..Default::default()
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  does_not_add_unit_when_setting_variable_value,
  r#"
        import * as stylex from '@stylexjs/stylex';
        import {vars} from 'myTheme.stylex.js';
        const styles = stylex.create({
          default: {
            [vars.foo]: 500,
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
  transforms_style_object_with_gradient,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
              backgroundImage: {
                default: "linear-gradient(to bottom, rgb(214, 219, 220), white)",
                ["@media (prefers-color-scheme: dark)"]: "linear-gradient(to bottom, rgb(20, 22, 27), black)",
              }
            }
        });
    "#
);

#[test]
fn handles_camel_cased_transition_properties() {
  let camel_cased = "import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                transitionProperty: 'marginTop',
            },
        });";

  let kebab_cased = "import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                transitionProperty: 'margin-top',
            },
        });";

  assert_eq!(
    stringify_js(
      camel_cased,
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None
      )
    ),
    stringify_js(
      kebab_cased,
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None
      )
    )
  );

  insta::assert_snapshot!(stringify_js(
    camel_cased,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None
    )
  ));
}

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
  leaves_transition_properties_of_custom_properties_alone,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                transitionProperty: '--foo',
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
  transforms_nested_pseudo_class_to_css,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                ':hover': {
                    backgroundColor: 'red',
                    color: 'blue',
                },
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
  transforms_nested_pseudo_class_within_properties_to_css,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    ':hover': 'red',
                },
                color: {
                    ':hover': 'blue',
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_array_values_as_fallbacks,
  r#"
        import stylex from 'stylex';
            const styles = stylex.create({
            default: {
                position: ['sticky', 'fixed']
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
  transforms_array_values_as_fallbacks_within_media_query,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                position: {
                    default: 'fixed',
                    '@media (min-width: 768px)': ['sticky', 'fixed'],
                }
            },
        });
    "#
);

// TODO: add more vendor-prefixed properties and values
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
  transforms_properties_requiring_vendor_prefixes,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                userSelect: 'none',
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
  transforms_valid_shorthands,
  r#"
        const MEDIA_MOBILE = "@media (max-width: 700px)";

        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                overflow: 'hidden',
                borderStyle: 'dashed',
                borderWidth: {
                    default: "1px",
                    "@media (max-width: 700px)": "0",
                },
                borderColor: {
                    default: "rgba(131, 134, 135, 0)",
                    ":hover": "rgba(var(--xpue81e), var(--x1gflzcx), var(--x1363ko0), 0.1)",
                }
            }
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
  uses_stylex_first_that_works_correctly,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                position: stylex.firstThatWorks('sticky', 'fixed'),
            }
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
  transforms_complex_property_values_containing_custom_properties_variables,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                boxShadow: '0px 2px 4px var(--shadow-1)',
            }
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
  auto_expands_shorthands,
  r#"
        import stylex from 'stylex';
        const borderRadius = 2;
        const styles = stylex.create({
            default: {
                margin: 'calc((100% - 50px) * 0.5) 20px 0',
            },
            error: {
                borderColor: 'red blue',
                borderStyle: 'dashed',
                borderWidth: '0 0 2px 0',
            },
            root: {
                borderWidth: 1,
                borderStyle: 'solid',
                borderColor: 'var(--divider)',
                borderRadius: borderRadius * 2,
                borderBottomWidth: '5px',
                borderBottomStyle: 'solid',
                borderBottomColor: 'red',
            },
            short: {
                padding: 'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)',
                paddingTop: 0,
            },
            valid: {
                borderColor: 'green',
                borderStyle: 'solid',
                borderWidth: 1,
            }
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
  last_property_wins_even_if_shorthand,
  r#"
        import stylex from 'stylex';
        const borderRadius = 2;
        const styles = stylex.create({
            default: {
                marginTop: 5,
                marginEnd: 10,
                marginBottom: 15,
                marginStart: 20,
            },
            override: {
                marginBottom: 100,
                margin: 0,
            }
        });
        stylex(styles.default, styles.override);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| { StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None) },
  adds_null_for_constituent_properties_of_shorthands,
  r#"
    import stylex from 'stylex';
    const borderRadius = 2;
    export const styles = stylex.create({
      default: {
        margin: 'calc((100% - 50px) * 0.5) 20px 0',
      },
      error: {
        borderColor: 'red blue',
        borderStyle: 'dashed',
        borderWidth: '0 0 2px 0',
      },
      root: {
        borderWidth: 1,
        borderStyle: 'solid',
        borderColor: 'var(--divider)',
        borderRadius: borderRadius * 2,
        borderBottomWidth: '5px',
        borderBottomStyle: 'solid',
        borderBottomColor: 'red',
      },
      short: {
        padding: 'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)',
        paddingTop: 0,
      },
      shortReversed: {
        paddingTop: 0,
        padding: 'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)',
      },
      valid: {
        borderColor: 'green',
        borderStyle: 'solid',
        borderWidth: 1,
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
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::PropertySpecificity),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  can_leave_shorthands_as_is_when_configured,
  r#"
    import stylex from 'stylex';
    const borderRadius = 2;
    export const styles = stylex.create({
      default: {
        marginTop: 'calc((100% - 50px) * 0.5)',
        marginRight: 20,
        marginBottom: 0,
      },
      error: {
        borderVerticalColor: 'red',
        borderHorizontalColor: 'blue',
        borderStyle: 'dashed',
        borderBottomWidth: 2,
      },
      root: {
        borderWidth: 1,
        borderStyle: 'solid',
        borderColor: 'var(--divider)',
        borderRadius: borderRadius * 2,
        borderBottomWidth: 5,
        borderBottomStyle: 'solid',
        borderBottomColor: 'red',
      },
      short: {
        paddingVertical: 'calc((100% - 50px) * 0.5)',
        paddingHorizontal: 'var(--rightpadding, 20px)',
        paddingTop: 0,
      },
      shortReversed: {
        paddingTop: 0,
        paddingVertical: 'calc((100% - 50px) * 0.5)',
        paddingHorizontal: 'var(--rightpadding, 20px)',
      },
      valid: {
        borderColor: 'green',
        borderStyle: 'solid',
        borderWidth: 1,
      }
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
  transforms_style_object_with_key_containing_differend_types,
  r#"
        import stylex from 'stylex';

        const color = 'blue';
        const paddingTop = '1px';

        const marginRight = 'marginRight';

        const margin = {
          left: 'marginLeft',
        }

        const values = {
          paddingBottom: '2px',
          paddingLeft: '3px',
          paddingRight: '4px',
          marginRight: '5px',
          marginLeft: '6px',
          margin: {
            bottom: '7px',
          }
        };

        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                color: color,
                paddingTop,
                paddingBottom: values.paddingBottom,
                paddingLeft: values['paddingLeft'],
                paddingRight: values[`paddingRight`],
                marginRight: values[marginRight],
                marginLeft: values[margin.left],
                marginBottom: values.margin.bottom,
            }
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
  transforms_style_with_url_property,
  r#"
      import stylex from 'stylex';

      export const styles = stylex.create({
          default: {
            backgroundImage:'url("https://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")',
          },
          foo:{
            backgroundImage:'url("http://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")',
          },
          bar:{
            backgroundImage:'url("https://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")',
          },
          baz:{
            backgroundImage:'url("http://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")',
          },
          boo:{
            backgroundImage:'url("/photo-1634170380004-4b3b3b3b3b3b")',
          },
          far:{
            backgroundImage:'url("./photo-1634170380004-4b3b3b3b3b3b")',
          },
      });
    "#
);
