//! A `props` call that is given a plain object beside the compiled styles.
//!
//! The merge keeps such an object as the inline style the runtime applies, so
//! the call answers a `style` property holding the declarations the author
//! wrote, under the names they wrote them with.
//!
//! The last case reads the same object through `attrs`, where the answer is
//! the CSS text a `style` attribute holds. The two spellings of one name are
//! the reason the cases sit together: only the contrast shows that each result
//! names the declaration the way its own reader expects.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

stylex_test!(
  inline_style_on_its_own,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: 'blue' });
  "#
);

stylex_test!(
  inline_style_beside_a_compiled_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export default stylex.props(styles.red, { color: 'blue' });
  "#
);

stylex_test!(
  inline_style_holding_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({});
  "#
);

// A value the compiler cannot read leaves the whole call to the runtime.
stylex_test!(
  inline_style_whose_value_is_not_static,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const withColor = (color) => stylex.props({ color });
  "#
);

// A `style` property is read by the runtime, not by CSS, so each name keeps
// the spelling the author wrote. A custom property already spells its own.
stylex_test!(
  inline_style_keeps_the_name_the_author_wrote,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      backgroundColor: 'blue',
      '--myColor': 'red',
    });
  "#
);

// A later inline style wins over an earlier one, declaration by declaration,
// the way the merge orders compiled styles.
stylex_test!(
  a_later_inline_style_wins_over_an_earlier_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props(
      { color: 'blue', margin: '1px' },
      { color: 'green' },
    );
  "#
);

// A declaration set to null declares nothing, so the merge writes no `style`
// property at all.
stylex_test!(
  inline_style_holding_only_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: null });
  "#
);

// A condition is answered by one object per outcome, and only the outcome
// that holds the inline style carries the `style` property.
stylex_test!(
  inline_style_under_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export const maybe = (flag) => stylex.props(styles.red, flag && { color: 'blue' });
  "#
);

// The same object read as attributes is text, because an attribute is text.
stylex_test!(
  inline_style_read_as_attributes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export default stylex.attrs(styles.red, { backgroundColor: 'blue' });
  "#
);
