use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  stylex_metadata_is_correctly_set,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
              color: 'red',
              height: 5,
              ':hover': {
                  start: 10,
              },
              '@media (min-width: 1000px)': {
                  end: 5
              }
          },
        });

        export const name = stylex.keyframes({
            from: {
                start: 0,
            },
            to: {
                start: 100,
            }
        });
    "#
);
