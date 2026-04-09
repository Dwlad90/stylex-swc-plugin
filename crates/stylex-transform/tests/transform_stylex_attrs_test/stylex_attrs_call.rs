use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

stylex_test!(
  empty_stylex_props_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import stylex from 'stylex';
        stylex.attrs();
    "#
);

stylex_test!(
  basic_stylex_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_attrs_named_import_resolves_in_inline_objects,
  |tr| {
    let mut env = IndexMap::new();

    env.insert(
      "primaryColor".to_string(),
      EnvEntry::Expr(create_string_expr("#00ffaa")),
    );

    stylex_transform(tr.comments.clone(), |b| b.with_env(env))
  },
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
