use crate::utils::prelude::*;

fn file_transform(
  comments: std::rc::Rc<swc_core::common::comments::SingleThreadedComments>,
) -> impl swc_core::ecma::ast::Pass {
  build_test_transform(comments, |b| {
    b.with_pass(PluginPass::test_default())
      .with_runtime_injection()
  })
}

stylex_test_panic!(
  invalid_export_not_bound,
  "The return value of defineConsts() must be bound to a named export.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
  "#
);

stylex_test_panic!(
  invalid_export_not_bound_unbound,
  "defineConsts() calls must be bound to a bare variable.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.defineConsts({});
  "#
);

stylex_test_panic!(
  invalid_argument_none,
  "defineConsts() should have 1 argument.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts();
  "#
);

stylex_test_panic!(
  invalid_argument_too_many,
  "defineConsts() should have 1 argument.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({}, {});
  "#
);

stylex_test_panic!(
  invalid_argument_number,
  "defineConsts() can only accept an object.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts(1);
  "#
);

stylex_test_panic!(
  invalid_argument_string,
  "defineConsts() can only accept an object.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts('1');
  "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts(genStyles());
  "#
);

stylex_test!(
  valid_argument_object,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({});
  "#
);

stylex_test!(
  valid_export_separate_const_and_export_statement,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
    export { constants };
  "#
);

stylex_test_panic!(
  invalid_export_re_export_from_another_file_does_not_count,
  "The return value of defineConsts() must be bound to a named export.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
    export { constants } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_renamed_re_export_from_another_file_does_not_count,
  "The return value of defineConsts() must be bound to a named export.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
    export { constants as otherConstants } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_default_export_does_not_count,
  "The return value of defineConsts() must be bound to a named export.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
    export default constants;
  "#
);

stylex_test_panic!(
  invalid_export_renamed_export_with_as_syntax,
  "The return value of defineConsts() must be bound to a named export.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const constants = stylex.defineConsts({});
    export { constants as themeConstants };
  "#
);

/* Properties */

stylex_test!(
  valid_key_starts_with_double_dash,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      '--small': '8px'
    });
  "#
);

stylex_test_panic!(
  invalid_key_non_static,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      [labelColor]: 'red',
    });
  "#
);

/* Values */

stylex_test_panic!(
  invalid_value_non_static_variable,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      labelColor: labelColor,
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_function_call,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      labelColor: labelColor(),
    });
  "#
);

stylex_test!(
  valid_value_number,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      small: 5,
    });
  "#
);

stylex_test!(
  valid_value_string,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      small: '5px',
    });
  "#
);

stylex_test_transform!(
  #[ignore],
  valid_value_keyframes,
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      fadeIn: stylex.keyframes({
        '0%': { opacity: 0 },
        '100%': { opacity: 1}
      }),
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      fadeIn: stylex.keyframes({
        '0%': { opacity: 0 },
        '100%': { opacity: 1}
      }),
    });
  "#
);

// A theme reference read as a constant's value. Refused by both compilers, and
// here by the check that owns the position rather than by the value arm -- a
// constant is inlined at its use sites, so nothing it holds may be a reference
// to resolve later. Recorded as
// `modules-1266-a-theme-object-as-a-define-consts-value`.
stylex_test_panic!(
  a_theme_reference_read_as_a_constant_value_is_refused,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| theme_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const constants = stylex.defineConsts({ a: zIndex });
  "#
);

// A folded function map read as a constant's value. The reference implementation
// folds the same reference to a plain object and accepts it, emitting nothing
// for the constant; refused here, for the reason every shape with no value form
// is refused in this position. A decided divergence, measured here rather than
// in the parity corpus: `defineConsts` hashes the file that declares it, and
// every corpus subject is handed the same filename, so such a subject refuses
// for the filename before the value under test is read.
stylex_test_panic!(
  a_folded_function_map_read_as_a_constant_value_is_refused,
  "Only static values are allowed inside of a defineConsts() call.",
  |tr| file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const constants = stylex.defineConsts({ a: stylex });
  "#
);
