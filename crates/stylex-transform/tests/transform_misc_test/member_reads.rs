use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_dev(false)))
}

// A private name is a class field, so it names no style namespace. The style
// variable it is read off still survives, because the read gives the compiler
// no namespace to narrow it to.
stylex_test!(
  a_private_name_read_names_no_namespace,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_treeshake_compensation(true)
    .with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ root: { color: 'red' } });

    export class Card {
      #root = 1;

      read() {
        return styles.#root;
      }
    }
  "#
);

// The same read as an argument of a merge. A private name names no namespace,
// so the argument carries no compiled style and the merge stays at runtime.
stylex_test!(
  a_private_name_argument_stays_at_runtime,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ root: { color: 'red' } });

    export class Card {
      #root = 1;

      read() {
        return stylex.props(styles.#root);
      }
    }
  "#
);

// Only `&&` names a style behind a condition. Any other operator gives the
// merge a value it cannot read, so the whole call stays at runtime.
stylex_test!(
  an_argument_behind_another_operator_stays_at_runtime,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ root: { color: 'red' } });

    export const props = (flag) => stylex.props(flag || styles.root);
  "#
);

// Each condition doubles the set of answers the merge has to write out, so past
// four of them the call is left for the runtime to merge.
stylex_test!(
  more_than_four_conditions_stay_at_runtime,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      a: { color: 'red' },
      b: { margin: '1px' },
      c: { padding: '1px' },
      d: { opacity: 0.5 },
      e: { zIndex: 1 },
    });

    export const props = (p, q, r, s, t) => stylex.props(
      p && styles.a, q && styles.b, r && styles.c, s && styles.d, t && styles.e
    );
  "#
);

// Four conditions still fold: the merge writes one answer per combination and
// the runtime reads the one its conditions name.
stylex_test!(
  four_conditions_still_fold,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      a: { color: 'red' },
      b: { margin: '1px' },
      c: { padding: '1px' },
      d: { opacity: 0.5 },
    });

    export const props = (p, q, r, s) => stylex.props(
      p && styles.a, q && styles.b, r && styles.c, s && styles.d
    );
  "#
);

// An argument that is not a style at all, and a namespace the style variable
// does not carry. Neither names a compiled style, so the merge keeps the call.
stylex_test!(
  an_argument_that_names_no_style_stays_at_runtime,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ root: { color: 'red' } });

    export const text = stylex.props('a string');
    export const guarded = stylex.props(1 && styles.root);
    export const missing = stylex.props(styles.notAStyle);
  "#
);

// A method is not a namespace: it declares no styles the compiler can read, so
// the call is refused rather than compiled into styles the author never wrote.
stylex_test_panic!(
  a_method_is_not_a_namespace,
  "Unsupported object method.",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ root() { return 1 } });
  "#
);

// A dynamic style is an arrow that answers a style object. A body written as a
// block holds statements the compiler cannot fold.
stylex_test_panic!(
  a_dynamic_style_written_as_a_block_is_refused,
  "Block statement is not allowed in Dynamic Style functions",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ root: (color) => { return { color } } });
  "#
);

// A method inside a namespace declares no value the compiler can read, so the
// call is refused rather than compiled into a namespace missing a declaration.
stylex_test_panic!(
  a_method_inside_a_namespace_is_refused,
  "Unsupported object method.",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ root: { color() { return 'red' } } });
  "#
);
