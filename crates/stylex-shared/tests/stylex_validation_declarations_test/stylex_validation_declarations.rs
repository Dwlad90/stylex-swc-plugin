use std::panic;

use stylex_shared::{
  StyleXTransform,
  shared::{constants::messages, structures::plugin_pass::PluginPass},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

use crate::utils::transform::stringify_js;

#[test]
#[ignore]
fn validation_stylex_invalid_properties() {
  let border_value = "1px solid red";
  let invalid_property_declarations = [
    ("animation", "anim 1s"),
    ("background", "red"),
    ("border", border_value),
    ("borderBlock", border_value),
    ("borderBlockEnd", border_value),
    ("borderBlockStart", border_value),
    ("borderBottom", border_value),
    ("borderImage", "url(./img.jpg) 30 space"),
    ("borderInline", border_value),
    ("borderInlineEnd", border_value),
    ("borderInlineStart", border_value),
    ("borderLeft", border_value),
    ("borderRight", border_value),
    ("borderTop", border_value),
    ("flexFlow", "row wrap"),
    ("font", "16px/16 Arial"),
    ("listStyle", "square inside"),
    ("textDecoration", "1px solid underline"),
    ("transition", "opacity 1s"),
  ];

  for (prop, value) in invalid_property_declarations {
    let code = format!(
      r#"
        import stylex from 'stylex';
        const styles = stylex.create({{ x: {{ {}: "{}" }} }});
      "#,
      prop, value
    );

    let result = panic::catch_unwind(|| {
      stringify_js(
        code.as_str(),
        Syntax::Typescript(TsSyntax {
          tsx: true,
          ..Default::default()
        }),
        |tr| {
          StyleXTransform::new_test_force_runtime_injection_with_pass(
            tr.comments.clone(),
            PluginPass::default(),
            None,
          )
        },
      );
    });
    assert!(
      result.is_err(),
      "Test for invalid property '{}' should fail",
      prop
    );
    let binding = result.unwrap_err();
    let error = binding.downcast_ref::<&str>().unwrap();
    assert_eq!(
      error,
      &messages::UNKNOWN_PROP_KEY,
      "Property '{}' should trigger UNKNOWN_PROP_KEY error",
      prop
    );
  }
}

#[test]
#[ignore]
fn stylex_invalid_property_values() {
  let multi_length = "1px 2px";

  let mut invalid_value_declarations = Vec::new();

  // No !important
  invalid_value_declarations.push(("display", "block !important"));

  // No multi-value short-forms
  let invalid_shortform_value_declarations = [
    ("backgroundPosition", "top left"),
    ("borderColor", "red blue"),
    ("borderRadius", multi_length),
    ("borderStyle", "solid dashed"),
    ("borderWidth", multi_length),
    ("inset", multi_length),
    ("insetBlock", multi_length),
    ("insetInline", multi_length),
    ("flex", "1 1 0"),
    ("grid", "1 1 0"),
    ("margin", multi_length),
    ("marginBlock", multi_length),
    ("marginInline", multi_length),
    ("outline", "1px solid red"),
    ("overflow", "hidden visible"),
    ("padding", multi_length),
    ("paddingBlock", multi_length),
    ("paddingInline", multi_length),
  ];

  invalid_value_declarations.extend_from_slice(&invalid_shortform_value_declarations);

  // No CPU intensive property transitions
  let invalid_transition_property_values = [
    "all",
    "bottom",
    "end",
    "height",
    "inset",
    "inset-block",
    "inset-inline",
    "inset-block-end",
    "inset-block-start",
    "inset-inline-end",
    "inset-inline-start",
    "margin",
    "left",
    "padding",
    "right",
    "start",
    "top",
    "width",
  ];

  for transition_value in invalid_transition_property_values {
    invalid_value_declarations.push(("transitionProperty", transition_value));
  }

  for (prop, value) in invalid_value_declarations {
    let code = format!(
      r#"
        import stylex from 'stylex';
        const styles = stylex.create({{ x: {{ {}: "{}" }} }});
      "#,
      prop, value
    );

    let result = panic::catch_unwind(|| {
      stringify_js(
        code.as_str(),
        Syntax::Typescript(TsSyntax {
          tsx: true,
          ..Default::default()
        }),
        |tr| {
          StyleXTransform::new_test_force_runtime_injection_with_pass(
            tr.comments.clone(),
            PluginPass::default(),
            None,
          )
        },
      );
    });
    assert!(
      result.is_err(),
      "Test for invalid value '{}' for property '{}' should fail",
      value,
      prop
    );
    let binding = result.unwrap_err();
    let error = binding.downcast_ref::<&str>().unwrap();
    assert_eq!(
      error,
      &messages::ILLEGAL_PROP_VALUE,
      "Value '{}' for property '{}' should trigger ILLEGAL_PROP_VALUE error",
      value,
      prop
    );
  }
}
