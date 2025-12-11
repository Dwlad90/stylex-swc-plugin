use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptions, StyleXOptionsParams},
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  empty_stylex_props_call,
  r#"
        import stylex from 'stylex';
        stylex.props();
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
  basic_stylex_call,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.props(styles.red);
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
  stylex_call_with_number,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            0: {
                color: 'red',
            },
            1: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles[0], styles[1]]);
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
  stylex_call_with_computed_number,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            [0]: {
                color: 'red',
            },
            [1]: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles[0], styles[1]]);
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
  stylex_call_with_computed_string,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            'default': {
                color: 'red',
            }
        });
        stylex.props(styles['default']);
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
  stylex_call_with_multiple_namespaces,
  r#"
        import {create, props} from 'stylex';
        const styles = create({
            default: {
                color: 'red',
            },
        });
        const otherStyles = create({
            default: {
                backgroundColor: 'blue',
            }
        });
        props([styles.default, otherStyles.default]);
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
  stylex_call_within_variable_declarations,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export const a = function() {
            return stylex.props(styles.foo);
        }
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
  stylex_call_with_styles_variable_assignment,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                color: 'red',
            },
            bar: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles.foo, styles.bar]);
        export const foo = styles;
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
  stylex_call_within_export_declarations,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export default function MyExportDefault() {
            return stylex.props(styles.foo);
        }
        export function MyExport() {
            return stylex.props(styles.foo);
        }
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
  stylex_call_with_short_form_properties,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.props(styles.foo);
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
  stylex_call_with_exported_short_form_properties,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.props([styles.foo]);
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
    export const styles = stylex.create({
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
    export const result = stylex.props(styles.default, styles.override);
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
  stylex_call_using_styles_with_pseudo_selectors,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
        default: {
            color: 'red',
            ':hover': {
                color: 'blue',
            }
        }
        });
        stylex.props(styles.default);
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
  stylex_call_using_styles_with_pseudo_selectors_within_property,
  r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: {
                    default: 'red',
                    ':hover': 'blue',
                }
            }
        });
        stylex.props(styles.default);
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
  stylex_call_using_styles_with_media_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@media (min-width: 1000px)': {
                    backgroundColor: 'blue',
                },
                '@media (min-width: 2000px)': {
                    backgroundColor: 'purple',
                },
            },
        });
        stylex.props(styles.default);
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
  stylex_call_using_styles_with_media_queries_within_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    default:'red',
                    '@media (min-width: 1000px)': 'blue',
                    '@media (min-width: 2000px)': 'purple',
                },
            },
        });
        stylex.props(styles.default);
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
  stylex_call_using_styles_with_support_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@supports (hover: hover)': {
                    backgroundColor: 'blue',
                },
                '@supports not (hover: hover)': {
                    backgroundColor: 'purple',
                },
            },
        });
        stylex.props(styles.default);
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
  stylex_call_using_styles_with_support_queries_within_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    default:'red',
                    '@supports (hover: hover)': 'blue',
                    '@supports not (hover: hover)': 'purple',
                },
            },
        });
        stylex.props(styles.default);
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
  stylex_call_with_spread_operator,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            },
            blue: {
                backgroundColor: 'blue',
            },
            green: {
                color: 'green',
            }
        });
        stylex.props(...[styles.red, styles.blue,...[styles.green]]);
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
  stylex_call_with_spread_operator_of_variable,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            },
            blue: {
                backgroundColor: 'blue',
            },
            green: {
                color: 'green',
            }
        });

        const stylesArr = [styles.red, styles.blue,...[styles.green]]

        stylex.props(...stylesArr);
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
  transform_props_with_conditional_array,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      base: {
        backgroundColor: 'blue',
      },
      active: {
        color: 'red',
      },
      inactive: {
        color: 'green',
      },
    });

    export function Props_With_Conditional_Array (status)  {
      const isActive = status === 'active';

      return <button {...stylex.props(styles.base, ...isActive ? [styles.active]: [styles.inactive])} />
    };
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
  transform_props_with_regex,
  r#"
  import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      base: {
        backgroundColor: 'blue',
      },
      active: {
        color: 'red',
      },
      inactive: {
        color: 'green',
      },
    });

    export function Props_With_Conditional_Array (status)  {
      const isActive = /status/.test(status);

      return <>
      <button {...stylex.props(styles.base, ...isActive ? [styles.active]: [styles.inactive])} />
      {isActive ? <div {...stylex.props(styles.active)}>Active</div> : <div {...stylex.props(styles.inactive)}>Inactive</div>}
      <div {...stylex.props(isActive && styles.active)}>Active</div>
      </>
    };
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
    Some(&mut StyleXOptionsParams {
      style_resolution: Some(StyleResolution::ApplicationOrder),
      dev: Some(true),
      treeshake_compensation: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      enable_minified_keys: Some(false),
      enable_debug_class_names: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transform_props_with_null,
  r#"
  import * as stylex from '@stylexjs/stylex';
  import { useState } from 'react';

  const styles = stylex.create({
    base: {
      backgroundColor: 'blue',
    },
    active: {
      right: 0,
    },
    inactive: {
      left: 0,
    },
    answered: {
      right: 10,
    },
    unanswered: {
      left: 10,
    },
  });

  export function Props_With_Null(isActive, isInactive,items) {
  const isAnswered = items[isActive] !== null;
  const [isFirst, setIsFirst] = useState(false);

    return <>
  <button {...stylex.props(
        styles.base,
        ...isFirst === true ? [ styles.active] : [],
        ...isFirst === true ? [styles.answered, styles.active] : [styles.base],
        isAnswered ? styles.answered : null,
        isAnswered ? styles.answered : isInactive ? styles.inactive : null,
        isAnswered ? styles.answered : styles.unanswered
      )} />
    <button {...stylex.props(
        styles.base,
        ...isFirst === true ? [ styles.active] : [],
      )}
      >Active</button>
      <button {...stylex.props(
        styles.base,
        ...isFirst === true ? [ styles.active] : [],
      )}
      >Inactive</button>
      <button {...stylex.props(
        styles.base,
        ...isFirst === true ? [ styles.active] : [],
      )}
      >Answered</button>
      <button {...stylex.props(
        styles.base,
        ...isFirst === true ? [ styles.active] : [],
      )}
      >Unanswered</button>
      </>
  };
"#
);
