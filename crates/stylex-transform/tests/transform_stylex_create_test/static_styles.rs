use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  unused_style_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  style_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  nested_referenced_style_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          function fooBar() {
            const styles = stylex.create({
              root: {
                backgroundColor: 'red',
                color: 'blue',
              }
            });
            console.log(styles);
          }
        "#
);

stylex_test!(
  multiple_nested_referenced_style_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          function fooBar() {
            const styles = stylex.create({
              root: {
                backgroundColor: 'red',
                color: 'blue',
              }
            });
            const styles2 = stylex.create({
              root: {
                backgroundColor: 'blue',
                color: 'green',
              }
            });
            console.log(styles);
            console.log(styles2);
          }
          export const otherFunction = () => {
            const styles3 = stylex.create({
              root: {
                backgroundColor: 'green',
                color: 'red',
              }
            });
            console.log(styles3);
          }
        "#
);

stylex_test!(
  style_object_multiple,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  style_object_with_custom_properties,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  style_object_with_shortform_properties,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  style_object_with_shortform_properties_property_specificity,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(false))
      .with_style_resolution(StyleResolution::PropertySpecificity)
  }),
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

stylex_test!(
  style_object_requiring_vendor_prefixes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            root: {
              userSelect: 'none',
            },
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
  }),
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

stylex_test!(
  set_transition_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                transitionProperty: 'marginTop',
              },
            });
          "#
);

stylex_test!(
  set_transition_property_kebab_cased,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                transitionProperty: 'margin-top',
              },
            });
          "#
);

stylex_test!(
  set_transition_property_custom_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                transitionProperty: '--foo',
              },
            });
          "#
);

stylex_test!(
  set_transition_property_multi_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  set_will_change,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'insetInlineStart',
              },
            });
          "#
);

stylex_test!(
  set_will_change_kebab_cased,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'inset-inline-start',
              },
            });
          "#
);

stylex_test!(
  set_will_change_custom_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: '--foo',
              },
            });
          "#
);

stylex_test!(
  set_will_change_multi_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  set_will_change_keyword,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                willChange: 'scroll-position'
              }
            });
          "#
);

stylex_test!(
  use_attr_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                content: 'attr(some-attribute)',
              },
            });
          "#
);

stylex_test!(
  use_array_fallbacks,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                position: ['sticky', 'fixed']
              },
            });
          "#
);

stylex_test!(
  use_css_variable,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: 'var(--background-color)',
              }
            });
          "#
);

stylex_test!(
  use_string_containing_css_variables,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                boxShadow: '0px 2px 4px var(--shadow-1)',
              }
            });
          "#
);

stylex_test!(
  args_value_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                position: stylex.firstThatWorks('sticky', 'fixed'),
              }
            });
          "#
);

stylex_test!(
  args_value_var,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('red', 'var(--color)'),
              }
            });
          "#
);

stylex_test!(
  args_var_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'red'),
              }
            });
          "#
);

stylex_test!(
  args_var_var_var,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'var(--secondColor)', 'var(--thirdColor)'),
              }
            });
          "#
);

stylex_test!(
  args_var_var,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('var(--color)', 'var(--otherColor)'),
              }
            });
          "#
);

stylex_test!(
  args_func_var_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: stylex.firstThatWorks('color-mix(in srgb, currentColor 20%, transparent)', 'var(--color)', 'red'),
              }
            });
          "#
);

stylex_test!(
  args_func_var_value_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  invalid_pseudo_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  valid_pseudo_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  pseudo_class_generated_order,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  pseudo_class_generated_order_nested_same_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  pseudo_class_generated_order_nested_different_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  attribute_selector_with_pseudo_class_nested_same_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                color: {
                  ':hover': {
                    '[data-state="open"]': 'red',
                  },
                  '[data-state="open"]': {
                    ':hover': 'red',
                  },
                },
              },
            });
          "#
);

stylex_test!(
  pseudo_class_with_array_fallbacks,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  before_and_after,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  placeholder,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  thumb,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

// BUG: Generates invalid CSS, need to revisit this API
stylex_test!(
  before_containing_pseudo_classes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  media_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  media_queries_with_last_query_wins,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  media_queries_without_last_query_wins,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(false)
  }),
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

stylex_test!(
  media_queries_without_last_query_wins_v2,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(true)
  }),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              root: {
                backgroundColor: {
                  default: 'red',
                  '@media screen and (max-width: 900px)': 'blue',
                  '@media screen and (max-width: 500px)': 'purple',
                  '@media screen and (max-width: 400px)': 'green',
                }
              },
            });
          "#
);

stylex_test!(
  supports_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  media_query_with_pseudo_classes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  media_query_with_array_fallbacks,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  legacy_compound_hover_after_selector_as_single_key,
  r#"
              import * as stylex from '@stylexjs/stylex';
              export const styles = stylex.create({
                foo: {
                  ':hover::after': {
                    color: 'red',
                  },
                },
              });
            "#
);

stylex_test!(
  compound_hover_after_selector_as_single_key,
  r#"
              import * as stylex from '@stylexjs/stylex';
              export const styles = stylex.create({
                foo: {
                  color: {
                    default: 'red',
                    ':hover::after': 'blue',
                  },
                },
              });
            "#
);
