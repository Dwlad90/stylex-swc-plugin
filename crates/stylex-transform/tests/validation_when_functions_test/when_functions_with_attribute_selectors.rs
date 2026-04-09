use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  when_ancestor_with_attribute_selector,
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [when.ancestor('[data-panel-state="open"]')]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);

stylex_test!(
  when_descendant_with_attribute_selector,
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [when.descendant('[data-panel-state="open"]')]: 'green',
        },
      },
    });

    console.log(styles.container);
  "#
);
