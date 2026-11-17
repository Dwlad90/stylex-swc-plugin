//! A module-level arrow whose body calls `keyframes`, called by name in a
//! style value.
//!
//! The call runs the arrow, so each call declares the rule of the keyframes it
//! builds. Two calls that build the same keyframes declare one rule.

use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| b.with_runtime_injection())
}

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  an_arrow_with_a_parameter_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const mk = (c) => stylex.keyframes({ from: { color: c } });
    export const styles = stylex.create({
      root: { animationName: mk('red') },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  an_arrow_with_no_parameter_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const mk = () => stylex.keyframes({ from: { color: 'red' } });
    export const styles = stylex.create({
      root: { animationName: mk() },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `@keyframes xl7tn2a-B{from{color:blue;}}`,
// `.xsxtzgd{animation-name:xbjjx1r-B}`, `.x4xwbg3{animation-name:xl7tn2a-B}`
stylex_test!(
  two_calls_declare_two_rules,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const mk = (c) => stylex.keyframes({ from: { color: c } });
    export const styles = stylex.create({
      root: { animationName: mk('red') },
      other: { animationName: mk('blue') },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  two_calls_with_one_argument_declare_one_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const mk = (c) => stylex.keyframes({ from: { color: c } });
    export const styles = stylex.create({
      root: { animationName: mk('red') },
      other: { animationName: mk('red') },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  an_arrow_through_a_named_import_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { create, keyframes } from '@stylexjs/stylex';
    const mk = (c) => keyframes({ from: { color: c } });
    export const styles = create({
      root: { animationName: mk('red') },
    });
  "#
);
