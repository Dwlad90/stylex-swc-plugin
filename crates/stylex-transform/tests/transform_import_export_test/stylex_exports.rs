use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  export_named_property,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
        export {styles}
      "#
);

stylex_test!(
  export_named_declaration,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
      "#
);

stylex_test!(
  export_default,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export default (stylex.create({
          root: {
            color: 'red',
          }
        }));
      "#
);

stylex_test!(
  module_export,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
        module.export = styles;
      "#
);
