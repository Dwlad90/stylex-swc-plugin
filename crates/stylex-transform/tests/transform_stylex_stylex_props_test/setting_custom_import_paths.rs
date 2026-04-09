use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_import_sources(vec![ImportSources::Regular(
        "custom-stylex-path".to_string(),
      )])
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection(),
    )
  })
}

stylex_test!(
  basic_stylex_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import stylex from 'custom-stylex-path';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.props(styles.red);
"#
);
