use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

use crate::utils::transform::stringify_js;

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
      &camel_cased,
      Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
      }),
      |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
      )
    ),
    stringify_js(
      &kebab_cased,
      Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
      }),
      |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
      )
    )
  );

  insta::assert_snapshot!(stringify_js(
    &camel_cased,
    Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None
    )
  ));

  // assert_eq!(
  //     stringify_js(&camel_cased),
  //     r#"import _inject from "@stylexjs/stylex/lib/stylex-inject";
  //     var _inject2 = _inject;
  //     import stylex from 'stylex';
  //     _inject2(".x1cfch2b{transition-property:margin-top}", 3000);
  //     "#
  // );
}

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
  ),
  transforms_valid_shorthands,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                overflow: 'hidden',
                borderStyle: 'dashed',
                borderWidth: 1
            }
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
  ),
  uses_stylex_include_correctly_with_member_expressions,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                ...stylex.include(importedStyles.foo)
            }
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
  ),
  using_stylex_include_keeps_the_compiled_object,
  r#"
        import stylex from 'stylex';

        const importedStyles = stylex.create({
            foo: {
                color: 'blue',
            },
        });


        const styles = stylex.create({
            foo: {
                ...stylex.include(importedStyles.foo),
                color: 'red',
            }
        });

        stylex.props(styles.foo);
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
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
