use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  stylex_call_produces_dev_class_names,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
