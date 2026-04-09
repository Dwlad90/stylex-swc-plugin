use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  stylex_metadata_is_correctly_set,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
