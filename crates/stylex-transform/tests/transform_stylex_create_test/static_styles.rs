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
  direct_member_access_stylex_create,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const root = stylex.create({
      root: { display: "flex" },
    }).root;
  "#
);

stylex_test!(
  direct_computed_member_access_stylex_create,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const root = stylex.create({
      root: { display: "flex" },
    })["root"];
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
  style_object_with_system_font_family_list,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        fontFamily:
          '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
      },
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
        root_dir: None,
        theme_file_extension: None,
        ..ModuleResolution::haste(None)
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
  // Placeholder for `function value: stylex.types.*()` coverage.
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
  after_with_multiple_pseudo_class_conditions,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      button: {
        '::after': {
          content: '""',
          boxShadow: {
            default: '0 0 0 1px gray',
            ':hover': '0 0 0 1px blue',
            ':active': '0 0 0 1px darkblue',
          },
        },
      },
    });
  "#
);

stylex_test!(
  media_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
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
      .with_runtime_injection()
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
  media_queries_with_last_query_wins_over_a_media_type,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(true)
      .with_runtime_injection()
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
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
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

stylex_test!(
  create_with_position_try_object_rtl,
  r#"
    import * as stylex from '@stylexjs/stylex';

    const positionTryObject = stylex.positionTry({ top: '0', left: '10px' });

    export const styles = stylex.create({ foo: { positionTryFallbacks: positionTryObject } });
  "#
);

stylex_test!(
  create_with_position_try_object_logical_rtl,
  r#"
    import * as stylex from '@stylexjs/stylex';

    const positionTryObject = stylex.positionTry({ insetInlineStart: '0', top: '10px' });

    export const styles = stylex.create({ foo: { positionTryFallbacks: positionTryObject } });
  "#
);

stylex_test!(
  create_background_position_without_rtl,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ foo: { backgroundPosition: 'center' } });
  "#
);

stylex_test!(
  create_within_first_that_works_and_rtl,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      foo: { float: stylex.firstThatWorks('inline-start', 'left') },
    });
  "#
);

stylex_test!(
  direct_member_access_stylex_create_parenthesized,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const root = (stylex.create({
      root: { display: "flex" },
    })).root;
  "#
);

stylex_test!(
  direct_member_access_stylex_create_non_null_assertion,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const root = stylex.create({
      root: { display: "flex" },
    })!.root;
  "#
);

stylex_test!(
  direct_member_access_stylex_create_optional_chain,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const root = stylex.create({
      root: { display: "flex" },
    })?.root;
  "#
);

// A call bound to a top-level pattern is already program level, so its result
// stays inline rather than being hoisted into a temporary.
stylex_test!(
  destructured_export_of_a_create_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const { foo } = stylex.create({
      foo: {
        color: 'red',
      },
    });
  "#
);

stylex_test!(
  destructured_statement_of_a_create_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const { foo } = stylex.create({
      foo: {
        color: 'red',
      },
    });
    console.log(foo);
  "#
);

// ──────────────────────────────────────────────
// A constant named after the name an import was aliased away from.
//
// `import { "spacing" as sp }` binds `sp`; `spacing` is free for the module to
// declare, and a reference to it names that declaration. The import lookup used
// to answer for the aliased-away name and the declaration was never read, so
// these two cases are what deleting that fallback buys: the declaration folds,
// and the alias's own binding still resolves to the theme it names.
//
// Measured against `@stylexjs/babel-plugin` 0.19.0 as
// `modules-1266-a-constant-named-after-an-aliased-away-import` in the parity
// corpus.
// ──────────────────────────────────────────────

fn aliased_import_transform(comments: TestComments) -> impl Pass {
  stylex_transform(comments, |b| {
    b.with_filename(swc_core::common::FileName::Real("MyComponent.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_runtime_injection()
  })
}

stylex_test!(
  a_constant_named_after_an_aliased_away_import_is_what_folds,
  |tr| aliased_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    const spacing = '4px';

    export const styles = stylex.create({
      wrapper: { padding: spacing, margin: sp.small },
    });
  "#
);

// The identifier spelling of the alias, so both spellings of an imported name
// are pinned to the same answer.
stylex_test!(
  a_constant_named_after_an_identifier_aliased_import_is_what_folds,
  |tr| aliased_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { spacing as sp } from 'tokens.stylex.js';

    const spacing = '4px';

    export const styles = stylex.create({
      wrapper: { padding: spacing, margin: sp.small },
    });
  "#
);

// Two specifiers of one declaration aliased away from names the module declares
// itself, read in one style object -- so the answer is per specifier and per
// reference, not "this declaration was mentioned".
stylex_test!(
  two_aliased_away_names_declared_beside_their_import,
  |tr| aliased_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp, colors as c } from 'tokens.stylex.js';

    const spacing = '4px';
    const colors = 'red';

    export const styles = stylex.create({
      wrapper: { padding: spacing, color: colors, margin: sp.small, backgroundColor: c.bg },
    });
  "#
);
