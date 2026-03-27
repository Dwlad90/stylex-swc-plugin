#[cfg(test)]
mod class_methods {
  use swc_core::ecma::ast::Expr;

  use crate::shared::transformers::stylex_types::FN_MAP;
  use stylex_ast::ast::factories::{create_object_expression, create_string_key_value_prop};
  use stylex_enums::value_with_default::ValueWithDefault;

  fn assert_type_factory(syntax: &str, value: ValueWithDefault, expected_value: &str) {
    let angle = FN_MAP.get(syntax).unwrap();

    let result = angle(value);

    assert_eq!(result, type_factory(syntax, expected_value));
  }

  fn type_factory(syntax: &str, value: &str) -> Expr {
    create_object_expression(vec![
      create_string_key_value_prop("syntax", format!("<{}>", syntax).as_str()),
      create_string_key_value_prop("value", value),
    ])
  }

  #[test]
  fn angle() {
    assert_type_factory(
      "angle",
      ValueWithDefault::String("45deg".to_string()),
      "45deg",
    );
  }

  #[test]
  fn color() {
    assert_type_factory("color", ValueWithDefault::String("red".to_string()), "red");
  }

  #[test]
  fn image() {
    assert_type_factory(
      "image",
      ValueWithDefault::String("url(#image)".to_string()),
      "url(#image)",
    );
  }

  #[test]
  fn integer() {
    assert_type_factory("integer", ValueWithDefault::Number(1.0), "1");
  }

  #[test]
  fn length() {
    let value = "1px";

    assert_type_factory("length", ValueWithDefault::String(value.to_string()), value);

    assert_type_factory("length", ValueWithDefault::Number(1.0), value);
  }

  #[test]
  fn percentage() {
    let value = "50%";

    assert_type_factory(
      "percentage",
      ValueWithDefault::String(value.to_string()),
      value,
    );

    assert_type_factory("percentage", ValueWithDefault::Number(0.5), value);
  }

  #[test]
  fn number() {
    assert_type_factory("number", ValueWithDefault::Number(1.0), "1");
  }

  #[test]
  fn resolution() {
    let value = "96dpi";

    assert_type_factory(
      "resolution",
      ValueWithDefault::String(value.to_string()),
      value,
    );
  }

  #[test]
  fn time() {
    let value = "1s";

    assert_type_factory("time", ValueWithDefault::String(value.to_string()), value);
  }

  #[test]
  fn transform_function() {
    let value = "translateX(10px)";

    assert_type_factory(
      "transformFunction",
      ValueWithDefault::String(value.to_string()),
      value,
    );
  }

  #[test]
  fn transform_list() {
    let value = "translateX(10px)";

    assert_type_factory(
      "transformList",
      ValueWithDefault::String(value.to_string()),
      value,
    );
  }

  #[test]
  fn url() {
    let value = "url(#image)";

    assert_type_factory("url", ValueWithDefault::String(value.to_string()), value);
  }
}
