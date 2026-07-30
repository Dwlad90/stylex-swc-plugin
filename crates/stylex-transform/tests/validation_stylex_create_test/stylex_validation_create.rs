use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_pass(PluginPass::test_default())
        .with_runtime_injection(),
    )
  })
}

stylex_test_panic!(
  invalid_use_not_bound,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.create({});
  "#
);

// An unrelated top-level array must not vouch for an unbound call.
stylex_test_panic!(
  invalid_use_not_bound_with_unrelated_top_level_array,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const nums = [1, 2];
    stylex.create({ root: { display: 'flex' } });
  "#
);

// A bound `stylex.create({...}).root` must not vouch for a structurally
// identical but genuinely unbound call elsewhere in the same file.
stylex_test_panic!(
  invalid_use_not_bound_with_identical_member_access_twin,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const root = stylex.create({ root: { display: 'flex' } }).root;
    function f() {
      stylex.create({ root: { display: 'flex' } });
    }
  "#
);

stylex_test_panic!(
  invalid_argument_none,
  "create() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create();
  "#
);

stylex_test_panic!(
  invalid_argument_too_many,
  "create() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({}, {});
  "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "create() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create(genStyles());
  "#
);

stylex_test!(
  valid_argument_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({});
  "#
);

// Type assertions can only be member-accessed through parentheses, and the
// emitter drops that grouping (`(x as any).root` prints as `x as any.root`).
// The call must stay rejected rather than compile to invalid output.
stylex_test_panic!(
  invalid_use_not_bound_through_type_assertion,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const root = (stylex.create({ root: { display: 'flex' } }) as any).root;
  "#
);
