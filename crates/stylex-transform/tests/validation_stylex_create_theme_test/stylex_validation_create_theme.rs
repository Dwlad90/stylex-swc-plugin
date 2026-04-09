use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

stylex_test_panic!(
  must_be_bound_to_a_variable,
  "createTheme() calls must be bound to a bare variable.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            stylex.createTheme({__varGroupHash__: 'x568ih9'}, {});
        "#
);

stylex_test_panic!(
  it_must_have_two_arguments_no_args,
  "createTheme() should have 1 argument",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme();
        "#
);

stylex_test_panic!(
  it_must_have_two_arguments_one_args,
  "createTheme() should have 1 argument.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme({});
        "#
);

stylex_test_panic!(
  it_must_have_two_arguments_fn_args,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(genStyles(),{});
        "#
);

stylex_test_panic!(
  it_must_have_two_arguments_empty_object_args,
  "Can only override variables theme created with defineVars().",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme({},{});
        "#
);

stylex_test!(
  it_must_have_two_arguments_valid,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
      .with_runtime_injection()
      .into_pass()
  },
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {}
        );
    "#
);

stylex_test_panic!(
  variable_keys_must_be_a_static_value,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {[labelColor]: 'red',}
            );
        "#
);

stylex_test!(
  values_must_be_static_number_or_string_in_stylex_create_theme_v1,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
      .with_runtime_injection()
      .into_pass()
  },
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', cornerRadius: 'var(--cornerRadiusHash)'},
            {cornerRadius: 5,}
        );
    "#
);

stylex_test!(
  values_must_be_static_number_or_string_in_stylex_create_theme_v2,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
      .with_runtime_injection()
      .into_pass()
  },
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: 'red',}
        );
    "#
);

stylex_test_panic!(
  values_must_be_static_number_or_string_in_stylex_create_theme_var,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor,}
            );
        "#
);

stylex_test_panic!(
  values_must_be_static_number_or_string_in_stylex_create_theme_fn,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor(),}
            );
        "#
);

stylex_test_panic!(
  second_arg_cant_be_imported_variable_in_stylex_create_theme_fn,
  "createTheme() can only accept an object as the second argument",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';
            import { buttonTokens } from "./ButtonTokens";

            export const variables = stylex.createTheme(buttonTokens, buttonTokens);
        "#
);

stylex_test!(
  second_arg_can_be_local_variable_in_stylex_create_theme_fn,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
  r#"
            import stylex from 'stylex';

            const buttonTokens ={
                __varGroupHash__: 'TestTheme.stylex.js//buttonTheme',
                bgColor: 'var(--xgck17p)',
            };

            const simpleTheme = {
                bgColor: {
                    default: {
                        default: 'green',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.3 -0.4)',
                    },
                    '@media (prefers-color-scheme: dark)': {
                        default: 'lightgreen',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.2 -0.4)',
                    },
                },
            }

            export const variables = stylex.createTheme(buttonTokens, simpleTheme);
        "#
);
