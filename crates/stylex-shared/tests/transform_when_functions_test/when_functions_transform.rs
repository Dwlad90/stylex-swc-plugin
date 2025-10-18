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
  when_ancestor_function,
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor(':hover')]: 'red',
          },
        },
    });

    console.log(styles.container);
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
  when_sibling_before_function,
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [when.siblingBefore(':focus')]: 'red',
        },
      },
    });

    console.log(styles.container);
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
  when_functions_namespace_imports,
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        backgroundColor: {
          default: 'blue',
          [stylex.when.ancestor(':hover')]: 'red',
          [stylex.when.siblingBefore(':focus')]: 'green',
          [stylex.when.anySibling(':active')]: 'yellow',
          [stylex.when.siblingAfter(':focus')]: 'purple',
          [stylex.when.descendant(':focus')]: 'orange',
        },
      },
    });

    console.log(styles.container);
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
  when_functions_aliased_imports,
  r#"
    import { when as w, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [w.ancestor(':hover')]: 'red',
          [w.siblingBefore(':focus')]: 'green',
        },
      },
    });

    console.log(styles.container);
  "#
);
