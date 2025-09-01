use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleResolution, StyleXOptionsParams},
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  unused_style_object,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  style_object,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  style_object_multiple,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: {
              backgroundColor: 'red',
            },
            other: {
              color: 'blue',
            },
            'bar-baz': {
              color: 'green',
            },
            1: {
              color: 'blue',
            },
            [2]: {
              color: 'purple',
            },
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  style_object_with_custom_properties,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: {
              '--background-color': 'red',
              '--otherColor': 'green',
              '--foo': 10
            }
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  style_object_with_shortform_properties,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const borderRadius = 2;
          export const styles = stylex.create({
            error: {
              borderColor: 'red blue',
              borderStyle: 'dashed solid',
              borderWidth: '0 0 2px 0',
              margin: 'calc((100% - 50px) * 0.5) 20px 0',
              padding: 'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)',
            },
            short: {
              borderBottomWidth: '5px',
              borderBottomStyle: 'solid',
              borderBottomColor: 'red',
              borderColor: 'var(--divider)',
              borderRadius: borderRadius * 2,
              borderStyle: 'solid',
              borderWidth: 1,
              marginTop: 'calc((100% - 50px) * 0.5)',
              marginRight: 20,
              marginBottom: 0,
              paddingTop: 0,
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
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(false),
      style_resolution: Some(StyleResolution::PropertySpecificity),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  style_object_with_shortform_properties_property_specificity,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const borderRadius = 2;
          export const styles = stylex.create({
            error: {
              borderColor: 'red blue',
              borderStyle: 'dashed solid',
              borderWidth: '0 0 2px 0',
              margin: 'calc((100% - 50px) * 0.5) 20px 0',
              padding: 'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)',
            },
            short: {
              borderBottomWidth: '5px',
              borderBottomStyle: 'solid',
              borderBottomColor: 'red',
              borderColor: 'var(--divider)',
              borderRadius: borderRadius * 2,
              borderStyle: 'solid',
              borderWidth: 1,
              marginTop: 'calc((100% - 50px) * 0.5)',
              marginRight: 20,
              marginBottom: 0,
              paddingTop: 0,
            },
          });
        "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  style_object_requiring_vendor_prefixes,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: {
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
  |tr| {
    let mut config = StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_with_pass(
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
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_transition_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                transitionProperty: 'marginTop',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_transition_property_kebab_cased,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                transitionProperty: 'margin-top',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_transition_property_custom_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_transition_property_multi_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              one: {
                transitionProperty: 'opacity, insetInlineStart',
              },
              two: {
                transitionProperty: 'opacity, inset-inline-start',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_will_change,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'insetInlineStart',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_will_change_kebab_cased,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'inset-inline-start',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_will_change_custom_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: '--foo',
              },
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_will_change_multi_property,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              one: {
                willChange: 'opacity, insetInlineStart',
              },
              two: {
                willChange: 'opacity, inset-inline-start',
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  set_will_change_keyword,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'scroll-position'
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  use_attr_function,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  use_array_fallbacks,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  use_css_variable,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: 'var(--background-color)',
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  use_string_containing_css_variables,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_value_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_value_var,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('red', 'var(--color)'),
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_var_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'red'),
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_var_var_var,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'var(--secondColor)', 'var(--thirdColor)'),
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_var_var,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'var(--otherColor)'),
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_func_var_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('color-mix(in srgb, currentColor 20%, transparent)', 'var(--color)', 'red'),
              }
            });
          "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  args_func_var_value_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('color-mix(in srgb, currentColor 20%, transparent)', 'var(--color)', 'red', 'green'),
              }
            });
          "#
);

#[test]
#[ignore]
fn stylex_types_functions_todo() {
  // Placeholder for describe.skip('function value: stylex.types.*()') in JS tests.
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  invalid_pseudo_class,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: {
                  ':invalidpseudo': 'blue'
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  valid_pseudo_class,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  pseudo_class_generated_order,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: {
                  ':hover': 'blue',
                  ':active':'red',
                  ':focus': 'yellow',
                  ':nth-child(2n)': 'purple',
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  pseudo_class_generated_order_nested_same_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: {
                  ':hover': {
                    ':active':'red',
                  },
                  ':active': {
                    ':hover':'red',
                  },
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  pseudo_class_generated_order_nested_different_value,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: {
                  ':hover': {
                    ':active':'red',
                  },
                  ':active': {
                    ':hover':'green',
                  },
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  pseudo_class_with_array_fallbacks,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                position: {
                  ':hover': ['sticky', 'fixed'],
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  before_and_after,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: {
                '::before': {
                  color: 'red'
                },
                '::after': {
                  color: 'blue'
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  placeholder,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: {
                '::placeholder': {
                  color: 'gray',
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  thumb,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: {
                '::thumb': {
                  width: 16,
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  before_containing_pseudo_classes,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              foo: {
                '::before': {
                  color: {
                    default: 'red',
                    ':hover': 'blue',
                  }
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  media_queries,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: {
                  default: 'red',
                  '@media (min-width: 1000px)': 'blue',
                  '@media (min-width: 2000px)': 'purple',
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
    let mut options = StyleXOptionsParams {
      enable_media_query_order: Some(true),
      ..Default::default()
    };
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut options),
    )
  },
  media_queries_with_last_query_wins,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: {
                  default: 'red',
                  '@media (max-width: 900px)': 'blue',
                  '@media (max-width: 500px)': 'purple',
                  '@media (max-width: 400px)': 'green',
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  supports_queries,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: {
                  default:'red',
                  '@supports (hover: hover)': 'blue',
                  '@supports not (hover: hover)': 'purple',
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  media_query_with_pseudo_classes,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                fontSize: {
                  default: '1rem',
                  '@media (min-width: 800px)': {
                    default: '2rem',
                    ':hover': '2.2rem'
                  }
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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  media_query_with_array_fallbacks,
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              default: {
                position: {
                  default: 'fixed',
                  '@media (min-width: 768px)': ['sticky', 'fixed'],
                }
              },
            });
          "#
);
