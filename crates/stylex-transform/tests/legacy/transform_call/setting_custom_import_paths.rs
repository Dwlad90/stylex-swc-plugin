use crate::utils::prelude::*;
use stylex_structures::named_import_source::NamedImportSource;

stylex_test!(
  basic_stylex_call,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_import_sources(vec![ImportSources::Regular(
        "custom-stylex-path".to_string(),
      )])
      .with_runtime_injection()
  }),
  r#"
    import stylex from 'custom-stylex-path';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    stylex(styles.red);
  "#
);

stylex_test!(
  named_import_from_custom_source,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_import_sources(vec![ImportSources::Named(NamedImportSource {
        from: "custom-stylex-path".to_string(),
        r#as: "css".to_string(),
      })])
      .with_runtime_injection()
  }),
  r#"
    import {css as stylex} from 'custom-stylex-path';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    stylex(styles.red);
  "#
);

stylex_test!(
  named_import_with_other_name_from_custom_source,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_import_sources(vec![ImportSources::Named(NamedImportSource {
        from: "custom-stylex-path".to_string(),
        r#as: "css".to_string(),
      })])
      .with_runtime_injection()
  }),
  r#"
    import {css} from 'custom-stylex-path';
    const styles = css.create({
      red: {
        color: 'red',
      }
    });
    css(styles.red);
  "#
);
