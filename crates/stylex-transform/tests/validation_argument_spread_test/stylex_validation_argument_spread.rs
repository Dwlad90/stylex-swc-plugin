//! A spread written in an argument position of a StyleX producer.
//!
//! Every producer reads its arguments through the same two steps: the validator
//! beside it checks how many arguments there are and what shape each one is
//! written in, and then the handler takes the argument out of the list. A
//! spread passes the first step, because the expression a spread carries is the
//! object the validator asks for, and it is refused at the second. So each API
//! is asked here with the same source, and each one must give the same
//! sentence.
//!
//! The spread operand is a literal object on purpose. Spread a name instead and
//! the shape check refuses first, on a different sentence, and nothing reaches
//! the reader under test.
//!
//! The sentence is written out at each case rather than named once.
//! `should_panic(expected = …)` takes a literal, so a shared constant cannot
//! reach it.

use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_pass(PluginPass::test_default())
      .with_runtime_injection()
  })
}

stylex_test_panic!(
  create_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create(...{ root: { color: 'red' } });
  "#
);

stylex_test_panic!(
  keyframes_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes(...{ from: { color: 'red' } });
  "#
);

stylex_test_panic!(
  position_try_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fallback = stylex.positionTry(...{ positionAnchor: '--anchor' });
  "#
);

stylex_test_panic!(
  view_transition_class_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.viewTransitionClass(...{ old: { animationName: 'none' } });
  "#
);

stylex_test_panic!(
  define_vars_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars(...{ color: 'red' });
  "#
);

stylex_test_panic!(
  define_vars_nested_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested(...{ color: 'red' });
  "#
);

stylex_test_panic!(
  define_consts_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const consts = stylex.defineConsts(...{ sm: '@media (max-width: 600px)' });
  "#
);

stylex_test_panic!(
  define_consts_nested_refuses_a_spread_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const consts = stylex.unstable_defineConstsNested(...{ sm: '@media (max-width: 600px)' });
  "#
);

// A theme reads two arguments, so each position is asked on its own. The other
// argument is written plainly in each case, so the sentence names the position
// under test and not the one beside it.
stylex_test_panic!(
  create_theme_refuses_a_spread_first_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.createTheme(...{ __varGroupHash__: 'x568ih9' }, {});
  "#
);

stylex_test_panic!(
  create_theme_refuses_a_spread_second_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.createTheme({ __varGroupHash__: 'x568ih9' }, ...{});
  "#
);

stylex_test_panic!(
  create_theme_nested_refuses_a_spread_first_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.unstable_createThemeNested(...{ __varGroupHash__: 'x568ih9' }, {});
  "#
);

stylex_test_panic!(
  create_theme_nested_refuses_a_spread_second_argument,
  "The spread operator (...) is not supported in this context.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.unstable_createThemeNested({ __varGroupHash__: 'x568ih9' }, ...{});
  "#
);
