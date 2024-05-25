#[cfg(test)]
mod class_methods {
  use swc_core::ecma::ast::Expr;

  use crate::shared::{
    enums::data_structures::value_with_default::ValueWithDefault,
    transformers::stylex_types::FN_MAP,
    utils::ast::factories::{object_expression_factory, prop_or_spread_string_factory},
  };

  fn assert_type_factory(syntax: &str, value: ValueWithDefault, expected_value: &str) {
    let angle = FN_MAP.get(syntax).unwrap();

    let result = angle(value);

    assert_eq!(result, type_factory(syntax, expected_value));
  }

  fn type_factory(syntax: &str, value: &str) -> Expr {
    object_expression_factory(vec![
      prop_or_spread_string_factory("syntax", format!("<{}>", syntax).as_str()),
      prop_or_spread_string_factory("value", value),
    ])
    .unwrap()
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
