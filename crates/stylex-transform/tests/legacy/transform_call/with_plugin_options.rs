use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  crate::legacy::transform_call::legacy_call_transform(comments, customize)
}

stylex_test!(
  stylex_call_produces_dev_class_names_and_enable_inlined_conditional_merge_false,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_enable_debug_class_names(true)
      .with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
  // dev:true
  r#"
    import stylex from 'stylex';

    const styles = stylex.create({
      default: {
        color: 'red',
      },
    });
    stylex(styles.default);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_conditions,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_enable_debug_class_names(true)
      .with_runtime_injection()
  }),
  // dev:true and enable_inlined_conditional_merge:true
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
      },
    });
    const otherStyles = stylex.create({
      default: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles.default, isActive && otherStyles.default);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_conditions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_enable_debug_class_names(true)
      .with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
      },
    });
    const otherStyles = stylex.create({
      default: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles.default, isActive && otherStyles.default);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_enable_debug_class_names(true)
      .with_runtime_injection()
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
      },
      active: {
        color: 'blue',
      }
    });
    stylex(styles.default, isActive && styles.active);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_collisions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_enable_inlined_conditional_merge(false)
      .with_enable_debug_class_names(true)
      .with_runtime_injection()
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
      },
      active: {
        color: 'blue',
      }
    });
    stylex(styles.default, isActive && styles.active);
  "#
);
