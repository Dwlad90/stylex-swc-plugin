use crate::utils::prelude::*;

stylex_test!(
  empty_stylex_props_call,
  r#"
        import stylex from 'stylex';
        stylex.attrs();
    "#
);

stylex_test!(
  basic_stylex_call,
  r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.attrs(styles.red);
    "#
);

stylex_test!(
  basic_stylex_attrs_call,
  r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.attrs(styles.red);
    "#
);

stylex_test!(
  stylex_call_with_number,
  r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            0: {
                color: 'red',
            },
            1: {
                backgroundColor: 'blue',
            }
        });
        stylex.attrs([styles[0], styles[1]]);
    "#
);

stylex_test!(
  stylex_call_with_computed_number,
  r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            [0]: {
                color: 'red',
            },
            [1]: {
                backgroundColor: 'blue',
            }
        });
        stylex.attrs([styles[0], styles[1]]);
    "#
);

stylex_test!(
  stylex_call_with_computed_string,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            'default': {
                color: 'red',
            }
        });
        stylex.attrs(styles['default']);
    "#
);

stylex_test!(
  stylex_call_with_multiple_namespaces,
  r#"
        import {create, attrs} from 'stylex';
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
        attrs([styles.default, otherStyles.default]);
    "#
);

stylex_test!(
  stylex_call_within_variable_declarations,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export const a = function() {
            return stylex.attrs(styles.foo);
        }
    "#
);

stylex_test!(
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
        stylex.attrs([styles.foo, styles.bar]);
        export const foo = styles;
    "#
);

stylex_test!(
  stylex_call_within_export_declarations,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export default function MyExportDefault() {
            return stylex.attrs(styles.foo);
        }
        export function MyExport() {
            return stylex.attrs(styles.foo);
        }
    "#
);

stylex_test!(
  stylex_call_with_short_form_properties,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.attrs(styles.foo);
    "#
);

stylex_test!(
  stylex_call_with_exported_short_form_properties,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.attrs([styles.foo]);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

stylex_test!(
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
        stylex.attrs(styles.default);
    "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();

    env.insert(
      "primaryColor".to_string(),
      EnvEntry::Expr(create_string_expr("#00ffaa")),
    );

    StyleXTransform::test(tr.comments.clone())
      .with_options(&mut env_config(env))
      .with_runtime_injection()
      .into_pass()
  },
  stylex_attrs_named_import_resolves_in_inline_objects,
  r#"
    import { attrs, create, env } from 'stylex';
    const styles = create({
      red: {
        color: env.primaryColor,
      }
    });
    attrs(styles.red);
  "#
);
