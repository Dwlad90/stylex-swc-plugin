use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test_transform,
};

#[test]
#[should_panic(expected = "Pseudo selector must start with \":\"")]
fn validates_pseudo_selector_format() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('hover')]: 'red',
          },
        },
      });
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Pseudo selector cannot start with \"::\"")]
fn rejects_pseudo_elements() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('::before')]: 'red',
          },
        },
      });
    "#,
    r#""#,
  )
}
