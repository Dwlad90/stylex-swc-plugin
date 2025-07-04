use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams::default())
  ),
  basic_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const cls = stylex.viewTransitionClass({
      group: {
        transitionProperty: 'none',
      },
      imagePair: {
        borderRadius: 16,
      },
      old: {
        animationDuration: '0.5s',
      },
      new: {
        animationTimingFunction: 'ease-out',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams::default())
  ),
  local_variables_used_in_view_transition_class,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const animationDuration = '1s';
    export const cls = stylex.viewTransitionClass({
      old: { animationDuration },
      new: { animationDuration },
      group: { animationDuration },
      imagePair: { animationDuration },
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams::default())
  ),
  using_keyframes,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: {opacity: 0},
      to: {opacity: 1},
    });
    export const fadeOut = stylex.keyframes({
      from: {opacity: 1},
      to: {opacity: 0},
    });
    export const cls = stylex.viewTransitionClass({
      old: {
        animationName: fadeOut,
        animationDuration: '1s',
      },
      new: {
        animationName: fadeIn,
        animationDuration: '1s',
      },
    });
  "#
);

#[test]
#[ignore]
fn using_contextual_styles() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut StyleXOptionsParams::default()),
      )
    },
    r#"
      import * as stylex from 'stylex';
      export const cls = stylex.viewTransitionClass({
        group: {
          animationDuration: {
            default: '1s',
            '@media (min-width: 800px)': '2s'
          }
        },
      });
    "#,
    r#""#,
  )
}
