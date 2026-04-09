use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  when_ancestor_function,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(StyleXOptions::get_haste_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  when_sibling_before_function,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(StyleXOptions::get_haste_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  when_functions_namespace_imports,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(StyleXOptions::get_haste_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  when_functions_aliased_imports,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(StyleXOptions::get_haste_module_resolution(None))
    .with_runtime_injection()
    .into_pass(),
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
