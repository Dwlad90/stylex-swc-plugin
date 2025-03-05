use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_before_and_after,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::before': {
                    color: 'red'
                },
                '::after': {
                    color: 'blue'
                },
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_placeholder,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::placeholder': {
                    color: 'gray',
                },
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_thumb,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::thumb': {
                    width: 16,
                },
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_pseudo_class_within_a_pseudo_element,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
            '::before': {
              color: {
                default: 'red',
                ':hover': 'blue',
              }
            },
          },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_legacy_pseudo_class_within_a_pseudo_element,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
            '::before': {
              color: 'red',
              ':hover': {
                color: 'blue',
              },
            },
          },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_pseudo_elements_within_legeacy_pseudo_class,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
            '::before': {
              color: 'red',
            },
            ':hover': {
              '::before': {
                color: 'blue',
              },
            },
          },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_pseudo_elements_sandwiched_within_pseudo_classes,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
            '::before': {
              color: 'red',
            },
            ':hover': {
              '::before': {
                color: {
                  default: 'blue',
                  ':hover': 'green',
                  ':active': 'purple',
                },
              },
            },
          },
        });
    "#
);
