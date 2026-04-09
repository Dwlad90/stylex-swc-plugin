use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      ))),
    )
  })
}

stylex_test!(
  export_named_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
