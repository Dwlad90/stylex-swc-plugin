use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
        .with_dev(true)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  stylex_call_produces_dev_class_names,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  // dev:true
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
        default: {
            color: 'red',
        },
    });
    stylex.attrs(styles.default);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_conditions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  // dev:true and genConditionalClasses:true
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
    stylex.attrs([styles.default, isActive && otherStyles.default]);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_conditions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
    stylex.attrs([styles.default, isActive && otherStyles.default]);
  "#
);

stylex_test!(
  stylex_call_produces_dev_class_name_with_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
    stylex.attrs([styles.default, isActive && styles.active]);
  "#
);
