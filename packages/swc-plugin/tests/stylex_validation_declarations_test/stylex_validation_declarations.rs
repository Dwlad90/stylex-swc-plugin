use std::panic;

use stylex_swc_plugin::{
  shared::{constants::messages, structures::plugin_pass::PluginPass},
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

use crate::utils::transform::stringify_js;

#[test]
#[ignore]
fn validation_stylex_invalid_properties() {
  // TODO: Not fully implemented
  let camel_cased = format!(
    r#"
      import stylex from 'stylex';
      const styles = stylex.create({{ x: {{ {}: "{}" }} }});
    "#,
    "animation", "anim 1s"
  );

  let result = panic::catch_unwind(|| {
    stringify_js(
      camel_cased.as_str(),
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      |tr| {
        ModuleTransformVisitor::new_test_styles(
          tr.comments.clone(),
          &PluginPass::default(),
          None,
        )
      },
    );
  });
  assert!(result.is_err());
  let binding = result.unwrap_err();
  let error = binding.downcast_ref::<&str>().unwrap();
  assert_eq!(error, &messages::UNKNOWN_PROP_KEY);
}

#[test]
#[ignore]
fn stylex_invalid_property_values() {
  // TODO: Not fully implemented - originally skipped
  let camel_cased = format!(
    r#"
      import stylex from 'stylex';
      const styles = stylex.create({{ x: {{ {}: "{}" }} }});
    "#,
    "backgroundPosition", "anim 1s"
  );

  let result = panic::catch_unwind(|| {
    stringify_js(
      camel_cased.as_str(),
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      |tr| {
        ModuleTransformVisitor::new_test_styles(
          tr.comments.clone(),
          &PluginPass::default(),
          None,
        )
      },
    );
  });
  assert!(result.is_err());
  let binding = result.unwrap_err();
  let error = binding.downcast_ref::<&str>().unwrap();
  assert_eq!(error, &messages::UNKNOWN_PROP_KEY);
}
