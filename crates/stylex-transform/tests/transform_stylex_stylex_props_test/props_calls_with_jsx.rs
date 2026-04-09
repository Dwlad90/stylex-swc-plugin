use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  local_static_styles,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
  r#"
import stylex from 'stylex';
const styles = stylex.create({
  red: {
    color: 'red',
  }
});
function Foo() {
  return (
    <>
      <div id="test" {...stylex.props(styles.red)}>Hello World</div>
      <div className="test" {...stylex.props(styles.red)} id="test">Hello World</div>
      <div id="test" {...stylex.props(styles.red)} className="test">Hello World</div>
    </>
  );
}
  "#
);

stylex_test!(
  local_dynamic_styles,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      },
      opacity: (opacity) => ({
        opacity
      })
    });
    function Foo() {
      return (
        <div id="test" {...stylex.props(styles.red, styles.opacity(1))}>
          Hello World
        </div>
      );
    }
  "#
);

stylex_test!(
  non_local_styles,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
  r#"
import stylex from 'stylex';
const styles = stylex.create({
  red: {
    color: 'red',
  }
});
function Foo(props) {
  return (
    <div id="test" {...stylex.props(props.style, styles.red)}>
      Hello World
    </div>
  );
}
  "#
);
