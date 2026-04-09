use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  basic_stylex_call,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_import_sources(vec![ImportSources::Regular(
        "custom-stylex-path".to_string(),
      )])
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
