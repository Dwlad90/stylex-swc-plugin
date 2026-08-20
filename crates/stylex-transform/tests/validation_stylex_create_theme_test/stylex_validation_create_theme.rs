use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_unstable_module_resolution(ModuleResolution::common_js(None))
        .with_runtime_injection(),
    )
  })
}

stylex_test_panic!(
  must_be_bound_to_a_variable,
  "createTheme() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    stylex.createTheme({__varGroupHash__: 'x568ih9'}, {});
  "#
);

stylex_test_panic!(
  it_must_have_two_arguments_no_args,
  "createTheme() should have 1 argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const variables = stylex.createTheme();
  "#
);

stylex_test_panic!(
  it_must_have_two_arguments_one_args,
  "createTheme() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const variables = stylex.createTheme({});
  "#
);

stylex_test_panic!(
  it_must_have_two_arguments_fn_args,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const variables = stylex.createTheme(genStyles(),{});
  "#
);

stylex_test_panic!(
  it_must_have_two_arguments_empty_object_args,
  "Can only override variables theme created with defineVars().",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const variables = stylex.createTheme({},{});
  "#
);

stylex_test!(
  it_must_have_two_arguments_valid,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import { buttonTokens } from "./ButtonTokens";

    export const variables = stylex.createTheme(buttonTokens, buttonTokens);
  "#
);

stylex_test!(
  second_arg_can_be_local_variable_in_stylex_create_theme_fn,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
// The second-argument check asks whether the identifier is an import, and an
// import is no longer answered for by the name it was aliased away from. So this
// input reaches the check as an ordinary unbound reference: it refused as a
// non-object second argument before, and refuses as a non-static value now.
// Refused either way, and by the arm that describes it -- `buttonTokens` names
// nothing here, which is not the same fault as naming an import.
//
// Measured against `@stylexjs/babel-plugin` 0.19.0 as
// `modules-1266-create-theme-second-arg-named-after-an-aliased-away-import`:
// both compilers refuse.
stylex_test_panic!(
  second_arg_named_after_an_aliased_away_import_is_refused,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import { "buttonTokens" as bt } from "./ButtonTokens";

    export const variables = stylex.createTheme(bt, buttonTokens);
  "#
);

// The control the entry above needs: the *local* binding of that same import is
// still an import, so the second-argument check still refuses it with the
// message it was written for. A lookup that stopped answering for imports
// altogether would pass the case above on its own.
stylex_test_panic!(
  second_arg_bound_by_a_string_named_import_is_refused_as_an_import,
  "createTheme() can only accept an object as the second argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import { "buttonTokens" as bt } from "./ButtonTokens";

    export const variables = stylex.createTheme(bt, bt);
  "#
);
